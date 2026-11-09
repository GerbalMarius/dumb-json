use crate::JsonValue;

pub trait Decode<T> {
    fn decode(json: JsonValue) -> crate::djson::Result<T>;
}