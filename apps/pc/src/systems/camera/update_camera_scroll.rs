use crate::scenes::SplashBackground;
use bevy::{prelude::*, window::PrimaryWindow};

pub fn update_camera_scroll(
    time: Res<Time>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<SplashBackground>>,
) {
    let scroll_speed = 100.0;
    let bg_width = 2048.0;
    let window = query_window.single();
    let window_width = window.width();
    let frame_move = scroll_speed * time.delta_secs();

    for mut transform in query.iter_mut() {
        // 向左滚动
        transform.translation.x -= frame_move;

        // 关键：在即将出现空白时提前重置
        // 检查点 = 窗口宽度 + 一帧移动距离
        let check_point = -(bg_width - window_width - frame_move);

        if transform.translation.x <= check_point {
            // 屏幕宽度的一半
            let offset = 0.0;
            transform.translation.x = offset;
        }
    }
}
