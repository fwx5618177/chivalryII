use crate::config::GameSettings;

use super::components::*;
use super::resources::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::cosmic_text::rustybuzz::shape;
use bevy::text::FontFamily;
use bevy::{prelude::*, text::FontSmoothing};
use bevy_rapier3d::parry::shape::Shape;
use bincode::de;

pub fn setup_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        SplashBackground,
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
                .with_rotation(Quat::from_rotation_x(-0.1))  // 添加倾斜效果
                .with_scale(Vec3::new(1.2, 1.2, 1.0))
    ));
}
