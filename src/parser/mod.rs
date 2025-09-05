use crate::{
    ast::{Expr, Stmt},
    error::NZErrors,
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
    pub fn parse(&mut self) -> Result<Vec<Stmt>, NZErrors> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, NZErrors> {
        if self.match_token(&[TokenType::VAR]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::FUN]) {
            return self.function("function");
        }
        self.statement()
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, NZErrors> {
        let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LEFTPAREN,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if parameters.len() >= 255 {
                    return Err(NZErrors::ParseError(
                        self.peek(),
                        "Cannot have more than 255 parameters.".to_string(),
                    ));
                }
                parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name.")?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHTPAREN, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LEFTBRACE,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, NZErrors> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;
        let mut initializer = None;
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = Some(self.equality()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, NZErrors> {
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::LEFTBRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        if self.match_token(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::RETURN]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, NZErrors> {
        let keyword = self.previous();
        let value = if self.check(&TokenType::SEMICOLON) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn for_statement(&mut self) -> Result<Stmt, NZErrors> {
        self.consume(TokenType::LEFTPAREN, "Expect '(' after 'for'.")?;
        let initializer = if self.match_token(&[TokenType::SEMICOLON]) {
            None
        } else if self.match_token(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition = if self.check(&TokenType::SEMICOLON) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment = if self.check(&TokenType::RIGHTPAREN) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::RIGHTPAREN, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            }
        }
        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::Boolean(true)));
        }
        body = Stmt::While {
            condition: condition.unwrap(),
            body: Box::new(body),
        };
        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            }
        }
        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, NZErrors> {
        self.consume(TokenType::LEFTPAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, NZErrors> {
        self.consume(TokenType::LEFTPAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "Expect ')' after if condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_token(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, NZErrors> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RIGHTBRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, NZErrors> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, NZErrors> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, NZErrors> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, NZErrors> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }
            return Err(NZErrors::ParseError(
                equals,
                "Invalid assignment target.".to_string(),
            ));
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, NZErrors> {
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
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, NZErrors> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        return self.call();
    }

    fn call(&mut self) -> Result<Expr, NZErrors> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(&[TokenType::LEFTPAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, NZErrors> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err(NZErrors::ParseError(
                        self.peek(),
                        "Can't have more than 255 arguments.".to_string(),
                    ));
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RIGHTPAREN, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, NZErrors> {
        if self.match_token(&[TokenType::FALSE]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        } else if self.match_token(&[TokenType::TRUE]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        } else if self.match_token(&[TokenType::NIL]) {
            return Ok(Expr::Literal(Literal::Nil));
        } else if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(self.previous().literal));
        } else if self.match_token(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(self.previous()));
        } else if self.match_token(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(NZErrors::ParseError(
            self.peek(),
            format!("Expect expression, got {}", &self.peek().lexeme),
        ))
    }
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, NZErrors> {
        if self.check(&token_type) {
            Ok(self.next())
        } else {
            Err(NZErrors::ParseError(self.peek(), message.to_string()))
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
