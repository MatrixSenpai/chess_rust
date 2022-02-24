#![allow(unused, dead_code)]
#![forbid(unsafe_code)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;
mod board_plugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1200.0,
            height: 1200.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(board_plugin::BoardPlugin)
        .add_startup_system(setup)
        .add_system(helpers::set_texture_filters_to_nearest)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}