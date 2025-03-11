use bevy::{prelude::*, window::PrimaryWindow};

use crate::scenes::{BackgroundFade, SplashBackground};

pub fn update_background_fade(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut BackgroundFade, &Transform), With<SplashBackground>>,
) {
    for (mut sprite, mut fade, transform) in query.iter_mut() {
        // 当背景即将重置位置时开始淡出
        if transform.translation.x <= -1024.0 && !fade.is_fading {
            fade.is_fading = true;
            fade.fade_timer.reset();
        }

        // 处理淡入淡出
        if fade.is_fading {
            fade.fade_timer.tick(time.delta());
            let progress = fade.fade_timer.fraction();

            // 淡出效果
            if transform.translation.x <= -1024.0 {
                sprite.color.with_alpha(1.0 - progress);
            }
            // 淡入效果
            else {
                sprite.color.with_alpha(progress);
            }

            // 重置淡入淡出状态
            if fade.fade_timer.finished() {
                fade.is_fading = false;
                sprite.color.with_alpha(1.0);
            }
        }
    }
}
