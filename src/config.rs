// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GraphicsApi {
    Vulkan,
    D3D11,
    D3D12,
    OpenGL,
}

fn graphics_api_selectable_value(
    ui: &mut egui::Ui,
    current: &mut GraphicsApi,
    selected: GraphicsApi,
) {
    ui.selectable_value(current, selected, format!("{:?}", selected));
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    graphics_api: GraphicsApi,
    noninteractive: bool,
    more_args: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            graphics_api: GraphicsApi::Vulkan,
            noninteractive: true,
            more_args: Default::default(),
        }
    }
}

pub struct ConfigResult {
    pub should_launch: bool,
    pub new_config: Config,
}

impl Config {
    pub fn add_ui(&self, ui: &mut egui::Ui) -> ConfigResult {
        let mut graphics_api = self.graphics_api;
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Graphics API")
                .selected_text(format!("{:?}", self.graphics_api))
                .show_ui(ui, |ui| {
                    graphics_api_selectable_value(ui, &mut graphics_api, GraphicsApi::Vulkan);
                    graphics_api_selectable_value(ui, &mut graphics_api, GraphicsApi::D3D11);
                    graphics_api_selectable_value(ui, &mut graphics_api, GraphicsApi::D3D12);
                    graphics_api_selectable_value(ui, &mut graphics_api, GraphicsApi::OpenGL);
                })
        });
        let mut noninteractive = self.noninteractive;
        ui.checkbox(&mut noninteractive, "Skip interactive tests");

        let mut more_args = self.more_args.clone();
        let should_launch = ui
            .horizontal(|ui| {
                ui.label("Additional arguments");
                let response = ui.text_edit_singleline(&mut more_args);
                return response.lost_focus() && ui.input().key_pressed(egui::Key::Enter);
            })
            .inner;
        ConfigResult {
            should_launch,
            new_config: Config {
                graphics_api,
                noninteractive,
                more_args,
            },
        }
    }
}
