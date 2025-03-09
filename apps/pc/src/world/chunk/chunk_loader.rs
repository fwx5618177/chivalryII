use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bincode::{deserialize, serialize};
use noise::{NoiseFn, Perlin};

use super::render::apply_2_5d_effect;
use super::{Chunk, ChunkCoord, ChunkData, ChunkLoadState, ChunkManager, Direction, CHUNK_SIZE};
use crate::logging::{GameLogger, LogLevel};
use crate::world::map::{MapManager, MapRules};

/// 区块加载系统
/// 提供区块数据的加载、保存和管理功能
///
/// 设计原则：
/// 1. 异步操作：所有IO操作都是异步的，避免阻塞主线程
/// 2. 资源控制：通过预算系统限制同时进行的操作数量
/// 3. 优先级管理：根据距离和时间动态调整加载顺序
/// 4. 内存优化：自动清理不活跃区块释放内存

/// 计算两个区块坐标之间的曼哈顿距离
///
/// 曼哈顿距离是在网格上从一点到另一点所需的最小步数，只考虑水平和垂直方向。
/// 这种计算方式比欧几里得距离更高效，因为：
/// 1. 避免了平方根运算
/// 2. 更适合网格系统
/// 3. 计算结果为整数，便于后续比较和排序
///
/// # 参数
/// - a: 起始区块坐标
/// - b: 目标区块坐标
///
/// # 返回值
/// 返回两个坐标之间的曼哈顿距离
fn manhattan_distance(a: ChunkCoord, b: ChunkCoord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// 保存区块数据到持久化存储
///
/// 该函数负责将区块数据异步保存到持久化存储中，支持多种存储后端。
/// 使用异步IO避免阻塞主线程，确保游戏主循环的流畅运行。
///
/// # 设计考虑
/// 1. 异步操作：使用tokio的异步文件IO，避免阻塞主线程
/// 2. 错误处理：详细记录保存失败的原因，便于调试
/// 3. 可扩展性：通过抽象存储接口，支持多种存储后端
/// 4. 数据完整性：使用bincode序列化确保数据一致性
///
/// # 参数
/// - coord: 区块坐标，用于生成唯一的存储路径
/// - data: 区块数据，包含区块的所有状态信息
///
/// # 返回值
/// 返回Result类型，成功时返回Ok(()), 失败时返回包含错误信息的Err
///
/// # 示例
/// ```rust
/// let coord = ChunkCoord { x: 1, y: 2 };
/// let data = ChunkData::new();
/// save_chunk_data(coord, &data).await?;
/// ```
async fn save_chunk_data(coord: ChunkCoord, data: &ChunkData) -> Result<(), String> {
    // 1. 序列化数据
    let serialized = bincode::serialize(&data).map_err(|e| format!("序列化区块数据失败: {}", e))?;

    // 2. 构建文件路径
    let path = format!("chunks/{}_{}.dat", coord.x, coord.y);

    // 3. 异步写入文件
    tokio::fs::write(&path, &serialized)
        .await
        .map_err(|e| format!("写入区块文件失败: {}", e))?;

    Ok(())
}

/// 加载区块数据
///
/// 该函数负责从持久化存储中异步加载区块数据，并将其插入到ECS系统中。
/// 使用异步操作避免阻塞主线程，确保游戏主循环的流畅运行。
///
/// # 设计考虑
/// 1. 异步加载：使用tokio的异步文件IO，避免阻塞主线程
/// 2. 数据完整性：使用bincode反序列化确保数据一致性
/// 3. 错误处理：详细记录加载失败的原因，便于调试
/// 4. ECS集成：通过commands将加载的数据插入到ECS系统中
/// 5. 线程安全：使用&mut Commands确保线程安全
///
/// # 参数
/// - coord: 区块坐标，用于定位要加载的区块数据
/// - entity: 区块实体ID，用于将数据关联到正确的实体
/// - commands: ECS命令缓冲区，用于更新实体组件
///
/// # 返回值
/// 返回Result类型，成功时返回Ok(()), 失败时返回包含错误信息的Err
///
/// # 示例
/// ```rust
/// let coord = ChunkCoord { x: 1, y: 2 };
/// let entity = commands.spawn_empty().id();
/// load_chunk_data(coord, entity, &mut commands).await?;
/// ```
async fn load_chunk_data(
    coord: ChunkCoord,
    entity: Entity,
    commands: &mut Commands<'_, '_>,
) -> Result<(), String> {
    // 1. 尝试从文件加载数据
    let path = format!("chunks/{}_{}.dat", coord.x, coord.y);
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("读取区块文件失败: {}", e))?;

    // 2. 反序列化数据
    let chunk_data: ChunkData =
        deserialize(&data).map_err(|e| format!("反序列化区块数据失败: {}", e))?;

    // 3. 更新区块实体

    commands.entity(entity).insert(chunk_data);

    Ok(())
}

