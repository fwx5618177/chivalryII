use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Interact,
    OpenInventory,
    OpenMap,
    ExitGame,
    ZoomIn,
    ZoomOut,
}

#[derive(Debug, Clone, Resource)]
pub struct KeyBindings {
    pub bindings: HashMap<GameAction, KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(GameAction::MoveForward, KeyCode::KeyW);
        bindings.insert(GameAction::MoveBackward, KeyCode::KeyS);
        bindings.insert(GameAction::MoveLeft, KeyCode::KeyA);
        bindings.insert(GameAction::MoveRight, KeyCode::KeyD);
        bindings.insert(GameAction::Jump, KeyCode::Space);
        bindings.insert(GameAction::Attack, KeyCode::KeyF);
        bindings.insert(GameAction::Interact, KeyCode::KeyE);
        bindings.insert(GameAction::OpenInventory, KeyCode::KeyI);
        bindings.insert(GameAction::OpenMap, KeyCode::KeyM);
        bindings.insert(GameAction::ExitGame, KeyCode::Escape);
        bindings.insert(GameAction::ZoomIn, KeyCode::Equal);
        bindings.insert(GameAction::ZoomOut, KeyCode::Minus);
        Self { bindings }
    }
}

pub fn handle_input_events(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut input_state: ResMut<crate::resources::InputState>,
) {
    input_state.previous_actions = input_state.active_actions.clone();
    input_state.active_actions.clear();

    for (action, key_code) in key_bindings.bindings.iter() {
        if keyboard.pressed(*key_code) {
            input_state.active_actions.push(*action);
        }
    }
}
