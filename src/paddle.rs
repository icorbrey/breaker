use crate::prelude::*;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_entity_blueprint::<Paddle>()
            .add_systems(Update, track_mouse_position);
    }
}

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
