//! Re-exports common definitions for ease of use.
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_ecs_ldtk::prelude::*;
pub(crate) use bevy_editor_pls::prelude::*;
pub(crate) use bevy_rapier2d::prelude::*;
pub(crate) use leafwing_input_manager::prelude::*;

pub(crate) use crate::blueprint::{BlueprintAppExt, EntityBlueprint, TileBlueprint};
pub(crate) use crate::mouse::MouseAction;

pub(crate) mod plugins {
    pub use crate::{
        ball::BallPlugin, brick::BrickPlugin, mouse::MousePlugin, paddle::PaddlePlugin,
        wall::WallPlugin,
    };
}
