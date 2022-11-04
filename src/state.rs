// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{config::Config, error::Error, process::CtsProcess};

#[derive(Debug, Default)]
pub struct RunningData {
    process: Option<CtsProcess>,
    output: String,
    done: bool,
}

impl RunningData {
    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn poll(&mut self) -> Result<(), Error> {
        if self.done {
            return Ok(());
        }
        match &mut self.process {
            Some(process) => {
                if let Some(new_output) = process.poll()? {
                    // if new_output.is_empty() {
                    //     self.done = true;
                    //     return Ok(());
                    // }
                    self.output.push('\n');
                    self.output.push_str(&new_output);
                }
                if process.is_done() {
                    self.done = true;
                }
            }
            None => {
                self.done = true;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum State {
    Configuring,
    Running(RunningData),
    Err(Error),
}

impl State {
    pub fn start_running(config: &Config) -> State {
        match CtsProcess::new(config.to_args()) {
            Ok(process) => Self::Running(RunningData {
                process: Some(process),
                ..Default::default()
            }),
            Err(e) => Self::Err(e),
        }
    }
}
