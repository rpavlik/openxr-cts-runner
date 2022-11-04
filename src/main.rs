// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use ctsrunner::CtsLauncherApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eprintln!("{}", std::env::current_dir().unwrap().display());
    eframe::run_native(
        "OpenXR CTS Launcher",
        options,
        Box::new(|cc| Box::new(CtsLauncherApp::new(cc))),
    );
}
