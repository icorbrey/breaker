use crate::prelude::*;

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Brick>();
    }
}

#[derive(Component, Debug, Default)]
pub struct Brick;

impl EntityBlueprint for Brick {
    const NAME: &'static str = "Brick";

    fn components() -> impl Bundle {
        Collider::cuboid(24.0, 8.0)
    }
}
