use super::error::{Error, ErrorKind};
use log::{debug, error};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};
use which_crate;

pub trait Check {
    fn check(self) -> Result<(), Error>;
}

impl Check for ExitStatus {
    fn check(self) -> Result<(), Error> {
        if self.success() {
            Ok(())
        } else {
            Err(ErrorKind::ProcessFailed(self))?
        }
    }
}

impl Check for Output {
    fn check(self) -> Result<(), Error> {
        self.status.check()
    }
}

pub trait PathExt
where
    Self: Sized,
{
    fn if_exists(self) -> Option<Self>;
    fn is_descendant_of(&self, ancestor: &Path) -> bool;
}

impl PathExt for PathBuf {
    fn if_exists(self) -> Option<Self> {
        if self.exists() {
            Some(self)
        } else {
            None
        }
    }

    fn is_descendant_of(&self, ancestor: &Path) -> bool {
        self.iter().zip(ancestor.iter()).all(|(a, b)| a == b)
    }
}

pub fn which<T: AsRef<OsStr> + Debug>(binary_name: T) -> Option<PathBuf> {
    match which_crate::which(&binary_name) {
        Ok(path) => {
            debug!("Detected {:?} as {:?}", &path, &binary_name);
            Some(path)
        }
        Err(e) => {
            match e.kind() {
                which_crate::ErrorKind::CannotFindBinaryPath => {
                    debug!("Cannot find {:?}", &binary_name);
                }
                _ => {
                    error!("Detecting {:?} failed: {}", &binary_name, e);
                }
            }

            None
        }
    }
}
