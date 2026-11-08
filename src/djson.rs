mod lexer;

use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
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
            _ => Err(JsonFormatError::InvalidArray)
        }
    }

    pub fn string(&self) -> Result<&str> {
        match self {
            JsonValue::String(value) => Ok(value),
            _ => Err(JsonFormatError::InvalidString)
        }
    }

    pub fn number(&self) -> Result<f64> {
        match self {
            JsonValue::Number(value) => Ok(*value),
            _ => Err(JsonFormatError::InvalidNumber)
        }
    }

    pub fn bool(&self) -> Result<bool> {
        match self {
            JsonValue::Bool(value) => Ok(*value),
            _ => Err(JsonFormatError::InvalidArray)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonFormatError {
    MissingKey(String),
    ParsingIssue(String),
    InvalidObject,
    InvalidArray,
    InvalidString,
    InvalidNumber,
    InvalidBool,
    InvalidNull,
}

#[allow(unused)]
impl std::fmt::Display for JsonFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output_str: &str = match self {
            JsonFormatError::MissingKey(msg) => msg,
            JsonFormatError::ParsingIssue(msg) => msg,
            JsonFormatError::InvalidObject => "Value is not a valid json object",
            JsonFormatError::InvalidArray => "Value is not a valid json array",
            JsonFormatError::InvalidString => "Value is not a valid json string",
            JsonFormatError::InvalidNumber => "Value is not a valid json number",
            JsonFormatError::InvalidBool => "Value is not a valid json boolean",
            JsonFormatError::InvalidNull => "Value is not a valid json null value",
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

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.msg)
    }
}

pub trait Encode {
    fn encode(&self) -> String;
}

pub trait Decode<T> {
    fn decode(json: JsonValue) -> Result<T>;
}

pub fn decode_json_str(input: &str) -> Result<JsonValue> {
    let tokens = lexer::tokenize_str(input)
                                    .map_err(|err| JsonFormatError::ParsingIssue(err))?;

    let mut parser = internal::JsonParser::with_defaults(tokens);

    return parser.parse(0).map_err(|err| JsonFormatError::ParsingIssue(err.msg));
}

pub fn decode_file<P>(file_path: P) -> Result<JsonValue> 
where P: AsRef<std::path::Path> {
    let tokens = lexer::tokenize_path(file_path)
                                    .map_err(|err| JsonFormatError::ParsingIssue(err))?;
    
    let mut parser = internal::JsonParser::with_defaults(tokens);

    return parser.parse(0).map_err(|err| JsonFormatError::ParsingIssue(err.msg));
}

pub fn encode<T>(item: &T) -> String
where
    T: Encode,
{
    item.encode()
}

pub fn decode<T>(json_str: &str) -> Result<T>
where
    T: Decode<T>,
{
    let result = decode_json_str(json_str).map_err(|_| JsonFormatError::InvalidObject)?;

    return T::decode(result);
}

mod internal {
    use crate::djson::{
        JsonParseError, JsonValue,
        lexer::{bracket::Bracket, json_token::JsonToken},
    };
    use std::collections::HashMap;

    type JsonResult = Result<JsonValue, JsonParseError>;

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

        pub fn parse(&mut self, current_depth: usize) -> JsonResult {
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

        fn parse_object(&mut self, current_depth: usize) -> JsonResult {
            self.tokens.next();

            if let Some(JsonToken::Bracket(Bracket::CurlyClosed)) = self.tokens.peek() {
                return Ok(JsonValue::Object(HashMap::new()));
            }

            let mut json_object: HashMap<String, JsonValue> = HashMap::new();

            loop {
                let key_token = self.parse(current_depth + 1)?;

                if !matches!(key_token, JsonValue::String(_)) {
                    return Err(JsonParseError::new(&format!(
                        "key must be valid string instead got: {:?}",
                        key_token
                    )));
                }

                let key = key_token.string().expect("key_token is a valid string when entering this branch");

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
                        self.tokens.next();//consume the closing brace;
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

        fn parse_array(&mut self, current_depth: usize) -> JsonResult {
            self.tokens.next();

            if let Some(JsonToken::Bracket(Bracket::SquareClosed)) = self.tokens.peek() {
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

        fn parse_string(&mut self) -> JsonResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::String(_)))
                .map(|t| JsonValue::String(t.string()))
                .ok_or(JsonParseError::new("Expected a valid string token"))
        }

        fn parse_number(&mut self) -> JsonResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::Number(_)))
                .map(|t| JsonValue::Number(t.number()))
                .ok_or(JsonParseError::new("Expected a valid number token"))
        }

        fn parse_bool(&mut self) -> JsonResult {
            self.tokens
                .next()
                .filter(|t| matches!(*t, JsonToken::Bool(_)))
                .map(|t| JsonValue::Bool(t.boolean()))
                .ok_or(JsonParseError::new("Expected a valid bool token"))
        }

        fn parse_null(&mut self) -> JsonResult {
            self.tokens
                .next()
                .filter(|t| *t == JsonToken::Nil)
                .map(|_| JsonValue::Nil)
                .ok_or(JsonParseError::new("Expected a valid null token"))
        }
    }
}
