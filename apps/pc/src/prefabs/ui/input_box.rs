use bevy::{prelude::*, ui::FocusPolicy};

#[derive(Component)]
pub struct InputBox {
    pub placeholder: String,
    pub is_password: bool,
    pub max_length: usize,
    pub value: String,
    pub input_type: InputType,
    pub is_focused: bool,
}

#[derive(PartialEq)]
pub enum InputType {
    Text,
    Number,
    Password,
}

#[derive(Bundle)]
pub struct InputBoxBundle {
    input_box: InputBox,
    node_bundle: NodeBundle,
}

impl InputBoxBundle {
    pub fn new(placeholder: &str, input_type: InputType) -> Self {
        Self {
            input_box: InputBox {
                placeholder: placeholder.to_string(),
                is_password: matches!(input_type, InputType::Password),
                max_length: 32,
                value: String::new(),
                input_type,
                is_focused: false,
            },
            node_bundle: NodeBundle {
                node: Node {
                    width: Val::Px(300.0),
                    height: Val::Px(40.0),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.9, 0.9, 0.9)),
                border_color: BorderColor(Color::rgb(0.7, 0.7, 0.7)),
                focus_policy: FocusPolicy::Block,
                ..default()
            },
        }
    }
}
