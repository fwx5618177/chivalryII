/// 气候系统参数
///
/// # 设计思路
/// 1. 分层设计：将气候系统分为基础参数、季节影响和环境因素三层
/// 2. 可配置性：所有关键参数都可以调整，方便平衡游戏体验
/// 3. 真实性：模拟真实世界的气候规律，提供沉浸感
///
/// # 参数说明
/// - temperature_scale: 控制温度变化的幅度，影响游戏难度
/// - moisture_scale: 控制湿度变化的幅度，影响植被和天气
/// - altitude_temperature_factor: 模拟高度对温度的影响，创造地形多样性
/// - latitude_factors: 模拟纬度对气候的影响，创造区域特色
#[derive(Debug, Clone)]
pub struct ClimateParams {
    /// 温度缩放
    pub temperature_scale: f32,
    /// 温度偏移
    pub temperature_offset: f32,
    /// 湿度缩放
    pub moisture_scale: f32,
    /// 湿度偏移
    pub moisture_offset: f32,
    /// 海拔温度影响系数
    pub altitude_temperature_factor: f32,
    /// 纬度温度影响系数
    pub latitude_temperature_factor: f32,
    /// 纬度湿度影响系数
    pub latitude_moisture_factor: f32,
}

impl Default for ClimateParams {
    fn default() -> Self {
        Self {
            temperature_scale: 1.0,
            temperature_offset: 0.0,
            moisture_scale: 1.0,
            moisture_offset: 0.0,
            altitude_temperature_factor: 0.5,
            latitude_temperature_factor: 0.3,
            latitude_moisture_factor: 0.2,
        }
    }
}
