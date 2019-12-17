use std::ffi::OsString;

use super::build_path_error;
use crate::layout::env_paths;
use volta_fail::{Fallible, ResultExt};

/// A lightweight namespace type representing the system environment, i.e. the environment
/// with Volta removed.
pub struct System;

impl System {
    /// Produces a modified version of the current `PATH` environment variable that
    /// removes the Volta shims and binaries, to use for running system node and
    /// executables.
    pub fn path() -> Fallible<OsString> {
        let old_path = envoy::path().unwrap_or_else(|| envoy::Var::from(""));
        let mut new_path = old_path.split();

        for remove_path in env_paths()? {
            new_path = new_path.remove(remove_path);
        }

        new_path.join().with_context(build_path_error)
    }

    /// Reproduces the Volta-enabled `PATH` environment variable for situations where
    /// Volta has been deactivated
    #[cfg(not(feature = "volta-updates"))]
    pub fn enabled_path() -> Fallible<OsString> {
        let old_path = envoy::path().unwrap_or_else(|| envoy::Var::from(""));
        let mut new_path = old_path.split();

        for add_path in env_paths()? {
            if !old_path.split().any(|part| part == add_path) {
                new_path = new_path.prefix_entry(add_path);
            }
        }

        new_path.join().with_context(build_path_error)
    }
}
