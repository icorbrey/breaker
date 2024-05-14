use crate::prelude::*;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(LocalTile::Wall.into());
    }
}

#[derive(Component, Debug, Default)]
pub struct Wall;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
