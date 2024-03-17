use bevy::input::InputSystem;
use bevy::prelude::*;
use crate::player::ControlPlayerEvent;
use crate::player::PlayerAction;

pub fn input_control_plugin(app: &mut App) {
    app.add_systems(Update, (
        player_input2.in_set(InputSystem),
        bevy::window::close_on_esc,
    ));
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
        0b0001 => 0,
        0b1001 => 1,
        0b1000 => 2,
        0b1100 => 3,
        0b0100 => 4,
        0b0110 => 5,
        0b0010 => 6,
        0b0011 => 7,
        _      => {return},
    };
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Sprint {direction : dir}));
    } else {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Walk {direction : dir}));
        }  
}

// Collect player input by polling keypresses. Check first for WASD key combinations, then single WASD directions

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_control_player: EventWriter<ControlPlayerEvent>,
) { 
    let direction = 'block: {
        if keyboard_input.pressed(KeyCode::KeyW) {
            if keyboard_input.pressed(KeyCode::KeyA) {
                break 'block Some(7); // Northwest
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                break 'block Some(1); // Northeast
            }
                break 'block Some(0); // North
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            if keyboard_input.pressed(KeyCode::KeyA) {
                break 'block Some(5); // Southwest
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                break 'block Some(3); // Southeast
            }
            break 'block Some(4); // South
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            break 'block Some(6); // West
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            break 'block Some(2); // East
        }
        None
    };
    if let Some(dir) = direction {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            ev_control_player.send(ControlPlayerEvent(PlayerAction::Sprint {direction : dir}));
        } else {
            ev_control_player.send(ControlPlayerEvent(PlayerAction::Walk {direction : dir}));
        }  
    }
}   
