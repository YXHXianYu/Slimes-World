use std::sync::Arc;

use crate::resources::Team;


#[derive(Debug)]
pub struct GameController<'a> {
    team_id: u32,
    team_data: &'a Team,
}

impl<'a> GameController<'a> {
    // MARK: Public
    pub fn new(team_id: u32, team_data: &'a Team) -> Self {
        GameController {
            team_id,
            team_data,
        }
    }

    pub fn team_id(&self) -> u32 {
        self.team_id
    }

    pub fn team_data(&self) -> &'a Team {
        self.team_data
    }

    // MARK: Private
}