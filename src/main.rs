use std::io;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use lox::Visitor;
fn main() {
    let _expression = lox::BinaryExpr {
        left: Box::new(lox::NumberLiteralExpr{value: 1.0}),
        operator: lox::Token{
            lexeme: "+".to_string(),
            literal: None ,
            line: 1,
            token_type: lox::TokenType::Plus,
        },
        right: Box::new(lox::NumberLiteralExpr{value: 2.0}),
    };

    let mut visitor = lox::AstPrinter{
        indent: 0,
    };
    visitor.visit_binary_expr(&_expression);

    //let args: Vec<String> = std::env::args().collect();
    //if args.len() != 2 {
    //    println!("Usage: {} <output directory>", args[0]);
    //    std::process::exit(1);
    //}
    //let output_dir = &args[1];
    //let mut types = Vec::new();
    //types.push(String::from("Binary: Expr left, Token operator, Expr right"));
    //types.push(String::from("Grouping Expr expression"));
    //types.push(String::from("Literal: Object value"));
    //types.push(String::from("Unary: Token operator, Expr right"));
    // 
    //lox::define_ast(String::from(output_dir), String::from("Expr"), types);


    //Scanner test
    //let args: Vec<String> = std::env::args().collect();
    //if args.len() > 2 {
    //    println!("Usage: {} <filename>", args[0]);
    //    std::process::exit(1);
    //}else if args.len() == 2 {
    //    let _result = run_file(&args[1]);
    //}else{
    //    run_prompt();
    //}
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

