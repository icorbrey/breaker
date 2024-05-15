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
        Collider::ball(8.0)
    }
}
