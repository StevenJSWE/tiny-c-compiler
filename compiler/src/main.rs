#[derive(Debug, PartialEq, Clone)]// This lets us print and compare our tokens later!
pub enum Token{
    // We will list our token types here
    Keyword(String),
    Identifier(String),
    Assign,
    Number(i32),
    Semicolon,
    Plus,

}

#[derive(Debug)]
pub enum Expr{
    Number(i32),
    //Addition has a left side and a right side
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Statement{
    // "Let" represents variable assignment: Target Name and the Expressioin being assigned
    Let(String, Expr),
}
pub struct Lexer{
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self{ 
        Lexer {input, position: 0}
    }

    // Helper to grab the character at our current position 
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    //Method to skip spaces 
    fn skip_whitespace(&mut self){
        while let Some(c) = self.current_char(){
            if c.is_whitespace(){
                self.position += 1; // Move forward 1 step 
            }else { 
                break; // Stop looping when we hit a real char 
            }
        }
    }

    fn read_number(&mut self) -> i32 {
        let mut number_string = String::new();

        while let Some(c) = self.current_char(){
            if c.is_ascii_digit(){ // if the current char is an ascii digit 
                number_string.push(c); // Add it to our string 
                self.position += 1; // Move on to the next char 
            } else {
                break; // Stop when we hit a space
            }
        }
            number_string.parse::<i32>().unwrap()
    }


    fn read_identifier(&mut self) -> String{ 
        let mut id_string = String::new();

        while let Some(c) = self.current_char(){

            if c.is_alphabetic() || c == '_'{
                id_string.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        id_string
    }

    pub fn next_token(&mut self) -> Option<Token> {
        //Always skip whitespaces first 
        self.skip_whitespace();

        // Grab current char 
        // If there are no more return None
        let c = match self.current_char(){
            Some(character) => character,
            None => return None,
        };

        let token = match c {
            '=' => {
                self.position += 1; // Manually advance past the symbol
                Token::Assign
            }
            ';' => {
                self.position += 1;
                Token::Semicolon
            }
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '0'..='9' => {
                // It's a digit! Let our helper do the work of advancing position
                Token::Number(self.read_number())
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // It's a letter! Grab the whole word
                let text = self.read_identifier();
                
                // Check against our master list of keywords
                match text.as_str() {
                    "int" => Token::Keyword(text),
                    _ => Token::Identifier(text), // If not "int", it's a custom variable name
                }
            }
            _ => {
                // If it's a character we don't recognize yet, just skip it and try again
                self.position += 1;
                return self.next_token();
            }
        };
        Some(token)
    }
}


pub struct Parser{
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser{
    //Start the parser by pulling the very first token 
    pub fn new(mut lexer: Lexer) -> Self{
        let first_token = lexer.next_token();
        Parser { lexer,
             current_token: first_token,
          }
    }

    // Helper to step forward in our token stream
    fn advance(&mut self){
        self.current_token = self.lexer.next_token();
    }

    // Parses: int x = <expression>;
    pub fn parse_statement(&mut self) -> Option<Statement> {
        // 1. Look for the 'int' keyword
        if let Some(Token::Keyword(kw)) = &self.current_token {
            if kw == "int" {
                self.advance(); // Consume 'int'

                // 2. Look for the variable name (e.g, 'x')
                if let Some(Token::Identifier(name)) = self.current_token.clone() {
                    self.advance(); // Consume 'x'
                
                    // 3. Look for the '=' symbol (Notice this is now inside the block above!)
                    if let Some(Token::Assign) = self.current_token {
                        self.advance(); // Consume '='

                        // 4. Hand off to our expression parser for the right side
                        let expr = self.parse_expression().unwrap();

                        // 5. Look for the ';'
                        if let Some(Token::Semicolon) = self.current_token {
                            self.advance(); // Consume ';'

                            // Now 'name' is still alive here to be used!
                            return Some(Statement::Let(name, expr));
                        }
                    }
                }
            }
        }
        None
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        // Grab the first number
        if let Some(Token::Number(val1)) = self.current_token{
            self.advance();

            // Check if a '+' comes next
            if let Some(Token::Plus) = self.current_token{
                self.advance();

                // Grab the second number
                if let Some(Token::Number(val2)) = self.current_token{
                    self.advance();

                    return Some(Expr::Add(
                        Box::new(Expr::Number(val1)),
                        Box::new(Expr::Number(val2))
                ));
                }
            } else {

                return Some(Expr::Number(val1));
            }
        }
        None
    }
}

pub struct CodeGenerator{
    pub assembly: String,
}

impl CodeGenerator{
    pub fn new() -> Self{
        CodeGenerator {
            assembly: String::new(),
         }
    }

    pub fn generate_expr(&mut self, expr: &Expr, target_reg: usize) {
        match expr { 
            Expr::Number(val) => {
                // 🛠️ The first {} is the register, the second {} is the value
                let instruction = format!("mov w{}, #{}\n", target_reg, val);
                self.assembly.push_str(&instruction);
            }
            Expr::Add(left, right) => {
                // 🛠️ Generate left side into the current register
                self.generate_expr(left, target_reg);
                
                // 🛠️ Generate right side into the NEXT register (+1)
                self.generate_expr(right, target_reg + 1);
                
                // 🛠️ Add them! format: add result_reg, left_reg, right_reg
                let instruction = format!("add w{}, w{}, w{}\n", target_reg, target_reg, target_reg + 1);
                self.assembly.push_str(&instruction);
            }
        }
    }


}
fn main() {

// The C code we want to compile 

let source_code = String::from("int x = 5 + 3;");

// Create a new instance of our lexer 
let lexer = Lexer::new(source_code);

// Pass the lexer int our new parser
let mut parser = Parser::new(lexer);

print!("Scanning code...");

// Keep publishing tokens until next_token() returns NONE

if let Some(ast) = parser.parse_statement() {
    // The {:?} syntax tells Rust to use the Debug format we derived on our enum 
    println!("{:?}", ast);

    // Create out code generator
    let mut generator = CodeGenerator::new();

    // Unpack the math expression from our 'Let' statement
    if let Statement::Let(_name, expr) = ast{

        // Generate the assembly, telling it to start at register 0
        generator.generate_expr(&expr, 0);

        println!("\nGenerated Assembly:");
        println!("{}", generator.assembly);
        
    } 
} else {
    println!("Syntax error: Failed to parse.")
}

println!("Done!");

}

