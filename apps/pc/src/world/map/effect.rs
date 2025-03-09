/// 效果类型
#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Buff,          // 增益效果
    Debuff,        // 减益效果
    Environmental, // 环境效果
    Quest,         // 任务相关
    Combat,        // 战斗相关
    Special,       // 特殊效果
    None,          // 无效果
}

/// 效果触发条件
#[derive(Debug, Clone, Copy)]
pub enum EffectTrigger {
    OnEnter, // 进入区域
    OnExit,  // 离开区域
    OnStay,  // 在区域内持续
    OnTime,  // 定时触发
    OnEvent, // 事件触发
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeType {
    None,    // 无效属性
    Health,  // 生命值
    Mana,    // 魔法值
    Stamina, // 耐力值
    Attack,  // 攻击力
    Defense, // 防御力
    Speed,   // 移动速度
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierType {
    Add,      // 加法
    Multiply, // 乘法
    Divide,   // 除法
    Subtract, // 减法
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonType {
    Equal,        // 等于
    Greater,      // 大于
    Less,         // 小于
    GreaterEqual, // 大于等于
    LessEqual,    // 小于等于
}

/// 属性修改器
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeModifier {
    /// 属性类型
    pub attribute: AttributeType,
    /// 修改方式
    pub modifier: ModifierType,
    /// 修改值
    pub value: f32,
}

impl Default for AttributeModifier {
    fn default() -> Self {
        Self {
            attribute: AttributeType::None,
            modifier: ModifierType::Add,
            value: 0.0,
        }
    }
}
