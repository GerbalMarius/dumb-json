use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Bracket {
    CurlyOpen,
    CurlyClosed,
    SquareOpen,
    SquareClosed
}

#[derive(Debug, PartialEq)]
pub enum JsonToken {
    Bracket(Bracket),
    Colon,
    Comma,
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl JsonToken {
    pub fn number(&self) -> f64 {
        if let JsonToken::Number(num) = self  {
            return  *num;
        }
        else {
            panic!("Json token is not a number");
        }
    }

    pub fn string(&self) -> String {
        if let JsonToken::String(s) = self {
            return  s.to_string();
        }
        else {
            panic!("Json token is not a string");
        }
    }

    pub fn boolean(&self) -> bool {
        if let JsonToken::Bool(b) = self {
            return *b;
        }
        else {
            panic!("Json token is not a bool");
        }
    }
}

#[derive(Debug)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl JsonValue {
    pub fn string(&self) -> String {
        if let JsonValue::String(s) = self {
             s.to_string()
        }
        else {
            panic!("json is not a valid string")
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonParseError {
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

type JsonResult = Result<JsonValue, JsonParseError>;

#[derive(Debug)]
pub struct JsonParser {
    tokens: std::iter::Peekable<std::vec::IntoIter<JsonToken>>,
    max_depth: usize,
}

impl JsonParser {

    pub fn new(tokens: Vec<JsonToken>, max_depth: usize) -> Self {
        Self { 
            tokens: tokens.into_iter().peekable(), 
            max_depth
        }
    }

    pub fn parse(&mut self, current_depth: usize) -> JsonResult {
        if current_depth >= self.max_depth {
            return Err(JsonParseError::new(&format!("Max depth={} exceeded, current_depth={}", self.max_depth, current_depth)));
        }

        match self.tokens.peek() {

            Some(JsonToken::Bracket(Bracket::CurlyOpen)) => {
                self.parse_object(current_depth)
            }

            Some(JsonToken::Bracket(Bracket::SquareOpen)) => {
                self.parse_array(current_depth)
            }

            Some(JsonToken::String(_)) => {
                self.parse_string()
            }

            Some(JsonToken::Number(_)) => {
                self.parse_number()
            }
            
            Some(JsonToken::Bool(_)) => {
                self.parse_bool()
            }

            Some(JsonToken::Nil) => {
                self.parse_null()
            }

            _ => Err(JsonParseError::new("Expected a valid json token"))
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
                return Err(JsonParseError::new(&format!("key must be valid string instead got: {:?}", key_token)));
            }

            let key = key_token.string();

            if !matches!(self.tokens.peek(), Some(JsonToken::Colon)) {
                return Err(JsonParseError::new(&format!("Key={} must be followed by a colon", &key)));
            }

            let value = self.parse(current_depth + 1)?;

            json_object.insert(key, value);

            match self.tokens.peek() {
                Some(JsonToken::Comma) => {
                    self.tokens.next();
                    continue;
                }
                Some(JsonToken::Bracket(Bracket::CurlyClosed)) => {
                    return Ok(JsonValue::Object(json_object));
                }
                Some(token) => {
                    return Err(JsonParseError::new(&format!("Unexpected token when parsing json object: {:?}", token)));
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
            return  Ok(JsonValue::Array(Vec::new()));
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
                    return Ok(JsonValue::Array(values));
                }
                Some(token) => {
                    return Err(JsonParseError::new(&format!("Unexpected token when parsing array: {:?}", token)));
                }
                None => {
                    return Err(JsonParseError::new("Unexpected end of array json"));
                }
            }
        }
    }

    fn parse_string(&mut self) -> JsonResult {
        self.tokens.next().filter(|t| matches!(*t, JsonToken::String(_)))
                          .map(|t| JsonValue::String(t.string()))
                          .ok_or(JsonParseError::new("Expected a valid string token"))
    }

    fn parse_number(&mut self) -> JsonResult {
        self.tokens.next().filter(|t| matches!(*t, JsonToken::Number(_)))
                         .map(|t| JsonValue::Number(t.number()))
                         .ok_or(JsonParseError::new("Expected a valid number token"))
    }

    fn parse_bool(&mut self) -> JsonResult {
       self.tokens.next().filter(|t| matches!(*t, JsonToken::Bool(_)))
                         .map(|t| JsonValue::Bool(t.boolean()))
                         .ok_or(JsonParseError::new("Expected a valid bool token"))
    }

    fn parse_null(&mut self) -> JsonResult {
          self.tokens.next().filter(|t| *t == JsonToken::Nil)
                            .map(|_| JsonValue::Nil)
                            .ok_or(JsonParseError::new("Expected a valid null token"))
    }
}

fn main() {
    println!("Hello");
}

pub fn tokenize(input: &str) -> Result<Vec<JsonToken>, JsonParseError> {
    if input.is_empty() {
        return Ok(vec![]);
    }

    let mut chars = input.chars().peekable();

    let mut tokens = Vec::<JsonToken>::new();

    while let Some(ch) = chars.peek() {
        match ch {
            '{' => {
                tokens.push(JsonToken::Bracket(Bracket::CurlyOpen));
                chars.next();
            }
            '}' => {
                tokens.push(JsonToken::Bracket(Bracket::CurlyClosed));
                chars.next();
            }
            '[' => {
                tokens.push(JsonToken::Bracket(Bracket::SquareOpen));
                chars.next();
            }
            ']' => {
                tokens.push(JsonToken::Bracket(Bracket::SquareClosed));
                chars.next();
            }
            ':' => {
                tokens.push(JsonToken::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(JsonToken::Comma);
                chars.next();
            }
            '"' => {
                tokens.push(JsonToken::String(tok_str_literal(&mut chars)?));
            }
            '0'..='9' | '-' => {
                tokens.push(JsonToken::Number(tok_number(&mut chars)?));
            }
            'f' | 't' => {
                tokens.push(JsonToken::Bool(tok_bool(&mut chars)?));
            },
            'n' => {
                tokens.push(tok_null(&mut chars)?);
            }
            _ if ch.is_whitespace() => continue,
            _ => return  Err(JsonParseError::new(&format!("Inproperly formed json string: {}", input)))
        }
    }

    Ok(tokens)
}

fn tok_null(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<JsonToken, JsonParseError> {
    const NULL_LITERAL: &str = "null";

    for expected in NULL_LITERAL.chars() {
        match chars.next() {
            Some(actual) if actual == expected => continue,
            _ => return Err(JsonParseError::new("Invalid null literal")) 
        }
    }
    Ok(JsonToken::Nil)
}

fn tok_bool(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<bool, JsonParseError> {
    const TRUE_LITERAL: &str = "true";
    const FALSE_LITERAL: &str = "false";

    let literal = match chars.peek() {
        Some('t') => TRUE_LITERAL,
        Some('f') => FALSE_LITERAL,
        _ => return  Err(JsonParseError::new("Unexpected bool literal"))
    };

    for expected in literal.chars() {
        match chars.next() {
            Some(actual) if actual == expected => continue,
            _ => return Err(JsonParseError::new("Invalid bool literal"))
        }
    }

    return  Ok(literal == TRUE_LITERAL);
}

fn tok_number(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<f64, JsonParseError> {
    let mut buff = String::new();

    const ERR_MSG: &str = "Invalid number";

    if let Some(&'-') = chars.peek() {
        buff.push(chars.next().unwrap());
    }

    match chars.peek() {
        Some('0') => {
            buff.push(chars.next().unwrap());

            if let Some(ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    return Err(JsonParseError::new(ERR_MSG));
                }
            }
        }
        Some(ch) if ch.is_ascii_digit() => {
            while let Some(&val) = chars.peek()
                && val.is_ascii_digit()
            {
                buff.push(chars.next().unwrap());
            }
        }
        _ => return Err(JsonParseError::new(ERR_MSG)),
    }

    if let Some(&'.') = chars.peek() {
        buff.push(chars.next().unwrap());

        let mut count: i32 = 0;

        while let Some(&ch) = chars.peek()
            && ch.is_ascii_digit()
        {
            buff.push(chars.next().unwrap());
            count += 1;
        }

        if count == 0 {
            return Err(JsonParseError::new(ERR_MSG));
        }
    }

    if let Some(&ch) = chars.peek() {
        if ch == 'e' || ch == 'E' {
            buff.push(chars.next().unwrap());

            if let Some(&sign) = chars.peek() {
                if sign == '+' || sign == '-' {
                    buff.push(chars.next().unwrap());
                }
            }

            let mut count: i32 = 0;

            while let Some(&ch) = chars.peek()
                && ch.is_ascii_digit()
            {
                buff.push(chars.next().unwrap());
                count += 1;
            }

            if count == 0 {
                return Err(JsonParseError::new(ERR_MSG));
            }
        }
    }

    buff.parse::<f64>()
        .map_err(|err| JsonParseError::new(&err.to_string()))
}

fn tok_str_literal(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, JsonParseError> {
    let mut buff = String::new();
    chars.next(); //skip opening quote

    loop {
        match chars.next() {
            Some('"') => return Ok(buff),
            Some('\\') => match chars.next() {
                Some('"') => buff.push('"'),
                Some('\\') => buff.push('\\'),
                Some('/') => buff.push('/'),
                Some('b') => buff.push('\x08'),
                Some('f') => buff.push('\x0C'),
                Some('n') => buff.push('\n'),
                Some('r') => buff.push('\r'),
                Some('t') => buff.push('\t'),
                Some(other) => {
                    return Err(JsonParseError::new(&format!(
                        "Invalid escape sequence \\{}",
                        other
                    )));
                }
                None => return Err(JsonParseError::new("Unexpected end of string after escape")),
            },
            Some(ch) if ch.is_control() => {
                return Err(JsonParseError::new("Unexpected control char in string"));
            }

            Some(ch) => buff.push(ch),

            None => return Err(JsonParseError::new("Unterminated string")),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn tokenize_with_empty_str_parses_zero_tokens() {
        let input = "";

        let res = tokenize(input);
        assert!(res.is_ok());
        assert!(res.unwrap().is_empty());
    }

    #[test]
    fn tokenize_with_integer_parses_the_number_tokens() {
        let input = "1";
        let res = tokenize(input);

        assert!(res.is_ok());

        let tokens = res.unwrap();
        
        let item  = &tokens[0];

        let actual = item.number() as i32;

        assert_eq!(actual, input.parse::<i32>().unwrap());
    }

    #[test]
    fn tokenize_with_negative_integer_parses_the_number_tokens() {
        let input = "-1";
        let res = tokenize(input);

        assert!(res.is_ok());
        let tokens = res.unwrap();

        let item = &tokens[0];

        let actual = item.number() as i32;

        assert_eq!(actual, input.parse::<i32>().unwrap());
    }

    #[parameterized(input = {
        "1.0", "-1.523", "1e6", "1e-2", "-1e2"
    }, 
    expected = {
        1.0, -1.523, 1000_000.0, 0.01, -100.0
    })]
    fn tokenize_with_float_parses_the_number_tokens(input: &str, expected: f64) {
        let res = tokenize(input);
        assert!(res.is_ok());

        let tokens = res.unwrap();

        let item = &tokens[0];

        let actual = item.number();

        assert!((actual - expected).abs() < f64::EPSILON, "actual={}, expected={}", actual, expected);       
    }


}
