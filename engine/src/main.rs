use engine::{ecs::commands::CommandQueue, prelude::*};

fn main() {
    env_logger::init();

    let mut world = World::default();

    world.spawn(Name::from("Hello"));
}
