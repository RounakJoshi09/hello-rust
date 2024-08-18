#[derive(Debug)]
enum KeyPress {
    Up,
    Down,
}
struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

#[derive(Debug)]
enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(value: KeyEvent) -> Self {
        return InputEvent::Key(value.keycode, value.state);
    }
}

fn main() {
    let key_event = KeyEvent {
        keycode: 16,
        state: KeyPress::Down,
    };

    let input_event: InputEvent = key_event.into();

    let input_event_2 = InputEvent::from(KeyEvent {
        keycode: 9,
        state: KeyPress::Up,
    });

    println!("Input Event : {:?} ", input_event)
}
