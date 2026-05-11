use engine::prelude::*;

fn main() {
    env_logger::init();

    let mut world = World::default();

    world.spawn(Name::from("Hello"));
}
