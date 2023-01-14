use crate::lexer::{Lexer, Location};

#[derive(Debug)]
pub struct Parser<'a> {
    token: String,
    lexer: &'a mut Lexer,
}

#[derive(Debug)]
pub struct Token {
    typee: String,
    value: String,
    location: Location,
}

#[derive(Debug)]
pub struct AST {
    program: Vec<Token>,
}

struct NumericLiteral {}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            token: "".to_string(),
            lexer,
        }
    }

    fn numeric_literal(&mut self) -> Option<Token> {
        let token = self.eat("number".to_string());

        if let Some(t) = token {
            return Some(t);
        }
        None
    }

    pub fn parse(&mut self) -> AST {
        AST {
            program: vec![self.numeric_literal().unwrap()],
        }
    }

    fn eat(&mut self, token_type: String) -> Option<Token> {
        match self.lexer.next_token() {
            Some(t) => {
                if token_type == t.typee {
                    return Some(Token {
                        typee: t.typee,
                        value: t.value.to_string(),
                        location: t.loc,
                    });
                }
            }
            _ => (),
        }
        None
    }
}
