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

use crate::{Schema, DynamicValue, StructSchema};


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
