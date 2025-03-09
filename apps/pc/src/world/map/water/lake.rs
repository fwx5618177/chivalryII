#[derive(Debug, Clone)]
pub struct Lake {
    /// 湖泊出现频率
    pub frequency: f32,
    /// 最小湖泊尺寸
    pub min_size: i32,
    /// 最大湖泊尺寸
    pub max_size: i32,
    /// 深度变化
    pub depth_variation: f32,
    /// 岸线复杂度
    pub shore_complexity: f32,
}

impl Default for Lake {
    fn default() -> Self {
        Self {
            frequency: 0.05,
            min_size: 5,
            max_size: 15,
            depth_variation: 0.2,
            shore_complexity: 0.3,
        }
    }
}
