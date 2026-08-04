use std::sync::Arc;

use bevy_app::App;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::{renderer, window::Window};

/// Internal winit-compatable app for listening to winit events / managing the winit window. This
/// also takes constrol of updating bevy's world(s)
#[derive(Debug)]
pub(crate) struct WinitApp {
    bevy: App,
}

impl WinitApp {
    pub fn new(bevy: App) -> Self {
        Self { bevy }
    }

    pub fn bevy(&self) -> &App {
        &self.bevy
    }

    pub fn bevy_mut(&mut self) -> &mut App {
        &mut self.bevy
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.bevy.world().contains_resource::<Window>() {
            return;
        }

        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Hello World"))
            .map(Arc::new)
            .expect("Failed to create window");

        log::trace!("Setting up resources");

        let (device, queue, adapter, instance, window) =
            pollster::block_on(renderer::setup_init_render_resources(window))
                .expect("Failed to setup render resources");

        self.bevy.insert_resource(device);
        self.bevy.insert_resource(queue);
        self.bevy.insert_resource(adapter);
        self.bevy.insert_resource(instance);
        self.bevy.insert_resource(window);

        log::info!(
            "Inserted Render Device, Queue, Adapter, Instance, and Window into Bevy Resource's"
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.bevy.world().get_resource::<Window>() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        if event == WindowEvent::RedrawRequested {
            self.bevy.update();
        }
    }
}
