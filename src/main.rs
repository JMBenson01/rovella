use crate::event::{EventType};
use crate::keys::Key;

#[macro_use]
extern crate rovella_logger;
extern crate raw_window_handle;

#[cfg(target_os = "linux")]
extern crate libc;

pub mod event;
pub mod keys;
pub mod platform;
pub mod application;

fn main() {
    let mut app: application::App = application::App::create(
        "Vulkan",
        15,
        15,
        1920,
        1080
    ).unwrap(); // Only if your lazy :)

    // Note: I haven't tested the raw window handle much so it may have bugs
    let handle = app.get_raw_window_handle();

    while app.is_running() {

        let event_op = app.poll_events();
        if event_op.is_none() {
            continue;
        }

        let event = event_op.unwrap();

        match event.e_type {
            EventType::WinClose => {
                app.quit();
            }
            EventType::KeyDown => {
                if event.get_key() == Key::Escape {
                    app.quit();
                }
            }
            _ => {}
        }
    }

    app.shutdown();

}