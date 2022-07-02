use std::fmt;
use std::fs::File;
use std::io::Write;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn Expr>>,
    line: u32,
}

#[derive(Debug)]
pub enum TokenType {
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
    pub fn to_string(&mut self) -> String {
        match self.token_type {
             _ => {
                format!("{} {} {} {}", self.token_type, self.lexeme, self.literal.unwrap(), self.line)
            }
       }
    }
}

fn keyword(lexeme: &str) -> TokenType {
    match lexeme {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}


pub struct Scanner {
    source: String,
    had_error: bool,
    start: u32,
    current: u32,
    line: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            had_error: false,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn had_error(&mut self, flg: bool){
        self.had_error = flg;
    }

    pub fn scan_tokens(&mut self) -> &mut Vec<Token>{
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
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
        let lexeme = self.source[self.start as usize..self.current as usize].to_string();
        let token_type = keyword(lexeme.as_str());
        self.add_token(token_type);
    }

    fn add_token(&mut self, token_type: TokenType){
        let _token_type = token_type.clone();
        self.tokens.push(
            Token {
                token_type,
                lexeme: self.source[self.start as usize..self.current as usize].to_string(),
                literal: match _token_type {
                    TokenType::Number => Some(Box::new(NumberLiteral))
                },
                line: self.line,
            }
        )
    }

    fn add_string_token(&mut self, token_type: TokenType, literal: String){
        self.tokens.push(
            Token {
                token_type,
                lexeme: self.source[self.start as usize..self.current as usize].to_string(),
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


pub trait Expr{
    fn accept(&self, visitor: &mut dyn Visitor);
}



pub trait Stmt{
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct Program{
    pub statements: Vec<Box<dyn Stmt>>,
}

impl Expr for Program{
    fn accept(&self, visitor: &mut dyn Visitor){
        visitor.visit_program(self);
    }
}

pub struct Block{
    pub statements: Vec<Box<dyn Stmt>>,
}

impl Expr for Block{
    fn accept(&self, visitor: &mut dyn Visitor){
        visitor.visit_block(self);
    }
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<dyn Expr>,
}

pub struct StringLiteral {
    pub value: String,
}

pub struct NumberLiteral {
    pub value: f64,
}

pub trait Visitor{
    fn visit_program(&mut self, expr: &Program);
    fn visit_stmt(&mut self, expr: &dyn Stmt);
    fn visit_block(&mut self, expr: &Block);
    fn visit_unary_expr(&mut self, expr: &UnaryExpr);
    fn visit_binary_expr(&mut self, expr: &BinaryExpr);
    fn visit_string_literal(&mut self, expr: &StringLiteral);
    fn visit_number_literal(&mut self, expr: &NumberLiteral);
}

pub struct AstPrinter{
    pub indent: usize,
}

impl Visitor for AstPrinter {
    fn visit_program(&mut self, expr: &Program) {
        println!("Program");
        self.indent += 1;
        for stmt in &expr.statements {
            stmt.accept(self);
        }
        self.indent -= 1;
    }

    fn visit_stmt(&mut self, expr: &dyn Stmt) {
        println!("{}", " ".repeat(self.indent));
        expr.accept(self);
    }

    fn visit_block(&mut self, expr: &Block) {
        println!("GroupingExpr");
        self.indent += 1;
        for stmt in &expr.statements {
            stmt.accept(self);
        }
        self.indent -= 1;
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) {
        println!("UnaryExpr");
        self.indent += 1;
        expr.right.accept(self);
        self.indent -= 1;
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) {
        println!("BinaryExpr");
        self.indent += 1;
        expr.left.accept(self);
        expr.right.accept(self);
        self.indent -= 1;
    }

    fn visit_string_literal(&mut self, expr: &StringLiteral) {
        println!("StringLiteral");
        self.indent += 1;
        println!("{}", expr.value);
        self.indent -= 1;
    }

    fn visit_number_literal(&mut self, expr: &NumberLiteral) {
        println!("NumberLiteral");
        self.indent += 1;
        println!("{}", expr.value);
        self.indent -= 1;
    }
}


pub fn define_ast(_output_dir: String, _base_name: String, _types: Vec<String>){
    let _path = _output_dir + "/" + _base_name.as_str() + ".rs";
    let mut file = File::create(_path).unwrap();
    file.write(b"pub struct AST;").unwrap();
}

pub struct Writer {
    output: String,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            output: String::new(),
        }
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn write_line(&mut self, s: &str) {
        self.write(s);
        self.write("\n");
    }

    fn write_indent(&mut self, indent: usize) {
        for _ in 0..indent {
            self.write("    ");
        }
    }
}

pub fn define_types(_writer : &mut Writer, _name: String, _types: Vec<String>) {
    _writer.write_line((String::from("pub enum ") + _name.as_str() + " {").as_str());
    for _type in _types {
        _writer.write_line((String::from("    ") + _type.as_str() + ",").as_str());
    }
    _writer.write_line("}");
}


pub fn define_visitor(_writer: &mut Writer, _base_name: String, _types: Vec<String>){
   for _type in _types {
       _writer.write_line(format!("pub trait {}Visitor<'a> {{
           fn visit_{}(&mut self, expr: &'a Expr);
       }}", _type, _type).as_str());
   }
   println!("{}",_writer.output);
}
