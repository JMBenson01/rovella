use raw_window_handle::HasRawWindowHandle;
use crate::event::EventManager;
use crate::platform::*;

pub struct App {
    window: Window,
    event_manager: EventManager,
    running: bool
}

impl App {
    /// Creates the application by creating the window and event manager
    #[inline]
    pub fn create(name: &'static str, x: i16, y: i16, width: u16, height: u16) -> Option<Self> {
        if cfg!(debug_assertions) {
            //log_info!("Debug Mode is on, so logging will be more verbose.");
        }

        let proto_window = Window::new(name, width, height, x, y);

        if proto_window.is_none() {
            log_fatal!("Failed to create window");
            return None;
        }

        return Some(App {
            window: proto_window.unwrap(),
            event_manager: EventManager::new(),
            running: true
        })
    }

    /// Gets the next event
    #[inline]
    pub fn poll_events(&mut self) -> Option<crate::event::Event> {
        // Todo: Benchmark this, while its simpler, it causes excess calls
        self.window.update(self.event_manager.get_event_que());
        return self.event_manager.poll_events();
    }

    /// Sets an internal 'running' boolean to false
    #[inline]
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Gets whether or not the application is running
    #[inline]
    pub fn is_running(&self) -> bool {
        return self.running;
    }

    /// Shuts the application down, freeing memory and calling any necessary
    /// functions on the platform or with the renderer
    #[inline]
    pub fn shutdown(&self) {
        self.window.shutdown();
    }

    #[inline]
    pub fn get_raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return self.window.raw_window_handle();
    }

    #[inline]
    pub fn get_window_ref(&self) -> &Window {
        return &self.window;
    }
}