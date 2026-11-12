use std::{
    fmt,
    io::{self, Write},
};

use crate::{
    lexer::lexer,
    traits::{decode::Decode, encode::Encode},
};

#[cfg(any(
    all(feature = "ordered", feature = "unordered"),
    all(feature = "ordered", feature = "sorted"),
    all(feature = "unordered", feature = "sorted"),
))]
compile_error!("Features ordered, unordered, and sorted are mutually exclusive");


#[cfg(feature = "ordered")]
pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[cfg(feature = "sorted")]
pub type Map<K, V> = std::collections::BTreeMap<K, V>;

#[cfg(feature = "unordered")]
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Map<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Double(f64),
    Long(i64),
    UnsignedLong(u64),
    Int(i32),
    Float(f32),
    UnsignedInt(u32),
    Bool(bool),
    Nil,
}

pub type Result<T> = core::result::Result<T, JsonFormatError>;

impl JsonValue {
    pub fn get(&self, key: &str) -> Result<&JsonValue> {
        match self {
            JsonValue::Object(obj) => {
                let err = JsonFormatError::MissingKey(format!("key={} not found in object", key));
                return obj.get(key).ok_or_else(|| err);
            }
            _ => Err(JsonFormatError::InvalidObject),
        }
    }

    pub fn array(&self) -> Result<&[JsonValue]> {
        match self {
            JsonValue::Array(values) => Ok(values),
            _ => Err(JsonFormatError::InvalidArray),
        }
    }

    pub fn string(&self) -> Result<&str> {
        match self {
            JsonValue::String(v) => Ok(v),
            _ => Err(JsonFormatError::InvalidString),
        }
    }

    pub fn float(&self) -> Result<f32> {
        match self {
            JsonValue::Float(v) => Ok(*v),
            _ => Err(JsonFormatError::InvalidFloat),
        }
    }

    pub fn double(&self) -> Result<f64> {
        match self {
            JsonValue::Double(v) => Ok(*v),
            _ => Err(JsonFormatError::InvalidFloat),
        }
    }

    pub fn long(&self) -> Result<i64> {
        match self {
            JsonValue::Long(v) => Ok(*v),
            JsonValue::UnsignedLong(v) => Ok(*v as i64),
            _ => Err(JsonFormatError::InvalidInt),
        }
    }

    pub fn unsigned_long(&self) -> Result<u64> {
        match self {
            JsonValue::UnsignedLong(v) => Ok(*v),
            JsonValue::Long(v) => Ok(*v as u64),
            _ => Err(JsonFormatError::InvalidUnsingedInt),
        }
    }

    pub fn int(&self) -> Result<i32> {
        match self {
            JsonValue::Int(v) => Ok(*v),
            JsonValue::UnsignedInt(v) => Ok(*v as i32),
            _ => Err(JsonFormatError::InvalidInt),
        }
    }

    pub fn unsigned_int(&self) -> Result<u32> {
        match self {
            JsonValue::UnsignedInt(v) => Ok(*v),
            JsonValue::Int(v) => Ok(*v as u32),
            _ => Err(JsonFormatError::InvalidUnsingedInt),
        }
    }

    pub fn bool(&self) -> Result<bool> {
        match self {
            JsonValue::Bool(value) => Ok(*value),
            _ => Err(JsonFormatError::InvalidBool),
        }
    }

    pub fn pretty_print_str(&self) -> String {
        let mut buff = String::new();

        Self::write_json(self, &mut buff, 0, Some(2)).expect("Buy more RAM lol");

        return buff;
    }

    fn write_to<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut adapter = internal::IOWriter { writer };

