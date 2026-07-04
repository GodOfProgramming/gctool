pub mod cmd;
pub mod dol;
pub mod gcm;
pub mod util;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GcError {
    #[error("Could not read {0}-{1} bytes into data")]
    OutOfRange(usize, usize),

    #[error("Generic Error: {0}")]
    Generic(String),
}

impl GcError {
    pub fn generic(s: impl ToString) -> Self {
        Self::Generic(s.to_string())
    }
}
