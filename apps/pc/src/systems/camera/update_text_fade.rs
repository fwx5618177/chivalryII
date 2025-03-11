use bevy::{prelude::*, window::PrimaryWindow};

use crate::scenes::{BackgroundFade, SplashBackground, TextFadeIn};

pub fn update_text_fade(time: Res<Time>, mut query: Query<(&mut Text, &mut TextFadeIn)>) {
    for (mut text, mut fade) in query.iter_mut() {
        // 先等待延迟时间
        if !fade.delay_timer.finished() {
            fade.delay_timer.tick(time.delta());
            continue;
        }

        // 延迟结束后开始淡入
        if !fade.is_fading {
            fade.is_fading = true;
        }

        if fade.is_fading && !fade.timer.finished() {
            fade.timer.tick(time.delta());
            let progress = fade.timer.fraction();
        }
    }
}
