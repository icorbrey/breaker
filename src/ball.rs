use crate::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BallBundle>(Ball::ID);
    }
}

#[derive(Component, Debug, Default)]
pub struct Ball;

impl Ball {
    const ID: &'static str = "Ball";
}

#[derive(Bundle, LdtkEntity)]
pub struct BallBundle {
    ball: Ball,
    name: Name,
    collider: Collider,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            name: Name::new(Ball::ID),
            ball: Ball,
            collider: Collider::ball(8.0),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
        }
    }
}
