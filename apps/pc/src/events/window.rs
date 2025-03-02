use bevy::prelude::*;
use bevy::window::{WindowMode, MonitorSelection};

pub fn handle_window_events(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
    let mut window = windows.single_mut();

    // Alt+Enter 或 Alt+Tab 切换全屏
    if keyboard.just_pressed(KeyCode::Tab) || keyboard.just_pressed(KeyCode::Enter) {
        if keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                _ => WindowMode::Windowed,
            };
        }
    }
    
    // Esc 切换到窗口模式
    if keyboard.just_pressed(KeyCode::Escape) {
        window.mode = WindowMode::Windowed;
    }

    // Alt+F4 关闭窗口
    if keyboard.just_pressed(KeyCode::F4) && 
       keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}