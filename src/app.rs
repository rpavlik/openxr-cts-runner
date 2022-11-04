// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{config::Config, state::State};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CtsLauncherApp {
    config: Config,
    #[serde(skip)]
    state: State,
}

impl Default for CtsLauncherApp {
    fn default() -> Self {
        Self {
            config: Default::default(),
            state: State::Configuring,
        }
    }
}

impl CtsLauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // restore from persistent storage, if possible
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // TODO customize look and feel here
        Default::default()
    }
}

// impl CtsLauncherApp {
//     fn add_config_parts(&self)
// }
impl eframe::App for CtsLauncherApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let state = self.state.take()
        match &self.state {
            State::Configuring => {
                let should_launch = egui::TopBottomPanel::bottom("launch")
                    .show(ctx, |ui| ui.add(egui::Button::new("Launch")).clicked())
                    .inner;
                let config_result = egui::CentralPanel::default()
                    .show(ctx, |ui| ui.vertical(|ui| self.config.add_ui(ui)).inner)
                    .inner;
                self.config = config_result.new_config;
                if should_launch || config_result.should_launch {
                    self.state = State::start_running(&self.config);
                }
            }
            State::Running(_) => {
                let cancel = egui::TopBottomPanel::bottom("cancel")
                    .show(ctx, |ui| ui.add(egui::Button::new("Cancel")).clicked())
                    .inner;
                let _ = egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_enabled_ui(false, |ui| self.config.add_ui(ui))
                });
                if cancel {
                    self.state = State::Configuring;
                }
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }
}
