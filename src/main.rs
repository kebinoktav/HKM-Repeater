mod logic;
mod ui;

fn main() -> iced::Result {
    iced::application(ui::Hkm::new, ui::Hkm::work, ui::Hkm::gui)
        .resizable(true)
        .title("HKM")
        .run()
}
