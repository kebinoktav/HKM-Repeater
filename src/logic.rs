use std::str::FromStr;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::{
    thread,
    time::{Duration, Instant},
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Direction::Click, Enigo, Key, Keyboard, Mouse};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub(crate) enum Trigger {
    Key(Keycode),
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

pub(crate) struct Data {
    input: Arc<RwLock<Option<Trigger>>>,
    output: Arc<RwLock<Option<Trigger>>>,
    is_run: Arc<AtomicBool>,
    should_run: Arc<AtomicBool>, // Different from is_run: this stops the entire worker thread
    time: Arc<AtomicU64>,
    time_opt: Arc<AtomicU8>,
}

pub(crate) fn parse_data(data: &str) -> Option<Trigger> {
    match data {
        "Mouse Left" | "m_left" | "ml" => Some(Trigger::MouseLeft),
        "Mouse Middle" | "m_middle" | "mm" => Some(Trigger::MouseMiddle),
        "Mouse Right" | "m_right" | "mr" => Some(Trigger::MouseRight),

        // Letters
        "A" => Some(Trigger::Key(Keycode::A)),
        "B" => Some(Trigger::Key(Keycode::B)),
        "C" => Some(Trigger::Key(Keycode::C)),
        "D" => Some(Trigger::Key(Keycode::D)),
        "E" => Some(Trigger::Key(Keycode::E)),
        "F" => Some(Trigger::Key(Keycode::F)),
        "G" => Some(Trigger::Key(Keycode::G)),
        "H" => Some(Trigger::Key(Keycode::H)),
        "I" => Some(Trigger::Key(Keycode::I)),
        "J" => Some(Trigger::Key(Keycode::J)),
        "K" => Some(Trigger::Key(Keycode::K)),
        "L" => Some(Trigger::Key(Keycode::L)),
        "M" => Some(Trigger::Key(Keycode::M)),
        "N" => Some(Trigger::Key(Keycode::N)),
        "O" => Some(Trigger::Key(Keycode::O)),
        "P" => Some(Trigger::Key(Keycode::P)),
        "Q" => Some(Trigger::Key(Keycode::Q)),
        "R" => Some(Trigger::Key(Keycode::R)),
        "S" => Some(Trigger::Key(Keycode::S)),
        "T" => Some(Trigger::Key(Keycode::T)),
        "U" => Some(Trigger::Key(Keycode::U)),
        "V" => Some(Trigger::Key(Keycode::V)),
        "W" => Some(Trigger::Key(Keycode::W)),
        "X" => Some(Trigger::Key(Keycode::X)),
        "Y" => Some(Trigger::Key(Keycode::Y)),
        "Z" => Some(Trigger::Key(Keycode::Z)),

        // Symbols
        "`" => Some(Trigger::Key(Keycode::Grave)),
        "-" => Some(Trigger::Key(Keycode::Minus)),
        "=" => Some(Trigger::Key(Keycode::Equal)),
        "[" => Some(Trigger::Key(Keycode::LeftBracket)),
        "]" => Some(Trigger::Key(Keycode::RightBracket)),
        "\\" => Some(Trigger::Key(Keycode::BackSlash)),
        ";" => Some(Trigger::Key(Keycode::Semicolon)),
        "'" => Some(Trigger::Key(Keycode::Apostrophe)),
        "," => Some(Trigger::Key(Keycode::Comma)),
        "." => Some(Trigger::Key(Keycode::Dot)),
        "/" => Some(Trigger::Key(Keycode::Slash)),

        // Control
        "Tab" => Some(Trigger::Key(Keycode::Tab)),
        "CapsLock" => Some(Trigger::Key(Keycode::CapsLock)),
        "Shift" => Some(Trigger::Key(Keycode::LShift)),
        "Ctrl" => Some(Trigger::Key(Keycode::LControl)),
        "Alt" => Some(Trigger::Key(Keycode::LAlt)),
        "Space" => Some(Trigger::Key(Keycode::Space)),
        "Enter" => Some(Trigger::Key(Keycode::Enter)),
        "Backspace" => Some(Trigger::Key(Keycode::Backspace)),
        "Escape" => Some(Trigger::Key(Keycode::Escape)),

        // Navigation
        "ArrowUp" => Some(Trigger::Key(Keycode::Up)),
        "ArrowDown" => Some(Trigger::Key(Keycode::Down)),
        "ArrowLeft" => Some(Trigger::Key(Keycode::Left)),
        "ArrowRight" => Some(Trigger::Key(Keycode::Right)),
        "Home" => Some(Trigger::Key(Keycode::Home)),
        "End" => Some(Trigger::Key(Keycode::End)),
        "PageUp" => Some(Trigger::Key(Keycode::PageUp)),
        "PageDown" => Some(Trigger::Key(Keycode::PageDown)),
        "Insert" => Some(Trigger::Key(Keycode::Insert)),
        "Delete" => Some(Trigger::Key(Keycode::Delete)),

        // Function
        "F1" => Some(Trigger::Key(Keycode::F1)),
        "F2" => Some(Trigger::Key(Keycode::F2)),
        "F3" => Some(Trigger::Key(Keycode::F3)),
        "F4" => Some(Trigger::Key(Keycode::F4)),
        "F5" => Some(Trigger::Key(Keycode::F5)),
        "F6" => Some(Trigger::Key(Keycode::F6)),
        "F7" => Some(Trigger::Key(Keycode::F7)),
        "F8" => Some(Trigger::Key(Keycode::F8)),
        "F9" => Some(Trigger::Key(Keycode::F9)),
        "F10" => Some(Trigger::Key(Keycode::F10)),
        "F11" => Some(Trigger::Key(Keycode::F11)),
        "F12" => Some(Trigger::Key(Keycode::F12)),

        "" => None,

        _ => None,
    }
}

pub(crate) fn is_trigger_active(trigger: &Trigger, device_state: &DeviceState) -> bool {
    match trigger {
        Trigger::Key(k) => device_state.get_keys().contains(k),
        Trigger::MouseLeft => device_state.get_mouse().button_pressed[1],
        Trigger::MouseRight => device_state.get_mouse().button_pressed[2],
        Trigger::MouseMiddle => device_state.get_mouse().button_pressed[3],
    }
}


fn execute_trigger(trigger: &Trigger, enigo: &mut Enigo) {
    match trigger {
        Trigger::Key(k) => {
            let key = match k {
                // Letters
                Keycode::A => Key::Unicode('a'),
                Keycode::B => Key::Unicode('b'),
                Keycode::C => Key::Unicode('c'),
                Keycode::D => Key::Unicode('d'),
                Keycode::E => Key::Unicode('e'),
                Keycode::F => Key::Unicode('f'),
                Keycode::G => Key::Unicode('g'),
                Keycode::H => Key::Unicode('h'),
                Keycode::I => Key::Unicode('i'),
                Keycode::J => Key::Unicode('j'),
                Keycode::K => Key::Unicode('k'),
                Keycode::L => Key::Unicode('l'),
                Keycode::M => Key::Unicode('m'),
                Keycode::N => Key::Unicode('n'),
                Keycode::O => Key::Unicode('o'),
                Keycode::P => Key::Unicode('p'),
                Keycode::Q => Key::Unicode('q'),
                Keycode::R => Key::Unicode('r'),
                Keycode::S => Key::Unicode('s'),
                Keycode::T => Key::Unicode('t'),
                Keycode::U => Key::Unicode('u'),
                Keycode::V => Key::Unicode('v'),
                Keycode::W => Key::Unicode('w'),
                Keycode::X => Key::Unicode('x'),
                Keycode::Y => Key::Unicode('y'),
                Keycode::Z => Key::Unicode('z'),

                // Symbols
                Keycode::Grave => Key::Unicode('`'),
                Keycode::Minus => Key::Unicode('-'),
                Keycode::Equal => Key::Unicode('='),
                Keycode::LeftBracket => Key::Unicode('['),
                Keycode::RightBracket => Key::Unicode(']'),
                Keycode::BackSlash => Key::Unicode('\\'),
                Keycode::Semicolon => Key::Unicode(';'),
                Keycode::Apostrophe => Key::Unicode('\''),
                Keycode::Comma => Key::Unicode(','),
                Keycode::Dot => Key::Unicode('.'),
                Keycode::Slash => Key::Unicode('/'),

                // Control
                Keycode::Tab => Key::Tab,
                Keycode::CapsLock => Key::CapsLock,
                Keycode::LShift => Key::LShift,
                Keycode::LControl => Key::LControl,
                Keycode::LAlt => Key::Alt,
                Keycode::Space => Key::Space,
                Keycode::Enter => Key::Return,
                Keycode::Backspace => Key::Backspace,
                Keycode::Escape => Key::Escape,

                // Navigation
                Keycode::Up => Key::UpArrow,
                Keycode::Down => Key::DownArrow,
                Keycode::Left => Key::LeftArrow,
                Keycode::Right => Key::RightArrow,
                Keycode::Home => Key::Home,
                Keycode::End => Key::End,
                Keycode::PageUp => Key::PageUp,
                Keycode::PageDown => Key::PageDown,
                Keycode::Insert => Key::Insert,
                Keycode::Delete => Key::Delete,

                // Function
                Keycode::F1 => Key::F1,
                Keycode::F2 => Key::F2,
                Keycode::F3 => Key::F3,
                Keycode::F4 => Key::F4,
                Keycode::F5 => Key::F5,
                Keycode::F6 => Key::F6,
                Keycode::F7 => Key::F7,
                Keycode::F8 => Key::F8,
                Keycode::F9 => Key::F9,
                Keycode::F10 => Key::F10,
                Keycode::F11 => Key::F11,
                Keycode::F12 => Key::F12,

                _ => return,
            };

            let _ = enigo.key(key, Click);
        }

        Trigger::MouseLeft => {
            let _ = enigo.button(Button::Left, Click);
        }

        Trigger::MouseRight => {
            let _ = enigo.button(Button::Right, Click);
        }

        Trigger::MouseMiddle => {
            let _ = enigo.button(Button::Middle, Click);
        }
    }
}

impl Data {
    pub(crate) fn new() -> Self {
        Data {
            input: Arc::new(RwLock::new(None)),
            output: Arc::new(RwLock::new(None)),
            is_run: Arc::new(AtomicBool::new(false)),
            should_run: Arc::new(AtomicBool::new(true)),
            time: Arc::new(AtomicU64::new(0)),
            time_opt: Arc::new(AtomicU8::new(0)),
        }
    }

    pub(crate) fn get_data_io(&self) -> Vec<Option<Trigger>> {
        // The current input/output state is read from self.input and self.output.
        let data: Vec<Option<Trigger>> = vec![
            self.input.read().unwrap().clone(),
            self.output.read().unwrap().clone(),
        ];
        data
    }

    // we convert &str from struct to enum Trigger
    pub(crate) fn give_data(&self, input: &str, output: &str) {
        *self.input.write().unwrap() = parse_data(input);
        *self.output.write().unwrap() = parse_data(output);
    }

    pub(crate) fn stop(&self) {
        self.should_run.store(false, SeqCst);
    }

    pub(crate) fn check_time(&self) {
        println!("Time: {}", self.time.load(Ordering::SeqCst));
        println!("Time Option: {}", self.time_opt.load(Ordering::SeqCst));
    }

    pub(crate) fn give_time(&self, time: u64) {
        self.time.store(time, Ordering::SeqCst);
    }

    pub(crate) fn give_time_option(&self, option: &str) {
        let u8_opt: u8 = match option {
            "Hours" => 1,
            "Minutes" => 2,
            "Seconds" => 3,
            "Millis" => 4,
            "Micros" => 5,
            _ => 3,
        };
        self.time_opt.store(u8_opt, Ordering::SeqCst);
    }

    pub(crate) fn reverse_run_state(&self) {
        self.is_run.fetch_xor(true, Ordering::SeqCst);
    }

    pub(crate) fn get_interval(&self) -> Duration {
        let time = self.time.load(Ordering::SeqCst);
        let option = self.time_opt.load(Ordering::SeqCst);

        match option {
            1 => Duration::from_hours(time),
            2 => Duration::from_mins(time),
            3 => Duration::from_secs(time),
            4 => Duration::from_millis(time),
            5 => Duration::from_micros(time),
            6 => Duration::from_nanos(time),
            _ => Duration::from_millis(10),
        }
    }

    pub(crate) fn process(&self, device_state: &DeviceState, enigo: &mut Enigo) {
        let mut input_held = false;
        let mut last_execute = Instant::now();

        loop {
            if !self.should_run.load(Ordering::SeqCst) {
                break;
            }

            // Always check the input every 10 ms, regardless of whether the loop is running or not.
            let input_active = {
                let input = self.input.read().unwrap();
                match input.as_ref() {
                    Some(trigger) => is_trigger_active(trigger, device_state),
                    None => false,
                }
            };

            if input_active && !input_held {
                self.reverse_run_state();
            }
            input_held = input_active;

            if self.is_run.load(Ordering::SeqCst) {
                let interval = self.get_interval();
                if last_execute.elapsed() >= interval {
                    let output_trigger = self.output.read().unwrap().clone();
                    if let Some(trigger) = output_trigger {
                        execute_trigger(&trigger, enigo);
                    }
                    last_execute = Instant::now();
                }
            }

            thread::sleep(Duration::from_millis(10)); // A short delay keeps polling stable and avoids a busy loop.
        }
    }
}
