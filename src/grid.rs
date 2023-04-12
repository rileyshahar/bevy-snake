use crate::*;

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::square(1.0)
    }
}

/// Transform every sprite to the size of the window.
fn size_scaling(windows: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.single();
    let dim = window.height().min(window.width());
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / f32::from(ARENA_WIDTH) * dim,
            sprite_size.height / f32::from(ARENA_HEIGHT) * dim,
            1.0,
        );
    }
}

/// Convert a position in our game to a position on the screen.
///
/// Works along a single axis.
fn convert(pos: f32, window_size: f32, game_size: f32) -> f32 {
    let tile_size = window_size / game_size; // size of one grid tile
    (pos / game_size).mul_add(window_size, -window_size / 2.0) + (tile_size / 2.0)
}

/// Transform positions based on the grid.
fn pos_transform(windows: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.single();
    let dim = window.height().min(window.width());
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(f32::from(pos.x.0), dim, f32::from(ARENA_WIDTH)),
            convert(f32::from(pos.y.0), dim, f32::from(ARENA_HEIGHT)),
            0.0,
        );
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        // Run after update, so that we transform after movement etc. happens.
        app.add_systems((pos_transform, size_scaling).in_base_set(CoreSet::PostUpdate));
    }
}
