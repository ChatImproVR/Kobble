use bincode::Options as BincodeOptions;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};

#[derive(Debug, Clone)]
enum Record {
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
        fields: HashMap<String, Record>,
    },
}

#[derive(Debug, Clone)]
struct Recorder(Vec<Record>);

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

        let mut rec = Recorder(vec![]);
        A::deserialize(&mut rec);
        dbg!(rec);

        panic!();
    }
}

// TODO: Make this more descriptive
#[derive(Debug)]
pub struct GenericError;

impl ser::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(_msg: T) -> Self {
        panic!("Custom error unsupported")
    }
}

impl de::Error for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn custom<T: Display>(_msg: T) -> Self {
        panic!("Custom error unsupported")
    }
}

impl Display for GenericError {
    // Don't ask... This stuff just exists to make the compiler happy.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for GenericError {}

impl<'de> Deserializer<'de> for &mut Recorder {
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
        let mut rec = StructRecorder::new(fields.len());
        let ret = visitor.visit_seq(&mut rec);

        let fields = fields
            .iter()
            .map(|s| s.to_string())
            .zip(rec.records.0)
            .collect();

        self.0.push(Record::Struct {
            name: name.into(),
            fields,
        });

        ret
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
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
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Seq);
        let seq = SeqDeserializer::new(Vec::<()>::new().into_iter());
        visitor.visit_seq(seq)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Map);
        let map = MapDeserializer::new(Vec::<((), ())>::new().into_iter());
        visitor.visit_map(map)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Str);
        visitor.visit_str(Default::default())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::I8);
        visitor.visit_i8(Default::default())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::U8);
        visitor.visit_u8(Default::default())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::I16);
        visitor.visit_i16(Default::default())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::U16);
        visitor.visit_u16(Default::default())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::I32);
        visitor.visit_i32(Default::default())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::U32);
        visitor.visit_u32(Default::default())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::I64);
        visitor.visit_i64(Default::default())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::U64);
        visitor.visit_u64(Default::default())
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::I128);
        visitor.visit_i128(Default::default())
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::U128);
        visitor.visit_u128(Default::default())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::F32);
        visitor.visit_f32(Default::default())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::F64);
        visitor.visit_f64(Default::default())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Bool);
        visitor.visit_bool(Default::default())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Char);
        visitor.visit_char(Default::default())
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Unit);
        visitor.visit_unit()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Bytes);
        visitor.visit_bytes(Default::default())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::Option);
        visitor.visit_none()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::ByteBuf);
        visitor.visit_byte_buf(Default::default())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.push(Record::String);
        visitor.visit_string(Default::default())
    }
}

struct StructRecorder {
    records: Recorder,
    len: usize,
}

impl<'de> SeqAccess<'de> for StructRecorder {
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

impl StructRecorder {
    pub fn new(len: usize) -> Self {
        Self {
            records: Recorder::new(),
            len,
        }
    }
}
