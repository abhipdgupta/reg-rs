#[derive(Debug)]
#[allow(dead_code)]
enum Token {
    Literal(char),
    EscapedChar(char),  // eg. \d, \w, \s, \S, \W char is the escaped character
    Quantifier(char),   // eg. *, +, ? char is the quantifier character
    SquareBracketOpen,  // [
    SquareBracketClose, // ]
    CurlyBracketOpen,   // {
    CurlyBracketClose,  // }
    RoundBracketOpen,   // (
    RoundBracketClose,  // )
    Colon,              // :
    Comma,              // ,
    Dash,               // -
    Pipe,               // |
    NewLine,            // \n
    Tab,                // \t
    CarriageReturn,     // \r
    Equal,              // =
    EOF,                // end of file
    UNSUPPORTED(char),
}

struct Regx {
    input: String,
}

impl Regx {
    fn new(input: &str) -> Regx {
        Regx {
            input: input.to_string(),
        }
    }

    fn tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // lexer for regular expression string is too easy to implement
        let e = self.input.len();
        let mut i = 0;
        while i < e {
            let c = self.input.chars().nth(i).unwrap();

            let tok = match c {
                '[' => Token::SquareBracketOpen,
                ']' => Token::SquareBracketClose,
                '(' => Token::RoundBracketOpen,
                ')' => Token::RoundBracketClose,
                '{' => Token::CurlyBracketOpen,
                '}' => Token::CurlyBracketClose,
                ':' => Token::Colon,
                ',' => Token::Comma,
                '|' => Token::Pipe,
                '=' => Token::Equal,
                '\n' => Token::NewLine,
                '\t' => Token::Tab,
                '\r' => Token::CarriageReturn,
                '\\' => {
                    let n = self.input.chars().nth(i + 1).unwrap();
                    match n {
                        'd' | 'D' | 'w' | 'W' | 's' | 'S' => {
                            i = i + 1;
                            Token::EscapedChar(n)
                        }
                        _ => Token::Literal(n),
                    }
                }
                '-' => Token::Dash,
                '*' | '+' | '?' => Token::Quantifier(c),
                _ => Token::Literal(c),
            };
            i = i + 1;
            tokens.push(tok);
        }

        tokens.push(Token::EOF);

        tokens
    }
}

fn main() {
    /*
       REG-RS is a simple regex engine written in Rust.
       for now only try to add suppport for the following regex patterns:
       - Literal characters (e.g., "abc")
       - Character classes (e.g., "[a-z]", "[0-9]"."[^a-z]", "\d", "\w", "\s", "\S", "\W")
       - Quantifiers (e.g., "*", "+", "?")

       And will use state machine to implement the regex engine.
       which is standard way to implement regex engine.

       check this link: https://regexr.com/
    */

    let pattern = "a[b-d]1?2c+\\\\WW\\d/?";

    let reg = Regx::new(pattern);

    let tokens = reg.tokens();

    println!("REG-RS is a simple regex engine written in Rust.");
    println!("List of tokens are {:?}", tokens)
}
