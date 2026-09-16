mod logic;
mod ui;
use eframe::egui;
use ui::AutoClick;

fn main() -> Result<(), eframe::Error> {
    let opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0]) // Default window size
            .with_min_inner_size([400.0, 500.0]) // Prevents user from making the window too small
            .with_max_inner_size([600.0, 600.0])
            .with_resizable(true) // Allows the user to resize the window
            .with_title("HKM Repeater") // Window title
            .with_active(true) // Window starts active
            .with_maximize_button(true), // Allows maximizing the window
        ..Default::default()
    };

    eframe::run_native(
        "HKM Repeater",
        opt,
        Box::new(|_cc| Ok(Box::new(AutoClick::new()))),
    )
}
// 12,9,2026
