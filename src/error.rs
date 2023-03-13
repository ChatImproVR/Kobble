use bincode::Options as BincodeOptions;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};


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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for GenericError {}

