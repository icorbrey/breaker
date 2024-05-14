mod ball;
mod entities;
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
        .add_plugins((BallPlugin, WallPlugin, PaddlePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Name::new("Platform"),
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));

    commands.spawn((
        Name::new("Ball"),
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)),
    ));
}
