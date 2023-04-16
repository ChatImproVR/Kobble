use crate::{string_to_static, DynamicValue};
use serde::ser::*;
use serde::Serialize;

impl Serialize for DynamicValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DynamicValue::UniformSequence(values) => {
                let mut ser = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    ser.serialize_element(value)?;
                }
                ser.end()
            }
            DynamicValue::Struct { name, fields } => {
                let mut ser =
                    serializer.serialize_struct(string_to_static(name.clone()), fields.len())?;

                for (name, value) in fields {
                    ser.serialize_field(string_to_static(name.clone()), value)?
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
            DynamicValue::NewtypeStruct(name, value) => {
                serializer.serialize_newtype_struct(string_to_static(name.clone()), value)
            }
            DynamicValue::TupleStruct(name, tuple) => {
                let mut ser = serializer
                    .serialize_tuple_struct(string_to_static(name.clone()), tuple.len())?;
                for field in tuple {
                    ser.serialize_field(field)?;
                }
                ser.end()
            }
            DynamicValue::UnitStruct(name) => {
                serializer.serialize_unit_struct(string_to_static(name.clone()))
            }
            DynamicValue::Enum(schema, idx) => serializer.serialize_unit_variant(
                string_to_static(schema.name.clone()),
                *idx,
                string_to_static(schema.variants[*idx as usize].clone()),
            ),
            DynamicValue::String(s) => serializer.serialize_str(s),
            DynamicValue::I8(v) => serializer.serialize_i8(*v),
            DynamicValue::U8(v) => serializer.serialize_u8(*v),
            DynamicValue::I16(v) => serializer.serialize_i16(*v),
            DynamicValue::U16(v) => serializer.serialize_u16(*v),
            DynamicValue::I32(v) => serializer.serialize_i32(*v),
            DynamicValue::U32(v) => serializer.serialize_u32(*v),
            DynamicValue::I64(v) => serializer.serialize_i64(*v),
            DynamicValue::U64(v) => serializer.serialize_u64(*v),
            DynamicValue::I128(v) => serializer.serialize_i128(*v),
            DynamicValue::U128(v) => serializer.serialize_u128(*v),
            DynamicValue::Char(v) => serializer.serialize_char(*v),
            DynamicValue::F32(v) => serializer.serialize_f32(*v),
            DynamicValue::F64(v) => serializer.serialize_f64(*v),
            DynamicValue::Bool(v) => serializer.serialize_bool(*v),
            DynamicValue::Unit => serializer.serialize_unit(),
        }
    }
}
