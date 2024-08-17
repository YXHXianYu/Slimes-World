use crate::core::*;

use crate::ai_framework::control_event::*;

// #[derive(Debug)]
pub struct GameController<'a> {
    team_data: &'a Team,

    // 事件列表
    control_events: Vec<ControlEvent>,

    // TODO: 添加一些能够根据EntityID查询每个Entity更详细数据的数据结构

    // TODO: Controller的Log（比如move to的事件覆盖等）
}

impl<'a> GameController<'a> {
    // MARK: Public to User

    pub fn new(team_data: &'a Team) -> Self {
        GameController {
            team_data,
            control_events: Vec::new(),
        }
    }

    pub fn team_data(&self) -> &'a Team {
        self.team_data
    }

    pub fn move_slime(&mut self, slime: Slime, direction: MoveDirection) {
        self.control_events.push(ControlEvent::MoveSlime {
            slime,
            direction,
        });
    }

    // MARK: Public

    pub fn control_events_retain(&mut self, f: impl FnMut(&ControlEvent) -> bool) {
        self.control_events.retain(f);
    }

    // MARK: Private
}