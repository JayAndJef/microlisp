use crate::lexer::TokenKind;

/// Represents a microlisp language object
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    /// Represents no value or void
    Void,
    /// Floating-point number
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// Symbol or identifier
    Symbol(String),
    /// Lambda function with parameters and body
    Lambda(Vec<String>, Vec<Object>),
    /// List of objects
    List(Vec<Object>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError(String);

/// Parses a sequence of tokens into a nested Object structure
/// 
/// Takes a mutable vector of tokens and converts them into the appropriate
/// Object representation, handling nested expressions recursively
pub fn parse(tokens: &mut Vec<TokenKind>) -> Result<Object, ParseError> {
    let top = tokens.pop().unwrap();
    if top != TokenKind::LParen {
        ParseError(format!("Expected Start of list, found {:?}", top));
    }

    let mut built_list = Vec::new();
    while let Some(top) = tokens.last() {
        built_list.push(match top {
            TokenKind::Float(f) => Object::Float(*f),
            TokenKind::Symbol(s) => Object::Symbol(s.to_string()),
            TokenKind::LParen => parse(tokens)?, // take that nerd
            TokenKind::RParen => {
                return Ok(Object::List(built_list));
            }
        });
        tokens.pop();
    }

    Ok(Object::List(built_list))
}

#[cfg(test)]
mod tests {
    use super::Object::*;
    use super::*;
    use crate::lexer::TokenKind;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse(&mut vec![
                TokenKind::RParen,
                TokenKind::RParen,
                TokenKind::Float(2.0),
                TokenKind::Float(1.0),
                TokenKind::Symbol("+".to_string()),
                TokenKind::LParen,
                TokenKind::Float(3.5),
                TokenKind::Symbol("*".to_string()),
                TokenKind::LParen,
            ])
            .unwrap(),
            List(vec![
                Symbol("*".to_string()),
                Float(3.5),
                List(vec![Symbol("+".to_string()), Float(1.0), Float(2.0),])
            ])
        )
    }
}
