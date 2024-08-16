use bevy::prelude::*;

use crate::resources::*;
use crate::components::*;

pub struct TeamSystemPlugin;
impl Plugin for TeamSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,
            post_startup_register_be_controlled_components
        );
    }
}

fn post_startup_register_be_controlled_components(
    mut game_resource: ResMut<GameResource>,
    query: Query<(
        Entity,
        &BeControlledComponent,
        Option<&SlimeComponent>,
        Option<&BuildingComponent>,
    )>,
) {
    for (entity, control, slime, building) in query.iter() {
        if slime.is_some() {
            game_resource.teams[control.team_id as usize].slimes.push(entity);
        } else if building.is_some() {
            game_resource.teams[control.team_id as usize].buildings.push(entity);
        } else {
            panic!("Exist an entity with BeControlledComponent but without any other tag component! (such as SlimeComponent or BuildingComponent)");
        }
    }
}