use super::super::MapNoise;
use bevy::{math::Vec2, utils::HashMap};
use rand::Rng;

use super::{Lake, River, Waterfall};

/// 水系分布系统
///
/// 负责生成和管理游戏中的水系，包括河流、湖泊和瀑布
/// 设计原则：
/// 1. 真实感：水系遵循地形高度流向低处
/// 2. 多样性：支持不同规模和形状的水系
/// 3. 性能优化：使用缓存避免重复计算
#[derive(Debug, Clone)]
pub struct WaterManager {
    /// 河流生成参数
    pub river_params: River,
    /// 湖泊生成参数
    pub lake_params: Lake,
    /// 瀑布生成参数
    pub waterfall_params: Waterfall,
    /// 种子
    pub seed: u32,
    /// 水系缓存
    water_cache: HashMap<(i32, i32), bool>,
}

impl Default for WaterManager {
    fn default() -> Self {
        Self {
            river_params: River::default(),
            lake_params: Lake::default(),
            waterfall_params: Waterfall::default(),
            seed: 0,
            water_cache: HashMap::new(),
        }
    }
}

impl WaterManager {
    /// 创建武侠风格的水系配置
    pub fn wuxia_style() -> Self {
        Self {
            river_params: River {
                min_width: 2,
                max_width: 5,
                meandering: 0.4,
                branch_probability: 0.2,
                max_branches: 3,
            },
            lake_params: Lake {
                frequency: 0.08,
                min_size: 8,
                max_size: 20,
                depth_variation: 0.3,
                shore_complexity: 0.5,
            },
            waterfall_params: Waterfall {
                position: Vec2::ZERO,
                min_height: 1.5,
                max_height: 8.0,
                min_slope: 0.7,
                flow_strength: 1.5,
                splash_range: 3.0,
            },
            seed: 12345,
            water_cache: HashMap::new(),
        }
    }

    /// 初始化水系系统
    pub fn initialize(&mut self, seed: u32) {
        self.seed = seed;
        self.water_cache.clear();
    }

    /// 检查指定位置是否有水
    pub fn has_water_at(&self, x: i32, y: i32) -> bool {
        // 查询缓存
        if let Some(has_water) = self.water_cache.get(&(x, y)) {
            return *has_water;
        }

        // TODO: 实际水系检测逻辑
        // 当前简化实现，使用噪声函数模拟水系分布
        let noise = MapNoise::new(self.seed, 0.02, 0.0);
        let nx = (x as f32) * 0.02;
        let ny = (y as f32) * 0.02;
        let value = noise.get(nx, ny);

        // 水系判定，低洼处更可能有水
        let has_water = value < -0.4;

        // 更新缓存
        let mut cache_map = self.water_cache.clone();
        cache_map.insert((x, y), has_water);

        has_water
    }

    /// 生成河流
    pub fn generate_river(&self, start: Vec2, height_map: &[f32], chunk_size: i32) -> Vec<Vec2> {
        let mut path = Vec::new();
        let mut current = start;
        let mut rng = rand::thread_rng();

        // 设置最大路径长度，防止无限循环
        let max_length = 100;

        // 随机选择河道宽度
        // let river_width = rng.gen_range(self.river_params.min_width..=self.river_params.max_width);

        // 生成主河道
        while path.len() < max_length {
            path.push(current);

            // 查找最陡峭的下坡方向作为流向
            let flow_angle = self.calculate_flow_angle(current, height_map, chunk_size);

            // 添加蜿蜒度
            let meandering_offset =
                rng.gen_range(-self.river_params.meandering..=self.river_params.meandering);

            // 计算下一个位置
            current += Vec2::from_angle(flow_angle + meandering_offset) * 1.0;

            // 检查是否到达区块边界或不合适的地形
            if !self.is_valid_river_point(current, height_map, chunk_size) {
                break;
            }

            // 尝试生成分支
            if rng.gen::<f32>() < self.river_params.branch_probability {
                let branches_count = rng.gen_range(1..=self.river_params.max_branches);
                for _ in 0..branches_count {
                    let branch_path = self.generate_branch(current, height_map, chunk_size);
                    path.extend(branch_path);
                }
            }
        }

        path
    }

