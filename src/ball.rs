use crate::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct _Ball;
