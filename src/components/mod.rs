pub mod core;

components! {
    Components {
        #[hot] position: core::Position,
        #[hot] velocity: core::Velocity,
    }
}
