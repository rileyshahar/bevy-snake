use bevy::prelude::*;

/// Time elapsed per physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

/// The size of the snake's head.
const HEAD_SIZE: Vec3 = Vec3::new(10.0, 10.0, 0.0);
/// The color of the snake's head.
const HEAD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

/// The snake's starting speed.
const STARTING_SPEED: f32 = 1.0;
/// The snake's starting position.
const STARTING_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
/// The snake's starting orientation.
const STARTING_ORIENTATION: Orientation = Orientation::North;

const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

/// The size of the apples.
const APPLE_SIZE: Vec3 = Vec3::new(10.0, 10.0, 0.0);
/// The color of the apples.
const APPLE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

/// The height of the arena.
const ARENA_HEIGHT: f32 = 10.0;
/// The width of the arena.
const ARENA_WIDTH: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(spawn_apple)
        .add_systems(
            (apply_velocity, orient_head.before(apply_velocity))
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

#[derive(Component)]
struct Head;

// #[derive(Component)]
// struct Body;

#[derive(Component)]
struct Apple;

#[derive(Component, Deref)]
struct Speed(f32);

#[derive(Component)]
enum Orientation {
    North,
    South,
    East,
    West,
}

fn apply_velocity(mut query: Query<(&mut Transform, &Speed, &Orientation)>) {
    for (mut transform, speed, orientation) in &mut query {
        dbg!(**speed);
        match orientation {
            Orientation::North => transform.translation.y += **speed,
            Orientation::South => transform.translation.y -= **speed,
            Orientation::East => transform.translation.x += **speed,
            Orientation::West => transform.translation.x -= **speed,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Head,
        STARTING_ORIENTATION,
        Speed(STARTING_SPEED),
        SpriteBundle {
            transform: Transform {
                translation: STARTING_POS,
                scale: HEAD_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: HEAD_COLOR,
                ..default()
            },
            ..default()
        },
    ));
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

fn spawn_apple(mut commands: Commands, query: Query<With<Apple>>) {
    if query.is_empty() {
        commands.spawn((
            Apple,
            SpriteBundle {
                transform: Transform {
                    translation: apple_pos().extend(0.0),
                    scale: APPLE_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: APPLE_COLOR,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn apple_pos() -> Vec2 {
    Vec2::new(
        rand::random::<f32>() * ARENA_WIDTH,
        rand::random::<f32>() * ARENA_HEIGHT,
    )
}
