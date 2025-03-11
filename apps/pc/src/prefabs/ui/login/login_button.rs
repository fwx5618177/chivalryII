use bevy::prelude::*;

#[derive(Component)]
pub struct LoginButton;

#[derive(Bundle)]
pub struct LoginButtonBundle {
    button: LoginButton,
    node_bundle: ButtonBundle,
}

impl LoginButtonBundle {
    pub fn new() -> Self {
        Self {
            button: LoginButton,
            node_bundle: ButtonBundle {
                node: Node {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.2, 0.6, 1.0)),
                ..default()
            },
        }
    }
}
