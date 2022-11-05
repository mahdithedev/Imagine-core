use crate::common::Errors;

#[derive(PartialEq , Debug)]
pub enum Token {
    Number(i32),
    Operator(String),
    Float(f32),
    Ident(String),
    Keyword(String),
    LPAR,
    RPAR,
    LBR,
    RBR,
    // used outside of the Machine
    Text(String),
    Blank,
}

fn is_operator(input: &char) -> bool {
    ['+', '-' , '=' , '/' , '*' , '!' , '<' , '>' , '~' , '|' , '&' , '^'].contains(input)
}

// every transition returns the new state and optinaly a token
pub type Update = (Box<dyn State> , Option<Token>);

const KEYWORD_LIST: &'static [&str] = &["if"];

// represents the state of the current token being processed
// get_type and get_value methods are used for debugging and testing
pub trait State {

    // returns the new state to transition and optinaly a token if everything went ok
    fn feed(&self , input: char) -> Result<Update , Errors>;
    fn get_type(&self) -> String;
    fn get_token(&self) -> Token;

    // only used for testing
    fn get_value(&self) -> String {
        return String::from("");
    }

}

// input handling and transition and token emitting logic is implemented
// un structs that implement the State trait

// used for all the single/double character operators
// + , - , * , / , == and etc.

struct Lpar;
struct Rpar;
struct Lbr;
struct Rbr;

impl State for Lpar {

    fn feed(&self , input: char) -> Result<Update , Errors> {

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }
        
        let new_state = (Blank {}).feed(input).unwrap().0;
        let token = self.get_token();
        return Ok((new_state , Some(token)));

    }

    fn get_token(&self) -> Token {
        Token::LPAR
    }

    fn get_type(&self) -> String {
        String::from("Lpar")
    }

    fn get_value(&self) -> String {
        String::from("(")
    }

}

impl State for Rpar {

    fn feed(&self , input: char) -> Result<Update , Errors> {

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }
        
        let new_state = (Blank {}).feed(input).unwrap().0;
        let token = self.get_token();
        return Ok((new_state , Some(token)));

    }

    fn get_token(&self) -> Token {
        Token::RPAR
    }

    fn get_type(&self) -> String {
        String::from("Rpar")
    }

    fn get_value(&self) -> String {
        String::from(")")
    }

}

impl State for Lbr {

    fn feed(&self , input: char) -> Result<Update , Errors> {

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }
        
        let new_state = (Blank {}).feed(input).unwrap().0;
        let token = self.get_token();
        return Ok((new_state , Some(token)));

    }

    fn get_token(&self) -> Token {
        Token::LBR
    }

    fn get_type(&self) -> String {
        String::from("Lbr")
    }

    fn get_value(&self) -> String {
        String::from("{")
    }

}

impl State for Rbr {

    fn feed(&self , input: char) -> Result<Update , Errors> {

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }
        
        let new_state = (Blank {}).feed(input).unwrap().0;
        let token = self.get_token();
        return Ok((new_state , Some(token)));

    }

    fn get_token(&self) -> Token {
        Token::RBR
    }

    fn get_type(&self) -> String {
        String::from("Rbr")
    }

    fn get_value(&self) -> String {
        String::from("}")
    }

}

struct Operator {
    value: String,
}

impl Operator {
    fn new(value: String) -> Operator {
        Operator { value }
    }       
}

impl State for Operator {

    fn feed(&self , input: char) -> Result<Update , Errors> {

        if is_operator(&input) {
            let new_operator = format!("{}{}" , self.value , input);
            let new_state = Box::new(Operator::new(new_operator));
            return Ok((new_state , None));
        }

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }
        
        let new_state = (Blank {}).feed(input).unwrap().0;
        let token = self.get_token();
        return Ok((new_state , Some(token)));

    }

    fn get_token(&self) -> Token {
        Token::Operator(self.value.clone())
    }

    fn get_type(&self) -> String {
        String::from("Operator")
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

}

// idents are varible names or keywords
struct Ident {
    value: String,
    is_keyword: bool,
}

impl Ident {
    fn new(value: String , is_keyword: bool) -> Ident {
        Ident { value , is_keyword }
    }
}

impl State for Ident {
    
    fn feed(&self , input: char) -> Result<Update , Errors> {

        // idents can state with only alphabetic characters (see the blank state)
        // and can contain _ or alphanumerical characters
        if input.is_alphanumeric() || input == '_' {

            let new_value = format!("{}{}" , self.value , input);

            if KEYWORD_LIST.contains(&new_value.as_str()) {
                let new_state = Box::new(Ident::new(new_value , true));
                return Ok((new_state , None));
            }

            let new_state = Box::new(Ident::new(new_value , false)); 
            return Ok((new_state , None));

        }

        if is_operator(&input) {

            let new_state = Box::new(Operator::new(String::from(input)));
            let token = self.get_token();
            return Ok((new_state , Some(token)));

        }

        if ['(' ,')' , '{' , '}'].contains(&input) {

            let new_state = (Blank {}).feed(input).unwrap().0;
            let token = self.get_token();
            return Ok((new_state , Some(token)))

        }
        
        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }

