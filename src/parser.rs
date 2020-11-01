use crate::lexer::Lexer;
use crate::ast::*;
use crate::token::Tokens;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    prev: Option<Tokens>,
}

impl Parser<'_> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            prev: None,
        }
    }

    pub fn parse(&mut self) -> Result<Ast,String> {
        let mut ast = Ast::new();
        while let Some(token) = self.lexer.next()? {
            //println!("Token: {:#?}", token);
            match token {
                Tokens::Function => ast.add_decl(self.parse_fn()?),
                Tokens::Struct => ast.add_decl(self.parse_struct()?),
                _ => return Err(format!("Unknown token '{:?}'", token)),
            }
        }
        Ok(ast)
    }

    fn parse_fn(&mut self) -> Result<Declaration, String> {
        let ident = self.lexer.next()?.expect("Expected function identifier").ident()?;
        self.require_next(Tokens::LeftParenthesis)?;

        let mut args : Vec<String> = Vec::new();
        let mut right_parens_found = false;
        let mut expect_comma = false;
        while let Some(tok) = self.lexer.next()? {
            match tok {
                Tokens::RightParenthesis => {
                    right_parens_found = true;
                    break;
                },
                Tokens::Identifier(v) => {
                    args.push(v);
                    expect_comma = true;
                },
                Tokens::Comma => {
                    if !expect_comma {
                        return Err(format!("Unexpected comma in function parameter list for fn {}", ident));
                    }
                    expect_comma = false;
                },
                _ => return Err(format!("Unexpected token {:?} in function parameter list for fn {}", tok, ident)),
            }
        }
        if !right_parens_found {
            return Err(format!("Unterminated function parameter list in fn {}", ident));
        }
        self.require_next(Tokens::LeftBrace)?;
        self.require_next(Tokens::RightBrace)?;
        let func = Func::new(ident, args);
        Ok(Declaration::Fn(func))
    }

    fn parse_struct(&mut self) -> Result<Declaration, String> {
        Ok(Declaration::Struct(Struct{}))
    }

    fn parse_expr(&mut self) -> Result<Expression, String> {
        Ok(Expression::Constant(Constant::Unit))
    }

    fn require_next(&mut self, token: Tokens) -> Result<(), String> {
        if let Some(next) = self.lexer.next()? {
            Self::require(next.clone(), token)
        } else {
            Err(format!("Expected token {:?}, reached end of text", token))
        }
    }

    fn require(actual: Tokens, expected: Tokens) -> Result<(), String> {
        if matches!(actual.clone(), expected) {
            Ok(())
        } else {
            Err(format!("Expected token {:?}, but was {:?}", expected, actual))
        }
    }
}