/// 区块加载系统 - 实现懒加载机制
///
/// 该函数是区块加载的核心系统，负责管理区块的加载、卸载和优先级计算。
/// 采用懒加载策略，只在需要时加载区块，优化内存使用和性能。
///
/// # 设计考虑
/// 1. 懒加载：只在玩家附近加载区块，减少内存占用
/// 2. 优先级：根据距离和访问时间动态调整加载优先级
/// 3. 内存管理：定期清理不活跃区块，防止内存泄漏
/// 4. 异步操作：通过异步加载避免阻塞主线程
///
/// # 系统流程
/// 1. 更新加载队列优先级
/// 2. 清理不活跃区块
/// 3. 处理加载队列
///
/// # 参数
/// - commands: ECS命令缓冲区，用于创建和销毁实体
/// - time: 时间资源，用于计算区块访问时间
/// - chunk_manager: 区块管理器，维护区块状态
/// - chunk_query: 区块查询，用于访问区块组件
/// - logger: 日志记录器，用于调试信息
pub fn chunk_loading_system(
    mut commands: Commands,
    time: Res<Time>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_query: Query<(Entity, &mut Chunk)>,
    mut logger: Option<ResMut<GameLogger>>,
) {
    let current_time = time.elapsed_secs_f64();

    // 1. 更新加载队列优先级
    update_loading_priorities(&mut chunk_manager, current_time, &mut chunk_query);

    // 2. 内存管理
    if current_time - chunk_manager.last_cleanup > 5.0 {
        // 每5秒进行一次清理
        cleanup_inactive_chunks(
            &mut commands,
            &mut chunk_manager,
            current_time,
            &mut logger,
            &mut chunk_query,
        );
        chunk_manager.last_cleanup = current_time;
    }

    // 3. 处理加载队列
    process_loading_queue(&mut commands, &mut chunk_manager, current_time, &mut logger);
}

/// 更新加载优先级
///
/// 该函数负责动态计算区块的加载优先级，基于以下因素：
/// 1. 与玩家的距离：距离越近优先级越高
/// 2. 等待时间：等待时间越长优先级越高
/// 3. 访问频率：最近访问的区块优先级更高
///
/// # 设计考虑
/// 1. 性能优化：使用曼哈顿距离代替欧几里得距离，减少计算开销
/// 2. 公平性：通过时间加成避免某些区块长期得不到加载
/// 3. 动态调整：根据玩家位置变化实时更新优先级
///
/// # 优先级计算公式
/// 基础优先级 = 1000.0 / (距离 + 1.0)
/// 时间加成 = 1.0 + (等待时间 * 0.1)
/// 最终优先级 = 基础优先级 * 时间加成
///
/// # 参数
/// - chunk_manager: 区块管理器的可变引用，包含加载队列和区块状态
/// - current_time: 当前时间戳，用于计算区块等待时间
/// - chunk_query: 区块查询，用于访问和更新区块组件
fn update_loading_priorities(
    chunk_manager: &mut ChunkManager,
    current_time: f64,
    chunk_query: &mut Query<(Entity, &mut Chunk)>,
) {
    // 根据距离和等待时间更新优先级
    for coord in &chunk_manager.loading_queue {
        // 计算与玩家的曼哈顿距离
        let dx = (coord.x - chunk_manager.player_chunk.unwrap().x).abs();
        let dy = (coord.y - chunk_manager.player_chunk.unwrap().y).abs();
        let distance = (dx * dx + dy * dy) as f32;

        if let Some(&entity) = chunk_manager.chunks.get(coord) {
            if let Ok(mut chunk) = chunk_query.get_mut(entity) {
                // 计算区块等待时间（秒）
                let waiting_time = current_time - chunk.1.last_accessed;

                // 优先级计算公式：
                // 基础优先级：距离越近优先级越高（1000.0 / (distance + 1.0)）
                // 等待时间加成：等待时间越长优先级越高
                // 最终优先级 = 基础优先级 * (1.0 + waiting_time * 0.1)
                let base_priority = 1000.0 / (distance + 1.0);
                let time_bonus = 1.0 + (waiting_time * 0.1) as f32;
                let priority = (base_priority * time_bonus) as i32;

                chunk.1.priority = priority;

                // 更新最后访问时间
                chunk.1.last_accessed = current_time;
            }
        }
    }

    // 根据优先级排序加载队列
    chunk_manager.loading_queue.sort_by_key(|coord| {
        if let Some(&entity) = chunk_manager.chunks.get(coord) {
            if let Ok(chunk) = chunk_query.get(entity) {
                return -chunk.1.priority; // 负号使高优先级排在前面
            }
        }
        0
    });
}

