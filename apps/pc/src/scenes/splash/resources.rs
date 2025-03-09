use bevy::prelude::*;

#[derive(Resource)]
pub struct SplashState {
    pub background_timer: Timer,
    pub text_timer: Timer,
    pub bgm_timer: Timer,
    pub ui_timer: Timer,
}

impl Default for SplashState {
    fn default() -> Self {
        Self {
            background_timer: Timer::from_seconds(0.0, TimerMode::Once),
            text_timer: Timer::from_seconds(1.0, TimerMode::Once),
            bgm_timer: Timer::from_seconds(0.5, TimerMode::Once),
            ui_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

impl SplashState {
    pub fn reset(&mut self) {
        self.background_timer.reset();
        self.text_timer.reset();
        self.bgm_timer.reset();
        self.ui_timer.reset();
    }
}
