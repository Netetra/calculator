#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Bracket {
    Start,
    End,
}

#[derive(Debug, Clone)]
enum Token {
    Indent(f64),
    Operator(Operator),
    Bracket(Bracket),
}

struct Tokenizer {
    code: String,
    pos: usize,
}

impl Tokenizer {
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        loop {
            if self.code.len() <= self.pos {
                break;
            }
            let char = self.code.chars().nth(self.pos).unwrap();
            let token = match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    self.convert_num_token()
                }
                '+' => Token::Operator(Operator::Add),
                '-' => Token::Operator(Operator::Sub),
                '*' => Token::Operator(Operator::Mul),
                '/' => Token::Operator(Operator::Div),
                '(' => Token::Bracket(Bracket::Start),
                ')' => Token::Bracket(Bracket::End),
                _ => panic!("Unknown character."),
            };
            self.pos += 1;
            tokens.push(token);
        }
        return tokens;
    }
    fn convert_num_token(&mut self) -> Token {
        let mut num_chars = Vec::<char>::new();
        loop {
            let char = self.code.chars().nth(self.pos);
            if let Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.') = char {
                num_chars.push(char.unwrap());
                self.pos += 1;
            } else {
                self.pos -= 1;
                break;
            }
        }
        let num = String::from_iter(num_chars).parse().unwrap();
        return Token::Indent(num);
    }
}

struct Calculator {
    tokens: Vec<Token>,
    pos: usize,
}

impl Calculator {
    fn get_token(&self, diff: usize) -> Option<Token> {
        return self.tokens.get(self.pos + diff).map(|value| value.clone());
    }

    fn move_pos(&mut self, diff: usize) {
        self.pos += diff;
    }

    fn expr(&mut self) -> f64 {
        let mut value = self.term();
        loop {
            let token = self.get_token(1);
            match token {
                Some(Token::Operator(Operator::Add)) => {
                    self.move_pos(2);
                    value += self.term();
                }
                Some(Token::Operator(Operator::Sub)) => {
                    self.move_pos(2);
                    value -= self.term();
                }
                _ => break,
            }
        }
        return value;
    }

    fn term(&mut self) -> f64 {
        let mut value = self.factor();
        loop {
            let token = self.get_token(1);
            match token {
                Some(Token::Operator(Operator::Mul)) => {
                    self.move_pos(2);
                    value *= self.factor();
                }
                Some(Token::Operator(Operator::Div)) => {
                    self.move_pos(2);
                    value /= self.factor();
                }
                _ => break,
            }
        }
        return value;
    }

    fn factor(&mut self) -> f64 {
        let token = self.get_token(0);
        match token {
            Some(Token::Indent(num)) => {
                return num;
            }
            Some(Token::Bracket(Bracket::Start)) => {
                self.move_pos(1);
                let ans = self.expr();
                self.move_pos(1); // )
                return ans;
            }
            _ => panic!("Syntax Error"),
        }
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let input = line.trim().to_string().replace(" ", "");
    //let input = "100*(10-2/3)".replace(" ", "").to_string();
    let mut tokenizer = Tokenizer {
        code: input,
        pos: 0,
    };
    let tokens = tokenizer.tokenize();
    let mut calculator = Calculator { tokens, pos: 0 };
    println!("{}", calculator.expr());
}
