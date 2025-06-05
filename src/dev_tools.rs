//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        states::log_transitions,
        picking_debug::{DebugPickingMode, DebugPickingPlugin}
     }, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use avian2d::prelude::PhysicsDebugPlugin;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DebugPickingPlugin,
        PhysicsDebugPlugin::default(),
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin::default(),
    ));
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(KeyCode::KeyR)),
    );

    app.insert_resource(DebugPickingMode::Disabled)
        // A system that cycles the debugging state when you press F3:
        .add_systems(
            PreUpdate,
            (|mut mode: ResMut<DebugPickingMode>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
                }
            })
            .distributive_run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::KeyT,
            )),
        );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        wireframe_config.global = !wireframe_config.global;
    }
}
