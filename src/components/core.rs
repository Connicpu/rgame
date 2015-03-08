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
pub struct Acceleration {
    /// X-units moved per second
    pub ax: f32,
    /// Y-units moved per second
    pub ay: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scale {
    /// Uniform scale value
    pub size: f32,
    /// Non-uniform scale X-value
    pub sx: f32,
    /// Non-uniform scale Y-value
    pub sy: f32,
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            size: 1.0,
            sx: 1.0,
            sy: 1.0,
        }
    }

    pub fn uniform(size: f32) -> Scale {
        Scale {
            size: size,
            sx: 1.0,
            sy: 1.0,
        }
    }
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
