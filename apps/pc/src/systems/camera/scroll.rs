use crate::scenes::SplashBackground;
use bevy::prelude::*;

pub fn update_camera_scroll(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SplashBackground>>,
) {
    for mut transform in query.iter_mut() {
        // 向左滚动
        transform.translation.x -= 30.0 * time.delta_secs();

        // 当图片滚动到一半时，立即重置位置
        // 因为图片宽度是屏幕的2倍，所以滚动到一半时视觉上是连续的
        if transform.translation.x <= -960.0 {
            // 屏幕宽度的一半
            transform.translation.x = 0.0;
        }
    }
}
