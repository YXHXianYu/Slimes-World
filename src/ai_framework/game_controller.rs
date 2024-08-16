use crate::core::*;


#[derive(Debug)]
pub struct GameController<'a> {
    team_data: &'a Team,
    // TODO: 添加一个event事件列表，用于添加事件（可以用trait）
}

impl<'a> GameController<'a> {
    // MARK: Public

    pub fn new(team_data: &'a Team) -> Self {
        GameController {
            team_data,
        }
    }

    pub fn team_data(&self) -> &'a Team {
        self.team_data
    }

    pub fn slime_move_to(&self, slime: &Slime, x: i32, z: i32) {
        // TODO
    }

    // MARK: Private
}