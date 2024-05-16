//! Adds functionality for bricks.
use crate::prelude::*;

/// A plugin responsible for setting up the Brick entity and its associated functionality.
pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Brick>();
    }
}

/// Marker type for the Brick component.
#[derive(Component, Debug, Default)]
pub struct Brick;

impl EntityBlueprint for Brick {
    const NAME: &'static str = "Brick";

    fn components() -> impl Bundle {
        Collider::cuboid(24.0, 8.0)
    }
}
