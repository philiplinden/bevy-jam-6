use bevy::prelude::*;

pub mod elements;
pub mod particle;
pub mod reaction;
pub mod sandbox;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((particle::plugin, sandbox::plugin, reaction::plugin));
}
