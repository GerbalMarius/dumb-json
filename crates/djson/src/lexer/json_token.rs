use crate::lexer::bracket::Bracket;

#[derive(Debug, PartialEq)]
pub enum JsonToken {
    String(String),
    Number(String),
    Bracket(Bracket),
    Bool(bool),
    Colon,
    Comma,
    Nil,
}

impl JsonToken {
    //helpers for extracting terminal tokens, used internally, do not care if they crash only
    pub fn number(&self) -> String {
        if let JsonToken::Number(num) = self {
            return num.to_string();
        } else {
            panic!("Json token is not a number");
        }
    }

    pub fn string(&self) -> String {
        if let JsonToken::String(s) = self {
            return s.to_string();
        } else {
            panic!("Json token is not a string");
        }
    }

    pub fn boolean(&self) -> bool {
        if let JsonToken::Bool(b) = self {
            return *b;
        } else {
            panic!("Json token is not a bool");
        }
    }
}
