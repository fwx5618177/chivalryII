use crate::config::GameSettings;
use crate::prefabs::InputBoxBundle;
use crate::prefabs::InputType;
use crate::prefabs::LoginButtonBundle;
use crate::prefabs::LoginFormBundle;

use super::components::*;
use super::resources::*;
use bevy::{prelude::*, text::FontSmoothing};

pub fn setup_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    // 初始化状态
    commands.insert_resource(SplashState::default());

    // 背景
    commands.spawn((
        Sprite {
            image: asset_server.load("images/splash_bg.png"),
            custom_size: None,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        SplashBackground,
        BackgroundFade {
            fade_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_fading: false,
        },
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("images/splash_bg.png"),
            custom_size: None,
            ..default()
        },
        Transform::from_xyz(2048.0, 0.0, 0.0),
        SplashBackground,
        BackgroundFade {
            fade_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_fading: false,
        },
    ));

    // 相机
    commands.spawn(Camera2d::default());

    // 3D 文字效果
    commands.spawn((
        Text2d::new(settings.splash.title.clone()),
        TextColor::from(Color::WHITE),
        TextFont {
            font: asset_server.load("fonts/PingFang.ttc"),
            font_size: 50.0,
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
        Transform::from_xyz(0.0, 100.0, 1.0)
            .with_rotation(Quat::from_rotation_x(-0.1)) // 添加倾斜效果
            .with_scale(Vec3::new(1.2, 1.2, 1.0)),
        SplashText,
        TextFadeIn {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            delay_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_fading: false,
        },
    ));

    // 登陆表单
    commands
        .spawn(LoginFormBundle::new())
        .with_children(|parent| {
            parent.spawn(InputBoxBundle::new("请输入账号", InputType::Text));
            parent.spawn(InputBoxBundle::new("请输入密码", InputType::Password));

            parent
                .spawn(LoginButtonBundle::new())
                .with_children(|parent| {
                    parent.spawn((
                        Text2d::new("登陆"),
                        TextFont {
                            font: asset_server.load("fonts/PingFang.ttc"),
                            font_size: 20.0,
                            font_smoothing: FontSmoothing::None,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                        TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
                    ));
                });
        });
}
