use serde::{de, ser};

use std::fmt::{self, Display};

// TODO: Make this more descriptive
#[derive(Debug)]
pub struct GenericError;

impl ser::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(_msg: T) -> Self {
        panic!("Custom error unsupported")
    }
}

impl de::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(_msg: T) -> Self {
        panic!("Custom error unsupported")
    }
}

impl Display for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for GenericError {}