/// 清理不活跃的区块
///
/// 该函数负责清理不再活跃的区块，以释放内存并优化性能。
/// 采用基于时间和空间的策略来决定哪些区块需要被卸载。
///
/// # 设计考虑
/// 1. 内存管理：当区块数量超过内存预算时，清理最不活跃的区块
/// 2. 时间因素：长时间未被访问的区块会被优先清理
/// 3. 空间因素：距离玩家过远的区块会被清理
/// 4. 数据持久化：在清理前保存被修改过的区块数据
///
/// # 清理条件
/// 1. 区块数量超过内存预算
/// 2. 区块超过30秒未被访问
/// 3. 区块距离玩家超过视距的两倍
///
/// # 参数
/// - commands: ECS命令缓冲区，用于销毁实体
/// - chunk_manager: 区块管理器，维护区块状态
/// - current_time: 当前时间戳，用于计算区块访问时间
/// - logger: 日志记录器，用于调试信息
/// - chunk_query: 区块查询，用于访问区块组件
fn cleanup_inactive_chunks(
    commands: &mut Commands,
    chunk_manager: &mut ChunkManager,
    current_time: f64,
    logger: &mut Option<ResMut<GameLogger>>,
    chunk_query: &mut Query<(Entity, &mut Chunk)>,
) {
    let memory_limit = chunk_manager.memory_budget;
    let mut chunks_to_unload = Vec::new();

    // 1. 找出需要卸载的区块
    for (&coord, &entity) in chunk_manager.chunks.iter() {
        if let Ok(chunk) = chunk_query.get(entity) {
            let inactive_time = current_time - chunk.1.last_accessed;
            let distance = manhattan_distance(coord, chunk_manager.player_chunk.unwrap());

            // 卸载条件：
            // 1. 超过内存预算
            // 2. 长时间未访问
            // 3. 距离过远
            if chunk_manager.chunks.len() > memory_limit
                || inactive_time > 30.0  // 30秒未访问
                || distance > chunk_manager.view_distance * 2
            {
                chunks_to_unload.push((coord, entity));
            }
        }
    }

    // 2. 卸载区块
    for (coord, entity) in chunks_to_unload {
        if let Ok(chunk) = chunk_query.get(entity) {
            // 如果区块被修改过，保存数据
            if let Some(data) = &chunk.1.data {
                if data.modified {
                    save_chunk_data(coord, data);
                }
            }
        }

        commands.entity(entity).despawn_recursive();
        chunk_manager.chunks.remove(&coord);

        if let Some(logger) = logger {
            logger.log(
                LogLevel::Debug,
                &format!("卸载不活跃区块: ({}, {})", coord.x, coord.y),
            );
        }
    }
}

