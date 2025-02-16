use bevy::prelude::*;
use build_map::build_map_plugin;
use input_control::input_control_plugin;
use player::player_plugin;
use game_state::game_state_plugin;
use physics::physics_plugin;

mod build_map;
mod input_control;
mod player;
mod game_state;
mod physics;
mod apple;

fn main() {
    App::new()
//        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dwarf Dungeon".into(),
                resolution: (1920.*0.8, 1080.*0.8).into(),
                ..default()
            }),
            ..default()
        })
             .set(ImagePlugin::default_nearest()))
        .add_plugins((
            game_state_plugin,
            build_map_plugin,
            input_control_plugin,
            player_plugin,
            physics_plugin,
        ))
        .run();
}
