use serde::de::SeqAccess;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::cell::RefCell;
use std::fmt;

use crate::{leak_string, DynamicValue, Schema, StructSchema, TupleSchema};

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
        let schema = Self::SCHEMA.with(|f| f.take()).expect("Schema not set!");
        deserialize_dynamic(schema, deserializer).map(SchemaDeserializer)
    }
}

/// Construct a DynamicValue based on `schema` using the given deserializer
pub fn deserialize_dynamic<'de, D>(schema: Schema, deser: D) -> Result<DynamicValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match schema {
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
        Schema::NewtypeStruct(name, schema) => {
            let struct_ = deser.deserialize_newtype_struct(
                leak_string(name.clone()),
                TupleVisitor(vec![*schema]),
            )?;
            let DynamicValue::Tuple(mut tuple) = struct_ else { panic!() };
            Ok(DynamicValue::NewtypeStruct(name, Box::new(tuple.remove(0))))
        }
        Schema::Tuple(schema) => deser.deserialize_tuple(schema.len(), TupleVisitor(schema)),
        Schema::TupleStruct(name, schema) => {
            let tuple = deser.deserialize_tuple_struct(
                leak_string(name.clone()),
                schema.len(),
                TupleVisitor(schema),
            )?;
            let DynamicValue::Tuple(tuple) = tuple else { panic!() };
            Ok(DynamicValue::TupleStruct(name, tuple))
        }
        Schema::UnitStruct(name) => Ok(DynamicValue::UnitStruct(name)),
        Schema::U8 => Ok(DynamicValue::U8(u8::deserialize(deser)?)),
        Schema::I8 => Ok(DynamicValue::I8(i8::deserialize(deser)?)),
        Schema::U16 => Ok(DynamicValue::U16(u16::deserialize(deser)?)),
        Schema::I16 => Ok(DynamicValue::I16(i16::deserialize(deser)?)),
        Schema::U32 => Ok(DynamicValue::U32(u32::deserialize(deser)?)),
        Schema::I32 => Ok(DynamicValue::I32(i32::deserialize(deser)?)),
        Schema::U64 => Ok(DynamicValue::U64(u64::deserialize(deser)?)),
        Schema::I64 => Ok(DynamicValue::I64(i64::deserialize(deser)?)),
        Schema::U128 => Ok(DynamicValue::U128(u128::deserialize(deser)?)),
        Schema::I128 => Ok(DynamicValue::I128(i128::deserialize(deser)?)),
        Schema::F32 => Ok(DynamicValue::F32(f32::deserialize(deser)?)),
        Schema::F64 => Ok(DynamicValue::F64(f64::deserialize(deser)?)),
        Schema::Bool => Ok(DynamicValue::Bool(bool::deserialize(deser)?)),
        Schema::Char => Ok(DynamicValue::Char(char::deserialize(deser)?)),
        Schema::Unit => Ok(DynamicValue::Unit),
        Schema::Str => todo!(),
        Schema::Map => todo!(),
        Schema::Bytes => todo!(),
        Schema::ByteBuf => todo!(),
        Schema::Option => todo!(),
        Schema::String => todo!(),
        Schema::Seq => todo!(),
    }
}

/// Visitor for structs; converts a StructSchema into a DynamicValue under the given deserializer
struct StructVisitor(StructSchema);

impl<'de> Visitor<'de> for StructVisitor {
    type Value = DynamicValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Struct")
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

/// Visitor for structs; converts a StructSchema into a DynamicValue under the given deserializer
struct TupleVisitor(TupleSchema);

impl<'de> Visitor<'de> for TupleVisitor {
    type Value = DynamicValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TODO")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut fields = vec![];

        for schema in self.0 {
            SchemaDeserializer::set_schema(schema);
            let SchemaDeserializer(dynamic) = seq
                .next_element::<SchemaDeserializer>()?
                .expect("Schema mismatch");

            fields.push(dynamic);
        }

        Ok(DynamicValue::Tuple(fields))
    }
}
