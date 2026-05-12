use std::sync::Arc;

use bevy_app::App;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Debug)]
pub(crate) struct WinitApp {
    bevy: App,
    window: Option<Arc<Window>>,
}

impl WinitApp {
    pub fn new(bevy: App) -> Self {
        Self { bevy, window: None }
    }

    #[must_use]
    pub fn bevy(&self) -> &App {
        &self.bevy
    }

    #[must_use]
    pub fn bevy_mut(&mut self) -> &mut App {
        &mut self.bevy
    }

    #[must_use]
    pub fn window(&self) -> Option<&Arc<Window>> {
        self.window.as_ref()
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        self.window = event_loop
            .create_window(WindowAttributes::default().with_title("Hello World"))
            .map(Arc::new)
            .ok();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_deref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => (),
            _ => {}
        }
    }
}
