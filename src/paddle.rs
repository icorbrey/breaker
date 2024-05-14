use crate::prelude::*;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PaddleBundle>(Paddle::ID);
    }
}

#[derive(Component, Debug, Default)]
pub struct Paddle;

impl Paddle {
    pub const ID: &'static str = "Paddle";
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PaddleBundle {
    paddle: Paddle,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
