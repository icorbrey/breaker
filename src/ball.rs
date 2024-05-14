use crate::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BallBundle>(LocalEntity::Ball.into());
    }
}

#[derive(Component, Debug, Default)]
pub struct Ball;

#[derive(Bundle, Default, LdtkEntity)]
pub struct BallBundle {
    ball: Ball,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