        Err(Errors::SyntaxError)

    }

    fn get_token(&self) -> Token {

        // while mabey not the best solution it works

        if self.is_keyword {
            return Token::Keyword(self.value.clone());
        }

        Token::Ident(self.value.clone())

    }

    fn get_type(&self) -> String {
        if self.is_keyword {String::from("Keyword")} else {String::from("Ident")}
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

}

// used for float numbers like 3.14
struct Float {
    value: String,
}

impl Float {
    fn new(value: String) -> Float {
        Float {value}
    }
}

impl State for Float {

    fn feed(&self , input: char) -> Result<Update , Errors> {
        
        if input.is_numeric() {
            let new_value = format!("{}{}" , self.value , input);
            let new_state = Box::new(Float::new(new_value)); 
            return Ok((new_state , None));
        }

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }

        Err(Errors::SyntaxError)

    }

    fn get_token(&self) -> Token {
        Token::Float(self.value.parse::<f32>().unwrap())
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn get_type(&self) -> String {
        String::from("Float")
    }

}

// used for i32 numbers
#[derive(PartialEq , Debug)]
struct Number {
    value: String,
}

impl Number {

    fn new(value: String) -> Number {
        Number {value}
    }

}

impl State for Number {

    fn feed(&self , input: char) -> Result<Update , Errors> {
        
        if input.is_numeric() {
            let new_value = format!("{}{}" , self.value , input);
            let new_state = Box::new(Number::new(new_value)); 
            return Ok((new_state , None));
        }

        if input == '.' {
            let new_value = format!("{}." , self.value);
            let new_state = Box::new(Float::new(new_value)); 
            return Ok((new_state , None));
        }

        if ['(' ,')' , '{' , '}'].contains(&input) {

            let new_state = (Blank {}).feed(input).unwrap().0;
            let token = self.get_token();
            return Ok((new_state , Some(token)))

        }
        
        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }

        if input.is_whitespace() || input == '\n'  {
            let new_state = Box::new(Blank {});
            let token = self.get_token();
            return Ok((new_state , Some(token)));
        }

        Err(Errors::SyntaxError)

    }

    fn get_token(&self) -> Token {
        Token::Number(self.value.parse::<i32>().unwrap())
    }

    fn get_type(&self) -> String {
        return String::from("Number");
    }

    // only used for testing
    fn get_value(&self) -> String {
        return self.value.clone();
    }

}

struct Blank;

// this state represnts the nothing state and can be used to transition into every other state
impl State for Blank {

    fn feed(&self , input: char) -> Result<Update , Errors> {
        
        if input.is_numeric() {
            let state = Box::new(Number::new(input.to_string())); 
            return Ok((state , None));
        }

        if input.is_alphabetic() || input == '_' {
            let state = Box::new(Ident::new(input.to_string() , false)); 
            return Ok((state , None));
        }

        if input.is_whitespace() || input == '\n'  {
            let state = Box::new(Blank {});
            return Ok((state , None));
        }

        if is_operator(&input) {
            let state = Box::new(Operator::new(String::from(input)));
            return Ok((state , None));
        }

        if input == '(' {
            let state = Box::new(Lpar);
            return Ok((state , None));
        }

        if input == ')' {
            let state = Box::new(Rpar);
            return Ok((state , None));
        }

        if input == '{' {
            let state = Box::new(Lbr);
            return Ok((state , None));
        }

        if input == '}' {
            let state = Box::new(Rbr);
            return Ok((state , None));
        }

        Err(Errors::SyntaxError)

    }

    fn get_token(&self) -> Token {
        Token::Blank
    }

    fn get_type(&self) -> String {
        return String::from("Blank");
    }

}

// this trait represnts the behaviour of a state machine and is used in the lexer
// it starts with a Blank state then feeds input (1 character) to the current state
// and then it transitions to the new state returned from the previous state
// and returns a token if the previous state emitted a token

// example:

// '1' -> |Blank| -> |Number|
// '2' -> |Number| -> |Number|
// ' ' -> |Number| -> |Blank| and a Token::number(12) 

// see https://en.wikipedia.org/wiki/Finite-state_machine for more detail

pub trait Machine {
    fn get_final_token(&self) -> Token;
    fn feed(&mut self , input: char) -> Result<Option<Token> , Errors>; 
}

// an implementation of the Machine trait
pub struct ImagineMachine {
    current_state: Box<dyn State>,
}

impl ImagineMachine {

    pub fn new() -> ImagineMachine {
        ImagineMachine {current_state: Box::new(Blank {})}
    }

}

impl Machine for ImagineMachine {

    fn get_final_token(&self) -> Token {
        self.current_state.get_token()
    }

