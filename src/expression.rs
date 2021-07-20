use itertools::Itertools;

use crate::my_math::ten_to_the_power_of;
use crate::types::{Fraction, MathError, Token};

pub struct Expression<'a> {
    string: &'a str,
    pub infix_token: Vec<Token>,
    pub postfix_token: Vec<Token>,
    result: Token,
}

impl<'a> Expression<'a> {
    pub fn new(string: &str) -> Expression {
        Expression {
            string,
            infix_token: vec![],
            postfix_token: vec![],
            result: Token::None,
        }
    }
    #[allow(clippy::manual_range_contains)]
    pub fn tokenise(&mut self) -> MathError {
        let string_char_len = self.string.chars().count();
        let mut decimal_point_index = None;
        let mut checking_number = false;
        let mut new_string: String = String::new();
        for (idx, (elem, next)) in self.string.chars().chain([' ']).tuple_windows().enumerate() {
            if !checking_number && '0' <= elem && elem <= '9' {
                checking_number = true;
                decimal_point_index = None;
                new_string = String::new();
            } // Not using else if as checking number can change inside previous if statement.
            if checking_number {
                if elem == '.' {
                    new_string.push('.');
                    match decimal_point_index {
                        None => {
                            decimal_point_index = Some(new_string.len() - 1);
                        }
                        Some(_) => {
                            return MathError::InvalidDecimalPoint;
                        }
                    }
                } else if '0' <= elem && elem <= '9' {
                    new_string.push(elem);
                }
                if !(('0' <= next && next <= '9') || next == ' ' || next == '.')
                    || idx == string_char_len - 1
                {
                    checking_number = false;
                    match decimal_point_index {
                        None => match new_string.parse::<i128>() {
                            Ok(result) => {
                                self.infix_token.push(Token::Integer(result));
                            }
                            Err(_) => match match_string_to_float(&new_string) {
                                Some(result) => {
                                    self.infix_token.push(Token::Double(result));
                                }
                                None => {
                                    return MathError::DoubleOverflow;
                                }
                            },
                        },
                        Some(index) => {
                            let integer: i128;
                            let decimal: i128;
                            match (&new_string[0..index]).parse::<i128>() {
                                Ok(result) => {
                                    integer = result;
                                }
                                Err(_) => match match_string_to_float(&new_string) {
                                    Some(x) => {
                                        self.infix_token.push(Token::Double(x));
                                        continue;
                                    }
                                    None => {
                                        return MathError::DoubleOverflow;
                                    }
                                },
                            }

                            match (&new_string[index + 1..]).parse::<i128>() {
                                Ok(result) => {
                                    decimal = result;
                                }
                                Err(_) => match match_string_to_float(&new_string) {
                                    Some(x) => {
                                        self.infix_token.push(Token::Double(x));
                                        continue;
                                    }
                                    None => {
                                        return MathError::DoubleOverflow;
                                    }
                                },
                            }
                            if decimal == 0 {
                                self.infix_token.push(Token::Integer(integer));
                            } else {
                                match ten_to_the_power_of((new_string.len() - index - 1) as i128) {
                                    None => match match_string_to_float(&new_string) {
                                        Some(x) => {
                                            self.infix_token.push(Token::Double(x));
                                            continue;
                                        }
                                        None => {
                                            return MathError::DoubleOverflow;
                                        }
                                    },
                                    Some(result) => {
                                        let mut fraction = Fraction {
                                            int: integer,
                                            num: decimal,
                                            den: result,
                                        };
                                        self.infix_token.push(match fraction.normalise() {
                                            Err(MathError::InvalidFraction) => {
                                                Token::Integer(fraction.int)
                                            }
                                            Err(MathError::Overflow) => {
                                                match match_string_to_float(&new_string) {
                                                    Some(x) => Token::Double(x),
                                                    None => {
                                                        return MathError::DoubleOverflow;
                                                    }
                                                }
                                            }
                                            _ => Token::Fraction(fraction),
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            } else if elem != ' ' {
                self.infix_token.push(match elem {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Multiply,
                    '/' => Token::Divide,
                    '(' => Token::LBracket,
                    ')' => Token::RBracket,
                    _ => {
                        return MathError::UnknownOperator;
                    }
                });
            }
        }
        MathError::None
    }
    pub fn postfix(&mut self) -> MathError {
        self.postfix_token = vec![];
        let mut operator_stack = vec![];
        for token in self.infix_token.iter().copied() {
            match token {
                Token::Integer(_)
                | Token::Fraction(_)
                | Token::Power(_, _, _)
                | Token::Double(_) => {
                    self.postfix_token.push(token);
                }
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    while let Some(operator) = operator_stack.pop() {
                        if operator != Token::LBracket
                            && precedence!(operator) >= precedence!(token)
                        {
                            //If implementing powers precedence cannot be equal as it is right associative and not left.
                            self.postfix_token.push(operator);
                        } else {
                            operator_stack.push(operator);
                            break;
                        }
                    }
                    operator_stack.push(token);
                }
                Token::LBracket => {
                    operator_stack.push(token);
                }
                Token::RBracket => {
                    let mut l_bracket_reached = false;
                    while let Some(operator) = operator_stack.pop() {
                        if operator == Token::LBracket {
                            l_bracket_reached = true;
                            break;
                        }
                        self.postfix_token.push(operator);
                    }
                    if !l_bracket_reached {
                        return MathError::UnmatchedBracket;
                    }
                }
                Token::None => {
                    return MathError::UnknownOperator;
                }
            }
        }
        while let Some(operator) = operator_stack.pop() {
            if let Token::LBracket | Token::RBracket = operator {
                return MathError::UnmatchedBracket;
            }
            self.postfix_token.push(operator);
        }
        MathError::None
    }
}

fn match_string_to_float(string: &str) -> Option<f64> {
    match string.parse::<f64>() {
        Ok(i) => {
            if i == f64::INFINITY || i == f64::NEG_INFINITY {
                return None;
            }
            Some(i)
        }
        Err(_) => None,
    }
}
