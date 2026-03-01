#[derive(Debug, PartialEq)]// This lets us print and compare our tokens later!
pub enum Token{
    // We will list our token types here
    Keyword(String),
    Identifier(String),
    Assign,
    Number(i32),
    Semicolon,
    Plus,

}

struct Lexer{
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self{ 
        Lexer { input, position: 0}
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
fn main() {

    // The C code we want to compile 

let source_code = String::from("int x = 5;");

//Create a new instance of our lexer 
let mut lexer = Lexer::new(source_code);

print!("Scanning code...");

// Keep publishing tokens until next_token() returns NONE

while let Some(token) = lexer.next_token(){
    //The {:?} syntax tells Rust to use the Debug format we derived on our enum 
    println!("{:?}", token);
}

println!("Done!");

}

