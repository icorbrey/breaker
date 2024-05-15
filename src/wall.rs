use crate::prelude::*;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_tile_blueprint::<Wall>();
    }
}

ldtk_tile!(Wall);
impl TileBlueprint<BlueprintBundle> for Wall {
    const NAME: &'static str = "Wall";
    const ID: i32 = 1;

    fn components() -> impl Bundle {
        Collider::cuboid(8.0, 8.0)
    }
}
