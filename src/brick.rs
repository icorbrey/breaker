use crate::prelude::*;

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BrickBundle>(Brick::ID);
    }
}

#[derive(Component, Debug, Default)]
pub struct Brick;

impl Brick {
    pub const ID: &'static str = "Brick";
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct BrickBundle {
    brick: Brick,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
