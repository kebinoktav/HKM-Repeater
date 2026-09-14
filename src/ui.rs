use eframe::egui;

use crate::logic::Data;

pub(crate) struct AutoClick {
    data: Data,
    string_time: String,
    time_option: String,
}

impl AutoClick {
    fn get_time(&mut self,t: String) {
        self.string_time = t;
    }
}

impl Default for AutoClick {
    fn default() -> Self {
        let h = Data::new();
        Self { data: h , string_time: String::new(), time_option: String::new()}
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
            self.data.get_time_option(option);
        }

        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            // click interval
            ui.horizontal(|ui| {
                 ui.label("Time: ");
                 ui.text_edit_singleline(&mut self.string_time);
                    if ui.button("Confirm").clicked() {
                        match self.string_time.parse::<u64>() {
                            Ok(t) => {
                                self.data.get_time(t);
                                self.data.check_time();
                            },
                            Err(_) => {}
                        }
                    }
                    ui.separator();
                    ui.label("Time Option: ");
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
    }
}