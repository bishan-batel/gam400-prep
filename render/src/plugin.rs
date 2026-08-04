use std::num::NonZeroU8;

use bevy_app::{App, AppExit, Plugin};
use bevy_ecs::resource::Resource;
use winit::event_loop::EventLoop;

use crate::winit_app::WinitApp;

/// Plugin for using the test GAM400 custom rendererer for bevy
#[derive(Debug, Default)]
pub struct GamRenderer;

impl Plugin for GamRenderer {
    fn build(&self, app: &mut App) {
        app.set_runner(winit_runner);
    }
}

/// runner for the app using winit
fn winit_runner(mut app: App) -> AppExit {
    let event_loop = match EventLoop::new() {
        Ok(ev) => ev,
        Err(err) => {
            log::error!("Failed to create event loop: {err}");
            return AppExit::Error(NonZeroU8::new(1).expect("1 is not zero."));
        }
    };

    app.finish();
    app.cleanup();

    if let Err(err) = event_loop.run_app(&mut WinitApp::new(app)) {
        log::error!("Failed to run event loop: {err}");
        return AppExit::Error(NonZeroU8::new(2).expect("2 is non-zero."));
    }

    AppExit::Success
}
