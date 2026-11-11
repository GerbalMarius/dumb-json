#![allow(unused)]

use std::collections::HashMap;

use djson::{Decode, JsonValue};

#[derive(Decode)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Decode)]
struct Value<T>
where
    T: Decode,
{
    val: T,
}

#[test]
fn decode_named_struct_produces_valid_struct_result() {
    let name = "Marius";
    let age = 25;
    let json = JsonValue::Object(HashMap::from_iter([
        (
            String::from("name"),
            JsonValue::String(String::from(name)),
        ),
        (String::from("age"), JsonValue::UnsignedInt(age)),
    ]));

    let person = Person::decode(json);

    assert!(person.is_ok());

    let p_value = person.unwrap();
    assert_eq!(p_value.age, age);
    assert_eq!(p_value.name, name);
}

#[test]
fn decode_generic_named_struct_produces_valid_struct_result() {
    let json = JsonValue::Object(HashMap::from_iter([
        (String::from("val"), JsonValue::UnsignedInt(10)),
    ]));
     
     let value =  Value::<i32>::decode(json);

    assert!(value.is_ok());

    let p_value = value.unwrap();
    assert_eq!(p_value.val, 10);
}
