use crate::*;

pub type Num = u8;

/// Physics frames per second.
pub const FRAME_RATE: f32 = 60.0;
/// Time elapsed per physics step.
pub const TIME_STEP: f32 = 1.0 / FRAME_RATE;

/// The height of the arena.
pub const ARENA_HEIGHT: Num = 100;
/// The width of the arena.
pub const ARENA_WIDTH: Num = 100;
/// The arena's color.
pub const ARENA_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

/// The snake's starting speed.
pub const STARTING_SPEED: Num = 1;
/// The snake's starting position.
pub const STARTING_POS: Position = Position {
    x: Mod(ARENA_WIDTH / 2),
    y: Mod(ARENA_HEIGHT / 2),
};
/// The snake's starting orientation.
pub const STARTING_ORIENTATION: Orientation = Orientation::North;

/// The size of the snake's head.
pub const HEAD_SIZE: Size = Size {
    width: 1.0,
    height: 1.0,
};
/// The color of the snake's head.
pub const HEAD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

/// The size of the apples.
pub const APPLE_SIZE: Vec3 = Vec3::new(10.0, 10.0, 0.0);
/// The color of the apples.
pub const APPLE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
