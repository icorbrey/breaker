use crate::prelude::*;

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Brick>();
    }
}

ldtk_entity!(Brick);
impl EntityBlueprint<BlueprintBundle> for Brick {
    const NAME: &'static str = "Brick";

    fn components() -> impl Bundle {
        Collider::cuboid(24.0, 8.0)
    }
}
