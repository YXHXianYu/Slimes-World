// Write your AI code here

// use bevy::prelude::*;
use crate::prelude::*;

use rand::{thread_rng, Rng};

// TODO: 添加数据查询（比如根据Slime对象查询对应slime的其他信息）

pub fn ai_team_0(_game_controller: &mut GameController) { }

pub fn ai_team_1(game_controller: &mut GameController) {

    let data = game_controller.team_data();
    
    if data.slimes.len() > 0 {

        let mut rng = thread_rng();

        let direction = match rng.gen_range(0..4) {
            0 => MoveDirection::Up,
            1 => MoveDirection::Down,
            2 => MoveDirection::Left,
            3 => MoveDirection::Right,
            _ => panic!("ai_team_1: invalid direction"),
        };

        game_controller.move_slime(
            data.slimes[0],
            direction,
        );
    } else {
        println!("ai_team_1: no slimes");
    }
}

pub fn ai_team_2(_game_controller: &mut GameController) { }

pub fn ai_team_3(_game_controller: &mut GameController) { }

pub fn ai_team_4(_game_controller: &mut GameController) { }

pub fn ai_team_5(_game_controller: &mut GameController) { }

pub fn ai_team_6(_game_controller: &mut GameController) { }

pub fn ai_team_7(_game_controller: &mut GameController) { }

pub fn ai_team_8(_game_controller: &mut GameController) { }
