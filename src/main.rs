use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use std::fmt;

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexame: String,
    literal: String,
    line: u32,
}

#[derive(Debug)]
enum TokenType {
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Number => write!(f, "Number"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::For => write!(f, "For"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

impl Token {
    fn to_string(&mut self) -> String {
        match self.token_type {
            TokenType::Number => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Plus => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Minus => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Eof => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::LeftParen => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::RightParen => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::LeftBrace => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::RightBrace => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Comma => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Dot => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Semicolon => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Star => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Bang => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::BangEqual => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Slash => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Equal => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::EqualEqual => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Greater => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::GreaterEqual => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Less => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::LessEqual => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Identifier => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::And => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Class => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Else => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::False => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Fun => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::For => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::If => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Nil => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Or => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Print => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Return => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Super => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::This => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::True => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::Var => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
            TokenType::String => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
             TokenType::While => {
                format!("{} {} {} {}", self.token_type, self.lexame, self.literal, self.line)
            }
       }
    }
}

struct Scanner {
    source: String,
    had_error: bool,
    start: u32,
    current: u32,
    line: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            had_error: false,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) -> &mut Vec<Token>{
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexame: String::new(),
            literal: String::new(),
            line: self.line,
        });
        &mut self.tokens
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    while !(self.match_char('*') && self.match_char('/')) && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            'o' => {
                if self.match_char('r') {
                    self.add_token(TokenType::Or);
                } else {
                    self.add_token(TokenType::Identifier);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    self.error_token(format!("Unexpected character {}", c));
                }
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if (self.source.chars().nth(self.current as usize).unwrap()) == expected {
            self.current += 1;
            return true;
        }else {
            return false;
        }
    }

    fn peek(&mut self) -> char {
        if self.is_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }
        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        self.add_token(TokenType::Identifier);
    }

    fn add_token(&mut self, token_type: TokenType){
        self.tokens.push(
            Token {
                token_type,
                lexame: self.source[self.start as usize..self.current as usize].to_string(),
                literal: String::new(),
                line: self.line,
            }
        )
    }

    fn add_string_token(&mut self, token_type: TokenType, literal: String){
        self.tokens.push(
            Token {
                token_type,
                lexame: self.source[self.start as usize..self.current as usize].to_string(),
                literal,
                line: self.line,
            }
        )
    }

    fn string(&mut self){
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_end() {
            self.error_token("Unterminated string.".to_string());
        } else {
            self.advance();
            let value = self.source[self.start as usize + 1..self.current as usize - 1].to_string();
            self.add_string_token(TokenType::String, value);
        }
    }

    fn error_token(&mut self, message: String) {
        println!("Error on line {}: {}", self.line, message);
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }else if args.len() == 2 {
        let _result = run_file(&args[1]);
    }else{
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut scanner = Scanner::new(contents.to_string());
    run_program(&mut scanner);
    scanner.had_error = false; 
    Ok(())
}

fn run_prompt(){
    loop {
        let mut stdout = io::stdout();
        match stdout.write(b">> ") {
            Ok(_) => {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = String::from(input.trim());
                if input == "exit" {
                    break;
                }

                let mut scanner = Scanner::new(input.to_string());
                run_program(&mut scanner);
            },
            Err(e) => {
                println!("Error writing to stdout: {}", e);
                std::process::exit(1);
            }
        };
    }
}

fn run_program(scanner: &mut Scanner) {
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token.to_string());
    }
}

