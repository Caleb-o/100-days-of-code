use std::collections::HashMap;
use super::tokens::TokenType;

// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub struct Lang
{
    pub keywords: HashMap<String, TokenType>,
}

impl Lang
{
    pub fn new() -> Lang
    {
        use TokenType::*;

        Lang
        {
            keywords: hashmap![
                "and".to_string() => And,
                "or".to_string() => Or,
                "class".to_string() => Class,
                "else".to_string() => Else, 
                "true".to_string() => True,
                "false".to_string() => False,
                "for".to_string() => For,
                "while".to_string() => While,
                "fn".to_string() => Func,
                "if".to_string() => If,
                "null".to_string() => Null,
                "print".to_string() => Print,
                "return".to_string() => Return,
                "parent".to_string() => Super,
                "this".to_string() => This,
                "let".to_string() => Var
            ],
        }
    }
}