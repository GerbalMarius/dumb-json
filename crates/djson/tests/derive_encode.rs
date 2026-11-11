use djson::{Encode, JsonValue};

#[derive(Encode)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Encode)]
struct Value<T> where T: Encode {
    val: T
}

#[derive(Encode)]
struct Point(f32, f32);

#[derive(Encode)]
struct Maybe<T: Encode>(T);

#[derive(Encode)]
struct Marker;

#[test]
fn encode_named_struct_produces_valid_json() {
    let person= Person {
        name: String::from("Alice"),
        age: 25,
    };

    let pjson = person.encode();

    match pjson {
        JsonValue::Object(obj) => {
            assert_eq!(
                obj.get("name"),
                Some(&JsonValue::String(person.name))
            );

            assert_eq!(obj.get("age"), Some(&JsonValue::UnsignedInt(person.age)));
        }
        _ => panic!("Expected valid json object "),
    };
}

#[test]
fn encode_positional_struct_produces_valid_json() {
    let point = Point(0., 0.);

    let point_json = point.encode();
     match point_json {
        JsonValue::Array(arr) => {
            assert_eq!(arr[0], JsonValue::Float(point.0));
            assert_eq!(arr[1], JsonValue::Float(point.1));
        }
        _ => panic!("Expected valid json array "),
    };
}

#[test]
fn encode_unit_struct_produces_valid_json() {
    let marker = Marker;

    let marker_json = marker.encode();
    match marker_json {
        JsonValue::Object(obj) => assert!(obj.is_empty()),
        _ => panic!("Expected empty valid json object"),
    }
}


#[test]
fn encode_generic_named_struct_produces_valid_json() {
    let value = Value {val: 0};

    let value_json = value.encode();
    match value_json {
        JsonValue::Object(obj) => {
            assert_eq!(obj.get("val"), Some(&JsonValue::Int(value.val)))
        }
        _ => panic!("Expected valid json object")
    }
}

#[test]
fn encode_generic_positional_struct_produces_valid_json() {
    let value = Maybe(0);

    let value_json = value.encode();
    match value_json {
        JsonValue::Array(array) => {
            assert_eq!(array[0], JsonValue::Int(value.0))
        }
        _ => panic!("Expected valid json object")
    }
}