use crate::scenes::SplashBackground;
use bevy::prelude::*;

pub fn update_camera_scroll(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SplashBackground>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 50.0 * time.delta_secs();

        // 循环滚动
        if transform.translation.x < -800.0 {
            transform.translation.x = 800.0;
        }
    }
}
