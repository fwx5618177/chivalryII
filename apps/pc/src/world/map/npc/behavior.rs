/// NPC行为模式
#[derive(Debug, Clone)]
pub struct Behavior {
    /// 警戒范围
    pub alert_range: f32,
    /// 互动范围
    pub interaction_range: f32,
}
