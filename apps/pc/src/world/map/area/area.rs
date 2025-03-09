use super::super::{
    effect::{AttributeModifier, EffectTrigger, EffectType},
    environment::EnvironmentRequirements,
};

/// 区域配置
#[derive(Debug, Clone)]
pub struct Area {
    /// 生成权重
    pub weight: f32,
    /// 占用面积范围
    pub size_range: (f32, f32),
    /// 环境要求
    pub environment_requirements: EnvironmentRequirements,
    /// 特殊效果
    pub effects: Vec<AreaEffect>,
}

impl Default for Area {
    fn default() -> Self {
        Self {
            weight: 1.0,
            size_range: (1.0, 10.0),
            environment_requirements: EnvironmentRequirements::default(),
            effects: Vec::new(),
        }
    }
}

/// 区域效果
///
/// # 设计思路
/// 1. 定义区域的特殊效果
/// 2. 控制效果的触发条件
/// 3. 管理效果的持续时间
#[derive(Debug, Clone)]
pub struct AreaEffect {
    /// 效果类型
    pub effect_type: EffectType,
    /// 效果范围
    pub range: f32,
    /// 效果强度
    pub intensity: f32,
    /// 持续时间
    pub duration: Option<f32>,
    /// 触发条件
    pub trigger_conditions: Vec<EffectTrigger>,
    /// 影响的属性
    pub affected_attributes: Vec<AttributeModifier>,
}

impl Default for AreaEffect {
    fn default() -> Self {
        Self {
            effect_type: EffectType::None,
            range: 0.0,
            intensity: 0.0,
            duration: None,
            trigger_conditions: Vec::new(),
            affected_attributes: Vec::new(),
        }
    }
}
