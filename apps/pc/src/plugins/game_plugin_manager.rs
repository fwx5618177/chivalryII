use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::input::ButtonInput;
use crate::config;
use super::core_game_plugin::CoreGamePlugin;

pub struct GamePluginManager;

impl GamePluginManager {
    pub fn handle_window_events(
        keys: Res<ButtonInput<KeyCode>>,
        mut windows: Query<&mut Window>,
        mut app_exit_events: EventWriter<bevy::app::AppExit>,
    ) {
        let mut window = windows.single_mut();

        // Alt+Tab 切换全屏
        if keys.just_pressed(KeyCode::Tab) && keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                _ => WindowMode::Windowed,
            };
            
        }

        // Esc 切换到窗口模式
        if keys.just_pressed(KeyCode::Escape) {
            window.mode = WindowMode::Windowed;
        }

        // Alt+F4 关闭窗口
        if keys.just_pressed(KeyCode::F4) && keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
            app_exit_events.send(bevy::app::AppExit::Success);
        }
    }

    pub fn run(settings: &config::GameSettings) {
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

        app.add_systems(Update, Self::handle_window_events);

        // 添加游戏核心插件
        app.add_plugins((
            CoreGamePlugin,
            // RenderPlugin,      // 渲染插件
            // NetworkPlugin,     // 网络插件
            // PhysicsPlugin,     // 物理插件
            // UIPlugin,          // UI插件
            // AudioPlugin,       // 音频插件
        ));

        // 运行游戏
        app.run();
    }
}

