#[derive(Debug, PartialEq)]
// #[allow(dead_code)]
enum Token {
    Literal(char),
    EscapedChar(char),

    Star,
    Plus,
    QuestionMark,

    SquareBracketOpen,
    SquareBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    RoundBracketOpen,
    RoundBracketClose,

    Colon,
    Comma,
    Dash,
    Pipe,
    Caret,
    Equal,

    NewLine,
    Tab,
    CarriageReturn,

    EOF,
    UNSUPPORTED(char),
}

struct Regx {
    input: String,
}

impl Regx {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    fn tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(c) = chars.next() {
            let token = match c {
                '[' => Token::SquareBracketOpen,
                ']' => Token::SquareBracketClose,
                '(' => Token::RoundBracketOpen,
                ')' => Token::RoundBracketClose,
                '{' => Token::CurlyBracketOpen,
                '}' => Token::CurlyBracketClose,

                ':' => Token::Colon,
                ',' => Token::Comma,
                '-' => Token::Dash,
                '|' => Token::Pipe,
                '^' => Token::Caret,
                '=' => Token::Equal,

                '*' => Token::Star,
                '+' => Token::Plus,
                '?' => Token::QuestionMark,

                '\n' => Token::NewLine,
                '\t' => Token::Tab,
                '\r' => Token::CarriageReturn,

                '\\' => match chars.next() {
                    Some(c @ ('d' | 'D' | 'w' | 'W' | 's' | 'S')) => Token::EscapedChar(c),

                    Some('n') => Token::NewLine,
                    Some('t') => Token::Tab,
                    Some('r') => Token::CarriageReturn,

                    // Treat other escaped characters as literals.
                    Some(c) => Token::Literal(c),

                    // Invalid trailing backslash.
                    None => Token::UNSUPPORTED('\\'),
                },

                _ => Token::Literal(c),
            };

            tokens.push(token);
        }

        tokens.push(Token::EOF);
        tokens
    }
}

fn main() {
    let pattern = r"a[b-d]1?2c+\\WW\d/?";

    let reg = Regx::new(pattern);
    let tokens = reg.tokens();

    println!("REG-RS is a simple regex engine written in Rust.");
    println!("List of tokens: {:?}", tokens);
}
