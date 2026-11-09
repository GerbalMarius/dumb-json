use crate::JsonValue;

pub trait Encode {
    fn encode(&self) -> JsonValue;
}


impl<T: Encode + Copy> Encode for Option<T>  {
    fn encode(&self) -> JsonValue {
        match *self {
            Some(value) => value.encode(),
            None => JsonValue::Nil
        }
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


impl Encode for usize {
    fn encode(&self) -> JsonValue {
        todo!("NOT IMPLEMENTED")
    }
}

impl Encode for u64 {
    fn encode(&self) -> JsonValue {
        todo!("NOT IMPLEMENTED")
    }
}

impl Encode for u32 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}
impl Encode for u16 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}
impl Encode for u8 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}

impl Encode for isize {
    fn encode(&self) -> JsonValue {
        todo!("NOT IMPLEMENTED")
    }
}

impl Encode for i64 {
    fn encode(&self) -> JsonValue {
        const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
        const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;

        assert!(
            (MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(self),
            "i64 cannot be represented exactly as f64"
        );

        JsonValue::Number(*self as f64)
    }
}

impl Encode for i32 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}

impl Encode for i16 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}

impl Encode for i8 {
    fn encode(&self) -> JsonValue {
        JsonValue::Number(f64::from(*self))
    }
}

impl Encode for bool {
    fn encode(&self) -> JsonValue {
        JsonValue::Bool(*self)
    }
}