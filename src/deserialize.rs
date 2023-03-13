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

/// Construct a DynamicValue based on `schema` using the given deserializer
pub fn deserialize_dynamic<'de, D>(schema: Schema, deser: D) -> Result<DynamicValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match schema {
        Schema::I32 => Ok(DynamicValue::I32(i32::deserialize(deser)?)),
        Schema::Struct(schema) => {
            // Make field names static so serde is happy
            let field_names: Vec<&'static str> = schema
                .fields
                .iter()
                .map(|(name, _)| leak_string(name.clone()))
                .collect();

            let field_names: &'static [&'static str] = Box::leak(field_names.into_boxed_slice());

            // Deserialize the struct
            deser.deserialize_struct(
                leak_string(schema.name.clone()),
                field_names,
                StructVisitor(schema),
            )
        }
        _ => todo!(),
    }
}

/// Visitor for structs; converts a StructSchema into a DynamicValue under the given deserializer
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
            SchemaDeserializer::set_schema(schema);
            let SchemaDeserializer(dynamic) = seq
                .next_element::<SchemaDeserializer>()?
                .expect("Schema mismatch");

            fields.push((name, dynamic));
        }

        Ok(DynamicValue::Struct {
            name: self.0.name,
            fields,
        })
    }
}

/// A struct which pretends to be the schema set with set_schema. 
/// Note that schema are set on a per-thread basis!
pub struct SchemaDeserializer(pub DynamicValue);

impl SchemaDeserializer {
    thread_local! {
        static SCHEMA: RefCell<Option<Schema>> = RefCell::new(None);
    }

    /// Set the schema (for the current thread!)
    pub fn set_schema(schema: Schema) {
        Self::SCHEMA.with(|f| *f.borrow_mut() = Some(schema));
    }
}

impl<'de> Deserialize<'de> for SchemaDeserializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = Self::SCHEMA
            .with(|f| f.take())
            .expect("Schema not set!");
        deserialize_dynamic(schema, deserializer).map(SchemaDeserializer)
    }
}

// TODO: This should be interned to prevent memory leaks...
fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
