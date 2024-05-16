//! Adds functionality for mouse position tracking.
use bevy::{input::InputSystem, window::PrimaryWindow};
use leafwing_input_manager::{
    axislike::DualAxisData, plugin::InputManagerSystem, systems::run_if_enabled,
};

use crate::prelude::*;

/// A plugin responsible for setting up mouse tracking for all components with
/// [`ActionState<MouseAction>`].
pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MouseAction>::default())
            .add_systems(Startup, register_window_listener)
            .add_systems(Update, fetch_action_states)
            .add_systems(PostUpdate, remove_action_states)
            .add_systems(
                Update,
                update_cursor_state
                    .run_if(run_if_enabled::<MouseAction>)
                    .in_set(InputManagerSystem::ManualControl)
                    .before(InputManagerSystem::ReleaseOnDisable)
                    .after(InputManagerSystem::Tick)
                    .after(InputManagerSystem::Update)
                    .after(InputSystem),
            );
    }
}

/// Mouse-related actions.
#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MouseAction {
    Move,
}

/// Injects a mouse action state driver into the window.
fn register_window_listener(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands.entity(window.single()).insert(ActionStateDriver {
        action: MouseAction::Move,
        targets: ().into(),
    });
}

/// Updates all targets with the current mouse position.
fn update_cursor_state(
    windows: Query<(&Window, &ActionStateDriver<MouseAction>)>,
    mut action_states: Query<&mut ActionState<MouseAction>>,
) {
    for (window, driver) in windows.iter() {
        let axis_pair = match window.cursor_position() {
            Some(position) => Some(DualAxisData::from_xy(position)),
            None => continue,
        };

        for target in driver.targets.iter() {
            action_states
                .get_mut(*target)
                .expect("Entity in target list no longer exists or no longer has the `ActionState` component.")
                .action_data_mut_or_default(&driver.action)
                .axis_pair = axis_pair;
        }
    }
}

/// Adds entities to the list of targets when spawned or given an action state component.
fn fetch_action_states(
    mut windows: Query<&mut ActionStateDriver<MouseAction>, With<Window>>,
    entities: Query<Entity, Added<ActionState<MouseAction>>>,
) {
    for mut driver in windows.iter_mut() {
        driver.targets.add(entities.iter());
    }
}

/// Removes entities to the list of targets when despawned or no longer has action state component.
fn remove_action_states(
    mut windows: Query<&mut ActionStateDriver<MouseAction>, With<Window>>,
    mut entities: RemovedComponents<ActionState<MouseAction>>,
) {
    for mut driver in windows.iter_mut() {
        for entity in entities.read() {
            driver.targets.remove(entity);
        }
    }
}
