use crate::*;

#[derive(Component, Deref)]
pub struct Speed(pub Num);

#[derive(Component)]
pub struct Position {
    pub x: Mod<ARENA_WIDTH>,
    pub y: Mod<ARENA_HEIGHT>,
}

#[derive(Component)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

fn apply_velocity(mut query: Query<(&mut Position, &Speed, &Orientation)>) {
    for (mut pos, speed, orientation) in &mut query {
        match orientation {
            Orientation::North => pos.y += **speed,
            Orientation::South => pos.y -= **speed,
            Orientation::East => pos.x += **speed,
            Orientation::West => pos.x -= **speed,
        }
    }
}

fn orient_head(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Orientation, With<Head>>) {
    // assert that there's only one head
    let mut orientation = query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        *orientation = Orientation::North;
    }

    if keyboard.pressed(KeyCode::Down) {
        *orientation = Orientation::South;
    }

    if keyboard.pressed(KeyCode::Left) {
        *orientation = Orientation::West;
    }

    if keyboard.pressed(KeyCode::Right) {
        *orientation = Orientation::East;
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_velocity, orient_head.before(apply_velocity))
                // force run every TIME_STEP increment, I think
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP));
    }
}
