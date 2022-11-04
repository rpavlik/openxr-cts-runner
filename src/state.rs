// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct RunningData {}

#[derive(Debug, Clone)]
pub enum State {
    Configuring,
    Running(RunningData),
}

impl State {
    pub fn start_running(_config: &Config) -> State {
        Self::Running(RunningData {})
    }
}
