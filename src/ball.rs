use crate::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Ball>();
    }
}

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
