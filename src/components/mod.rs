use bevy::prelude::*;

// Map Components

#[derive(Debug)]
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

#[derive(Component, Debug)]
pub struct TileComponent {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub tile_type: TileType,
}
