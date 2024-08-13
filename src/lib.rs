pub mod components;
pub mod systems;
pub mod resources;
pub mod utils;

use bevy::prelude::*;

use crate::systems::SlimesWorldSystemsPlugin;

pub struct SlimesWorldPlugin;

impl Plugin for SlimesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(SlimesWorldSystemsPlugin);
    }
}