#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    /// X-units moved per second
    pub dx: f32,
    /// Y-units moved per second
    pub dy: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimeData {
    pub dt: f32,
}

impl TimeData {
    pub fn new() -> TimeData {
        TimeData {
            dt: 0.0,
        }
    }
}
