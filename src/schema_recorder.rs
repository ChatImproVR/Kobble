use serde::de::{self, SeqAccess, value::{MapDeserializer, SeqDeserializer}, Visitor};
use serde::{Deserialize, Deserializer};
use crate::error::GenericError;
use crate::{Schema, StructSchema};

/// Use the given struct to record a schema
pub fn record_schema<'de, T: Deserialize<'de>>() -> Result<Schema, GenericError> {
    let mut rec = SchemaRecorder::new();
    T::deserialize(&mut rec)?;
    Ok(rec.0.remove(0))
}

#[derive(Debug, Clone)]
struct SchemaRecorder(Vec<Schema>);

impl SchemaRecorder {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl<'de> Deserializer<'de> for &mut SchemaRecorder {
    type Error = GenericError;

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut rec = SeqRecorder::new(fields.len());
        let ret = visitor.visit_seq(&mut rec);

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

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Seq);
        let seq = SeqDeserializer::new(Vec::<()>::new().into_iter());
        visitor.visit_seq(seq)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Map);
        let map = MapDeserializer::new(Vec::<((), ())>::new().into_iter());
        visitor.visit_map(map)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Str);
        visitor.visit_str(Default::default())
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

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Bytes);
        visitor.visit_bytes(Default::default())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::Option);
        visitor.visit_none()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Schema::ByteBuf);
        visitor.visit_byte_buf(Default::default())
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

impl SeqRecorder {
    pub fn new(len: usize) -> Self {
        Self {
            records: SchemaRecorder::new(),
            len,
        }
    }
}
