use crate::prelude::*;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug, Default)]
pub struct Paddle;
