use crate::events::input::GameAction;
use bevy::prelude::*;

// 输入状态资源
#[derive(Resource, Default)]
pub struct InputState {
    // 当前帧的输入动作
    pub active_actions: Vec<GameAction>,
    // 上一帧的输入动作
    pub previous_actions: Vec<GameAction>,
    // 鼠标位置
    pub mouse_position: Vec2,
    // 鼠标移动量
    pub mouse_delta: Vec2,
}

// 输入状态资源方法实现
impl InputState {
    // 检查是否正在执行某个动作
    pub fn is_action_active(&self, action: GameAction) -> bool {
        self.active_actions.contains(&action)
    }

    // 检查是否刚按下某个动作
    pub fn is_action_just_pressed(&self, action: GameAction) -> bool {
        self.active_actions.contains(&action) && !self.previous_actions.contains(&action)
    }

    // 检查是否刚释放某个动作
    pub fn is_action_just_released(&self, action: GameAction) -> bool {
        !self.active_actions.contains(&action) && self.previous_actions.contains(&action)
    }
}