/// 处理加载队列
///
/// 该函数负责从加载队列中取出区块并进行加载，同时控制加载的并发量。
/// 使用加载预算来限制同时加载的区块数量，避免资源过度消耗。
///
/// # 设计考虑
/// 1. 并发控制：通过加载预算限制同时加载的区块数量
/// 2. 优先级处理：加载队列已经按优先级排序，优先加载高优先级区块
/// 3. 异步加载：使用异步操作避免阻塞主线程
/// 4. 状态管理：在加载过程中维护区块的加载状态
///
/// # 参数
/// - commands: ECS命令缓冲区，用于创建实体
/// - chunk_manager: 区块管理器，维护加载队列和区块状态
/// - current_time: 当前时间戳，用于记录区块访问时间
/// - logger: 日志记录器，用于调试信息
fn process_loading_queue(
    commands: &mut Commands,
    chunk_manager: &mut ChunkManager,
    current_time: f64,
    logger: &mut Option<ResMut<GameLogger>>,
) {
    let mut loaded_count = 0;

    while loaded_count < chunk_manager.load_budget && !chunk_manager.loading_queue.is_empty() {
        if let Some(coord) = chunk_manager.loading_queue.pop() {
            // 1. 创建新区块实体
            let chunk_entity = commands
                .spawn((
                    Chunk {
                        coord,
                        load_state: ChunkLoadState::Loading,
                        last_accessed: current_time,
                        priority: 0,
                        data: None,
                        entity: None,
                    },
                    SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        coord.x as f32 * chunk_manager.chunk_size,
                        coord.y as f32 * chunk_manager.chunk_size,
                        0.0,
                    ))),
                ))
                .id();

            // 2. 异步加载区块数据
            load_chunk_data(coord, chunk_entity, commands);

            chunk_manager.chunks.insert(coord, chunk_entity);
            loaded_count += 1;

            if let Some(logger) = logger {
                logger.log(
                    LogLevel::Debug,
                    &format!("开始加载区块: ({}, {})", coord.x, coord.y),
                );
            }
        }
    }
}

/// 获取相邻区块信息
/// # 功能
/// - 获取当前区块周围8个相邻区块的信息
/// - 用于地形生成时的边界平滑处理
///
/// # 参数
/// - coord: 当前区块的坐标
///
/// # 返回值
/// - HashMap<Direction, Option<&ChunkData>>: 相邻区块数据映射
fn get_neighbor_chunks(
    coord: ChunkCoord,
    chunk_manager: &ChunkManager,
    chunk_query: &Query<(Entity, &Chunk)>,
) -> HashMap<Direction, Option<ChunkData>> {
    let mut neighbors = HashMap::new();

    // 定义8个方向的偏移
    let directions = [
        (Direction::North, (0, 1)),
        (Direction::South, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::West, (-1, 0)),
        (Direction::NorthEast, (1, 1)),
        (Direction::NorthWest, (-1, 1)),
        (Direction::SouthEast, (1, -1)),
        (Direction::SouthWest, (-1, -1)),
    ];

    // 获取每个方向的区块数据
    for (direction, (dx, dy)) in directions.iter() {
        let neighbor_coord = ChunkCoord {
            x: coord.x + dx,
            y: coord.y + dy,
        };

        let chunk_data = if let Some(&entity) = chunk_manager.chunks.get(&neighbor_coord) {
            if let Ok((_, chunk)) = chunk_query.get(entity) {
                chunk.data.clone()
            } else {
                None
            }
        } else {
            None
        };

        neighbors.insert(*direction, chunk_data);
    }

    neighbors
}

/// 生成区块地形
/// # 功能
/// - 根据地图规则生成区块的具体地形数据
/// - 支持混合生成(预设+程序化)
/// - 处理区块边界的平滑过渡
///
/// # 设计考虑
/// 1. 性能优化：使用缓存减少重复计算
/// 2. 连续性：确保区块间地形平滑过渡
/// 3. 可预测性：使用确定性算法保证重载时地形一致
/// 4. 内存效率：优化数据结构减少内存占用
fn generate_chunk_terrain(
    coord: ChunkCoord,
    map_rules: &MapRules,
    noise: &mut Perlin,
    chunk_manager: &ChunkManager,
    chunk_query: &Query<(Entity, &Chunk)>,
) -> ChunkData {
    let mut chunk_data = ChunkData::default();
    let scale = 0.05; // 噪声缩放因子，控制地形细节程度

    // 1. 获取相邻区块信息，用于边界平滑
    let neighbors = get_neighbor_chunks(coord, chunk_manager, chunk_query);

    // 2. 遍历区块中的每个瓦片
    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            // 计算世界坐标
            let world_x = coord.x * CHUNK_SIZE as i32 + x as i32;
            let world_y = coord.y * CHUNK_SIZE as i32 + y as i32;

            // // 3. 应用预设规则
            // if let Some(tile_type) = map_rules.get_fixed_tile(world_x, world_y) {
            //     chunk_data.set_tile(x, y, tile_type);
            //     continue;
            // }

            // // 4. 获取生物群系信息
            // let biome = map_rules.get_biome(world_x, world_y);

            // 5. 计算多层噪声
            // let base_noise = noise.get([world_x as f64 * scale, world_y as f64 * scale]);
            // let detail_noise =
            //     noise.get([world_x as f64 * scale * 4.0, world_y as f64 * scale * 4.0]) * 0.5;

            // // 6. 合并噪声层
            // let combined_noise = base_noise + detail_noise;

            // // 7. 根据生物群系规则确定地形
            // let tile_type = biome.get_tile_type(combined_noise);

            // // 8. 应用边界平滑
            // let final_type = smooth_boundaries(tile_type, x, y, &chunk_data, &neighbors);

            // // 9. 设置瓦片数据
            // chunk_data.set_tile(x, y, final_type);

            // // 10. 添加装饰物（如树木、石头等）
            // if let Some(decoration) =
            //     biome.try_add_decoration(world_x, world_y, tile_type, &mut rand::thread_rng())
            // {
            //     chunk_data.add_decoration(x, y, decoration);
            // }
        }
    }

    chunk_data
}