        Self::write_json(self, &mut adapter, 0, Some(2))
            .map_err(|_| io::Error::other("Failed to serialize json value"))?;
        Ok(())
    }

    fn parse_number(s: &str) -> Result<JsonValue> {
        if let Ok(v) = s.parse::<i32>() {
            return Ok(JsonValue::Int(v));
        }

        if let Ok(v) = s.parse::<u32>() {
            return Ok(JsonValue::UnsignedInt(v));
        }

        if let Ok(v) = s.parse::<i64>() {
            return Ok(JsonValue::Long(v));
        }

        if let Ok(v) = s.parse::<u64>() {
            return Ok(JsonValue::UnsignedLong(v));
        }

        if let Ok(v) = s.parse::<f64>() {
            if v.is_finite() {
                return Ok(JsonValue::Double(v));
            }
        }

        Err(JsonFormatError::InvalidNumber)
    }

    fn write_json(
        value: &JsonValue,
        f: &mut dyn fmt::Write,
        depth: u32,
        indent: Option<u32>,
    ) -> fmt::Result {
        match value {
            JsonValue::String(s) => Self::write_json_string(s, f),
            JsonValue::Object(obj) => Self::write_object(obj, f, depth, indent),
            JsonValue::Array(values) => Self::write_array(values, f, depth, indent),
            JsonValue::Double(value) => Self::write_double(*value, f),
            JsonValue::Float(value) => Self::write_float(*value, f),
            JsonValue::Long(long) => write!(f, "{long}"),
            JsonValue::UnsignedLong(ulong) => write!(f, "{ulong}"),
            JsonValue::Int(int) => write!(f, "{int}"),
            JsonValue::UnsignedInt(uint) => write!(f, "{uint}"),
            JsonValue::Bool(bool) => write!(f, "{bool}"),
            JsonValue::Nil => f.write_str("null"),
        }
    }

    fn write_indent(f: &mut dyn fmt::Write, spaces: u32) -> fmt::Result {
        for _ in 0..spaces {
            f.write_char(' ')?;
        }

        Ok(())
    }

    fn write_json_string(value: &str, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_char('"')?;

        for ch in value.chars() {
            match ch {
                '"' => f.write_str("\\\"")?,
                '\\' => f.write_str("\\\\")?,
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\t' => f.write_str("\\t")?,
                '\u{08}' => f.write_str("\\b")?,
                '\u{0C}' => f.write_str("\\f")?,
                ch if ch.is_control() => {
                    write!(f, "\\u{:04x}", ch as u32)?;
                }
                ch => f.write_char(ch)?,
            }
        }

        f.write_char('"')
    }

    fn write_double(value: f64, f: &mut dyn fmt::Write) -> fmt::Result {
        if !value.is_finite() {
            return Err(fmt::Error);
        }
        return write!(f, "{value}");
    }

    fn write_float(value: f32, f: &mut dyn fmt::Write) -> fmt::Result {
        if !value.is_finite() {
            return Err(fmt::Error);
        }
        return write!(f, "{value}");
    }

    fn write_array(
        values: &[JsonValue],
        f: &mut dyn fmt::Write,
        depth: u32,
        indent: Option<u32>,
    ) -> fmt::Result {
        f.write_char('[')?;

        for (idx, value) in values.iter().enumerate() {
            if idx > 0 {
                f.write_char(',')?;
            }

            if let Some(spaces) = indent {
                f.write_char('\n')?;
                Self::write_indent(f, (depth + 1) * spaces)?;
            }

            Self::write_json(value, f, depth + 1, indent)?;
        }

        if !values.is_empty() {
            if let Some(spaces) = indent {
                f.write_char('\n')?;
                Self::write_indent(f, depth * spaces)?;
            }
        }
        return f.write_char(']');
    }

    fn write_object(
        obj: &Map<String, JsonValue>,
        f: &mut dyn fmt::Write,
        depth: u32,
        indent: Option<u32>,
    ) -> fmt::Result {
        f.write_char('{')?;

        for (i, (key, value)) in obj.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }

            if let Some(spaces) = indent {
                f.write_char('\n')?;
                Self::write_indent(f, (depth + 1) * spaces)?;
            }

            Self::write_json_string(key, f)?;
            f.write_char(':')?;

            if indent.is_some() {
                f.write_char(' ')?;
            }

            Self::write_json(value, f, depth + 1, indent)?;
        }

        if !obj.is_empty() {
            if let Some(spaces) = indent {
                f.write_char('\n')?;
                Self::write_indent(f, depth * spaces)?;
            }
        }

        return f.write_char('}');
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::write_json(&self, f, 0, None)
    }
}

impl std::ops::Index<&str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array().unwrap()[index]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonFormatError {
    MissingKey(String),
    ParsingIssue(String),
    MissingField,
    InvalidObject,
    InvalidArray,
    InvalidString,
    InvalidNumber,
    InvalidFloat,
    InvalidInt,
    InvalidUnsingedInt,
    InvalidBool,
    InvalidNull,
}

