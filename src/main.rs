use std::env;

use crate::error::RloxError;
fn main() -> Result<(), RloxError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    app::execute(args)
}

mod app {
    use std::fs;

    use crate::error::RloxError;

    pub fn execute(args: Vec<String>) -> Result<(), RloxError> {
        match args.len() {
            l if l > 1 => {
                println!("Usage: rlox [script]");
                std::process::exit(64);
            }
            1 => run_file(&args[0]),
            _ => run_repl(),
        }
    }

    fn run_file(file_path: &str) -> Result<(), RloxError> {
        let data = fs::read_to_string(&file_path)?;
        run(&data)
    }

    fn run(data: &str) -> Result<(), RloxError> {
        data.chars().for_each(|c| print!("{}", c));
        Ok(())
    }

    fn run_repl() -> Result<(), RloxError> {
        let stdin = std::io::stdin();
        loop {
            print!("> ");
            let mut buffer = String::new();
            stdin.read_line(&mut buffer)?;
            if buffer == "exit" {
                break Ok(());
            }
            let _result = run(&buffer);
        }
    }
}

mod error {
    use std::fmt::{self, Display, Formatter};

    #[derive(Debug)]
    pub enum RloxError {
        IoError(std::io::Error),
        SyntaxError(RloxSyntaxError),
    }

    #[derive(Debug)]
    pub struct RloxSyntaxError {
        pub line_number: u32,
        pub location: String,
        pub description: String,
    }

    impl Display for RloxSyntaxError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "rlox syntax error: \nline_number: {}\n, location: {}\n, description: {}\n",
                self.line_number, self.location, self.description
            )
        }
    }

    impl From<std::io::Error> for RloxError {
        fn from(e: std::io::Error) -> Self {
            Self::IoError(e)
        }
    }

    impl From<RloxSyntaxError> for RloxError {
        fn from(e: RloxSyntaxError) -> Self {
            Self::SyntaxError(e)
        }
    }

    impl Display for RloxError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            use RloxError::*;
            match self {
                IoError(e) => write!(f, "error reading script: {}", e),
                SyntaxError(e) => write!(f, "Syntax error: {}", e),
            }
        }
    }
}

mod scanner {
    use std::convert::TryInto;

    use crate::{
        error::RloxSyntaxError,
        scanner::token::{Token, TokenType},
    };
    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: u32,
        current: u32,
        line: u32,
    }

    impl Scanner {
        fn scan_tokens(&mut self) -> &Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }
            self.tokens.push(Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line_number: self.line,
            });
            &self.tokens
        }

        fn scan_token(&mut self) -> Result<(), RloxSyntaxError> {
            let c: char = self.advance();
            let maybe_token_type = match c {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                ';' => Some(TokenType::Semicolon),
                '*' => Some(TokenType::Star),
                _ => None,
            };
            match maybe_token_type {
                Some(t) => Ok(self.add_token(t)),
                None => Err(RloxSyntaxError {
                    // TODO: instead of erroring here, build a list of these and keep scanning
                    line_number: self.line,
                    location: "".to_string(),
                    description: "Unexpected character.".to_string(),
                }),
            }
        }

        fn advance(&mut self) -> char {
            self.current += 1;
            self.source
                .chars()
                .nth(self.current.try_into().unwrap()) // TODO access via slice index instead?
                .unwrap()
        }

        fn add_token(&mut self, token_type: TokenType) {
            let start: usize = self.start.try_into().unwrap();
            let current: usize = self.start.try_into().unwrap();
            let text = &self.source[start..current];
            self.tokens.push(Token {
                token_type: token_type,
                lexeme: text.to_string(),
                line_number: self.line,
            })
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.chars().count().try_into().unwrap()
        }
    }

    mod token {
        use std::fmt::{self, Display};

        #[derive(Debug)]
        pub struct Token {
            pub token_type: TokenType,
            pub lexeme: String,
            //literal: there is no Object type in Rust <--- TODO: handle this
            pub line_number: u32,
        }

        impl Display for Token {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?} {}", self.token_type, self.lexeme)
            }
        }

        impl Token {
            fn new(c: char, char_index: u32, line_number: u32) -> Self {
                // TODO
                let start: usize = self.start.try_into().unwrap();
                let current: usize = self.start.try_into().unwrap();
                let text = &self.source[start..current].to_string();
            
                let Some(token_type) = match c {
                    '(' => Some(TokenType::LeftParen),
                    ')' => Some(TokenType::RightParen),
                    '{' => Some(TokenType::LeftBrace),
                    '}' => Some(TokenType::RightBrace),
                    ',' => Some(TokenType::Comma),
                    '.' => Some(TokenType::Dot),
                    '-' => Some(TokenType::Minus),
                    '+' => Some(TokenType::Plus),
                    ';' => Some(TokenType::Semicolon),
                    '*' => Some(TokenType::Star),
                    _ => None,
                };

                Token { 
                    token_type: token_type,
                    lexeme: lexeme,
                    line_number: line_number
                }
            }
        }

        #[derive(Debug, Clone)]
        pub enum TokenType {
            // Single-character tokens.
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            Comma,
            Dot,
            Minus,
            Plus,
            Semicolon,
            Slash,
            Star,

            // One or two character tokens.
            Bang,
            BangEqual,
            Equal,
            EqualEqual,
            Greater,
            GreaterEqual,
            Less,
            LessEqual,

            // Literals.
            Identifier,
            String,
            Number,

            // Keywords.
            And,
            Class,
            Else,
            False,
            Fun,
            For,
            If,
            Nil,
            Or,
            Print,
            Return,
            Super,
            This,
            True,
            Var,
            While,

            Eof,
        }
    }
}
