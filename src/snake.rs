use crate::*;

#[derive(Component)]
pub struct Head;

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Head,
        HEAD_SIZE,
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

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake);
    }
}
