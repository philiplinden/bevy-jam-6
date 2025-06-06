use bevy::prelude::*;

pub mod element;
pub mod particle;
pub mod sandbox;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((particle::plugin, sandbox::plugin));
}
