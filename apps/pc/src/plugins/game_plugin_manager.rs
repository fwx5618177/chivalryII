use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::config::GameSettings;
use crate::resources::{GameState, GlobalGameState, InputState};
use crate::events::{input::*, window::*, network::*};
use super::logging_plugin::LoggingPlugin;

use super::core_game_plugin::CoreGamePlugin;

pub struct GamePluginManager;

impl GamePluginManager {
    pub fn run(settings: &GameSettings) {
        let mut app = App::new();
        
        // 添加基础插件组
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: settings.window.title.clone(),
                resolution: (
                    settings.window.width as f32,
                    settings.window.height as f32
                ).into(),
                present_mode: if settings.window.vsync {
                    bevy::window::PresentMode::AutoVsync
                } else {
                    bevy::window::PresentMode::AutoNoVsync
                },
                mode: if settings.window.fullscreen {
                    WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                } else {
                    WindowMode::Windowed
                },
                resizable: true,
                ..default()
            }),
            ..default()
        }));

        // 添加状态
        app.init_state::<GameState>();

        // 添加资源
        app.init_resource::<GlobalGameState>()
        .init_resource::<InputState>()
        .init_resource::<NetworkState>()
        .init_resource::<KeyBindings>();

        //  添加事件
        app.add_event::<NetworkEvent>();

        // 添加事件处理系统
        app.add_systems(Update, (
            handle_window_events,
            handle_input_events,
            handle_network_events,
        ).chain());

        // 添加游戏核心插件
        app.add_plugins((
            LoggingPlugin::default(),
            CoreGamePlugin,
        ));

        // 设置调试标志
        if settings.graphics.debug_rendering {
            if let Some(mut state) = app.world_mut().get_resource_mut::<GlobalGameState>() {
                state.is_debug = true;
            }
        }

        // 运行游戏
        app.run();
    }
}

