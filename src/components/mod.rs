use bevy::{prelude::*, utils::HashMap};

// Map Components

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Empty,
    // Walkable
    Dirt,
    Grass,
    Sand,
    // NonWalkable
    Rock,
    Water,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Empty => false,

            TileType::Dirt => true,
            TileType::Grass => true,
            TileType::Sand => true,

            TileType::Rock => false,
            TileType::Water => false,
        }
    }
}

#[derive(Component, Debug)]
pub struct TileComponent {
    pub x: i32,
    pub z: i32,
    pub walkable: bool,
    pub tile_type: TileType,
}

impl TileComponent {
    pub fn new(x: i32, z: i32, tile_type: TileType) -> Self {
        TileComponent {
            x,
            z,
            walkable: TileType::is_walkable(&tile_type),
            tile_type,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BuildingType {
    // Nature
    Tree,
    Bush,
    // Building
    Home,
    Spring,
    Wall,
}

#[derive(Component, Debug)]
pub struct BuildingComponent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub building_type: BuildingType,
}

impl BuildingComponent {
    pub fn new(x: i32, y: i32, z: i32, building_type: BuildingType) -> Self {
        BuildingComponent {
            x,
            y,
            z,
            building_type,
        }
    }
}

#[derive(Component, Debug)]
pub struct MapComponent {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Entity>>,
    pub buildings: HashMap<IVec3, Entity>,
}

// #[derive(Component, Debug)]
// pub struct SubEntitiesComponent {
//     pub self_entity: Entity,
//     pub sub_entities: Vec<Entity>,
// }

// Slime

#[derive(Component, Debug)]
pub struct SlimeComponent {
    pub x: i32,
    pub z: i32,
}

// Team

#[derive(Component, Debug)]
pub struct BeControlledComponent {
    pub team_id: u32,
    // TODO: 如何使得AI可以快速调用自己拥有的建筑物和单位，应该需要register一下
}