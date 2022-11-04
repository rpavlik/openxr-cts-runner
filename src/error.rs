// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use subprocess::{CommunicateError, PopenError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not launch process: {0}")]
    LaunchError(#[source] PopenError),

    #[error("Error while checking for output: {0}")]
    OutputError(#[from] CommunicateError),
}
