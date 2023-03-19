use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet};

mod deserialize;
mod error;
mod schema_recorder;
mod serialize;

pub use deserialize::{deserialize_dynamic, SchemaDeserializer};
pub use schema_recorder::record_schema;

/// Representation of a data serde-compatible data structure
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Schema {
    //Seq,
    //Map,
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
    //Bytes,
    //Option,
    //ByteBuf,
    String,
    Struct(StructSchema),
    Tuple(TupleSchema),
    /// (name, tuple)
    TupleStruct(String, TupleSchema),
    /// (name, data)
    NewtypeStruct(String, Box<Schema>),
    /// Contains name
    UnitStruct(String),
    /// (Enum name, variants)
    /// NOTE This only describes data-less enums!
    Enum(EnumSchema),
}

pub type TupleSchema = Vec<Schema>;

/// Represents an enum
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct EnumSchema {
    pub name: String,
    pub variants: Vec<String>,
}

/// Represents a struct
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct StructSchema {
    pub name: String,
    pub fields: Vec<(String, Schema)>,
}

impl Schema {
    pub fn infer<'de, T: Deserialize<'de>>() -> Self {
        record_schema::<T>().expect("Failed to infer schema")
    }
}

/*
/// Represents an enum
#[derive(Debug, Clone)]
pub struct EnumSchema {
    name: String,
    variants: Vec<(String, VariantSchema)>,
}

/// Represents an enum variant
#[derive(Debug, Clone)]
pub enum VariantSchema {
    Struct(StructSchema),
    Tuple(TupleSchema),
    Unit,
}
*/

/// Runtime-modifiable representation of a data structure
#[derive(Debug, Clone)]
pub enum DynamicValue {
    //Seq(Vec<DynamicValue>),
    //Map(HashMap<String, DynamicValue>),
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
    Unit,
    //Bytes(Vec<u8>),
    //Option(Option<Box<DynamicValue>>),
    //ByteBuf(Vec<u8>),
    String(String),
    TupleStruct(String, Vec<DynamicValue>),
    NewtypeStruct(String, Box<DynamicValue>),
    Struct {
        name: String,
        fields: Vec<(String, DynamicValue)>,
    },
    Tuple(Vec<DynamicValue>),
    UnitStruct(String),
    Enum(EnumSchema, u32),
}

#[cfg(test)]
mod tests {
    use crate::{deserialize::SchemaDeserializer, Schema};
    use glam::{Vec3, Vec4};
    use serde::{Deserialize, Serialize};

    fn roundrip_test<'de, T: Serialize + Deserialize<'de>>(instance: T) {
        // Create a schema for the data type
        let schema = Schema::infer::<T>();

        // Serialize the instance as bytes
        let bytes = bincode::serialize(&instance).unwrap();

        // Deserialize the bytes into a DynamicValue using the schema
        SchemaDeserializer::set_schema(schema);
        let SchemaDeserializer(dynamic) = bincode::deserialize(&bytes).unwrap();

        // Serialize the DynamicValue into bytes again
        let re_serialized = bincode::serialize(&dynamic).unwrap();

        // Make sure they are the same!
        assert_eq!(bytes, re_serialized);
    }

    #[derive(Serialize, Deserialize)]
    pub struct A([Box<v::A>; 3]);

    mod v {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct A([Box<crate::tests::A>; 3]);
    }

    #[test]
    fn test_infinity() {
        Schema::infer::<A>();
    }

    #[test]
    fn test_tuple() {
        roundrip_test((0i32, 10f32, 8u128, 90f64))
    }

    #[test]
    fn test_string() {
        roundrip_test("Yeah babeeey")
    }

    #[test]
    fn test_primitives() {
        roundrip_test((
            0i8,
            1u8,
            2i16,
            3u16,
            4i32,
            5u32,
            6i64,
            7u64,
            8i128,
            9u128,
            10f32,
            11f64,
            true,
            'a',
            (),
        ))
    }

    // For now, we don't know how to do this!
    #[test]
    #[should_panic]
    fn test_data_enum() {
        #[derive(Serialize, Deserialize)]
        enum A {
            B(i32),
            Fork,
        }

        roundrip_test(A::B(23480));
    }

    // For now, we don't know how to do this!
    #[test]
    fn test_unit_enum() {
        #[derive(Serialize, Deserialize)]
        enum A {
            Spoon,
            Fork,
        }

        roundrip_test(A::Fork);
        roundrip_test(A::Spoon);
    }

    /* TODO: Handle this corner case!
    #[test]
    #[should_panic]
    fn test_data_enum_corner_case() {
    #[derive(Serialize, Deserialize)]
    enum A {
    Fork,
    B(i32),
    }

    roundrip_test(A::Fork);
    }
    */

    #[test]
    fn test_struct() {
        #[derive(Serialize, Deserialize)]
        struct A {
            a: i32,
            b: B,
        }

        #[derive(Serialize, Deserialize)]
        struct B {
            c: i32,
        }

        roundrip_test(A {
            a: 99,
            b: B { c: 23480 },
        });
    }

    /*
    #[test]
    fn test_newtype_struct() {
    #[derive(Serialize, Deserialize)]
    struct A(i32);

    roundrip_test(A(9999));
    }
    */

    #[test]
    fn test_tuple_struct() {
        #[derive(Serialize, Deserialize)]
        struct A(i32, String);

        roundrip_test(A(9999, "Binkus".to_string()));
    }

    #[test]
    fn test_unit_struct() {
        #[derive(Serialize, Deserialize)]
        struct A;

        roundrip_test(A);
    }

    #[test]
    fn test_vector() {
        roundrip_test(Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_matrix() {
        roundrip_test(glam::Mat4::from_cols(
            Vec4::new(1., 2., 3., 4.),
            Vec4::new(5., 6., 7., 8.),
            Vec4::new(9., 10., 11., 12.),
            Vec4::new(13., 14., 15., 16.),
        ));
    }
}

/// Converts a string to a static string
pub(crate) fn string_to_static(s: String) -> &'static str {
    // String cache, so that we don't leak memory converting the same string over and over
    // TODO: Use fxhash? This is non-crypto!
    thread_local! {
        static STRING_CACHE: RefCell<Lazy<HashSet<&'static str>>>
            = RefCell::new(Lazy::new(HashSet::new));
    }

    STRING_CACHE.with(|cache| {
        let maybe_cached: Option<&'static str> = cache.borrow().get(s.as_str()).copied();
        if let Some(cached) = maybe_cached {
            cached
        } else {
            let s: &'static str = Box::leak(s.into_boxed_str());
            cache.borrow_mut().insert(s);
            s
        }
    })
}
