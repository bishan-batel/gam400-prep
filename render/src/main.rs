use bevy_app::{App, FixedUpdate};
use bevy_asset::{AssetPlugin, AssetServer};
use render::plugin::WinitPlugin;

fn main() {
    App::new()
        .add_plugins((AssetPlugin::default(), WinitPlugin))
        .add_systems(FixedUpdate, system)
        .run();
}

fn system() {}
