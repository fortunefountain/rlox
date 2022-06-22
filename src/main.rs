use std::io;
use std::io::Read;
use std::fs::File;
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
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run_program(&contents);
    Ok(())
}

fn run_program(_program: &String) {
    println!("{}", _program);
}

fn run_prompt(){

}
