// The token type
enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    StringToken(String),
    NumberToken(f64),
    True,
    False,
    Null,
}
// The JSON value type
enum JSONValue {
    Object(Vec<(String, JSONValue)>),
    Array(Vec<JSONValue>),
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
}
// Stage 1: tokenizer
fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.current() {
            None => break,
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                lexer.advance();
            }
            Some(b'{') => {
                tokens.push(Token::LeftBrace);
                lexer.advance();
            }
            Some(b'}') => {
                tokens.push(Token::RightBrace);
                lexer.advance();
            }
            Some(b'[') => {
                tokens.push(Token::LeftBracket);
                lexer.advance();
            }
            Some(b']') => {
                tokens.push(Token::RightBracket);
                lexer.advance();
            }
            Some(b':') => {
                tokens.push(Token::Colon);
                lexer.advance();
            }
            Some(b',') => {
                tokens.push(Token::Comma);
                lexer.advance();
            }
            Some(b'"') => {
                let s = lexer.read_string();
                tokens.push(Token::StringToken(s));
            }
            Some(b't') => {
                lexer.read_keyword("true");
                tokens.push(Token::True);
            }
            Some(b'f') => {
                lexer.read_keyword("false");
                tokens.push(Token::False);
            }
            Some(b'n') => {
                lexer.read_keyword("null");
                tokens.push(Token::Null);
            }
            Some(b'-') | Some(b'0'..=b'9') => {
                let n = lexer.read_number();
                tokens.push(Token::NumberToken(n));
            }
            Some(c) => {
                panic!("Unextected character: {}", c as char);
            }
        }
    }
    tokens
}
// Stage 2: parser
fn parse(tokens: &[Token]) -> JSONValue {
    todo!()
}
struct Lexer {
    input: Vec<u8>,
    pos: usize,
}
impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.as_bytes().to_vec(),
            pos: 0,
        }
    }
    fn current(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
    }
    fn peek(&self) -> Option<u8> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }
    fn read_keyword(&mut self, keyword: &str) {
        for expect in keyword.as_bytes() {
            match self.current() {
                Some(c) if c == *expect => self.advance(),
                Some(c) => panic!(
                    "Unexpected character '{}' while reading keyword '{}'",
                    c as char, keyword
                ),
                None => panic!(
                    "Unexpected end of input while reading keyword '{}'",
                    keyword
                ),
            }
        }
    }
    fn read_string(&mut self) -> String {
        //consume opening "
        self.advance();

        let mut result = String::new();

        loop {
            match self.current() {
                None => panic!("Unterminated string"),
                Some(b'"') => {
                    self.advance(); //consume closing "
                    return result;
                }
                Some(b'\\') => {
                    self.advance(); //consume the backslash
                    match self.current() {
                        Some(b'"') => {
                            result.push('"');
                            self.advance();
                        }
                        Some(b'\\') => {
                            result.push('\\');
                            self.advance();
                        }
                        Some(b'/') => {
                            result.push('/');
                            self.advance();
                        }
                        Some(b'n') => {
                            result.push('\n');
                            self.advance();
                        }
                        Some(b't') => {
                            result.push('\t');
                            self.advance();
                        }
                        Some(b'r') => {
                            result.push('\r');
                            self.advance();
                        }
                        Some(b'b') => {
                            result.push('\x08');
                            self.advance();
                        }
                        Some(b'f') => {
                            result.push('\x0C');
                            self.advance();
                        }
                        Some(b'u') => {
                            self.advance(); //consume u
                        }
                    }
                }
            }
        }
    }
    fn read_number(&mut self) -> f64 {
        todo!()
    }
}
fn main() {
    let input = r#"{"name": "alice", "age": 30, "active": true}"#;
    let tokens = tokenize(input);
    let value = parse(&tokens);
}
