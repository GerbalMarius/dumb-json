use crate::djson::lexer::bracket::Bracket;



#[derive(Debug, PartialEq)]
pub(crate) enum JsonToken {
    Bracket(Bracket),
    Colon,
    Comma,
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl JsonToken {
    pub fn number(&self) -> f64 {
        if let JsonToken::Number(num) = self {
            return *num;
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

