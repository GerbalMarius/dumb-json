use djson::{Decode, JsonValue, Map};

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

#[derive(Decode)]
struct Point(f32, f32);

#[derive(Decode)]
struct Maybe<T: Decode>(T);

#[derive(Decode)]
struct Marker;

#[test]
fn decode_named_struct_produces_valid_instance() {
    let name = "Marius";
    let age = 25;
    let json = JsonValue::Object(Map::from_iter([
        (
            String::from("name"),
            JsonValue::String(String::from(name)),
        ),
        (String::from("age"), JsonValue::UnsignedInt(age)),
    ]));

    let person = Person::decode(&json);

    assert!(person.is_ok());

    let p_value = person.unwrap();
    assert_eq!(p_value.age, age);
    assert_eq!(p_value.name, name);
}

#[test]
fn decode_positional_struct_produces_valid_instance() {
    let json = JsonValue::Array(
        vec![JsonValue::Float(0.), JsonValue::Float(0.)]
    );

    let res = Point::decode(&json);
    assert!(res.is_ok());

    let point = res.unwrap();
    assert_eq!(point.0, 0.);
    assert_eq!(point.1, 0.);
}

#[test]
fn decode_generic_named_struct_produces_valid_struct_result() {
    let json = JsonValue::Object([
        (String::from("val"), JsonValue::UnsignedInt(10)),
    ].into_iter().collect());
     
    let value =  Value::<i32>::decode(&json);

    assert!(value.is_ok());

    let p_value = value.unwrap();
    assert_eq!(p_value.val, 10);
}

#[test]
fn decode_generic_positional_struct_produces_valid_struct_result() {
    let json = JsonValue::Array(
        vec![JsonValue::String("Marius".to_string())]
    );

    let res = Maybe::<String>::decode(&json);
    assert!(res.is_ok());

    let p_value = res.unwrap();
    assert_eq!(p_value.0, "Marius");
}

#[test]
fn decode_unit_struct_produces_valid_struct_result() {
    let json = JsonValue::Object(Map::new());

    let res = Marker::decode(&json); //is valid for empty objects
    assert!(res.is_ok());
}
