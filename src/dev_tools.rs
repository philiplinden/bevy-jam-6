//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        states::log_transitions,
        picking_debug::{DebugPickingMode, DebugPickingPlugin}
     }, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DebugPickingPlugin);

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.insert_resource(DebugPickingMode::Normal)
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
                KeyCode::F3,
            )),
        );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
