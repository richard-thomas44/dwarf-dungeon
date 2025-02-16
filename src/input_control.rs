use bevy::input::InputSystem;
use bevy::prelude::*;
use crate::game_state::GameState;
use crate::player::ControlPlayerEvent;
use crate::player::PlayerAction;
use crate::game_state::GameStateSet;

pub fn input_control_plugin(app: &mut App) {
    app.add_systems(Update, (
        player_input2.in_set(InputSystem),
        close_on_esc.in_set(InputSystem),       // close_on_esc was removed in 0.14
    ).in_set(GameStateSet::InGameSet)
    .run_if(in_state(GameState::InGame))
    );
}

#[derive(PartialEq, Copy, Clone)]
pub enum Directions {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn player_input2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_control_player: EventWriter<ControlPlayerEvent>,
    ) {
    let keys : u8 = keyboard_input.pressed(KeyCode::KeyW) as u8
                    | (keyboard_input.pressed(KeyCode::KeyA) as u8) << 1
                    | (keyboard_input.pressed(KeyCode::KeyS) as u8) << 2
                    | (keyboard_input.pressed(KeyCode::KeyD) as u8) << 3;

    let dir = match keys {

// temporarily remove diagonals, although they might improve responsiveness on ladders

        0b0001 => Directions::Up,            // 1            W
        0b1001 => Directions::Right,     //  UpRight      // 9            W, D
        0b1000 => Directions::Right,            // 8            D
        0b1100 => Directions::Right,  // DownRight          // 12           D, S
        0b0100 => Directions::Down,         // 4            S
        0b0110 => Directions::Left,       // DownLeft         // 6            S, A
        0b0010 => Directions::Left,            // 2            A
        0b0011 => Directions::Left,     // UpLeft        // 3            A, W
        _      => {return},
    };
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Sprint {direction : dir}));
    } else {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Walk {direction : dir}));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Jump {direction : dir}));
    }
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}