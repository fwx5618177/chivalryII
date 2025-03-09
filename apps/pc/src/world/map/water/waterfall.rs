use bevy::math::Vec2;

#[derive(Debug, Clone)]
pub struct Waterfall {
    // 瀑布位置
    pub position: Vec2,
    /// 最小瀑布高度
    pub min_height: f32,
    /// 最大瀑布高度
    pub max_height: f32,
    /// 最小坡度要求
    pub min_slope: f32,
    /// 水流强度
    pub flow_strength: f32,
    /// 溅水效果范围
    pub splash_range: f32,
}

impl Default for Waterfall {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            min_height: 1.0,
            max_height: 5.0,
            min_slope: 0.6,
            flow_strength: 1.0,
            splash_range: 2.0,
        }
    }
}
