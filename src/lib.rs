pub mod components;
pub mod systems;
pub mod resources;
pub mod utils;
pub mod config;

use bevy::prelude::*;

use crate::systems::SystemsPlugin;
use crate::resources::ResourcesPlugin;

pub struct SlimesWorldPlugin;

impl Plugin for SlimesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(ResourcesPlugin);
        app.add_plugins(SystemsPlugin);
    }
}