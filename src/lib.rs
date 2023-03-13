use bincode::Options as BincodeOptions;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};

mod schema_recorder;
mod error;

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
    Struct {
        name: String,
        fields: Vec<(String, Schema)>,
    },
}

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


pub fn read_dynamic<'de, D>(schema: Schema, deser: D) -> Result<DynamicValue, D::Error>
where
    D: serde::Deserializer<'de>
{
    match schema {
        Schema::I32 => {
            deser.deserialize_i32(todo!());
        }
        Schema::Struct { name, fields } => {
            let field_names: Vec<&'static str> = fields.into_iter().map(|(s, _)| leak_string(s)).collect();
            let filed_names: &'static [&'static str] = Box::leak(field_names.into_boxed_slice());

            let v = todo!();
            deser.deserialize_struct(leak_string(name), &field_names, v);

                /*
            let mut out_map = HashMap::new();
            for (k, sub_schema) in map {
                let v = read_dynamic(sub_schema, deser).unwrap();
                out_map.insert(k, v);
            }
            Ok(DynamicValue::Struct(out_map))
                */
            todo!()
        }
        _ => todo!()
    }
}


pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    read_dynamic(schema, &mut deser).unwrap();
}

fn bincode_opts() -> impl BincodeOptions {
    // NOTE: This is actually different from the default bincode serialize() function!!
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
}

#[cfg(test)]
mod tests {
    use crate::schema_recorder::record_schema;

    use super::*;

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
        dbg!(schema);

        panic!();
    }
}

// TODO: This should be interned to prevent memory leaks...
fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
