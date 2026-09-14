use std::str::FromStr;
use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Direction::Click, Enigo, Keyboard, Key, Mouse};

enum Trigger {
    Key(Keycode),
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

pub(crate) struct Data {
    input: Option<Trigger>,
    output: Option<Trigger>,
    is_run: bool,
    time: u64,
    time_opt: u8,
}

fn parse_data(data: &str) -> Option<Trigger> {
    match data {
        "m1" => Some(Trigger::MouseLeft),
        "m3" => Some(Trigger::MouseMiddle),
        "m2" => Some(Trigger::MouseRight),
        "" => None,
        _ => Keycode::from_str(&data.to_uppercase()).ok().map(Trigger::Key),
    }
}

fn is_trigger_active(trigger: &Trigger, device_state: &DeviceState) -> bool {
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
            if let Some(c) = format!("{:?}", k).chars().next() {
                let _ = enigo.key(Key::Unicode(c.to_ascii_lowercase()), Click);
            }
        }
        Trigger::MouseLeft => { let _ = enigo.button(Button::Left, Click); }
        Trigger::MouseRight => { let _ = enigo.button(Button::Right, Click); }
        Trigger::MouseMiddle => { let _ = enigo.button(Button::Middle, Click); }
    }
}

impl Data {
    pub(crate) fn new() -> Self {
        Data {
            input: None,
            output: None,
            is_run: false,
            time: 0,
            time_opt: 4,
        }
    }

    pub(crate) fn get_data(&mut self, input: &str, output: &str) {
        self.input = parse_data(input);
        self.output = parse_data(output);
    }

    pub(crate) fn check_time(&self) {
        println!("Time: {}", self.time);
        println!("Time Option: {}", self.time_opt);
    }

    pub(crate) fn get_time(&mut self, time: u64) {
        self.time = time;
    }

    pub(crate) fn get_time_option(&mut self, option: u8) {
        self.time_opt = option;
    }

    pub(crate) fn reverse_run_state(&mut self) {
        self.is_run = !self.is_run;
    }

    fn sleep_interval(&self) {
    let duration = match self.time_opt {
        1 => Duration::from_secs(self.time.saturating_mul(3600)), // hours
        2 => Duration::from_secs(self.time.saturating_mul(60)),   // minutes
        3 => Duration::from_secs(self.time),                       // seconds
        4 => Duration::from_millis(self.time),                     // milliseconds
        5 => Duration::from_micros(self.time),                     // microseconds
        6 => Duration::from_nanos(self.time),                      // nanoseconds
        _ => Duration::from_millis(self.time.max(10)),
    };

    thread::sleep(duration);
}

    pub(crate) fn process(&mut self, device_state: &DeviceState, enigo: &mut Enigo) {
        let mut input_held = false; // buat edge detection tombol input

        loop {
            // Cek input trigger AKTIF sekarang atau enggak
            let input_active = match &self.input {
                Some(trigger) => is_trigger_active(trigger, device_state),
                None => false,
            };

            // Rising edge: baru DITEKAN (sebelumnya enggak, sekarang aktif)
            if input_active && !input_held {
                self.reverse_run_state(); // toggle is_run pakai tombol input yang SAMA
            }
            input_held = input_active;

            // Kalau lagi aktif spam output terus-menerus
            if self.is_run {
                if let Some(output) = &self.output {
                    execute_trigger(output, enigo);
                }
                self.sleep_interval();
            } else {
                thread::sleep(Duration::from_millis(10)); // interval idle/polling
            }
        }
    }
}