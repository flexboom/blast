use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"@[a-zA-Z_]\w*")]
    Directive,
}

fn main() {
    let mut lex = Token::lexer("@csrf @include @yield");

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
        let mut lex = Token::lexer("@csrf @include @yield");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@csrf");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@include");

        assert_eq!(lex.next(), Some(Ok(Token::Directive)));
        assert_eq!(lex.slice(), "@yield");

        assert_eq!(lex.next(), None);
    }
}
