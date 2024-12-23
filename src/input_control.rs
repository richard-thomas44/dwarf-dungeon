use bevy::input::InputSystem;
use bevy::prelude::*;
use crate::player::ControlPlayerEvent;
use crate::player::PlayerAction;

pub fn input_control_plugin(app: &mut App) {
    app.add_systems(Update, (
        player_input2.in_set(InputSystem),
        close_on_esc.in_set(InputSystem),       // close_on_esc was removed in 0.14
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