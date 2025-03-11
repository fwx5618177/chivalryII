use bevy::prelude::*;

#[derive(Component)]
pub struct SplashBackground;

#[derive(Component)]
pub struct BackgroundFade {
    pub fade_timer: Timer,
    pub is_fading: bool,
}

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

#[derive(Component)]
pub struct TextFadeIn {
    pub timer: Timer,
    pub delay_timer: Timer,
    pub is_fading: bool,
}
