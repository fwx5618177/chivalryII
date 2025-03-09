use bevy::math::Rect;

/// 世界基础配置
///
/// # 设计思路
/// 1. 可配置性：关键参数可自由调整，适应不同游戏需求
/// 2. 边界管理：可选的世界边界，支持无限世界和有限世界
/// 3. 性能优化：通过区块大小控制内存使用和加载性能
///
/// # 参数说明
/// - seed: 世界种子，决定地形生成的随机性但保证可重复性
/// - world_bounds: 可选的世界边界，None表示无限世界
/// - chunk_size: 区块大小，影响加载性能和内存使用
///
/// # 使用场景
/// 1. 开放世界：设置较大的世界边界或无边界
/// 2. 竞技场景：设置较小的固定边界
/// 3. 任务地图：自定义大小的特定区域
#[derive(Debug, Clone)]
pub struct WorldConfig {
    /// 世界种子
    /// 用于生成一致的随机地形和特征
    /// 相同的种子将生成相同的世界
    pub seed: u64,

    /// 世界大小限制
    /// None表示无限世界
    /// Some包含具体的边界矩形
    pub world_bounds: Option<Rect>,

    /// 区块大小
    /// 影响以下方面：
    /// 1. 内存使用：更大的区块意味着更少的区块数量
    /// 2. 加载性能：更小的区块可以更快地加载
    /// 3. 渲染效率：影响视距和LOD系统
    pub chunk_size: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            world_bounds: None,
            chunk_size: 32,
        }
    }
}

impl WorldConfig {
    /// 创建新的世界配置
    pub fn new(seed: u64, world_bounds: Option<Rect>, chunk_size: i32) -> Self {
        Self {
            seed,
            world_bounds,
            chunk_size,
        }
    }

    /// 获取默认配置
    pub fn default_wuxia() -> Self {
        Self {
            seed: 42,
            world_bounds: Some(Rect::new(-1000.0, -1000.0, 1000.0, 1000.0)),
            chunk_size: 32,
        }
    }
}
