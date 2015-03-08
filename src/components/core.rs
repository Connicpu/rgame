#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    /// X position
    x: f32,
    /// Y position
    y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    /// X-units moved per second
    dx: f32,
    /// Y-units moved per second
    dy: f32,
}