/// 平滑区块边界
/// # 功能
/// - 处理区块边界的地形过渡
/// - 确保相邻区块之间地形自然衔接
/// - 避免明显的区块边界
///
/// # 参数
/// - current_type: 当前瓦片类型
/// - x: 瓦片X坐标
/// - y: 瓦片Y坐标
/// - chunk_data: 当前区块数据
/// - neighbors: 相邻区块数据
///
/// # 返回值
/// - 平滑处理后的瓦片类型
fn smooth_boundaries(
    current_type: u8,
    x: usize,
    y: usize,
    chunk_data: &ChunkData,
    neighbors: &HashMap<Direction, Option<ChunkData>>,
) -> u8 {
    // 如果不在边界，直接返回原始类型
    if x > 0 && x < CHUNK_SIZE - 1 && y > 0 && y < CHUNK_SIZE - 1 {
        return current_type;
    }

    let mut surrounding_types = Vec::new();

    // 收集当前区块内相邻瓦片类型
    if x > 0 {
        if let Some(tile) = chunk_data.get_tile(x - 1, y) {
            surrounding_types.push(tile);
        }
    }
    if x < CHUNK_SIZE - 1 {
        if let Some(tile) = chunk_data.get_tile(x + 1, y) {
            surrounding_types.push(tile);
        }
    }
    if y > 0 {
        if let Some(tile) = chunk_data.get_tile(x, y - 1) {
            surrounding_types.push(tile);
        }
    }
    if y < CHUNK_SIZE - 1 {
        if let Some(tile) = chunk_data.get_tile(x, y + 1) {
            surrounding_types.push(tile);
        }
    }

    // 收集相邻区块的瓦片类型
    for (direction, neighbor) in neighbors {
        if let Some(neighbor_data) = neighbor {
            let (nx, ny) = match direction {
                Direction::North if y == CHUNK_SIZE - 1 => (x, 0),
                Direction::South if y == 0 => (x, CHUNK_SIZE - 1),
                Direction::East if x == CHUNK_SIZE - 1 => (0, y),
                Direction::West if x == 0 => (CHUNK_SIZE - 1, y),
                Direction::NorthEast if x == CHUNK_SIZE - 1 && y == CHUNK_SIZE - 1 => (0, 0),
                Direction::NorthWest if x == 0 && y == CHUNK_SIZE - 1 => (CHUNK_SIZE - 1, 0),
                Direction::SouthEast if x == CHUNK_SIZE - 1 && y == 0 => (0, CHUNK_SIZE - 1),
                Direction::SouthWest if x == 0 && y == 0 => (CHUNK_SIZE - 1, CHUNK_SIZE - 1),
                _ => continue,
            };

            if let Some(tile_type) = neighbor_data.get_tile(nx, ny) {
                surrounding_types.push(tile_type);
            }
        }
    }

    // 应用平滑规则
    if !surrounding_types.is_empty() {
        // 使用众数作为平滑后的类型
        let most_common_type = get_most_common_type(&surrounding_types);
        // 如果当前类型与众数差异过大，使用过渡类型
        if (current_type as i32 - most_common_type as i32).abs() > 1 {
            return get_transition_type(current_type, most_common_type);
        }
    }

    current_type
}

/// 获取最常见的地形类型
/// # 功能
/// - 根据周围瓦片类型获取最常见的类型
/// - 用于处理区块边界的地形平滑
/// - 避免明显的地形边界
fn get_most_common_type(types: &[u8]) -> u8 {
    let mut counts = HashMap::new();
    for &t in types {
        *counts.entry(t).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(tile_type, _)| tile_type)
        .unwrap_or(0)
}

