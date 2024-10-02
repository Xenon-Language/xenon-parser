use lexer::LexerIter;
use node::Program;
use parser::Parser;

mod lexer;
mod parser;
pub mod node;

pub fn parse(code: &str) -> Program {
    let tokens: LexerIter = lexer::lex_tokens(code);  // Lex tokens from the source code

    let tokvec: Vec<_> = tokens.collect(); // Collect tokens into a Vec
    let mut parser = Parser::new(&tokvec); // Pass the slice of tokens to the parser
    
    let program = parser.parse();  // Parse the tokens (assuming `parse` is implemented)
    return program
}

#[cfg(test)]
mod tests {
    use node::{Expression, Statement, Term};

    use super::*; // Import everything from the parent module
    use crate::node::{Definition, FunctionDefinition}; // Adjust imports as necessary

    #[test]
    fn test_program_parsing() {
        let code = r#"
            use example.library;

            namespace Program {
                fn main() {
                    float i = 0 + 123 * 48 / 18 % 3;
                    i = 0 * 12 - 3 / 12;
                    core.io.writeLn("Hello World");
                }
            }
        "#;

        let program = parse(code); // Use the parse function

        println!("{:#?}", program);
    }
}
