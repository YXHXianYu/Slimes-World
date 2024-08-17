use crate::prelude::*;

#[derive(Debug)]
pub enum ControlEvent {
    MoveSlime {
        slime: Slime,
        direction: MoveDirection,
    }
}