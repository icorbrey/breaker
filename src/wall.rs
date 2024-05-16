//! Adds functionality for walls.
use crate::prelude::*;

/// A plugin responsible for setting up the Wall int cell and its associated functionality.
pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_tile_blueprint::<Wall>();
    }
}

/// Marker type for the Wall int cell.
#[derive(Component, Debug, Default)]
pub struct Wall;

impl TileBlueprint for Wall {
    const NAME: &'static str = "Wall";
    const ID: i32 = 1;

    fn components() -> impl Bundle {
        Collider::cuboid(8.0, 8.0)
    }
}
