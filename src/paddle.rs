//! Adds functionality for the paddle.
use crate::prelude::*;

/// A plugin responsible for setting up the Paddle entity and its associated functionality.
pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Paddle>()
            .add_systems(Update, track_mouse_position);
    }
}

/// Marker type for the Paddle entity.
#[derive(Component, Debug, Default)]
pub struct Paddle;

impl EntityBlueprint for Paddle {
    const NAME: &'static str = "Paddle";

    fn components() -> impl Bundle {
        (
            InputManagerBundle::<MouseAction>::default(),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(128.0, 8.0),
        )
    }
}

/// Updates the paddle's position to cling to the mouse where possible.
fn track_mouse_position(
    mut paddle: Query<(&mut Transform, &RigidBody, &ActionState<MouseAction>)>,
) {
    if let Ok((transform, rigid_body, action_state)) = paddle.get_single_mut() {
        println!(
            "Transform:   {:?}\nRigidBody:   {:?}\nActionState: {:?}",
            transform, rigid_body, action_state
        )
    }
}
