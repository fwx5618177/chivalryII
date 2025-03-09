/// 建筑功能
#[derive(Debug, Clone)]
pub enum BuildingType {
    None,
    House,
    Farm,
    Mine,
}

/// 建筑状态
#[derive(Debug, Clone)]
pub struct BuildingArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 建筑配置
///
/// # 设计思路
/// 1. 定义建筑的外观和功能
/// 2. 控制建筑的交互系统
/// 3. 管理建筑的状态变化
#[derive(Debug, Clone)]
pub struct Building {
    /// 建筑类型
    pub building_type: BuildingType,
    /// 建筑占用区域
    pub area: BuildingArea,
}

impl Default for Building {
    fn default() -> Self {
        Self {
            building_type: BuildingType::None,
            area: BuildingArea {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        }
    }
}
