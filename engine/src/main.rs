use std::{
    ops::Deref,
    time::{Duration, Instant},
};

use engine::{ecs::component::Component, prelude::*};

#[derive(Debug)]
struct Health(usize);

#[derive(Debug)]
struct Name(String);

fn main() -> eyre::Result<()> {
    let _ = color_eyre::install();

    let target_fps = Duration::from_secs_f64(1. / 120.);

    let mut world = World::default();

    world.add((Health(0),));

    world.add((Health(42), Name("hi".into())));

    loop {
        let frame_start = Instant::now();
        {
            for mut health in world.query_component_mut::<Health>() {
                health.0 += 1;
                println!("health={health:?}")
            }

            for mut name in world.query_component::<Name>() {
                println!("{name:?}")
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < target_fps {
            std::thread::sleep(target_fps - elapsed);
        }
    }

    // let fps = ticks as f64 / start_frame.elapsed().as_secs_f64();
    // println!("Average FPS: {fps}");
    // Ok(())
}
