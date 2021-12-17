use std::io::Read;
use std::process::exit;

use crate::error::{error, Error};
use crate::tokenizer::*;

pub struct Code(Vec<&'static TokenType>);

impl Code {
    pub fn new(tokens: Vec<&'static TokenType>) -> Code {
        Code(tokens)
    }
    pub fn eval(self) {
        let mut pos = 0usize;
        let mut pointer = 0usize;
        let mut stack = [0; u16::MAX as usize];

        while pos < self.0.len() {
            match self.0[pos] {
                TokenType::Ging => pointer += 1,
                TokenType::Gang => {
                    if pointer == 0 {
                        error(Error::PointerOutOfBounds, pos);
                        exit(1);
                    }
                    pointer -= 1;
                }
                TokenType::Kkong => stack[pointer] += 1,
                TokenType::Gi => stack[pointer] -= 1,
                TokenType::Gong => match char::from_u32(stack[pointer] as u32) {
                    Some(c) => print!("{}", c),
                    None => {
                        error(Error::NumberParseError, pos);
                        exit(1);
                    }
                },
                TokenType::Ppu => {
                    stack[pointer] = std::io::stdin().bytes().next().unwrap().unwrap() as usize
                }
                TokenType::Wa => {
                    if stack[pointer] == 0 {
                        let mut level = 0;
                        let mut p = pos;

                        loop {
                            p += 1;
                            if let TokenType::Wa = self.0[p] {
                                level += 1;
                            } else if let TokenType::Ang = self.0[p] {
                                if level == 0 {
                                    pos = p;
                                    break;
                                } else {
                                    level -= 1;
                                }
                            }
                        }
                    }
                }
                TokenType::Ang => {
                    if stack[pointer] != 0 {
                        let mut level = 0;
                        let mut p = pos;

                        loop {
                            p -= 1;
                            if let TokenType::Ang = self.0[p] {
                                level += 1;
                            } else if let TokenType::Wa = self.0[p] {
                                if level == 0 {
                                    pos = p;
                                    break;
                                } else {
                                    level -= 1;
                                }
                            }
                        }
                    }
                }
            }
            pos += 1;
        }
    }
}
