use logos::Logos;
use std::fs;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[regex(r"@[a-zA-Z_]\w*")]
    Directive,
}

fn main() {
    let content = fs::read_to_string("../fixtures/test.blade.php")
        .expect("Could not read the file test.blade.php");

    let mut lex = Token::lexer(&content);

    match lex.next() {
        Some(Ok(token)) => {
            println!("Token: {:?}", token);
            println!("Content: {}", lex.slice()); 
            println!("Position: {:?}", lex.span());
        }
        Some(Err(_)) => println!("Unknown token!"),
        None => println!("End"),
    }

    match lex.next() {
        Some(Ok(token)) => {
            println!("Token: {:?}", token);
            println!("Content: {}", lex.slice()); 
            println!("Position: {:?}", lex.span());
        }
        Some(Err(_)) => println!("Unknown token!"),
        None => println!("End"),
    }

    match lex.next() {
        Some(Ok(token)) => {
            println!("Token: {:?}", token);
            println!("Content: {}", lex.slice()); 
            println!("Position: {:?}", lex.span());
        }
        Some(Err(_)) => println!("Unknown token!"),
        None => println!("End"),
    }

    match lex.next() {
        Some(Ok(token)) => {
            println!("Token: {:?}", token);
            println!("Content: {}", lex.slice()); 
            println!("Position: {:?}", lex.span());
        }
        Some(Err(_)) => println!("Unknown token!"),
        None => println!("End"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_directive_parsing() {
        let content = fs::read_to_string("../fixtures/test.blade.php")
            .expect("Could not read the file test.blade.php");

        let mut lex = Token::lexer(&content);

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@csrf");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@include");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@yield");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@guest");

        assert_eq!(lex.next(), None);
    }
}
