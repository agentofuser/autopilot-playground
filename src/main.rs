use autopilot::key::*;

fn sleep(n: u64) {
    std::thread::sleep(std::time::Duration::from_millis(n * 1000));
}

fn main() {
    // this works
    type_string("hello world", &[], 0.0, 0.0);

    // expected: HELLO WORLD
    // actual: nothing happens
    sleep(4);
    type_string("hello world", &[Flag::Shift], 0.0, 0.0);

    // expected: _
    // actual: nothing happens
    sleep(4);
    tap(&Character('-'), &[Flag::Shift], 0, 0);

    // this works (Command+Tab to browser)
    tap(&Code(KeyCode::Tab), &[Flag::Meta], 0, 0);
    tap(&Code(KeyCode::Return), &[], 0, 0);

    // expected: open new tab (Command+t)
    // actual: nothing happens
    sleep(4);
    tap(&Character('t'), &[Flag::Meta], 0, 0);

    // this works (Command+Tab back to terminal)
    sleep(4);
    tap(&Code(KeyCode::Tab), &[Flag::Meta], 0, 0);
    tap(&Code(KeyCode::Return), &[], 0, 0);

    // this works
    sleep(4);
    tap(&Code(KeyCode::Backspace), &[], 0, 0);
    tap(&Character('D'), &[], 0, 0);

    // this doesn't work (trying to leave fullscreen mode)
    sleep(4);
    tap(&Character('f'), &[Flag::Meta, Flag::Control], 0, 0);
}
