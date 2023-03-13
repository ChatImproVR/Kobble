use bincode::Options as BincodeOptions;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::rc::Rc;
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};

mod error;
mod schema_recorder;

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

#[derive(Debug, Clone)]
pub struct StructSchema {
    name: String,
    fields: Vec<(String, Schema)>,
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
    D: serde::Deserializer<'de>,
{
    match schema {
        Schema::I32 => Ok(DynamicValue::I32(i32::deserialize(deser)?)),
        Schema::Struct(schema) => {
            let field_names: Vec<&'static str> = schema
                .fields
                .iter()
                .map(|(name, _)| leak_string(name.clone()))
                .collect();

            let field_names: &'static [&'static str] = Box::leak(field_names.into_boxed_slice());

            deser.deserialize_struct(
                leak_string(schema.name.clone()),
                field_names,
                StructVisitor(schema),
            )

            /*
            let mut out_map = HashMap::new();
            for (k, sub_schema) in map {
            let v = read_dynamic(sub_schema, deser).unwrap();
            out_map.insert(k, v);
            }
            Ok(DynamicValue::Struct(out_map))
            */
        }
        _ => todo!(),
    }
}

pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    Ok(read_dynamic(schema, &mut deser).unwrap())
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
        dbg!(&schema);

        let instance = A {
            a: 99,
            b: B { c: 23480 },
        };

        let bytes = bincode::serialize(&instance).unwrap();

        let dynamic = bincode_read_dynamic(schema, std::io::Cursor::new(bytes)).unwrap();

        dbg!(dynamic);

        panic!();
    }
}

// TODO: This should be interned to prevent memory leaks...
fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

struct StructVisitor(StructSchema);

impl<'de> Visitor<'de> for StructVisitor {
    type Value = DynamicValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TODO")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut fields = vec![];

        for (name, schema) in self.0.fields {
            HACKED_STRUCT_SCHEMA.with(|f| *f.borrow_mut() = Some(schema));
            let HackedStruct(dynamic) = seq
                .next_element::<HackedStruct>()?
                .expect("Schema mismatch");

            fields.push((name, dynamic));
        }

        Ok(DynamicValue::Struct {
            name: self.0.name,
            fields,
        })
    }
}

struct HackedStruct(DynamicValue);

thread_local! {
    static HACKED_STRUCT_SCHEMA: RefCell<Option<Schema>> = RefCell::new(None);
}

impl<'de> Deserialize<'de> for HackedStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = HACKED_STRUCT_SCHEMA
            .with(|f| f.take())
            .expect("Schema not set!");
        read_dynamic(schema, deserializer).map(HackedStruct)
    }
}
