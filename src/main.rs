mod ball;
mod blueprint;
mod brick;
mod paddle;
mod prelude;
mod wall;

use crate::prelude::{plugins::*, *};

fn main() {
    App::new()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierDebugRenderPlugin::default(),
            EditorPlugin::default(),
            LdtkPlugin,
        ))
        .add_plugins((BallPlugin, BrickPlugin, PaddlePlugin, WallPlugin))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        ..default()
    });
}
