use bevy::prelude::*;

#[derive(Debug)]
pub struct Slime(pub Entity);

#[derive(Debug)]
pub struct Building(pub Entity);

#[derive(Debug)]
pub struct Team {
    pub id: u32,
    pub slimes: Vec<Slime>,
    pub buildings: Vec<Building>,
}