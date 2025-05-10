fn main() {
    println!("Hello, world!");
}

type Token = String;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }
}
