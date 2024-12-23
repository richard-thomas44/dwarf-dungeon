use bevy::{prelude::*};
use build_map::build_map_plugin;
use input_control::input_control_plugin;
use player::player_plugin;

mod build_map;
mod input_control;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins((
            build_map_plugin,
            input_control_plugin,
            player_plugin,
        ))
        .run();
}
