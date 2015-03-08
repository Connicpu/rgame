pub mod core;
pub mod graphics;

components! {
    Components {
        #[hot] position: core::Position,
        #[hot] velocity: core::Velocity,
        #[hot] acceleration: core::Acceleration,

        #[hot] time_data: core::TimeData,
        #[hot] scale: core::Scale,

        #[cold] sprite: graphics::Sprite,
    }
}
