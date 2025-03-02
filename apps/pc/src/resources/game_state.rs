use bevy::prelude::*;

// 游戏运行状态
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading, // 加载中
    MainMenu, // 主菜单
    InGame, // 游戏中
    Paused, // 暂停
}

// 游戏全局状态资源
#[derive(Resource)]
pub struct GlobalGameState {
    // 是否为调试模式
    pub is_debug: bool,
    // 帧计数
    pub frame_count: u64,
    // 游戏时间
    pub game_time: f32,
}

// 游戏全局状态资源默认实现
impl Default for GlobalGameState {
    fn default() -> Self {
        Self {
            is_debug: false,
            frame_count: 0,
            game_time: 0.0,
        }
    }
}
