use super::{Condition, Effect};

/// 任务触发器
///
/// # 设计思路
/// 1. 定义触发条件和效果
/// 2. 控制任务的流程
/// 3. 管理任务状态
#[derive(Debug, Clone)]
pub struct QuestTrigger {
    /// 触发条件
    pub conditions: Vec<Condition>,
    /// 触发效果
    pub effects: Vec<Effect>,
    /// 任务ID
    pub quest_id: String,
    /// 触发优先级
    pub priority: i32,
    /// 触发冷却时间
    pub cooldown: f32,
    /// 是否一次性触发
    pub one_time: bool,
}
