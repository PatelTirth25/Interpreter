use crate::{
    ast::Expr,
    report_error,
    token::{token_types::TokenType, Literal, Token},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Option<Expr> {
        self.equality()
    }
    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Some(expr)
    }
    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Some(expr)
    }
    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Some(expr)
    }
    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Some(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }
    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::FALSE]) {
            return Some(Expr::Literal(Literal::Boolean(false)));
        } else if self.match_token(&[TokenType::TRUE]) {
            return Some(Expr::Literal(Literal::Boolean(true)));
        } else if self.match_token(&[TokenType::NIL]) {
            return Some(Expr::Literal(Literal::Nil));
        } else if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            return Some(Expr::Literal(self.previous().literal));
        } else if self.match_token(&[TokenType::LEFTPAREN]) {
            let expr = self.parse()?;
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return Some(Expr::Grouping(Box::new(expr)));
        }
        report_error(
            &self.peek(),
            format!("Expect expression, got {}", &self.peek().lexeme).as_str(),
        );
        None
    }
    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<Token> {
        if self.check(&token_type) {
            Some(self.next())
        } else {
            Self::error(self.peek(), message);
            None
        }
    }
    fn error(token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            report_error(&token, "Expect end of expression.");
        } else {
            report_error(&token, message);
        }
    }
    fn match_token(&mut self, expected: &[TokenType]) -> bool {
        for token_type in expected {
            if self.check(token_type) {
                self.next();
                return true;
            }
        }
        false
    }
    fn next(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }
    fn check(&self, expected: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *expected
        }
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn synchronize(&mut self) {
        self.next();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {
                    self.next();
                }
            }
        }
    }
}
