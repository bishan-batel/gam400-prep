use bevy_app::{App, Plugin, PluginGroup, Startup};
use bevy_ecs::{
    component::Component,
    system::{Commands, Res},
};
use render::plugin::WinitPlugin;

fn main() {
    App::new().add_plugins(WinitPlugin::default()).run();
}
