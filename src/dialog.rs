pub fn save_file_dialog() -> Option<String> {
    if cfg!(target_os = "macos") {
        tinyfiledialogs::save_file_dialog_with_filter(
            "Select where to store telemetry",
            "",
            &["ftm"],
            "ForzAnalyst telemetry",
        )
    } else {
        rfd::FileDialog::new()
            .set_title("Select where to store telemetry")
            .add_filter("ForzAnalyst telemetry", &["ftm"])
            .save_file()
            .and_then(|path| path.to_str().map(|s| s.to_owned()))
    }
}

pub fn pick_file_dialog() -> Option<String> {
    if cfg!(target_os = "macos") {
        tinyfiledialogs::open_file_dialog(
            "Select telemetry file to open",
            "",
            Some((&["*.ftm"], "ForzAnalyst telemetry")),
        )
    } else {
        rfd::FileDialog::new()
            .set_title("Select telemetry file to open")
            .add_filter("ForzAnalyst telemetry", &["ftm"])
            .pick_file()
            .and_then(|path| path.to_str().map(|s| s.to_owned()))
    }
}

pub fn error_dialog(title: &str, description: &str) {
    if cfg!(target_os = "macos") {
        tinyfiledialogs::message_box_ok(title, description, tinyfiledialogs::MessageBoxIcon::Error);
    } else {
        rfd::MessageDialog::new()
            .set_title(title)
            .set_description(description)
            .set_level(rfd::MessageLevel::Error)
            .show();
    }
}
