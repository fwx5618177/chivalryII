use bevy::prelude::*;

#[derive(Component)]
pub struct SplashBackground;

#[derive(Component)]
pub struct SplashText;

#[derive(Component)]
pub struct SplashUI;

// 动画状态
#[derive(Component, Default)]
pub struct TextAnimationState {
    pub timer: Timer,
    pub current_length: usize,
    pub target_text: String,
}

#[derive(Component)]
pub struct UISlideState {
    pub initial_position: Vec2,
    pub target_position: Vec2,
    pub timer: Timer,
}
