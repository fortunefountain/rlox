use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;

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
}

impl Token {
    fn to_string(&mut self) -> String {
        match self.token_type {
            TokenType::Number => {
                format!("{} {} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Plus => {
                format!("{} {} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            },
            TokenType::Minus => {
                format!("{} {} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
            }
            TokenType::Eof => {
                format!("{} {} {} {} {} {}", self.token_type, self.lexame, self.literal, self.line, self.column)
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
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            had_error: false,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        tokens.push(Token {
            token_type: TokenType::Eof,
            lexame: String::new(),
            literal: String::new(),
            line: self.line,
            column: self.current - self.start,
        });
        tokens
    }

    fn scan_token(&mut self) -> Token {
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
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            _ => self.error_handler(format!("Unexpected character {}", c)),
        }
        

        Token{
            token_type: TokenType::Number,
            lexame: "123".to_string(),
            literal: "123".to_string(),
            line: 1,
            column: 1,
        }
    }

    fn advance(&self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn add_token(&self, token_type: TokenType){

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
    for mut token in tokens {
        println!("{:?}", token.to_string());
    }
}

