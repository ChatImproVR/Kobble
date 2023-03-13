use bincode::Options as BincodeOptions;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
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
        fields: HashMap<String, Schema>,
    },
}

#[derive(Debug, Clone)]
struct Recorder(Vec<Schema>);

impl Recorder {
    pub fn new() -> Self {
        Self(vec![])
    }
}

/*
pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    //read_dynamic(schema, &mut deser).unwrap();
    todo!()
}
*/

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
