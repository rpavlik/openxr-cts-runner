use core::time;
use std::io;

// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0
use subprocess::{Communicator, Exec, Redirection};

use crate::error::Error;

#[derive(Debug)]
pub struct CtsProcess {
    communicator: Communicator,

    done: bool,
}

#[cfg(not(windows))]
const CONFORMANCE_BINARY: &str = "./conformance_cli";

#[cfg(windows)]
const CONFORMANCE_BINARY: &str = "conformance_cli.exe";

impl CtsProcess {
    pub fn new(args: impl Iterator<Item = String>) -> Result<Self, Error> {
        let args: Vec<_> = args.collect();
        let communicator = Exec::cmd(CONFORMANCE_BINARY)
            .args(&args)
            .stderr(Redirection::Pipe)
            .stdout(Redirection::Pipe)
            .communicate()
            .map_err(|e| Error::LaunchError(e))?
            .limit_time(time::Duration::from_millis(10));

        Ok(Self {
            communicator,
            done: false,
        })
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn poll(&mut self) -> Result<Option<String>, Error> {
        match self.communicator.read_string() {
            Ok((Some(stdout), Some(stderr))) => {
                // empty output is the signal that we're done.
                if stdout.is_empty() && stderr.is_empty() {
                    self.done = true;
                    return Ok(None);
                }
                Ok(Some(stdout + "\n" + &stderr))
            }
            Ok(_) => todo!(),
            Err(e) => {
                if e.kind() == io::ErrorKind::TimedOut {
                    return Ok(None);
                }
                Err(e.into())
            }
        }
    }
}
