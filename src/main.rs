use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct TokenData {
    token_type: String,
    lexame: String,
    literal: String,
    line: u32,
    column: u32,
}

#[derive(Debug)]
enum Token {
    Number(TokenData, i32),
    Plus(TokenData),
    Minus(TokenData),
    EOF(TokenData)
}

impl Token {
    fn to_string(&mut self) -> String {
        match self {
            Token::Number(data, value) => {
                format!("{} {} {} {} {} {}", data.token_type, data.lexame, data.literal, data.line, data.column, value)
            },
            Token::Plus(data) => {
                format!("{} {} {} {} {}", data.token_type, data.lexame, data.literal, data.line, data.column)
            },
            Token::Minus(data) => {
                format!("{} {} {} {} {}", data.token_type, data.lexame, data.literal, data.line, data.column)
            }
            Token::EOF(data) => {
                format!("{} {} {} {} {}", data.token_type, data.lexame, data.literal, data.line, data.column)
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
        tokens.push(Token::EOF(
            TokenData {
                token_type: "EOF".to_string(),
                lexame: "".to_string(),
                literal: "".to_string(),
                line: self.line,
                column: self.current as u32 - self.start as u32,
            }
                ));
        tokens
    }

    fn scan_token(&mut self) -> Token {
        self.current += 1;
        Token::Number(TokenData {
            token_type: "number".to_string(),
            lexame: "123".to_string(),
            literal: "123".to_string(),
            line: 1,
            column: 1,
        }, 123)
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn error_hander(&mut self, line_no: i32, message: String) {
        println!("Error on line {}: {}", line_no, message);
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

