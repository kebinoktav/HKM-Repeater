use device_query::DeviceState;
use enigo::{Enigo, Settings};
use iced::{
    Background, Color, Element, widget::{button, column, container, image, pick_list, row, space, text, text_input},
};
use std::thread;
use std::{sync::Arc, u64};

use crate::{logic::Data};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Choice {
    GiveTimeOpt(String),
    GiveTime(u64),
    GiveInputOutput(),
    Fill_Input(String),
    Fill_Output(String),
}

pub struct Hkm {
    process_thread: Option<thread::JoinHandle<()>>,
    data: Arc<Data>,
    string_time: u64,
    time_option: String,
    input: String,
    output: String,
    theme: Option<[String; 2]>, // [0] = background theme, [1] = text color
}

impl Drop for Hkm {
    fn drop(&mut self) {
        self.data.stop();
        if let Some(handle) = self.process_thread.take() {}
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
            theme : None,
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
                self.data.give_data(&self.input.as_str(), &self.output.as_str());
                let debug = self.data.get_data_io();
                println!("input: {:?}", debug[0]);
                println!("output: {:?}", debug[1])
            }
            Choice::Fill_Input(v) => {
                self.input = v
            }
            Choice::Fill_Output(v) => {
                self.output = v
            }
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
            "Shift",
            "Ctrl",
            "Alt",
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
            
            
                text("Time:   "),
                text_input("Time", &self.string_time.to_string())
                    .on_input(|value| {
                        let parsed = value.parse::<u64>().unwrap_or(0);
                        Choice::GiveTime(parsed)

                    })
                    .width(100),
                space().width(10),
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
            ,
            
                text("Input"),
                pick_list(KEYBOARD_KEY, if self.input.is_empty(){
                    None
                } else {
                    Some(self.input.as_str())
                },
                |val| {Choice::Fill_Input(val.to_string())})
                ,
                space().height(10),
            
                text("Output"),
                pick_list(KEYBOARD_KEY, if self.output.is_empty(){
                    None
                } else {
                    Some(self.output.as_str())
                },
                |val| {Choice::Fill_Output(val.to_string())}
            ),
            button("Submit").on_press(Choice::GiveInputOutput())

        ];

        container(gui)
            .width(iced::Fill)
            .height(iced::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::BLACK)),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}