    fn feed(&mut self , input: char) -> Result<Option<Token> , Errors> {

        let update = self.current_state.feed(input);

        match update {
            Ok((new_state , token)) => {
                self.current_state = new_state;
                return Ok(token);
            } 
            Err(err) => Err(err)
        }
        
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn blank_to_number() {

        let state = Blank {};
        let (new_state , _) = state.feed('1').unwrap();

        assert_eq!(new_state.get_type() , String::from("Number"));
        assert_eq!(new_state.get_value() , String::from("1"));

    }

    #[test]
    fn number_to_number() {

        let mut state: Box<dyn State> = Box::new(Number::new(String::from("1")));
        
        for i in 2..6 {

            let (new_state , _) = state.feed(char::from_digit(i, 10).unwrap()).unwrap();
            state = new_state;

        }

        assert_eq!(state.get_type() , String::from("Number"));
        assert_eq!(state.get_value() , String::from("12345"));

    }

    #[test]
    #[should_panic]
    fn number_syntax_error() {

        let state = Number::new(String::from("1"));
        state.feed('a').unwrap();

    }

    #[test]
    fn number_token() {

        let state = Number::new(String::from("12")); 
        let (_ , token) = state.feed(' ').unwrap();

        assert_eq!(token.unwrap() , Token::Number(12));

    }

    #[test]
    fn number_to_float() {

        let mut state: Box<dyn State> = Box::new(Number::new(String::from("3")));
        state = state.feed('.').unwrap().0;
        state = state.feed('1').unwrap().0;
        state = state.feed('4').unwrap().0;

        assert_eq!(state.get_type() , "Float");
        assert_eq!(state.get_value() , "3.14");

    }

    #[test]
    fn float_token() {

        let mut state: Box<dyn State> = Box::new(Number::new(String::from("3")));
        state = state.feed('.').unwrap().0;
        state = state.feed('1').unwrap().0;
        let (new_state , mut token) = state.feed('4').unwrap();
        state = new_state;

        assert!(token.is_none());
    
        (state , token) = state.feed(' ').unwrap();
        
        assert_eq!(state.get_type() , "Blank");
        assert_eq!(token.unwrap() , Token::Float(3.14));

    }

    #[test]
    fn ident_test() {

        let mut state: Box<dyn State> = Box::new(Ident::new(String::from("p") , false));
        state = state.feed('l').unwrap().0;
        state = state.feed('a').unwrap().0;
        state = state.feed('y').unwrap().0;

        let token = state.get_token();
        
        assert_eq!(state.get_type() , "Ident");
        assert_eq!(token , Token::Ident(String::from("play")));

    }

    #[test]
    fn keyword_test() {

        let mut state: Box<dyn State> = Box::new(Ident::new(String::from("i") , false));
        state = state.feed('f').unwrap().0;

        let token = state.get_token();

        assert_eq!(state.get_type() , "Keyword");
        assert_eq!(token , Token::Keyword(String::from("if")));

    }

    #[test]
    fn machine_test() {

        let mut machine = ImagineMachine::new();

        let mut token = machine.feed('7').unwrap();
        assert!(token.is_none());

        token = machine.feed('2').unwrap();
        assert!(token.is_none());      
        
        token = machine.feed(' ').unwrap();
        assert_eq!(token.unwrap() , Token::Number(72));

        token = machine.feed('3').unwrap();
        assert!(token.is_none());

        token = machine.feed('.').unwrap();
        assert!(token.is_none());

        token = machine.feed('1').unwrap();
        assert!(token.is_none());

        token = machine.feed('4').unwrap();
        assert!(token.is_none());

        token = machine.feed(' ').unwrap();
        assert_eq!(token.unwrap() , Token::Float(3.14));

        token = machine.feed('a').unwrap();
        assert!(token.is_none());

        token = machine.feed('b').unwrap();
        assert!(token.is_none());

        token = machine.feed('c').unwrap();
        assert!(token.is_none());

        token = machine.feed(' ').unwrap();
        assert_eq!(token.unwrap() , Token::Ident(String::from("abc")));

        // only test one keyword

        token = machine.feed('i').unwrap();
        assert!(token.is_none());

        token = machine.feed('f').unwrap();
        assert!(token.is_none());

        token = machine.feed(' ').unwrap();
        assert_eq!(token.unwrap() , Token::Keyword(String::from("if")));

        let token = machine.get_final_token();
        assert_eq!(token , Token::Blank);

    }

    #[test]
    fn operator_test() {

        for character in "+-*/!<>!~|&^=".chars() {
            let state = (Blank {}).feed(character).unwrap().0;
            assert_eq!(Token::Operator(String::from(character)) , state.get_token())
        }

    }

    #[test]
    fn multy_character_oprator() {

        for character in "+-*/!<>!~|&^=".chars() {
            for character2 in "+-*/!<>!~|&^".chars() {
                let state = (Blank {}).feed(character).unwrap().0;
                let state = state.feed(character2).unwrap().0;
                let operator_string = String::from(format!("{}{}" , character , character2));
                assert_eq!(Token::Operator(operator_string) , state.get_token());
            }
        }

    }

}