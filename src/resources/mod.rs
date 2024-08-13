use bevy::prelude::*;

use crate::config::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapResource {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        });
    }
}

#[derive(Resource, Debug)]
pub struct MapResource {
    pub width: usize,
    pub height: usize,
}