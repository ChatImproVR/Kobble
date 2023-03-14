use deserialize::deserialize_dynamic;
use schema_recorder::record_schema;
use serde::Deserialize;
use std::{collections::HashMap, io::Read};

mod deserialize;
mod error;
mod schema_recorder;
mod serialize;

/// Representation of a data serde-compatible data structure
#[derive(Debug, Clone)]
pub enum Schema {
    Seq,
    Map,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    F32,
    F64,
    Bool,
    Char,
    Unit,
    Bytes,
    Option,
    ByteBuf,
    String,
    Struct(StructSchema),
    Tuple(TupleSchema),
    TupleStruct(String, TupleSchema),
    NewtypeStruct(String, Box<Schema>),
    UnitStruct(String),
}

pub type TupleSchema = Vec<Schema>;

/// Represents a struct
#[derive(Debug, Clone)]
pub struct StructSchema {
    name: String,
    fields: Vec<(String, Schema)>,
}

/// Represents an enum
#[derive(Debug, Clone)]
pub struct EnumSchema {
    name: String,
    variants: Vec<(String, VariantSchema)>,
}

/// Represents an enum variant
#[derive(Debug, Clone)]
pub enum VariantSchema {
    Struct(StructSchema),
    Tuple(TupleSchema),
    Unit,
}

/// Runtime-modifiable representation of a data structure
#[derive(Debug, Clone)]
pub enum DynamicValue {
    //Seq(Vec<DynamicValue>),
    //Map(HashMap<String, DynamicValue>),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    Unit,
    //Bytes(Vec<u8>),
    //Option(Option<Box<DynamicValue>>),
    //ByteBuf(Vec<u8>),
    String(String),
    TupleStruct(String, Vec<DynamicValue>),
    NewtypeStruct(String, Box<DynamicValue>),
    Struct {
        name: String,
        fields: Vec<(String, DynamicValue)>,
    },
    Tuple(Vec<DynamicValue>),
    UnitStruct(String),
}

#[cfg(test)]
mod tests {
    use crate::{deserialize::SchemaDeserializer, Schema};
    use serde::{Deserialize, Serialize};

    fn roundrip_test<'de, T: Serialize + Deserialize<'de>>(instance: T) {
        // Create a schema for the datat type
        let schema = Schema::infer::<T>();

        // Serialize the instance as bytes
        let bytes = bincode::serialize(&instance).unwrap();

        // Deserialize the bytes into a DynamicValue using the schema
        SchemaDeserializer::set_schema(schema);
        let SchemaDeserializer(dynamic) = bincode::deserialize(&bytes).unwrap();

        // Serialize the DynamicValue into bytes again
        let re_serialized = bincode::serialize(&dynamic).unwrap();

        // Make sure they are the same!
        assert_eq!(bytes, re_serialized);
    }

    #[test]
    fn test_tuple() {
        roundrip_test((0i32, 10f32, 8u128, 90f64))
    }

    // For now, we don't know how to do this!
    #[test]
    #[should_panic]
    fn test_enum_basic() {
        #[derive(Serialize, Deserialize)]
        enum A {
            B(i32),
            Fork,
        }

        roundrip_test(A::B(23480));
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, Deserialize)]
        struct A {
            a: i32,
            b: B,
        }

        #[derive(Serialize, Deserialize)]
        struct B {
            c: i32,
        }

        roundrip_test(A {
            a: 99,
            b: B { c: 23480 },
        })
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Serialize, Deserialize)]
        struct A(i32);

        roundrip_test(A(9999));
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Serialize, Deserialize)]
        struct A(i32, String);

        roundrip_test(A(9999, "Binkus".to_string()));
    }
}

// TODO: This should be interned to prevent memory leaks...
pub(crate) fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

impl Schema {
    pub fn infer<'de, T: Deserialize<'de>>() -> Self {
        record_schema::<T>().expect("Failed to infer schema")
    }
}
