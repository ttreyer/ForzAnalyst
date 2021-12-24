use forzanalyst::app::App;

fn main() {
    eframe::run_native(
        Box::new(App::default()),
        eframe::NativeOptions {
            always_on_top: false,
            decorated: true,
            icon_data: None,
            initial_window_size: None,
            maximized: true,
            drag_and_drop_support: false,
            resizable: true,
            transparent: false,
        },
    )
}
