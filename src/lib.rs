#![warn(clippy::integer_arithmetic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::needless_pass_by_value
)]
use bevy::prelude::*;

mod apple;
mod consts;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (500.0, 500.0).into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_plugin(snake::SnakePlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(grid::GridPlugin)
        .insert_resource(ClearColor(ARENA_COLOR))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
