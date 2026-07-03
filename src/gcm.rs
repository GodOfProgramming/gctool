use gc_gcm::{GcmError, GcmFile};
use std::ops::{Deref, DerefMut};

use crate::GcError;

pub struct Gcm(GcmFile);

impl Deref for Gcm {
    type Target = GcmFile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Gcm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Gcm {
    pub fn new(iso_data: Vec<u8>) -> anyhow::Result<Self> {
        let mut cursor = std::io::Cursor::new(iso_data);
        let file = GcmFile::from_reader(&mut cursor).map_err(|e| match e {
            GcmError::ParseError(error) => GcError::generic(error),
            GcmError::IoError(error) => GcError::generic(error),
        })?;

        Ok(Self(file))
    }
}
