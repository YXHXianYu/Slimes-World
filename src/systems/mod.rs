use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub struct SlimesWorldSystemsPlugin;

impl Plugin for SlimesWorldSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello_system);
    }
}

// Hello System

fn hello_system() {
    println!("Hello, world!");
}