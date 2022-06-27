use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;
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
    let mut scanner = lox::Scanner::new(contents.to_string());
    run_program(&mut scanner);
    scanner.had_error(false); 
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

                let mut scanner = lox::Scanner::new(input.to_string());
                run_program(&mut scanner);
            },
            Err(e) => {
                println!("Error writing to stdout: {}", e);
                std::process::exit(1);
            }
        };
    }
}

fn run_program(scanner: &mut lox::Scanner) {
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token.to_string());
    }
}

