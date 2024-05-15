use crate::prelude::*;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Paddle>();
    }
}

ldtk_entity!(Paddle);
impl EntityBlueprint<BlueprintBundle> for Paddle {
    const NAME: &'static str = "Paddle";

    fn components() -> impl Bundle {
        Collider::cuboid(128.0, 8.0)
    }
}
