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
    pub building_type: BuildingType,
}

impl BuildingComponent {
    pub fn new(building_type: BuildingType) -> Self {
        BuildingComponent {
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

#[derive(Component, Debug)]
pub struct InMapComponent {
    pub map_id: Entity,
}

// Slime

#[derive(Component, Debug)]
pub struct SlimeComponent;

// Team

#[derive(Component, Debug)]
pub struct TransformComponent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// 在所有system startup完毕之后的PostStartup中，自动register所有的Slimes和Buildings
#[derive(Component, Debug)]
pub struct BeControlledComponent {
    pub team_id: u32,
}