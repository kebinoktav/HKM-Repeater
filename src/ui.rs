use device_query::DeviceState;
use enigo::{Enigo, Settings};
use iced::{
    Background, Color, Element,
    widget::{button, column, container, pick_list, row, space, text, text_input},
};
use std::thread;
use std::{sync::Arc, u64};

use crate::logic::Data;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Choice {
    GiveTimeOpt(String),
    GiveTime(u64),
    GiveInputOutput(),
    FillInput(String),
    FillOutput(String),
}

pub struct Hkm {
    process_thread: Option<thread::JoinHandle<()>>,
    data: Arc<Data>,
    string_time: u64,
    time_option: String,
    input: String,
    output: String,
    background: Option<Background>,
    text_color: Option<Color>,
}

impl Drop for Hkm {
    fn drop(&mut self) {
        self.data.stop();
        if let Some(handle) = self.process_thread.take() {
            let _ = handle.join();
        }
    }
}

impl Hkm {
    pub(crate) fn new() -> Self {
        let data = Arc::new(Data::new());
        let thread_data = Arc::clone(&data);

        // Start a separate thread for process() so the input loop runs independently.
        let process_thread = thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            thread_data.process(&device_state, &mut enigo);
        });

        Self {
            process_thread: Some(process_thread),
            data,
            string_time: 0,
            time_option: String::new(),
            input: String::new(),
            output: String::new(),
            background: Some(Background::Color(Color::BLACK)),
            text_color: Some(Color::WHITE),
        }
    }

    pub(crate) fn work(&mut self, choice: Choice) {
        match choice {
            Choice::GiveTimeOpt(v1) => {
                self.time_option = v1;
                self.data.give_time_option(&self.time_option);
                self.data.check_time();
            }
            Choice::GiveTime(v) => {
                self.string_time = v;
                self.data.give_time(v);
                self.data.check_time();
            }
            Choice::GiveInputOutput() => {
                self.data
                    .give_data(&self.input.as_str(), &self.output.as_str());
                let debug = self.data.get_data_io();
                println!("input: {:?}", debug[0]);
                println!("output: {:?}", debug[1])
            }
            Choice::FillInput(v) => self.input = v,
            Choice::FillOutput(v) => self.output = v,
        }
    }

    pub(crate) fn gui(&self) -> Element<'_, Choice> {
        const KEYBOARD_KEY: &[&str] = &[
            // Letter row for alphabetic keys shown in the selector.
            "A",
            "B",
            "C",
            "D",
            "E",
            "F",
            "G",
            "H",
            "I",
            "J",
            "K",
            "L",
            "M",
            "N",
            "O",
            "P",
            "Q",
            "R",
            "S",
            "T",
            "U",
            "V",
            "W",
            "X",
            "Y",
            "Z",
            // Numeric keys 0 through 9.
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            // Symbols and punctuation characters used for keyboard input.
            "`",
            "-",
            "=",
            "[",
            "]",
            "\\",
            ";",
            "'",
            ",",
            ".",
            "/",
            // Modifier and control keys for advanced trigger combinations.
            "Tab",
            "CapsLock",
            "L_Shift",
            "R_Shift",
            "Ctrl",
            "L_Alt",
            "R_Alt",
            "Meta",
            "Space",
            "Enter",
            "Backspace",
            "Escape",
            // Navigation keys for cursor and page movement.
            "ArrowUp",
            "ArrowDown",
            "ArrowLeft",
            "ArrowRight",
            "Home",
            "End",
            "PageUp",
            "PageDown",
            "Insert",
            "Delete",
            // Function keys F1 through F12.
            "F1",
            "F2",
            "F3",
            "F4",
            "F5",
            "F6",
            "F7",
            "F8",
            "F9",
            "F10",
            "F11",
            "F12",
            // Mouse trigger options supported by the automation logic.
            "Mouse Left",
            "Mouse Middle",
            "Mouse Right",
        ];
        let gui = column![
            row![
                text("Time:   "),
                text_input("Time", &self.string_time.to_string())
                    .on_input(|value| {
                        let parsed = value.parse::<u64>().unwrap_or(0);
                        Choice::GiveTime(parsed)
                    })
                    .width(100),
                text("Time Option: "),
                pick_list(
                    ["Hours", "Minutes", "Seconds", "Millis", "Micros"],
                    if self.time_option.is_empty() {
                        None
                    } else {
                        Some(self.time_option.as_str())
                    },
                    |val| { Choice::GiveTimeOpt(val.to_string()) }
                )
            ]
            .spacing(10),
            text("Input"),
            row![
                pick_list(
                    KEYBOARD_KEY,
                    if self.input.is_empty() {
                        None
                    } else {
                        Some(self.input.as_str())
                    },
                    |val| { Choice::FillInput(val.to_string()) }
                ),
                text(format!("current input: {:?}", self.data.get_data_io().get(0).and_then(|x| {x.as_ref()})))
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ].spacing(10),
            space().height(5),
            text("Output"),
            row![
                pick_list(
                    KEYBOARD_KEY,
                    if self.output.is_empty() {
                        None
                    } else {
                        Some(self.output.as_str())
                    },
                    |val| { Choice::FillOutput(val.to_string()) }
                ),
                text(format!("current output: {:?}", self.data.get_data_io().get(1).and_then(|x| {x.as_ref()})))
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ].spacing(10),
            space().height(5),
            button("Submit")
                .on_press(Choice::GiveInputOutput())
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(Color::from_rgb8(11, 121, 67))),
                    text_color: Color::WHITE,
                    ..Default::default()
                })
        ];

        container(gui)
            .width(iced::Fill)
            .height(iced::Fill)
            .style(|_theme| container::Style {
                background: self.background,
                text_color: self.text_color,
                ..Default::default()
            })
            .into()
    }
}
