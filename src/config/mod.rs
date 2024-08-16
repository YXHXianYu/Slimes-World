
// Map
pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

// Camera
pub const CAMERA_POS_X_OFFSET: f32 = 0.0;
pub const CAMERA_POS_Y: f32        = 13.0;
pub const CAMERA_POS_Z_OFFSET: f32 = 30.0;

pub const CAMERA_MOVE_SPEED: f32 = 10.0;
pub const CAMERA_ROTATE_SPEED: f32 = 0.003;

// Team
pub const TEAM_COUNT: usize = 3; // 0: default NPC; 1-8: player

// #region Derived Constants
pub const CAMERA_POS_X: f32 = MAP_WIDTH as f32 / 2.0 + CAMERA_POS_X_OFFSET;
pub const CAMERA_POS_Z: f32 = MAP_HEIGHT as f32 / 2.0 + CAMERA_POS_Z_OFFSET;
pub const CAMERA_LOOK_AT_X: f32 = MAP_WIDTH as f32 / 2.0;
pub const CAMERA_LOOK_AT_Y: f32 = 0.0;
pub const CAMERA_LOOK_AT_Z: f32 = MAP_HEIGHT as f32 / 2.0;
// #endregion
