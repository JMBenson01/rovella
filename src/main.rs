use event::*;
use keys::Key;

#[macro_use]
extern crate rovella_logger;
extern crate raw_window_handle;
extern crate wgpu;

#[cfg(target_os = "linux")]
extern crate libc;

pub mod application;
pub mod event;
pub mod graphics;
pub mod keys;
pub mod platform;

fn main() {
    let mut app: application::App =
        application::App::create("hello world", 15, 15, 1920, 1080).unwrap(); // Only if your lazy :)

    let rend_op = graphics::Renderer::new(app.get_window_ref());

    if rend_op.is_none() {
        return;
    }

    let mut renderer = rend_op.unwrap();

    while app.is_running() {
        renderer.render();

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