    /// 生成湖泊
    pub fn generate_lake(&self, center: Vec2, height_map: &[f32], chunk_size: i32) -> Vec<Vec2> {
        let mut lake_points = Vec::new();
        let mut rng = rand::thread_rng();

        // 根据中心点高度调整湖泊大小
        let center_height = self.get_height_at(center, height_map, chunk_size);
        let is_suitable_for_lake = center_height < 0.4; // 湖泊倾向于低洼处

        // 如果位置不适合生成湖泊，缩小生成尺寸
        let size_factor = if is_suitable_for_lake { 1.0 } else { 0.6 };
        let size = rng.gen_range(
            (self.lake_params.min_size as f32 * size_factor) as i32
                ..=(self.lake_params.max_size as f32 * size_factor) as i32,
        );

        // 使用噪声创建不规则的湖岸线
        let noise = MapNoise::new(rand::random(), 0.02, 0.0);
        let scale = self.lake_params.shore_complexity;

        for x in -size..=size {
            for y in -size..=size {
                let point = center + Vec2::new(x as f32, y as f32);

                // 确保点在区块内且有效
                if point.x < 0.0
                    || point.x >= chunk_size as f32
                    || point.y < 0.0
                    || point.y >= chunk_size as f32
                {
                    continue;
                }

                // 获取该点高度
                let point_height = self.get_height_at(point, height_map, chunk_size);

                // 高度检查：湖泊只应形成在低洼地带
                if point_height > center_height + self.lake_params.depth_variation {
                    continue;
                }

                let distance = point.distance(center);

                // 使用噪声和高度图创建不规则的湖岸线
                let noise_val =
                    noise.get(point.x as f32 * scale as f32, point.y as f32 * scale as f32) as f32;

                // 结合高度因素，较高的地方湖泊边界会缩小
                let height_factor = 1.0
                    - ((point_height - center_height) / self.lake_params.depth_variation).max(0.0);
                let effective_size = size as f32
                    * (1.0 + noise_val * self.lake_params.shore_complexity)
                    * height_factor;

                if distance < effective_size {
                    lake_points.push(point);
                }
            }
        }

        lake_points
    }

    /// 生成瀑布
    pub fn generate_waterfall(
        &self,
        position: Vec2,
        height_map: &[f32],
        chunk_size: i32,
    ) -> Option<Waterfall> {
        // 检查坡度是否足够形成瀑布
        let slope = self.calculate_slope(position, height_map, chunk_size);
        if slope < self.waterfall_params.min_slope {
            return None;
        }

        // 计算瀑布高度
        let height_diff = self.calculate_height_difference(position, height_map, chunk_size);
        if height_diff < self.waterfall_params.min_height {
            return None;
        }

        let height = (height_diff).min(self.waterfall_params.max_height);

        // 根据高度调整水流强度
        let flow_strength =
            self.waterfall_params.flow_strength * (height / self.waterfall_params.max_height);

        // 创建瀑布对象
        Some(Waterfall {
            position,
            min_height: self.waterfall_params.min_height,
            max_height: self.waterfall_params.max_height,
            min_slope: self.waterfall_params.min_slope,
            flow_strength,
            splash_range: self.waterfall_params.splash_range,
        })
    }

    /// 计算流向角度
    fn calculate_flow_angle(&self, pos: Vec2, height_map: &[f32], chunk_size: i32) -> f32 {
        let current_height = self.get_height_at(pos, height_map, chunk_size);
        let mut min_height = current_height;
        let mut flow_dir = Vec2::new(0.0, 1.0); // 默认向下流

        // 检查8个方向
        for angle in (0..8).map(|i| i as f32 * std::f32::consts::PI / 4.0) {
            let offset = Vec2::from_angle(angle);
            let check_pos = pos + offset;

            // 边界检查
            if check_pos.x < 0.0
                || check_pos.x >= chunk_size as f32
                || check_pos.y < 0.0
                || check_pos.y >= chunk_size as f32
            {
                continue;
            }

            let height = self.get_height_at(check_pos, height_map, chunk_size);

            // 寻找高度最低的方向
            if height < min_height {
                min_height = height;
                flow_dir = offset;
            }
        }

        // 返回流向角度
        flow_dir.to_angle()
    }

