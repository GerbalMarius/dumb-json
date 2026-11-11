use crate::{JsonFormatError, JsonValue};

pub trait Decode: Sized {
    fn decode(json: &JsonValue) -> crate::djson::Result<Self>;
}

impl<T: Copy + Decode> Decode for Option<T> {
    fn decode(json: &JsonValue) -> crate::djson::Result<Self> {
        match json {
            JsonValue::Nil => Ok(None),
            other => Ok(T::decode(other).ok())
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(json: &JsonValue) -> crate::djson::Result<Self> {
        let values = match json {
            JsonValue::Array(values) => values,
            _ => return Err(JsonFormatError::InvalidArray),
        };

        values.into_iter()
            .map(T::decode)
            .collect()
    }
}

impl Decode for u32 {
    fn decode(json: &JsonValue) -> crate::djson::Result<u32> {
        json.unsigned_int()
    }
}

impl Decode for u64 {
    fn decode(json: &JsonValue) -> crate::djson::Result<u64> {
        json.unsigned_long()
    }
}

impl Decode for i32 {
    fn decode(json: &JsonValue) -> crate::djson::Result<i32> {
        json.int()
    }
}

impl Decode for i64 {
    fn decode(json: &JsonValue) -> crate::djson::Result<i64> {
        json.long()
    }
}


impl Decode for f32 {
    fn decode(json: &JsonValue) -> crate::djson::Result<f32> {
        json.float()
    }
}

impl Decode for f64 {
    fn decode(json: &JsonValue) -> crate::djson::Result<f64> {
        json.double()
    }
}

impl Decode for String {
    fn decode(json: &JsonValue) -> crate::djson::Result<String> {
        json.string().map(String::from)
    }
}

impl Decode for bool {
    fn decode(json: &JsonValue) -> crate::djson::Result<bool> {
        json.bool()
    }
}