impl fmt::Display for JsonFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output_str: &str = match self {
            JsonFormatError::MissingKey(msg) => msg,
            JsonFormatError::ParsingIssue(msg) => msg,
            JsonFormatError::InvalidObject => "Value is not a valid  object",
            JsonFormatError::InvalidArray => "Value is not a valid  array",
            JsonFormatError::InvalidString => "Value is not a valid  string",
            JsonFormatError::InvalidFloat => "Value is not a valid  float",
            JsonFormatError::InvalidInt => "Value is not a valid  singed integer",
            JsonFormatError::InvalidUnsingedInt => "Value is not a valid unsinged integer",
            JsonFormatError::InvalidBool => "Value is not a valid  boolean",
            JsonFormatError::InvalidNull => "Value is not a valid  null value",
            JsonFormatError::InvalidNumber => "Value is not a valid number",
            JsonFormatError::MissingField => "Value is missing a required field",
        };
        f.write_str(output_str)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JsonParseError {
    msg: String,
}

impl JsonParseError {
    pub fn new(msg: &str) -> Self {
        JsonParseError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.msg)
    }
}

pub fn decode_json_str(input: &str) -> Result<JsonValue> {
    let tokens = lexer::tokenize_str(input).map_err(|err| JsonFormatError::ParsingIssue(err))?;

    let mut parser = internal::JsonParser::with_defaults(tokens);

    return parser
        .parse(0)
        .map_err(|err| JsonFormatError::ParsingIssue(err.msg));
}

pub fn decode_file<P>(file_path: P) -> Result<JsonValue>
where
    P: AsRef<std::path::Path>,
{
    let tokens =
        lexer::tokenize_path(file_path).map_err(|err| JsonFormatError::ParsingIssue(err))?;

    let mut parser = internal::JsonParser::with_defaults(tokens);

    return parser
        .parse(0)
        .map_err(|err| JsonFormatError::ParsingIssue(err.msg));
}

pub fn encode<T>(item: &T) -> JsonValue
where
    T: Encode,
{
    item.encode()
}

pub fn pp_print_stdout(json: &JsonValue) -> io::Result<()> {
    let mut out_lock = io::stdout().lock();

    JsonValue::write_to(&json, &mut out_lock)?;
    out_lock.write_all(crate::NEW_LINE.as_bytes())
}

pub fn pp_print<W: io::Write>(json: &JsonValue, writer: &mut W) -> io::Result<()> {
    JsonValue::write_to(&json, writer)
}

pub fn decode_str<T>(json_str: &str) -> Result<T>
where
    T: Decode,
{
    let result = decode_json_str(json_str).map_err(|_| JsonFormatError::InvalidObject)?;

    return T::decode(&result);
}

pub fn decode_json<T>(json: &JsonValue) -> Result<T>
where
    T: Decode,
{
    return T::decode(json);
}

mod internal {
    use crate::{
        djson::{JsonParseError, JsonValue, Map},
        lexer::{bracket::Bracket, json_token::JsonToken},
    };
    use core::fmt;
    use std::io;

    type JsonParserResult = Result<JsonValue, JsonParseError>;

    #[derive(Debug)]
    pub struct JsonParser {
        tokens: std::iter::Peekable<std::vec::IntoIter<JsonToken>>,
        max_depth: usize,
    }

    impl JsonParser {
        pub fn with_defaults(tokens: Vec<JsonToken>) -> Self {
            Self::new(tokens, 5)
        }

        fn new(tokens: Vec<JsonToken>, max_depth: usize) -> Self {
            Self {
                tokens: tokens.into_iter().peekable(),
                max_depth,
            }
        }

        pub fn parse(&mut self, current_depth: usize) -> JsonParserResult {
            if current_depth >= self.max_depth {
                return Err(JsonParseError::new(&format!(
                    "Max depth={} exceeded, current_depth={}",
                    self.max_depth, current_depth
                )));
            }

            match self.tokens.peek() {
                Some(JsonToken::Bracket(Bracket::CurlyOpen)) => self.parse_object(current_depth),

                Some(JsonToken::Bracket(Bracket::SquareOpen)) => self.parse_array(current_depth),

                Some(JsonToken::String(_)) => self.parse_string(),

                Some(JsonToken::Number(_)) => self.parse_number(),

                Some(JsonToken::Bool(_)) => self.parse_bool(),

                Some(JsonToken::Nil) => self.parse_null(),

                _ => Err(JsonParseError::new("Expected a valid json token")),
            }
        }

