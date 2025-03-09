/// Splits a source string into tokens for the lexer
pub fn tokenize(source: &str) -> Vec<String> {
    source
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

/// Different token types that can be produced by the lexer
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// Left parenthesis '('
    LParen,
    /// Right parenthesis ')'
    RParen,
    /// Floating-point numeric value
    Float(f64),
    /// Symbol or identifier string
    Symbol(String),
}

/// Converts a sequence of string tokens into typed TokenKind values
pub fn lex(tokens: &[String]) -> Vec<TokenKind> {
    tokens.iter().map(|t| lex_single_token(t)).collect()
}

/// Converts a single string token into its appropriate TokenKind
pub fn lex_single_token(token: &str) -> TokenKind {
    match token {
        "(" => TokenKind::LParen,
        ")" => TokenKind::RParen,
        _ => {
            if let Ok(float) = token.parse::<f64>() {
                TokenKind::Float(float)
            } else {
                TokenKind::Symbol(token.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            tokenize("(+ (- 2 4) (* 7 5))"),
            ["(", "+", "(", "-", "2", "4", ")", "(", "*", "7", "5", ")", ")"],
        )
    }

    #[test]
    fn test_lexer() {
        assert_eq!(
            lex(&["(", "+", "2", "3.4", ")"].map(|i| i.to_string())),
            [
                LParen,
                Symbol("+".to_string()),
                Float(2.0),
                Float(3.4),
                RParen,
            ]
        )
    }
}
