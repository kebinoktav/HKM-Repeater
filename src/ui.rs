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
                
                    ui.label(egui::RichText::new("Option: ").size(15.0));
                    egui::ComboBox::from_id_salt("time opt").selected_text(&self.time_option).show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.time_option, "1".to_string(),"Hours");
                        ui.selectable_value(&mut self.time_option, "2".to_string(),"Minutes");
                        ui.selectable_value(&mut self.time_option, "3".to_string(),"Seconds");
                        ui.selectable_value(&mut self.time_option, "4".to_string(),"Millis");
                        ui.selectable_value(&mut self.time_option, "5".to_string(),"Micros");
                        ui.selectable_value(&mut self.time_option, "6".to_string(),"Nanos");
                    });
                
            });
            
        });
        // bagian kirim input output
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui|{
                    ui.label(egui::RichText::new("Input: ").size(18.0));
                    ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(55.0));
                    
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Output: ").size(18.0));
                     ui.add(egui::TextEdit::singleline(&mut self.output).desired_width(55.0));
                });
                if ui.button("Submit").clicked() {
                    self.data.give_data(&self.input, &self.output);
                    if let Some(check_i) = self.data.get_input() {
                        println!("Input Status: {:?}", check_i);
                    }
                }
            });
            
        });
        
    }
}