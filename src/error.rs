use serde::{de, ser};

use std::fmt::{self, Display};

// TODO: Make this more descriptive
#[derive(Debug)]
pub struct GenericError(pub String);

impl ser::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl de::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Display for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GenericError {}