    /// 生成河流分支
    fn generate_branch(&self, start: Vec2, height_map: &[f32], chunk_size: i32) -> Vec<Vec2> {
        let mut path = Vec::new();
        let mut current = start;
        let mut rng = rand::thread_rng();

        // 分支长度比主河流短
        let max_length = (self.river_params.max_width * 3) as usize;

        // 为分支选择一个随机偏移方向
        let branch_angle = rng.gen_range(0.0..std::f32::consts::TAU);

        // 河流分支一般宽度较小
        let branch_width =
            rng.gen_range(self.river_params.min_width..=self.river_params.max_width / 2) as f32;

        // 根据分支宽度调整参数
        let step_size = 0.5 + branch_width / 5.0; // 宽度影响每步移动距离
        let branch_length = (max_length as f32
            * (0.5 + branch_width / (self.river_params.max_width as f32)))
            as usize; // 宽度影响分支长度

        // 生成分支路径
        while path.len() < branch_length
            && self.is_valid_river_point(current, height_map, chunk_size)
        {
            path.push(current);

            // 计算下一个点，与主河流逻辑类似但分支蜿蜒度更高
            let flow_angle = self.calculate_flow_angle(current, height_map, chunk_size);

            // 蜿蜒度与宽度成反比 - 窄的河流更蜿蜒
            let meandering_factor = self.river_params.meandering
                * (1.0
                    + (self.river_params.max_width as f32 - branch_width)
                        / (self.river_params.max_width as f32));

            let meandering_offset = rng.gen_range(-meandering_factor..=meandering_factor);

            // 结合基础流向和分支角度
            let combined_angle = flow_angle * 0.7 + branch_angle * 0.3 + meandering_offset;
            current += Vec2::from_angle(combined_angle) * step_size;
        }

        path
    }

    /// 计算高度差
    fn calculate_height_difference(
        &self,
        position: Vec2,
        height_map: &[f32],
        chunk_size: i32,
    ) -> f32 {
        // 获取当前位置高度
        let current_height = self.get_height_at(position, height_map, chunk_size);

        // 计算流向角度
        let flow_angle = self.calculate_flow_angle(position, height_map, chunk_size);

        // 获取下游位置（沿着流向的位置）
        let downstream_pos = position + Vec2::from_angle(flow_angle) * 2.0;
        let downstream_height = self.get_height_at(downstream_pos, height_map, chunk_size);

        // 返回高度差（如果结果为负，说明是上坡，返回0）
        (current_height - downstream_height).max(0.0)
    }

    /// 计算坡度
    fn calculate_slope(&self, position: Vec2, height_map: &[f32], chunk_size: i32) -> f32 {
        let current_height = self.get_height_at(position, height_map, chunk_size);
        let mut max_slope: f32 = 0.0;

        // 检查8个方向的高度差
        for angle in (0..8).map(|i| i as f32 * std::f32::consts::PI / 4.0) {
            let offset = Vec2::from_angle(angle);
            let next_pos = position + offset;
            let next_height = self.get_height_at(next_pos, height_map, chunk_size);

            // 计算高度差
            let height_diff = (current_height - next_height).abs();

            // 计算坡度 (高度差/水平距离)
            let slope: f32 = height_diff / offset.length();
            max_slope = max_slope.max(slope);
        }

        // 明确指定浮点数类型为f32
        (max_slope / 2.0f32).min(1.0f32)
    }

    /// 获取高度
    fn get_height_at(&self, pos: Vec2, height_map: &[f32], chunk_size: i32) -> f32 {
        let x = pos.x as i32;
        let y = pos.y as i32;

        // 检查是否在区块范围内
        if !self.is_valid_river_point(pos, height_map, chunk_size) {
            return 0.0;
        }

        // 计算在高度图中的索引
        let index = y * chunk_size + x;

        // 确保索引有效
        if index >= 0 && (index as usize) < height_map.len() {
            height_map[index as usize]
        } else {
            0.0
        }
    }

    /// 检查点是否在有效范围内
    fn is_valid_river_point(&self, pos: Vec2, height_map: &[f32], chunk_size: i32) -> bool {
        let x = pos.x as i32;
        let y = pos.y as i32;

        // 检查坐标是否在区块范围内
        if x < 0 || x >= chunk_size || y < 0 || y >= chunk_size {
            return false;
        }

        // 检查索引是否在高度图范围内
        let index = y * chunk_size + x;
        if index < 0 || (index as usize) >= height_map.len() {
            return false;
        }

        // 检查高度是否适合河流（避免在高山和太低的地方生成河流）
        let height = height_map[index as usize];
        let min_river_height = 0.1; // 最低水位
        let max_river_height = 0.8; // 最高水位（避免山顶）

        height >= min_river_height && height <= max_river_height
    }
}
