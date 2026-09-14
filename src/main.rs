mod ui;
mod logic;
use eframe::egui;
use ui::AutoClick;

fn main() -> Result<(), eframe::Error> {
    let opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 800.0]) // Default window size
            .with_min_inner_size([400.0, 500.0]) // Prevents user from making the window too small
            .with_resizable(true) // Allows the user to resize the window
            .with_title("auto clicker") // Window title
            .with_active(true) // Window starts active
            .with_maximize_button(true), // Allows maximizing the window
        ..Default::default()
    };

    eframe::run_native(
        "Auto Clicker",
        opt,
        Box::new(|_cc| Ok(Box::new(AutoClick::default()))),
    )
}
// 12,9,2026 
