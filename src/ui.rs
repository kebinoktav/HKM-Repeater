use std::sync::Arc;
use std::thread;

use device_query::DeviceState;
use eframe::egui;
use enigo::{Enigo, Settings};

use crate::logic::Data;

pub(crate) struct AutoClick {
    data: Arc<Data>,
    string_time: String,
    time_option: String,
    process_thread: Option<thread::JoinHandle<()>>,
    input: String,
    output: String,
}

impl AutoClick {
    pub(crate) fn new() -> Self {
        let data = Arc::new(Data::new());
        let thread_data = Arc::clone(&data);

        // buat thread terpisah untuk process()
        let process_thread = thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            thread_data.process(&device_state, &mut enigo);
        });

        Self {
            data,
            string_time: String::new(),
            time_option: "3".to_string(),
            process_thread: Some(process_thread),
            input: String::new(),
            output: String::new(),
        }
    }
}

impl Drop for AutoClick {
    // untuk memastikan thread worker mati
    fn drop(&mut self) {
        // set flag stop
        self.data.stop();
        if let Some(handle) = self.process_thread.take() {
            let _ = handle.join();
        }
    }
}

impl eframe::App for AutoClick {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply a custom dark theme: black backgrounds and white text for
        // non‑interactive elements. This improves readability on dark screens.
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::BLACK;
        style.visuals.window_fill = egui::Color32::BLACK;
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::BLACK;
        // Ensure text for non‑interactive, inactive, hovered, and active
        // widgets is white (instead of the default grey).
        let white_stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
        style.visuals.widgets.noninteractive.fg_stroke = white_stroke;
        style.visuals.widgets.inactive.fg_stroke = white_stroke;
        style.visuals.widgets.hovered.fg_stroke = white_stroke;
        style.visuals.widgets.active.fg_stroke = white_stroke;
        ctx.set_style(style);
        if let Ok(option) = self.time_option.parse::<u8>() {
            self.data.give_time_option(option);
        }

        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            // click interval
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Time: ").size(15.0));
                ui.add(egui::TextEdit::singleline(&mut self.string_time).desired_width(100.0));
                if ui.button("Confirm").clicked() {
                    match self.string_time.parse::<u64>() {
                        Ok(t) => {
                            self.data.give_time(t);
                            self.data.check_time();
                        }
                        Err(_) => {}
                    }
                }

                ui.label(egui::RichText::new("Option Time: ").size(15.0));
                egui::ComboBox::from_id_salt("time opt")
                    .selected_text(&self.time_option)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.time_option, "1".to_string(), "Hours");
                        ui.selectable_value(&mut self.time_option, "2".to_string(), "Minutes");
                        ui.selectable_value(&mut self.time_option, "3".to_string(), "Seconds");
                        ui.selectable_value(&mut self.time_option, "4".to_string(), "Millis");
                        ui.selectable_value(&mut self.time_option, "5".to_string(), "Micros");
                        ui.selectable_value(&mut self.time_option, "6".to_string(), "Nanos");
                    });
            });
        });

        const KEYBOARD_KEY: &[&str] = &[
            // Baris angka
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
            // 0-9
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
            // Simbol & tanda baca
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
            // Modifier & kontrol
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
            // Navigasi
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
            // Fungsi
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
            // Mouse
            "Mouse Left",
            "Mouse Middle",
            "Mouse Right",
        ];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Input: ").size(18.0));
                    egui::ComboBox::from_id_salt("input opt")
                        .selected_text(&self.input)
                        .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(100.0)
                                .show(ui, |ui| {
                                    for key in KEYBOARD_KEY {
                                        ui.selectable_value(&mut self.input, key.to_string(), *key);
                                    }
                                });
                        });
                    let i = format!("Output Current: {}", &self.input);
                    ui.label(i);
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Output: ").size(18.0));
                    egui::ComboBox::from_id_salt("output opt")
                        .selected_text(&self.output)
                        .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(100.0)
                                .show(ui, |ui| {
                                    for key in KEYBOARD_KEY {
                                        ui.selectable_value(
                                            &mut self.output,
                                            key.to_string(),
                                            *key,
                                        );
                                    }
                                });
                        });
                    let o = format!("Output Current: {}", &self.output);
                    ui.label(o);
                });
                if ui.button("Submit").clicked() {
                    self.data.give_data(&self.input, &self.output);
                    let d: Vec<Option<crate::logic::Trigger>> = self.data.get_data_io();
                    if let (Some(i_data), Some(o_data)) = (&d[0], &d[1]) {
                        println!("Input Value: {:?}", i_data);
                        println!("Iutput Value: {:?}", o_data);
                    }
                }
            });
        });
    }
}
