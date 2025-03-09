use bevy::prelude::*;

use super::{SplashBackground, SplashText};

pub fn debug_entities(
    splash_query: Query<Entity, With<SplashBackground>>,
    text_query: Query<Entity, With<SplashText>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    info!("Splash entities: {:?}", splash_query.iter().count());
    info!("Text entities: {:?}", text_query.iter().count());
    info!("Camera entities: {:?}", camera_query.iter().count());
}
