fn main() {
    println!("Hello, world!");
}

pub struct Lexer {
    tokens: Vec<String>,
}

impl Lexer {
    pub fn peek(&self) -> Option<&String> {
        self.tokens.last()
    }
}
