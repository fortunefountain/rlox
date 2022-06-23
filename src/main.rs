use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;
#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        tokens
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
    run_program(contents);
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
                run_program(input);
            },
            Err(e) => {
                println!("Error writing to stdout: {}", e);
                std::process::exit(1);
            }
        };
    }
}

fn run_program(program: String) {
    let mut scanner = Scanner::new(program.to_string());
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}

