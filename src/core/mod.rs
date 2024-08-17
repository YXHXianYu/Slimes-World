use bevy::prelude::*;


#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Slime(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct Building(pub Entity);

#[derive(Debug)]
pub struct Team {
    pub id: u32,
    pub slimes: Vec<Slime>,
    pub buildings: Vec<Building>,
}