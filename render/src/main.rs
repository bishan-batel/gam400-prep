use bevy_app::{App, FixedUpdate};
use bevy_asset::{AssetPlugin, AssetServer};
use render::plugin::GamRenderer;

fn main() {
    App::new()
        .add_plugins((AssetPlugin::default(), GamRenderer))
        .add_systems(FixedUpdate, system)
        .run();
}

fn system() {}
