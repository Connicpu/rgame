use ecs::System;
use components::Components;

pub mod quit;

systems! {
    Systems<Components> {
        quit: quit::QuitSystem = quit::QuitSystem::new()
    }
}
