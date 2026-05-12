use bevy_app::App;
use render::plugin::WinitPlugin;

fn main() {
    App::new().add_plugins(WinitPlugin).run();
}
