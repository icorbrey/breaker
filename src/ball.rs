use crate::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Ball>();
    }
}

ldtk_entity!(Ball);

impl EntityBlueprint<BlueprintBundle> for Ball {
    const NAME: &'static str = "Ball";

    fn components() -> impl Bundle {
        Collider::ball(8.0)
    }
}
