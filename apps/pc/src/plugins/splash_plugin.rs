use crate::{
    resources::GameState,
    scenes::{debug_entities, setup_splash, SplashState},
    systems::{update_camera_scroll, update_text_animation},
};
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        info!("SplashPlugin initialized.");
        app.init_resource::<SplashState>()
            .add_systems(OnEnter(GameState::Splash), setup_splash)
            .add_systems(
                Update,
                (
                    update_camera_scroll,
                    update_text_animation,
                    // debug_entities
                )
                    .run_if(in_state(GameState::Splash)),
            );
    }
}

impl Default for SplashPlugin {
    fn default() -> Self {
        Self
    }
}
