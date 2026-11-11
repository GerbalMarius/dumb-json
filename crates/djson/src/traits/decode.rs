use crate::{JsonFormatError, JsonValue};

pub trait Decode<T> {
    fn decode(json: JsonValue) -> crate::djson::Result<T>;
}

impl<T: Copy + Decode<T>> Decode<Option<T>> for Option<T> {
    fn decode(json: JsonValue) -> crate::djson::Result<Option<T>> {
        match json {
            JsonValue::Nil => Ok(None),
            _ => Ok(Some(T::decode(json)?))
        }
    }
}

impl<T: Decode<T>> Decode<Vec<T>> for Vec<T> {
    fn decode(json: JsonValue) -> crate::djson::Result<Vec<T>> {
        let values = match json {
            JsonValue::Array(values) => values,
            _ => return Err(JsonFormatError::InvalidArray),
        };

        values.into_iter()
            .map(T::decode)
            .collect()
    }
}

impl<T: Decode<T>> Decode<Vec<T>> for [&T] {
    fn decode(json: JsonValue) -> crate::djson::Result<Vec<T>> {
        let values = match json {
            JsonValue::Array(values) => values,
            _ => return Err(JsonFormatError::InvalidArray),
        };

        values.into_iter()
            .map(T::decode)
            .collect()
    }
}

impl Decode<u32> for u32 {
    fn decode(json: JsonValue) -> crate::djson::Result<u32> {
        json.unsigned_int()
    }
}

impl Decode<u64> for u64 {
    fn decode(json: JsonValue) -> crate::djson::Result<u64> {
        json.unsigned_long()
    }
}

impl Decode<i32> for i32 {
    fn decode(json: JsonValue) -> crate::djson::Result<i32> {
        json.int()
    }
}

impl Decode<i64> for i64 {
    fn decode(json: JsonValue) -> crate::djson::Result<i64> {
        json.long()
    }
}

impl Decode<String> for String {
    fn decode(json: JsonValue) -> crate::djson::Result<String> {
        json.string().map(String::from)
    }
}

impl Decode<String> for &str {
    fn decode(json: JsonValue) -> crate::djson::Result<String> {
        json.string().map(String::from)
    }
}

impl Decode<bool> for bool {
    fn decode(json: JsonValue) -> crate::djson::Result<bool> {
        json.bool()
    }
}