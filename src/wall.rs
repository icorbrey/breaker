use crate::prelude::*;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(Wall::ID);
    }
}

#[derive(Component, Debug, Default)]
pub struct Wall;

impl Wall {
    pub const ID: i32 = 1;
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
