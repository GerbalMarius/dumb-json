use crate::JsonValue;

pub trait Encode {
    fn djson_encode(&self) -> JsonValue;
}

impl<T: Encode + Copy> Encode for Option<T> {
    fn djson_encode(&self) -> JsonValue {
        match *self {
            Some(value) => value.djson_encode(),
            None => JsonValue::Nil,
        }
    }
}

impl Encode for &str {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::String(self.to_string())
    }
}

impl Encode for String {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl Encode for usize {
    fn djson_encode(&self) -> JsonValue {
        match usize::BITS {
            32 => JsonValue::UnsignedInt(*self as u32),
            64 => JsonValue::UnsignedLong(*self as u64),
            _ => unreachable!(),
        }
    }
}

impl Encode for u64 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::UnsignedLong(*self)
    }
}

impl Encode for u32 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(*self)
    }
}
impl Encode for u16 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(u32::from(*self))
    }
}
impl Encode for u8 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(u32::from(*self))
    }
}

impl Encode for isize {
    fn djson_encode(&self) -> JsonValue {
        match isize::BITS {
            32 => JsonValue::Int(*self as i32),
            64 => JsonValue::Long(*self as i64),
            _ => unreachable!(),
        }
    }
}

impl Encode for i64 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::Long(*self)
    }
}

impl Encode for i32 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::Int(*self)
    }
}

impl Encode for i16 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::Int(i32::from(*self))
    }
}

impl Encode for i8 {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::Int(i32::from(*self))
    }
}

impl Encode for bool {
    fn djson_encode(&self) -> JsonValue {
        JsonValue::Bool(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{JsonValue, traits::encode::Encode};
    use parameterized::parameterized;

    use std::assert_matches;

    #[test]
    fn encode_option_none_returns_json_null() {
        let value: Option<i32> = None;
        let actual = value.djson_encode();

        assert_matches!(actual, JsonValue::Nil);
    }

    #[test]
    fn encode_option_soome_returns_encoded_inner_value() {
        let value: Option<i32> = Some(1);
        let actual = value.djson_encode();

        assert_matches!(actual, JsonValue::Int(1));
    }

    #[test]
    fn encode_string_returns_valid_json_string() {
        let value = String::from("value");

        let actual = value.djson_encode();

        assert_matches!(actual, JsonValue::String(_));
    }

    #[test]
    fn encode_str_slice_returns_valid_json_string() {
        let value = "value";
        let actual = value.djson_encode();

        assert_matches!(actual, JsonValue::String(_));
    }

    #[test]
    fn encode_i8_returns_valid_json_int() {
        let value: i8 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Int(1))
    }

    #[test]
    fn encode_i16_returns_valid_json_int() {
        let value: i16 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Int(1))
    }

    #[test]
    fn encode_i32_returns_valid_json_int() {
        let value: i32 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Int(1))
    }

    #[test]
    fn encode_i64_returns_valid_json_long() {
        let value: i64 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Long(1))
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn encode_isize_64bit_target_returns_valid_json_long() {
        let value: isize = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Long(1))
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn encode_isize_32bit_target_returns_valid_json_int() {
        let value: isize = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::Int(1))
    }

    #[test]
    fn encode_u8_returns_valid_json_unsigned_int() {
        let value: u8 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedInt(1))
    }

    #[test]
    fn encode_u16_returns_valid_json_unsigned_int() {
        let value: u16 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedInt(1))
    }

    #[test]
    fn encode_u32_returns_valid_json_unsigned_int() {
        let value: u32 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedInt(1))
    }

    #[test]
    fn encode_u64_returns_valid_json_unsigned_long() {
        let value: u64 = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedLong(1))
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn encode_usize_32bit_target_returns_valid_json_unsigned_int() {
        let value: usize = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedInt(1))
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn encode_usize_64bit_target_returns_valid_json_unsigned_long() {
        let value: usize = 1;

        let actual = value.djson_encode();
        assert_matches!(actual, JsonValue::UnsignedLong(1))
    }

    #[parameterized(value = {
        true, false
    })]
    fn encode_bool_returns_valid_json_value(value: bool) {
        let actual = value.djson_encode();

        assert_matches!(actual, JsonValue::Bool(_));
    }
}
