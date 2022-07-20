# Rovella
#### A game engine/library which will be multipurpose but will be primarily aimed at 2D games and visual novels.
#### Note: This version is unstable

## Features

- Wraps Platform windowing code
- Wraps Platform events
- Wrappers are thin with minimal overhead (or at least planned minimal overhead)

### Planned

- New features will have optional usage
- A simplified frontend api 
- Multithreading API with SIMD, OMP, GPGPU and standard threading support
- Some rendering helper methods/functions (similar to glfw)
- A renderer (this is long term)
- An input manager

## Platforms:

- Windows (Full Support - Not well tested)
- Linux (X11 and XCB) (Partial Support - Window Close Event not working - Not well tested)
- MacOS (Planned)

## Example Program
Note that this just creates a window, a programmer must call `Window::get_platform_window_data()` on
their window object.
```rust
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
        500,
        500,
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
```
