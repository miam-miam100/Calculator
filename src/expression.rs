use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
pub use pest::Parser;

use crate::my_math::ten_to_the_power_of;
use crate::number::{add, div, exp, mul, sub};
use crate::types::{Fraction, MathError, Token};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Expression;

lazy_static! {
    pub static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

pub fn eval(expression: Pairs<Rule>) -> Result<Token, MathError> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| {
            match pair.as_rule() {
                Rule::expr => eval(pair.into_inner()),
                Rule::func => fn_eval(pair.into_inner()),
                Rule::int => {
                    let entire_int = pair.as_str();
                    let mut pairs = pair.into_inner();
                    let (integer, exponent) = (pairs.next().unwrap().as_str(), pairs.next());
                    match integer.parse::<i128>() {
                        Ok(integer) => match exponent {
                            Some(exponent) => match exponent.as_str().parse::<i128>() {
                                Ok(0) => Ok(Token::Integer(integer)),
                                Ok(exponent) if exponent > 0 => {
                                    match ten_to_the_power_of(exponent) {
                                        None => match_string_to_float(entire_int),
                                        Some(val) => match integer.checked_mul(val) {
                                            Some(int) => Ok(Token::Integer(int)),
                                            None => match_string_to_float(entire_int),
                                        },
                                    }
                                }
                                Ok(exponent) if exponent < 0 => {
                                    match ten_to_the_power_of(-exponent) {
                                        None => match_string_to_float(entire_int),
                                        Some(val) => {
                                            let mut frac = Fraction::new(0, integer, val);
                                            match frac.normalise() {
                                                Ok(_) => Ok(Token::Fraction(frac)),
                                                Err(MathError::InvalidFraction) => {
                                                    Ok(Token::Integer(frac.int))
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                }
                                Err(_) => Err(MathError::DoubleOverflow),
                                _ => unreachable!(),
                            },
                            None => Ok(Token::Integer(integer)),
                        },
                        Err(_) => match_string_to_float(entire_int),
                    }
                }
                Rule::dec => {
                    let entire_dec = pair.as_str();
                    let mut pairs = pair.into_inner();
                    let integer: &str;
                    let decimal: &str;
                    let pair = pairs.next().unwrap();
                    match pair.as_rule() {
                        Rule::basic_int => {
                            integer = pair.as_str();
                            decimal = pairs.next().unwrap().as_str();
                        }
                        Rule::basic_dec => {
                            // Integer may be empty as a decimal can be written like this: .5
                            decimal = pair.as_str();
                            integer = "0"
                        }
                        _ => unreachable!(),
                    }
                    let exponent = pairs.next();
                    match integer.parse::<i128>() {
                        Ok(integer) => match decimal.parse::<i128>() {
                            Ok(decimal_int) => match ten_to_the_power_of(decimal.len() as i128) {
                                None => match_string_to_float(entire_dec),
                                Some(result) => {
                                    let mut fraction = Fraction {
                                        int: integer,
                                        num: match integer {
                                            0..=i128::MAX => decimal_int,
                                            i128::MIN..=-1 => mul!(
                                                decimal_int,
                                                -1,
                                                match_string_to_float(entire_dec)
                                            ),
                                        },
                                        den: result,
                                    };
                                    if let Some(exponent) = exponent {
                                        match exponent.as_str().parse::<i128>() {
                                            Ok(exp) => {
                                                if exp > 0 {
                                                    match ten_to_the_power_of(exp) {
                                                        Some(val) => {
                                                            fraction.int = mul!(
                                                                fraction.int,
                                                                val,
                                                                match_string_to_float(entire_dec)
                                                            );
                                                            fraction.num = mul!(
                                                                fraction.num,
                                                                val,
                                                                match_string_to_float(entire_dec)
                                                            );
                                                        }
                                                        None => {
                                                            return match_string_to_float(
                                                                entire_dec,
                                                            );
                                                        }
                                                    }
                                                } else if exp < 0 {
                                                    match ten_to_the_power_of(-exp) {
                                                        Some(val) => {
                                                            fraction.num = add!(
                                                                fraction.num,
                                                                mul!(
                                                                    fraction.int,
                                                                    fraction.den,
                                                                    match_string_to_float(
                                                                        entire_dec
                                                                    )
                                                                ),
                                                                match_string_to_float(entire_dec)
                                                            );
                                                            fraction.den = mul!(fraction.den, val);
                                                            fraction.int = 0;
                                                        }
                                                        None => {
                                                            return match_string_to_float(
                                                                entire_dec,
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                return Err(MathError::DoubleOverflow);
                                            }
                                        }
                                    }
                                    match fraction.normalise() {
                                        Err(MathError::InvalidFraction) => {
                                            Ok(Token::Integer(fraction.int))
                                        }
                                        Err(MathError::Overflow) => {
                                            match_string_to_float(entire_dec)
                                        }
                                        _ => Ok(Token::Fraction(fraction)),
                                    }
                                }
                            },
                            Err(_) => match_string_to_float(entire_dec),
                        },
                        Err(_) => match_string_to_float(entire_dec),
                    }
                }
                _ => unreachable!(),
            }
        },
        |lhs: Result<Token, MathError>, op: Pair<Rule>, rhs: Result<Token, MathError>| match op
            .as_rule()
        {
            Rule::add => add(lhs?, rhs?),
            Rule::subtract => sub(lhs?, rhs?),
            Rule::multiply => mul(lhs?, rhs?),
            Rule::divide => div(lhs?, rhs?),
            Rule::power => exp(lhs?, rhs?),
            _ => unreachable!(),
        },
    )
}

fn fn_eval(mut function: Pairs<Rule>) -> Result<Token, MathError> {
    match function.next().unwrap().as_rule() {
        Rule::sqrt => exp(
            eval(function.next().unwrap().into_inner())?,
            Token::fraction(0, 1, 2),
        ),
        Rule::cbrt => exp(
            eval(function.next().unwrap().into_inner())?,
            Token::fraction(0, 1, 3),
        ),

        Rule::square => exp(
            eval(function.next().unwrap().into_inner())?,
            Token::Integer(2),
        ),
        Rule::cube => exp(
            eval(function.next().unwrap().into_inner())?,
            Token::Integer(3),
        ),
        _ => unreachable!(),
    }
}

#[inline]
fn match_string_to_float(string: &str) -> Result<Token, MathError> {
    match string.parse::<f64>() {
        Ok(i) if i != f64::INFINITY && i != f64::NEG_INFINITY => Ok(Token::Double(i)),
        _ => Err(MathError::DoubleOverflow),
    }
}
