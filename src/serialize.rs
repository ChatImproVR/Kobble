use serde::ser::*;
use serde::{Serialize, Serializer};

use crate::{leak_string, DynamicValue};

impl Serialize for DynamicValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DynamicValue::I32(v) => serializer.serialize_i32(*v),
            DynamicValue::Struct { name, fields } => {
                let mut ser =
                    serializer.serialize_struct(leak_string(name.clone()), fields.len())?;

                for (name, value) in fields {
                    ser.serialize_field(leak_string(name.clone()), value)?
                }

                ser.end()
            }
            DynamicValue::Tuple(fields) => {
                let mut ser = serializer.serialize_tuple(fields.len())?;
                for field in fields {
                    ser.serialize_element(field)?;
                }

                ser.end()
            }
            _ => todo!()
        }
    }
}
