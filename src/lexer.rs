
pub fn tokenize(source: &str) -> Vec<String> {
    source.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Integer(i64),
    Float(f64),
    Symbol(String),
}

pub fn lex(tokens: &[String]) -> Vec<TokenKind> {
    tokens.iter()
        .map(|t| lex_single_token(t))
        .collect()
}

pub fn lex_single_token(token: &str) -> TokenKind {
    match token {
        "(" => TokenKind::LParen,
        ")" => TokenKind::RParen,
        _ => {
            if let Ok(int) = token.parse::<i64>() {
                TokenKind::Integer(int)
            } else if let Ok(float) = token.parse::<f64>() {
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
                Integer(2),
                Float(3.4),
                RParen,
            ]
        )
    }
}
