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
