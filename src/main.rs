use crate::event::{EventManager, EventType};
use crate::keys::Key;
use crate::platform::*;

#[macro_use]
extern crate rovella_logger;

#[cfg(target_os = "linux")]
extern crate libc;

pub mod event;
pub mod keys;
pub mod platform;

fn main() {

    let mut running: bool = true;

    let mut ev_manager = EventManager::new();
    let window = Window::new(
        "hello world",
        1920,
        1080,
        100,
        100
    ).expect("window failed");

    log_info!("Initialised subsystems successfully");
    log_info!("Now entering mainloop");

    while running {
        window.update(ev_manager.get_event_que());
        loop {
            let ev_option = ev_manager.poll_events();
            if ev_option.is_none() {
                break;
            }

            let raw_event = ev_option.unwrap();

            match raw_event.e_type {
                EventType::WinClose => running = false,
                EventType::KeyDown => {
                    let key: Key = raw_event.get_key();
                    match key {
                        Key::Escape => {
                            running = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    window.shutdown();
}