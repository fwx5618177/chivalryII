use crate::logging::{GameLogger, LogLevel};
use super::components::*;
use super::resources::*;
use bevy::prelude::*;


pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>, mut logger: ResMut<GameLogger>,) {
    // 初始化状态
    commands.insert_resource(SplashState::default());

    // 背景
    commands.spawn((
        Sprite {
            image: asset_server.load("images/splash_bg.png"),
            ..default()
        },
        SplashBackground,
    ));

    // 相机
    commands.spawn(Camera2d::default());

    // 文本
    commands
        .spawn((
            Text {
                0: "".to_string(),
            },
            Transform::from_xyz(400.0, 300.0, 0.0),
            SplashText,
            TextAnimationState {
                target_text: "Welcome to Game".to_string(),
                timer: Timer::from_seconds(1.5, TimerMode::Once),
                current_length: 0,
                ..default()
            },
        ));
}
