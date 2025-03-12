use crate::prefabs::ui::input_box::{InputBox, InputType};
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::prelude::*;

fn get_char_from_key(key_code: KeyCode, shift_pressed: bool) -> Option<char> {
    match key_code {
        // 处理字母键
        key @ (KeyCode::KeyA
        | KeyCode::KeyB
        | KeyCode::KeyC
        | KeyCode::KeyD
        | KeyCode::KeyE
        | KeyCode::KeyF
        | KeyCode::KeyG
        | KeyCode::KeyH
        | KeyCode::KeyI
        | KeyCode::KeyJ
        | KeyCode::KeyK
        | KeyCode::KeyL
        | KeyCode::KeyM
        | KeyCode::KeyN
        | KeyCode::KeyO
        | KeyCode::KeyP
        | KeyCode::KeyQ
        | KeyCode::KeyR
        | KeyCode::KeyS
        | KeyCode::KeyT
        | KeyCode::KeyU
        | KeyCode::KeyV
        | KeyCode::KeyW
        | KeyCode::KeyX
        | KeyCode::KeyY
        | KeyCode::KeyZ) => {
            let base = match key {
                KeyCode::KeyA => 'a',
                KeyCode::KeyB => 'b',
                KeyCode::KeyC => 'c',
                KeyCode::KeyD => 'd',
                KeyCode::KeyE => 'e',
                KeyCode::KeyF => 'f',
                KeyCode::KeyG => 'g',
                KeyCode::KeyH => 'h',
                KeyCode::KeyI => 'i',
                KeyCode::KeyJ => 'j',
                KeyCode::KeyK => 'k',
                KeyCode::KeyL => 'l',
                KeyCode::KeyM => 'm',
                KeyCode::KeyN => 'n',
                KeyCode::KeyO => 'o',
                KeyCode::KeyP => 'p',
                KeyCode::KeyQ => 'q',
                KeyCode::KeyR => 'r',
                KeyCode::KeyS => 's',
                KeyCode::KeyT => 't',
                KeyCode::KeyU => 'u',
                KeyCode::KeyV => 'v',
                KeyCode::KeyW => 'w',
                KeyCode::KeyX => 'x',
                KeyCode::KeyY => 'y',
                KeyCode::KeyZ => 'z',
                _ => unreachable!(),
            };
            Some(if shift_pressed {
                base.to_ascii_uppercase()
            } else {
                base
            })
        }

        // 处理数字键
        key @ (KeyCode::Digit0
        | KeyCode::Digit1
        | KeyCode::Digit2
        | KeyCode::Digit3
        | KeyCode::Digit4
        | KeyCode::Digit5
        | KeyCode::Digit6
        | KeyCode::Digit7
        | KeyCode::Digit8
        | KeyCode::Digit9
        | KeyCode::Numpad0
        | KeyCode::Numpad1
        | KeyCode::Numpad2
        | KeyCode::Numpad3
        | KeyCode::Numpad4
        | KeyCode::Numpad5
        | KeyCode::Numpad6
        | KeyCode::Numpad7
        | KeyCode::Numpad8
        | KeyCode::Numpad9) => Some(match key {
            KeyCode::Digit0 | KeyCode::Numpad0 => '0',
            KeyCode::Digit1 | KeyCode::Numpad1 => '1',
            KeyCode::Digit2 | KeyCode::Numpad2 => '2',
            KeyCode::Digit3 | KeyCode::Numpad3 => '3',
            KeyCode::Digit4 | KeyCode::Numpad4 => '4',
            KeyCode::Digit5 | KeyCode::Numpad5 => '5',
            KeyCode::Digit6 | KeyCode::Numpad6 => '6',
            KeyCode::Digit7 | KeyCode::Numpad7 => '7',
            KeyCode::Digit8 | KeyCode::Numpad8 => '8',
            KeyCode::Digit9 | KeyCode::Numpad9 => '9',
            _ => unreachable!(),
        }),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}

pub fn handle_text_input(
    mut keyboard_events: EventReader<KeyboardInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut InputBox, &Interaction)>,
) {
    let shift_pressed =
        keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    for (mut input_box, interaction) in query.iter_mut() {
        // 只处理被点击的输入框
        if !matches!(interaction, Interaction::Pressed) {
            continue;
        }

        // 处理退格键
        if keyboard.just_pressed(KeyCode::Backspace) {
            input_box.value.pop();
            continue;
        }

        // 处理文本输入
        for ev in keyboard_events.read() {
            if !ev.state.is_pressed() || input_box.value.len() >= input_box.max_length {
                continue;
            }

            if let Some(c) = get_char_from_key(ev.key_code, shift_pressed) {
                match input_box.input_type {
                    InputType::Number if c.is_ascii_digit() => input_box.value.push(c),
                    InputType::Text | InputType::Password => input_box.value.push(c),
                    _ => {}
                }
            }
        }
    }
}
