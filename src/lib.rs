use bincode::Options as BincodeOptions;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};

pub enum Schema {
    I32,
    Struct {
        name: String,
        fields: HashMap<String, DynamicValue>
    }
}

pub enum DynamicValue {
    I32(i32),
    Struct {
        name: String,
        fields: HashMap<String, DynamicValue>
    }
}

pub fn read_dynamic<'a, D>(schema: Schema, deser: D) -> Result<DynamicValue, D::Error>
where
    D: serde::Deserializer<'a>
{
    match schema {
        Schema::I32 => {
            deser.deserialize_i32(MyVisitor::<i32>::new()).map(DynamicValue::I32)
        }
        Schema::Struct(map) => {
            deser.deserialize_struct(name, fields, visitor)

            let mut out_map = HashMap::new();
            for (k, sub_schema) in map {
                let v = read_dynamic(sub_schema, deser).unwrap();
                out_map.insert(k, v);
            }
            Ok(DynamicValue::Struct(out_map))
        }
    }
}

struct MyVisitor<T>(PhantomData<T>);

impl<T> MyVisitor<T> {
    pub fn new() -> Self { Self(PhantomData) }
}

impl<'de> Visitor<'de> for MyVisitor<i32> {
    type Value = i32;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(std::any::type_name::<i32>())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(v)
    }
}

pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    read_dynamic(schema, &mut deser).unwrap();
    todo!()
}

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

        let original = A {
            a: 234890,
            b: B { c: 92304 },
        };
    }
}
