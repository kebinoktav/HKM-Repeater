use std::str::FromStr;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time::{Duration, Instant}};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Direction::Click, Enigo, Key, Keyboard, Mouse};

#[derive(Clone,Debug)]
pub(crate) enum Trigger {
    Key(Keycode),
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

pub(crate) struct Data {
    input: Arc<Mutex<Option<Trigger>>>,
    output: Arc<Mutex<Option<Trigger>>>,
    is_run: Arc<AtomicBool>,
    should_run: Arc<AtomicBool>, // beda dari is_run: ini buat matiin thread total
    time: Arc<AtomicU64>,
    time_opt: Arc<AtomicU8>,
}

pub(crate) fn parse_data(data: &str) -> Option<Trigger> {
    match data {
        "m_left" | "ml" => Some(Trigger::MouseLeft),
        "m_middle" | "mm" => Some(Trigger::MouseMiddle),
        "m_right" | "mr" => Some(Trigger::MouseRight),
        "" => None,
        _ => Keycode::from_str(&data.to_uppercase()).ok().map(Trigger::Key),
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
            if let Some(c) = format!("{:?}", k).chars().next() {
                let _ = enigo.key(Key::Unicode(c.to_ascii_lowercase()), Click);
            }
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
            input: Arc::new(Mutex::new(None)),
            output: Arc::new(Mutex::new(None)),
            is_run: Arc::new(AtomicBool::new(false)),
            should_run: Arc::new(AtomicBool::new(true)),
            time: Arc::new(AtomicU64::new(0)),
            time_opt: Arc::new(AtomicU8::new(0)),
        }
    }

    pub(crate) fn get_input(&self) -> Option<Trigger> {
        self.input.lock().unwrap().clone()
    }

    pub(crate) fn give_data(&self, input: &str, output: &str) {
        *self.input.lock().unwrap() = parse_data(input);
        *self.output.lock().unwrap() = parse_data(output);
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

    pub(crate) fn give_time_option(&self, option: u8) {
        self.time_opt.store(option, Ordering::SeqCst);
    }

    pub(crate) fn reverse_run_state(&self) {
        self.is_run.fetch_xor(true, Ordering::SeqCst);
    }

    pub(crate) fn is_running(&self) -> bool {
        self.is_run.load(Ordering::SeqCst)
    }

    pub(crate) fn sleep_interval(&self) {
        let time = self.time.load(Ordering::SeqCst);
        let option = self.time_opt.load(Ordering::SeqCst);

        let duration = match option {
            1 => Duration::from_hours(time),
            2 => Duration::from_mins(time),
            3 => Duration::from_secs(time),
            4 => Duration::from_millis(time),
            5 => Duration::from_micros(time),
            6 => Duration::from_nanos(time),  
            _ => Duration::from_millis(10),
        };

        thread::sleep(duration);
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

            // SELALU cek input tiap 10ms, nggak peduli is_run atau nggak
            let input_active = {
                let input = self.input.lock().unwrap();
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
                    let output_trigger = self.output.lock().unwrap().clone();
                    if let Some(trigger) = output_trigger {
                        execute_trigger(&trigger, enigo);
                    }
                    last_execute = Instant::now();
                }
            }

            thread::sleep(Duration::from_millis(10)); // <- SATU delay kecil ini aja, apapun state-nya
        }
    }
}
