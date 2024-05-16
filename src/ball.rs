//! Adds functionality for balls.
use crate::prelude::*;

/// A plugin responsible for setting up the Ball entity and its associated functionality.
pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Ball>();
    }
}

/// Marker type for the Ball entity.
#[derive(Component, Debug, Default)]
pub struct Ball;

impl EntityBlueprint for Ball {
    const NAME: &'static str = "Ball";

    fn components() -> impl Bundle {
        (
            Velocity::linear(Vec2::new(500.0, 500.0)),
            Restitution::coefficient(2.0),
            Collider::ball(8.0),
            RigidBody::Dynamic,
            GravityScale(0.0),
        )
    }
}
