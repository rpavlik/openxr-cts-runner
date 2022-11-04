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

impl eframe::App for CtsLauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { config, state } = self;

        match state {
            State::Configuring => {
                let should_launch = egui::TopBottomPanel::bottom("launch")
                    .show(ctx, |ui| ui.add(egui::Button::new("Launch")).clicked())
                    .inner;
                let config_result = egui::CentralPanel::default()
                    .show(ctx, |ui| ui.vertical(|ui| config.add_ui(ui)).inner)
                    .inner;
                *config = config_result.new_config;
                if should_launch || config_result.should_launch {
                    self.state = State::start_running(config);
                }
            }
            State::Running(data) => {
                if let Err(e) = data.poll() {
                    self.state = State::Err(e);
                    return;
                }

                let to_config = egui::TopBottomPanel::bottom("cancel")
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .add_enabled(!data.is_done(), egui::Button::new("Cancel"))
                                .clicked()
                            {
                                return true;
                            }
                            if data.is_done() {
                                return ui
                                    .with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| ui.button("OK"),
                                    )
                                    .inner
                                    .clicked();
                            }
                            false
                        })
                        .inner
                    })
                    .inner;

                let _ = egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_enabled_ui(false, |ui| self.config.add_ui(ui));
                    ui.label("Output");
                    egui::ScrollArea::new([false, true]).show(ui, |ui| ui.label(data.get_output()))
                });

                if to_config {
                    self.state = State::Configuring;
                }
            }
            State::Err(e) => {
                let confirmed = egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        ui.heading("Error!");
                        let mut buf = e.to_string();

                        ui.add_enabled(false, egui::TextEdit::multiline(&mut buf));
                        ui.button("Ok")
                    })
                    .inner
                    .clicked();
                if confirmed {
                    self.state = State::Configuring;
                }
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }
}
