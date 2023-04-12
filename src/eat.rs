use crate::*;

fn check_eat(
    mut commands: Commands,
    heads: Query<&Position, With<Head>>,
    apples: Query<(Entity, &Position), With<Apple>>,
) {
    let head_pos = heads.single();
    for (entity, apple_pos) in &apples {
        if apple_pos == head_pos {
            commands.entity(entity).despawn();
        }
    }
}

pub struct EatPlugin;

impl Plugin for EatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_eat);
    }
}
