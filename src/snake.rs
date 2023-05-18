use crate::*;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct BodySegment;

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Head,
        ENT_SIZE,
        STARTING_ORIENTATION,
        STARTING_POSITION,
        Speed(STARTING_SPEED),
        SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_segment(mut commands: Commands, position: Position) {
    commands.spawn((
        BodySegment,
        ENT_SIZE,
        position,
        SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                ..default()
            },
            ..default()
        },
    ));
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake);
    }
}
