use bevy_app::{App, FixedUpdate, Update};
use bevy_asset::{AssetPlugin, AssetServer};
use memory_stats::memory_stats;
use render::plugin::GamRenderer;

fn main() {
    App::new()
        .add_plugins((AssetPlugin::default(), GamRenderer))
        .add_systems(Update, print_usages)
        .run();
}

fn print_usages() {
    let Some(usage) = memory_stats() else {
        println!("Cant get usage");
        return;
    };

    println!("{}", human_bytes::human_bytes(usage.physical_mem as f64));
}
