use crate::scenes::{SplashText, TextAnimationState};
use bevy::prelude::*;

pub fn update_text_animation(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut TextAnimationState), With<SplashText>>,
) {
    for (mut text, mut state) in query.iter_mut() {
        state.timer.tick(time.delta());

        if state.timer.finished() {
            if state.current_length < state.target_text.len() {
                state.current_length += 1;
                text.0 = state
                    .target_text
                    .chars()
                    .take(state.current_length)
                    .collect::<String>();
            }
        }
    }
}
