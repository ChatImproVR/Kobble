use crate::error::GenericError;
use crate::{Schema, StructSchema};
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

/// Use the given struct to record a schema
pub fn record_schema<'de, T: Deserialize<'de>>() -> Result<Schema, GenericError> {
    let mut rec = SchemaRecorder::new();
    T::deserialize(&mut rec)?;
    Ok(rec.0.remove(0))
}

/// Records the structure of a data type by acting as a Deserializer
#[derive(Debug, Clone)]
struct SchemaRecorder(Vec<Schema>);

impl SchemaRecorder {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl<'de> Deserializer<'de> for &mut SchemaRecorder {
    type Error = GenericError;

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Because visitor.visit_enum() necessarily consumes visitor, we cannot visit multiple
        // enum variants in one shot. One potential solution would be to invoke the entire
        // serializer many times. Each invocation would take a different path through the variant
        // tree. These trees would then be merged into a single schema containing all variants.
        // But this must wait!
        unimplemented!("Exploring multiple enum variants may prove challenging...")
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("Under the impression this is only relevant to enums")
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Visit the entries in the struct
        let mut rec = SeqRecorder::new(fields.len());
        let ret = visitor.visit_seq(&mut rec);

        // Zip the names of the fields with their respective schema
        let fields = fields
            .iter()
            .map(|s| s.to_string())
            .zip(rec.records.0)
            .collect();

        self.0.push(Schema::Struct(StructSchema {
            name: name.into(),
            fields,
        }));

        ret
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut rec = SeqRecorder::new(len);
        let ret = visitor.visit_seq(&mut rec);

        self.0.push(Schema::Tuple(rec.records.0));

        ret
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::UnitStruct(name.to_string()));
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut rec = SeqRecorder::new(len);
        let ret = visitor.visit_seq(&mut rec);

        self.0
            .push(Schema::TupleStruct(name.to_string(), rec.records.0));

        ret
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut rec = SeqRecorder::new(1);
        let ret = visitor.visit_seq(&mut rec);

        self.0.push(Schema::NewtypeStruct(
            name.to_string(),
            Box::new(rec.records.0.remove(0)),
        ));

        ret
    }

    fn is_human_readable(&self) -> bool {
        todo!("Not sure how this applies")
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("Any")
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("Any")
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!(
            "Is this gauranteed to deserialize a homogenous, variable-length collection?"
        )
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!(
            "Is this gauranteed to deserialize a homogenous, variable-length collection?"
        )
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::String);
        visitor.visit_borrowed_str(Default::default())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::I8);
        visitor.visit_i8(Default::default())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::U8);
        visitor.visit_u8(Default::default())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::I16);
        visitor.visit_i16(Default::default())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::U16);
        visitor.visit_u16(Default::default())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::I32);
        visitor.visit_i32(Default::default())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::U32);
        visitor.visit_u32(Default::default())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::I64);
        visitor.visit_i64(Default::default())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::U64);
        visitor.visit_u64(Default::default())
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::I128);
        visitor.visit_i128(Default::default())
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::U128);
        visitor.visit_u128(Default::default())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::F32);
        visitor.visit_f32(Default::default())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::F64);
        visitor.visit_f64(Default::default())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Bool);
        visitor.visit_bool(Default::default())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Char);
        visitor.visit_char(Default::default())
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Unit);
        visitor.visit_unit()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        //self.0.push(Schema::Bytes);
        //visitor.visit_bytes(Default::default())
        todo!("Byte buffers")
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        //self.0.push(Schema::Option);
        //visitor.visit_none()
        todo!("Enums")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        //self.0.push(Schema::ByteBuf);
        //visitor.visit_byte_buf(Default::default())
        todo!("Byte buffers")
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::String);
        visitor.visit_string(Default::default())
    }
}

struct SeqRecorder {
    records: SchemaRecorder,
    len: usize,
}

impl SeqRecorder {
    pub fn new(len: usize) -> Self {
        Self {
            records: SchemaRecorder::new(),
            len,
        }
    }
}

impl<'de> SeqAccess<'de> for SeqRecorder {
    type Error = GenericError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let value = serde::de::DeserializeSeed::deserialize(seed, &mut self.records)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

/*
struct EnumRecorder {
    variants: SchemaRecorder,
    len: usize,
}

impl EnumRecorder {
    pub fn new(len: usize) -> Self {
        Self {
            variants: SchemaRecorder::new(),
            len,
        }
    }
}

impl<'de> EnumAccess<'de> for EnumRecorder {
    type Error = GenericError;
    type Variant = EnumRecorder;

    fn variant_seed<V>(self, mut seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        dbg!(std::any::type_name::<V>());
        dbg!(std::any::type_name::<V::Value>());
        //let r = seed.deserialize(0.into_deserializer())?;
        //Ok((r, self))
        todo!()
    }
}

impl<'de> VariantAccess<'de> for EnumRecorder {
    type Error = GenericError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        todo!()
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        todo!()
    }
}
*/
