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
    column: u32,
}

#[derive(Debug)]
enum TokenType {
    Number,
    Plus,
    Minus,
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Star,
    Bang,
    BangEqual
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Number => write!(f, "Number"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Eof => write!(f, "Eof"),
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
        }
    }
}

impl Token {
    fn to_string(&mut self) -> String {
        match self.token_type {
            TokenType::Number => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Plus => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Minus => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Eof => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::LeftParen => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            }
            TokenType::RightParen => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::LeftBrace => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            }
            TokenType::RightBrace => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Comma => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Dot => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Semicolon => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Star => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            }
            TokenType::Bang => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            }
            TokenType::BangEqual => {
                format!("{} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
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
            column: self.current - self.start,
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
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            _ => self.error_handler(format!("Unexpected character {}", c)),
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

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn add_token(&mut self, token_type: TokenType){
        self.tokens.push(
            Token {
                token_type,
                lexame: self.source[self.start as usize..self.current as usize].to_string(),
                literal: String::new(),
                line: self.line,
                column: self.current - self.start,
            }
        )
    }

    fn error_handler(&mut self, message: String) {
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

