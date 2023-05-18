use crate::*;

#[derive(Component)]
pub struct Apple;

fn spawn_apple(mut commands: Commands, query: Query<With<Apple>>) {
    if query.is_empty() {
        commands.spawn((
            Apple,
            ENT_SIZE,
            apple_pos(),
            SpriteBundle {
                sprite: Sprite {
                    color: APPLE_COLOR,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn apple_pos() -> Position {
    Position {
        x: Mod::random(),
        y: Mod::random(),
    }
}

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_apple);
    }
}
