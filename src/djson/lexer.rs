use crate::djson::{lexer::{bracket::Bracket, json_token::JsonToken}};

pub(crate) mod bracket;
pub(crate) mod json_token;


pub fn tokenize(input: &str) -> Result<Vec<JsonToken>, String> {
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
            _ if ch.is_whitespace() => {
                chars.next();
            }
            _ => return  Err(format!("Inproperly formed json string: {}", input))
        }
    }

    Ok(tokens)
}


fn tok_null(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<JsonToken, String> {
    const NULL_LITERAL: &str = "null";

    for expected in NULL_LITERAL.chars() {
        match chars.next() {
            Some(actual) if actual == expected => continue,
            _ => return Err("Invalid null literal".to_string()) 
        }
    }
    Ok(JsonToken::Nil)
}

fn tok_bool(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<bool, String> {
    const TRUE_LITERAL: &str = "true";
    const FALSE_LITERAL: &str = "false";

    let literal = match chars.peek() {
        Some('t') => TRUE_LITERAL,
        Some('f') => FALSE_LITERAL,
        _ => return  Err("Unexpected bool literal".to_string())
    };

    for expected in literal.chars() {
        match chars.next() {
            Some(actual) if actual == expected => continue,
            _ => return Err("Invalid bool literal".to_string())
        }
    }

    return  Ok(literal == TRUE_LITERAL);
}

fn tok_number(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<f64, String> {
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
                    return Err(ERR_MSG.to_string());
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
        _ => return Err(ERR_MSG.to_string()),
    }

    if let Some(&'.') = chars.peek() {
        buff.push(chars.next().unwrap());

        let mut count: usize = 0;

        while let Some(&ch) = chars.peek()
            && ch.is_ascii_digit()
        {
            buff.push(chars.next().unwrap());
            count += 1;
        }

        if count == 0 {
            return Err(ERR_MSG.to_string());
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

            let mut count: usize = 0;

            while let Some(&ch) = chars.peek()
                && ch.is_ascii_digit()
            {
                buff.push(chars.next().unwrap());
                count += 1;
            }

            if count == 0 {
                return Err(ERR_MSG.to_string());
            }
        }
    }

    buff.parse::<f64>()
        .map_err(|err| err.to_string())
}

fn tok_str_literal(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, String> {
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
                    return Err(format!(
                        "Invalid escape sequence \\{}",
                        other
                    ));
                }
                None => return Err("Unexpected end of string after escape".to_string()),
            },
            Some(ch) if ch.is_control() => {
                buff.push_str(&format!("\\u{:04x}", ch as u32));
            }

            Some(ch) => buff.push(ch),

            None => return Err("Unterminated string".to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
        use std::assert_matches;
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

                assert_matches!(item, JsonToken::Number(1.0))
        }

        #[test]
        fn tokenize_with_negative_integer_parses_the_number_tokens() {
                let input = "-1";
                let res = tokenize(input);

                assert!(res.is_ok());
                let tokens = res.unwrap();

                let item = &tokens[0];

                assert_matches!(item, JsonToken::Number(-1.0));
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
