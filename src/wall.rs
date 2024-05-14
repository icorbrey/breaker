use crate::prelude::*;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct _Wall;
