use crate::{statemachine::{Token, Machine}, common::Errors};

// you can either get one token at a time from the Lexer
// or you can get a list of all tokens
pub trait Lexer {
    fn get_new_token(&mut self) -> Result<(Token , bool) , Errors>;
    fn lex(&mut self) -> Result<Vec<Token> , Errors>;
}

// text shows the source text
// pos shows the current character in the source text
// machine is an implementation of the Machine trait
pub struct ImagineLexer<T: Machine> {
    text: String,
    pos: usize,
    machine: T,
}

impl<T: Machine> ImagineLexer<T> {
    pub fn new(text: String , machine: T) -> ImagineLexer<T> {
        ImagineLexer {text , pos: 0 , machine }
    }
}

impl<T: Machine> Lexer for ImagineLexer<T> {

    fn get_new_token(&mut self) -> Result<(Token , bool) , Errors> {

        while self.pos < self.text.len() {

            let input = self.text.chars()
            .nth(self.pos)
            .unwrap();

            self.pos += 1;

            let token = self.machine.feed(input)?;

            // feed the input to the machine until a token is returned
            if let Some(token) = token {
                return Ok((token , false));
            }


        }

        Ok((self.machine.get_final_token() , true))

    }

    fn lex(&mut self) -> Result<Vec<Token> , Errors> {
        
        let (mut token , mut is_last_token) = self.get_new_token()?;
        let mut tokens = vec![];

        while !is_last_token  {

            tokens.push(token);
            (token , is_last_token) = self.get_new_token()?;

        }

        tokens.push(token);

        Ok(tokens)

    }

}

#[cfg(test)]
mod tests {

    use crate::{
    lexer::*,
    statemachine::{Token , ImagineMachine}
    };

    // this test both covers the get_next_token the lex method
    #[test]
    fn lex_test() {

        let machine = ImagineMachine::new();
        let mut lexer = ImagineLexer::new(String::from("72 3.14 player if
        player2 36 100 -7 + 8 var += 12 !bool !(2 + 2) block{code}") , machine);

        assert_eq!(lexer.lex().unwrap() , vec![
            Token::Number(72),
            Token::Float(3.14),
            Token::Ident("player".to_string()),
            Token::Keyword("if".to_string()),
            Token::Ident("player2".to_string()),
            Token::Number(36),
            Token::Number(100),
            Token::Operator("-".to_string()),
            Token::Number(7),
            Token::Operator(String::from("+")),
            Token::Number(8),
            Token::Ident("var".to_string()),
            Token::Operator("+=".to_string()),
            Token::Number(12),
            Token::Operator("!".to_string()),
            Token::Ident("bool".to_string()),
            Token::Operator("!".to_string()),
            Token::LPAR,
            Token::Number(2),
            Token::Operator("+".to_string()),
            Token::Number(2),
            Token::RPAR,
            Token::Ident("block".to_string()),
            Token::LBR,
            Token::Ident("code".to_string()),
            Token::RBR,
        ]);

    }

}