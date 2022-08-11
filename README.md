# Rovella
#### A game engine/library which will be multipurpose but will be primarily aimed at 2D games and visual novels.
#### Note: This version is unstable

## Features

- Wraps Platform windowing code
- Wraps Platform events
- Wrappers are thin with minimal overhead (or at least planned minimal overhead)
- Supports the raw-window-handle for xcb and win32 (or linux and windows)

### Planned

- New features will have optional usage
- Multithreading API with SIMD, OMP, GPGPU and standard threading support
- Some rendering helper methods/functions (similar to glfw)
- A renderer (this is long term)
- An input manager

## Platforms:

- Windows (Full Support - Not well tested)
- Linux (X11 and XCB) (Partial Support - Window Close Event not working - Not well tested)
- MacOS (Planned)

## Example Program
```rust
fn main() {
    let mut app: rovella::App = rovella::App::create(
        "hello world",
        15,
        15,
        1920,
        1080
    ).unwrap(); // Only if your lazy :)

    while app.is_running() {
        let event_op = app.poll_events();
        if event_op.is_none() {
            continue;
        }

        let event = event_op.unwrap();

        match event {
            rovella::Event::WinClose => {
                app.quit();
            }
            rovella::Event::KeyDown(key) => {
                if key == rovella::Key::Escape {
                    app.quit();
                }
            }
            _ => {}
        }
    }

    app.shutdown();
}

```