        fn parse_object(&mut self, current_depth: usize) -> JsonParserResult {
            self.tokens.next();

            if let Some(JsonToken::Bracket(Bracket::CurlyClosed)) = self.tokens.peek() {
                self.tokens.next();
                return Ok(JsonValue::Object(Map::new()));
            }

            let mut json_object: Map<String, JsonValue> = Map::new();

            loop {
                let key_token = self.parse(current_depth + 1)?;

                if !matches!(key_token, JsonValue::String(_)) {
                    return Err(JsonParseError::new(&format!(
                        "key must be valid string instead got: {:?}",
                        key_token
                    )));
                }

                let key = key_token
                    .string()
                    .expect("key_token is a valid string when entering this branch");

                if !matches!(self.tokens.peek(), Some(JsonToken::Colon)) {
                    return Err(JsonParseError::new(&format!(
                        "Key={} must be followed by a colon",
                        &key
                    )));
                }
                self.tokens.next(); //consume colon

                let value = self.parse(current_depth + 1)?;

                json_object.insert(key.to_string(), value);

                match self.tokens.peek() {
                    Some(JsonToken::Comma) => {
                        self.tokens.next();
                        continue;
                    }
                    Some(JsonToken::Bracket(Bracket::CurlyClosed)) => {
                        self.tokens.next(); //consume the closing brace;
                        return Ok(JsonValue::Object(json_object));
                    }
                    Some(token) => {
                        return Err(JsonParseError::new(&format!(
                            "Unexpected token when parsing json object: {:?}",
                            token
                        )));
                    }
                    None => {
                        return Err(JsonParseError::new("Unexpected end of json object"));
                    }
                }
            }
        }

        fn parse_array(&mut self, current_depth: usize) -> JsonParserResult {
            self.tokens.next();

            if let Some(JsonToken::Bracket(Bracket::SquareClosed)) = self.tokens.peek() {
                self.tokens.next();
                return Ok(JsonValue::Array(Vec::new()));
            }

            let mut values: Vec<JsonValue> = Vec::new();

            loop {
                let value = self.parse(current_depth + 1)?;

                values.push(value);

                match self.tokens.peek() {
                    Some(JsonToken::Comma) => {
                        self.tokens.next();
                        continue;
                    }
                    Some(JsonToken::Bracket(Bracket::SquareClosed)) => {
                        self.tokens.next(); //consume the closing brace
                        return Ok(JsonValue::Array(values));
                    }
                    Some(token) => {
                        return Err(JsonParseError::new(&format!(
                            "Unexpected token when parsing array: {:?}",
                            token
                        )));
                    }
                    None => {
                        return Err(JsonParseError::new("Unexpected end of array json"));
                    }
                }
            }
        }

        fn parse_string(&mut self) -> JsonParserResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::String(_)))
                .map(|t| JsonValue::String(t.string()))
                .ok_or(JsonParseError::new("Expected a valid string token"))
        }

        fn parse_number(&mut self) -> JsonParserResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::Number(_)))
                .map(|t| JsonValue::parse_number(&t.number()).unwrap())
                .ok_or(JsonParseError::new("Expected a valid number token"))
        }

        fn parse_bool(&mut self) -> JsonParserResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::Bool(_)))
                .map(|t| JsonValue::Bool(t.boolean()))
                .ok_or(JsonParseError::new("Expected a valid bool token"))
        }

        fn parse_null(&mut self) -> JsonParserResult {
            self.tokens
                .next()
                .filter(|t| *t == JsonToken::Nil)
                .map(|_| JsonValue::Nil)
                .ok_or(JsonParseError::new("Expected a valid null token"))
        }
    }

    pub struct IOWriter<'a, W: io::Write> {
        pub writer: &'a mut W,
    }

    impl<'a, W: io::Write> fmt::Write for IOWriter<'a, W> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.writer.write_all(s.as_bytes()).map_err(|_| fmt::Error)
        }
    }
}
