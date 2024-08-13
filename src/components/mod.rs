use bevy::prelude::*;

// Map Components

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Empty,
    // Walkable
    Dirt,
    Grass,
    Sand,
    Bush,
    // NonWalkable
    Rock,
    Tree,
    Water,
    Building, // Wall, Home, Spring
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Empty => false,
            TileType::Dirt => true,
            TileType::Grass => true,
            TileType::Sand => true,
            TileType::Bush => true,
            TileType::Rock => false,
            TileType::Tree => false,
            TileType::Water => false,
            TileType::Building => false,
        }
    }
}

#[derive(Component, Debug)]
pub struct TileComponent {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub tile_type: TileType,
}

impl TileComponent {
    pub fn new(x: i32, y: i32, tile_type: TileType) -> Self {
        TileComponent {
            x,
            y,
            walkable: TileType::is_walkable(&tile_type),
            tile_type,
        }
    }
}

#[derive(Component, Debug)]
pub struct SubEntitiesComponent {
    pub self_entity: Entity,
    pub sub_entities: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct MapComponent {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Entity>>,
}

// Slime

#[derive(Component, Debug)]
pub struct SlimeComponent {
    pub x: i32,
    pub y: i32,
}