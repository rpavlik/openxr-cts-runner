// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GraphicsApi {
    Vulkan,
    Vulkan2,
    D3D11,
    D3D12,
    OpenGL,
    #[cfg(not(windows))]
    OpenGLES,
}

impl fmt::Display for GraphicsApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicsApi::Vulkan => write!(f, "Vulkan (XR_KHR_vulkan_enable)"),
            GraphicsApi::Vulkan2 => write!(f, "Vulkan (XR_KHR_vulkan_enable2)"),
            GraphicsApi::D3D11 => write!(f, "Direct3D 11"),
            GraphicsApi::D3D12 => write!(f, "Direct3D 12"),
            GraphicsApi::OpenGL => write!(f, "OpenGL"),
            #[cfg(not(windows))]
            GraphicsApi::OpenGLES => write!(f, "OpenGL ES"),
        }
    }
}

impl GraphicsApi {
    pub fn to_arg(self) -> &'static str {
        match self {
            GraphicsApi::Vulkan => "Vulkan",
            GraphicsApi::Vulkan2 => "Vulkan2",
            GraphicsApi::D3D11 => "D3D11",
            GraphicsApi::D3D12 => "D3D12",
            GraphicsApi::OpenGL => "OpenGL",
            #[cfg(not(windows))]
            GraphicsApi::OpenGLES => "OpenGLES",
        }
    }
}

fn graphics_api_selectable_value(
    ui: &mut egui::Ui,
    current: &mut GraphicsApi,
    selected: GraphicsApi,
) {
    ui.selectable_value(current, selected, selected.to_string());
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
                .selected_text(self.graphics_api.to_string())
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

    pub fn to_args(&self) -> impl Iterator<Item = String> {
        let mut known_args = vec!["-G".to_string(), self.graphics_api.to_arg().to_string()];
        if self.noninteractive {
            known_args.push("exclude:[interactive]".to_string());
        }
        if !self.more_args.is_empty() {
            known_args.push(self.more_args.clone());
        }
        known_args.into_iter()
    }
}
