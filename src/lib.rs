use bincode::Options as BincodeOptions;
use deserialize::deserialize_dynamic;
use std::{collections::HashMap, io::Read};

mod deserialize;
mod error;
mod schema_recorder;

/// Representation of a data serde-compatible data structure
#[derive(Debug, Clone)]
pub enum Schema {
    Str,
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
}

/// Represents a struct
#[derive(Debug, Clone)]
pub struct StructSchema {
    name: String,
    fields: Vec<(String, Schema)>,
}

/// Runtime-modifiable representation of a data structure
#[derive(Debug, Clone)]
pub enum DynamicValue {
    Str(String),
    Seq(Vec<DynamicValue>),
    Map(HashMap<String, DynamicValue>),
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
    Unit(()),
    Bytes(Vec<u8>),
    Option(Option<Box<DynamicValue>>),
    ByteBuf(Vec<u8>),
    String(String),
    Struct {
        name: String,
        fields: Vec<(String, DynamicValue)>,
    },
}

/// Use bincode to read the given structure based on its schema
pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    Ok(deserialize_dynamic(schema, &mut deser).unwrap())
}

fn bincode_opts() -> impl BincodeOptions {
    // NOTE: This is actually different from the default bincode serialize() function!!
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
}

#[cfg(test)]
mod tests {
    use crate::{schema_recorder::record_schema, deserialize::SchemaDeserializer};
    use serde::{Serialize, Deserialize};

    #[test]
    fn it_works() {
        #[derive(Serialize, Deserialize)]
        struct A {
            a: i32,
            b: B,
        }

        #[derive(Serialize, Deserialize)]
        struct B {
            c: i32,
        }

        let schema = record_schema::<A>().unwrap();
        dbg!(&schema);

        let instance = A {
            a: 99,
            b: B { c: 23480 },
        };

        let bytes = bincode::serialize(&instance).unwrap();

        SchemaDeserializer::set_schema(schema);
        let SchemaDeserializer(dynamic) = bincode::deserialize(&bytes).unwrap();

        dbg!(dynamic);

        panic!();
    }
}
