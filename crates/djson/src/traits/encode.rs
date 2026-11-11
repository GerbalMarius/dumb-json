use std::{collections::HashMap, fmt::Display};

use crate::JsonValue;

pub trait Encode {
    fn encode(&self) -> JsonValue;
}

impl<T: Encode + Copy> Encode for Option<T> {
    fn encode(&self) -> JsonValue {
        match *self {
            Some(value) => value.encode(),
            None => JsonValue::Nil,
        }
    }
}

impl <T: Encode> Encode for Vec<T> {
    fn encode(&self) -> JsonValue {
        JsonValue::Array(self.iter().map(|value| value.encode()).collect())
    }
}
impl <T: Encode> Encode for &[T] {
    fn encode(&self) -> JsonValue {
        JsonValue::Array(self.iter().map(|value| value.encode()).collect())
    }
}

impl <K: Display, V: Encode> Encode for HashMap<K, V>  {
    fn encode(&self) -> JsonValue {
        JsonValue::Object(self.iter().map(|entry| (entry.0.to_string(), entry.1.encode())).collect())
    }
}

impl Encode for &str {
    fn encode(&self) -> JsonValue {
        JsonValue::String(self.to_string())
    }
}

impl Encode for String {
    fn encode(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl Encode for f64 {
    fn encode(&self) -> JsonValue {
        JsonValue::Double(*self)
    }
}

impl Encode for f32 {
    fn encode(&self) -> JsonValue {
        JsonValue::Float(*self)
    }
}

impl Encode for usize {
    fn encode(&self) -> JsonValue {
        match usize::BITS {
            32 => JsonValue::UnsignedInt(*self as u32),
            64 => JsonValue::UnsignedLong(*self as u64),
            _ => unreachable!(),
        }
    }
}

impl Encode for u64 {
    fn encode(&self) -> JsonValue {
        JsonValue::UnsignedLong(*self)
    }
}

impl Encode for u32 {
    fn encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(*self)
    }
}
impl Encode for u16 {
    fn encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(u32::from(*self))
    }
}
impl Encode for u8 {
    fn encode(&self) -> JsonValue {
        JsonValue::UnsignedInt(u32::from(*self))
    }
}

impl Encode for isize {
    fn encode(&self) -> JsonValue {
        match isize::BITS {
            32 => JsonValue::Int(*self as i32),
            64 => JsonValue::Long(*self as i64),
            _ => unreachable!(),
        }
    }
}

impl Encode for i64 {
    fn encode(&self) -> JsonValue {
        JsonValue::Long(*self)
    }
}

impl Encode for i32 {
    fn encode(&self) -> JsonValue {
        JsonValue::Int(*self)
    }
}

impl Encode for i16 {
    fn encode(&self) -> JsonValue {
        JsonValue::Int(i32::from(*self))
    }
}

impl Encode for i8 {
    fn encode(&self) -> JsonValue {
        JsonValue::Int(i32::from(*self))
    }
}

impl Encode for bool {
    fn encode(&self) -> JsonValue {
        JsonValue::Bool(*self)
    }
}
