#![warn(clippy::integer_arithmetic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::needless_pass_by_value
)]
use bevy::prelude::*;

mod apple;
mod consts;
mod eat;
mod grid;
mod modn;
mod movement;
mod snake;

use apple::Apple;
use consts::*;
use grid::Size;
use modn::Mod;
use movement::{Orientation, Position, Speed};
use snake::Head;

pub fn play() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(snake::SnakePlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(grid::GridPlugin)
        .add_plugin(apple::ApplePlugin)
        .add_plugin(eat::EatPlugin)
        .insert_resource(ClearColor(ARENA_COLOR))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
