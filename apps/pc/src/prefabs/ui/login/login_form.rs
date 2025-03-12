use crate::prefabs::ui::input_box::{InputBoxBundle, InputType};
use bevy::prelude::*;

#[derive(Component)]
pub struct LoginForm;

#[derive(Bundle)]
pub struct LoginFormBundle {
    login_form: LoginForm,
    node_bundle: NodeBundle,
}

impl LoginFormBundle {
    pub fn new() -> Self {
        Self {
            login_form: LoginForm,
            node_bundle: NodeBundle {
                node: Node {
                    width: Val::Px(320.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    right: Val::Auto,
                    top: Val::Percent(50.0),
                    bottom: Val::Auto,
                    margin: UiRect {
                        left: Val::Px(-160.0), // width的一半
                        top: Val::Px(-60.0),   // 预估高度的一半
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            },
        }
    }
}
