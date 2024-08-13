use bevy::prelude::*;

use slimes_world::SlimesWorldPlugin;

fn main() {
    App::new()
        .add_plugins(SlimesWorldPlugin)
        .run();
}