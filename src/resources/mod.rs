///
/// 尽量在ResourcesPlugin中初始化资源，这样可以保证资源在System被加载前初始化
/// 

use bevy::prelude::*;

use crate::config::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let mut teams = Vec::with_capacity(TEAM_COUNT);
        for i in 0..TEAM_COUNT {
            teams.push(Team {
                id: i as u32,
                slimes: Vec::new(),
                buildings: Vec::new(),
            });
        }
        app.insert_resource(GameResource {
            teams,
        });
    }
}

// Team

#[derive(Resource, Debug)]
pub struct GameResource {
    pub teams: Vec<Team>,
}

#[derive(Debug)]
pub struct Team {
    pub id: u32,
    pub slimes: Vec<Entity>,
    pub buildings: Vec<Entity>,
}