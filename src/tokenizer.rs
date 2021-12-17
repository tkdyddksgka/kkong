use std::process::exit;

use crate::error::*;

#[doc = r#"
Kkong    꽁

Gi       기

Ging     깅

Gang     강

Gong     공 

Ppu      뿌

Wa       와

Ang      앙
"#]
#[derive(Debug)]
pub enum TokenType {
    Kkong,
    Gi,
    Ging,
    Gang,
    Gong,
    Ppu,
    Wa,
    Ang,
}

pub fn parse<T>(code: T) -> Vec<&'static TokenType>
where
    T: AsRef<str>,
{
    let code = code.as_ref();
    let mut tokens = Vec::new();
    code.replace("\n", "")
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .for_each(|(index, value)| match value {
            '꽁' => tokens.push(&TokenType::Kkong),
            '기' => tokens.push(&TokenType::Gi),
            '깅' => tokens.push(&TokenType::Ging),
            '강' => tokens.push(&TokenType::Gang),
            '공' => tokens.push(&TokenType::Gong),
            '뿌' => tokens.push(&TokenType::Ppu),
            '와' => tokens.push(&TokenType::Wa),
            '앙' => tokens.push(&TokenType::Ang),
            _ => {
                error(Error::CommandNotFound(*value), index);
                exit(1)
            }
        });
    tokens
}
