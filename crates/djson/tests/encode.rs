use djson::{Encode, JsonValue};
use parameterized::parameterized;

#[test]
fn encode_option_none_returns_json_null() {
    let value: Option<i32> = None;
    let actual = value.encode();

    assert_eq!(actual, JsonValue::Nil);
}

#[test]
fn encode_option_soome_returns_encoded_inner_value() {
    let value: Option<i32> = Some(1);
    let actual = value.encode();

    assert_eq!(actual, JsonValue::Int(value.unwrap()));
}

#[test]
fn encode_string_returns_valid_json_string() {
    let value = String::from("value");

    let actual = value.encode();

    assert_eq!(actual, JsonValue::String(value.to_string()));
}

#[test]
fn encode_str_slice_returns_valid_json_string() {
    let value = "value";
    let actual = value.encode();

    assert_eq!(actual, JsonValue::String(value.to_string()));
}

#[test]
fn encode_i8_returns_valid_json_int() {
    let value: i8 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Int(1))
}

#[test]
fn encode_i16_returns_valid_json_int() {
    let value: i16 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Int(1))
}

#[test]
fn encode_i32_returns_valid_json_int() {
    let value: i32 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Int(1))
}

#[test]
fn encode_i64_returns_valid_json_long() {
    let value: i64 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Long(1))
}

#[test]
#[cfg(target_pointer_width = "64")]
fn encode_isize_64bit_target_returns_valid_json_long() {
    let value: isize = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Long(value as i64))
}

#[test]
#[cfg(target_pointer_width = "32")]
fn encode_isize_32bit_target_returns_valid_json_int() {
    let value: isize = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::Int(value as i32))
}

#[test]
fn encode_u8_returns_valid_json_unsigned_int() {
    let value: u8 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedInt(value as u32))
}

#[test]
fn encode_u16_returns_valid_json_unsigned_int() {
    let value: u16 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedInt(value as u32))
}

#[test]
fn encode_u32_returns_valid_json_unsigned_int() {
    let value: u32 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedInt(value))
}

#[test]
fn encode_u64_returns_valid_json_unsigned_long() {
    let value: u64 = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedLong(value))
}

#[test]
#[cfg(target_pointer_width = "32")]
fn encode_usize_32bit_target_returns_valid_json_unsigned_int() {
    let value: usize = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedInt(value as u32))
}

#[test]
#[cfg(target_pointer_width = "64")]
fn encode_usize_64bit_target_returns_valid_json_unsigned_long() {
    let value: usize = 1;

    let actual = value.encode();
    assert_eq!(actual, JsonValue::UnsignedLong(value as u64))
}

#[parameterized(value = {
        true, false
    })]
fn encode_bool_returns_valid_json_value(value: bool) {
    let actual = value.encode();

    assert_eq!(actual, JsonValue::Bool(value));
}
