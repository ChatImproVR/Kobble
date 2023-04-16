# Kobble
A schema format for Serde. Kobble abuses serde's reflection to allow you to transmit your data type and it's serialized representation seperately. Kobble provides `DynamicValue`, a generic instatiation of an arbtrary data type.

Here's a round trip test which demonstrates what kobble is capable of:
```rust
fn roundrip_test<T: Serialize + Deserialize>(instance: T) {
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
```

## Example
Consider the following data type:
```rust
/// The three genders
#[serde(Serialize, Deserialize)]
enum Gender {
    Truck,
    Sedan,
    Motorcycle,
}

/// People
#[serde(Serialize, Deserialize)]
struct Person {
    gender: Gender,
    age: usize,
}
```

Suppose we wanted to store `Person` in a database. We could use JSON, but JSON wastes space and processing time by requiring a verbose representations of the data. In the case where we're managing thousands of people, most of the information there is redundant. We could use `bincode` to store the records instead, thus reducing overhead. But the cost of using `bincode` is that there is no longer a dynamically editable representation of the data; **in order to manipulate the binary data an application must have been compiled with that data in mind**. 

Kobble intends to solve this problem by providing the `Schema` and `DynamicValue` data types. 

An instance of `Schema` describes the structure of a particular datatype. They can be inferred from an existing structure: 
```rust
let schema = Schema::infer::<Person>();
```

Now suppose we have an application running elsewhere, with no knowledge of the `Person` data type. We are building a GUI toolkit that is supposed to edit arbitrary data. Assuming `Schema` has already been sent to us, along with the binary representation of a `Person` in bincode, we can recover `DynamicValue`:
```rust
fn recombobulate_type(data: &[u8], schema: Schema) {
    SchemaDeserializer::set_schema(schema);
    let SchemaDeserializer(dynamic) = bincode::deserialize(data).unwrap();

    dbg!(dynamic);
}
```

We can then edit this `DynamicValue`, and serialize it back into the same format for storage.

\* See limitations.

# LIMITATIONS
Kobble currently cannot hande enums with values, e.g. the `Option` type does not work. This means you cannot have recursive types.

NOTE: Kobble is currently underpolished and likely buggy. Please do not use this in production!