/// 获取过渡地形类型
/// # 功能
/// - 根据当前类型和目标类型计算过渡类型
/// - 用于处理区块边界的地形平滑
/// - 避免明显的地形边界
///
/// # 参数
/// - current: 当前瓦片类型
/// - target: 目标瓦片类型
/// - 返回值: 过渡瓦片类型
///
/// # 示例
/// ```rust
/// let current = 1;
/// let target = 2;
/// let transition = get_transition_type(current, target);
/// ```
///
/// # 注意
/// - 过渡类型是当前类型和目标类型的平均值
/// - 用于处理区块边界的地形平滑
/// - 避免明显的地形边界
fn get_transition_type(current: u8, target: u8) -> u8 {
    // 简单的线性插值
    ((current as f32 + target as f32) / 2.0) as u8
}

pub struct ChunkLoader {
    /// 地图规则引用
    map_rules: MapRules,
    /// 噪声生成器
    noise: Perlin,
    /// 区块缓存
    chunk_cache: HashMap<ChunkCoord, ChunkData>,
}

impl ChunkLoader {
    /// 根据地图规则生成区块
    pub fn generate_chunk(&mut self, coord: ChunkCoord) {
        // // 1. 检查是否有预定义场景
        // if let Some(scene) = self.map_rules.get_fixed_scene(coord) {
        //     return self.generate_scene_chunk(coord, scene);
        // }

        // // 2. 获取环境参数
        // let env = self.map_rules.get_environment_at(coord);

        // // 3. 生成地形
        // let terrain = self.generate_terrain(coord, &env);

        // // 4. 添加装饰物
        // let decorations = self.generate_decorations(coord, &terrain, &env);

        // // 5. 创建区块数据
        // ChunkData::default()
    }
}

/// 区块加载系统
///
/// 负责区块的加载、卸载和渲染
#[derive(Default)]
pub struct ChunkLoaderSystem;

impl ChunkLoaderSystem {
    /// 处理区块加载
    pub fn process_chunk_loading(
        mut commands: Commands,
        mut chunk_manager: ResMut<ChunkManager>,
        map_manager: Res<MapManager>,
        time: Res<Time>,
        mut chunks: Query<&mut Chunk>,
    ) {
        // 获取需要加载的区块
        let chunks_to_load = chunk_manager.get_chunks_to_load();

        // 限制每帧加载的区块数量，防止卡顿
        let max_chunks_per_frame = 5;
        let chunks_to_process = chunks_to_load.iter().take(max_chunks_per_frame);

        // 处理区块加载
        for &coord in chunks_to_process {
            // 生成区块数据
            let data = chunk_manager.generate_chunk_data(coord, &map_manager);

            // 创建区块实体
            let chunk_entity = commands
                .spawn((
                    Chunk {
                        coord,
                        load_state: ChunkLoadState::Loaded,
                        data: Some(data),
                        entity: None,
                        last_accessed: time.elapsed_secs_f64(),
                        priority: 0,
                    },
                    Name::new(format!("Chunk ({}, {})", coord.x, coord.y)),
                    Transform::from_xyz(
                        coord.x as f32 * super::CHUNK_SIZE as f32 * 32.0,
                        coord.y as f32 * super::CHUNK_SIZE as f32 * 32.0,
                        0.0,
                    ),
                    GlobalTransform::default(),
                    Visibility::default(),
                ))
                .id();

            // 更新区块实体引用
            if let Ok(mut chunk) = chunks.get_mut(chunk_entity) {
                chunk.entity = Some(chunk_entity);
            }

            // 存储区块实体
            chunk_manager.chunks.insert(coord, chunk_entity);

            // 应用2.5D效果
            if let Ok(chunk) = chunks.get(chunk_entity) {
                apply_2_5d_effect(
                    &chunk,
                    chunk_entity,
                    &map_manager,
                    chunk_manager.render_settings(),
                    &mut commands,
                );
            }
        }

        // 获取需要卸载的区块
        let chunks_to_unload = chunk_manager.get_chunks_to_unload();

        // 处理区块卸载
        for coord in chunks_to_unload {
            if let Some(entity) = chunk_manager.get_chunk(coord) {
                commands.entity(*entity).despawn_recursive();
                chunk_manager.remove_chunk(coord);
            }
        }
    }
}
