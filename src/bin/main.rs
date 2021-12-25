use forzanalyst::app::App;

fn main() {
    eframe::run_native(
        Box::new(App::default()),
        eframe::NativeOptions {
            maximized: true,
            drag_and_drop_support: true,
            ..Default::default()
        },
    )
}
