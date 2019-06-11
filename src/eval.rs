use crate::config::parsers::{sp, value, snake_case};
use crate::types::Value;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::multi::{many0, many1};
use nom::IResult;
use nom::sequence::delimited;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
enum Operator{
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    In,
}
fn prio(t: &Operator) -> u8 {
    match t {
        Operator::Eq => 1,
        Operator::Ne => 1,
        Operator::Gt => 1,
        Operator::Lt => 1,
        Operator::Gte => 1,
        Operator::Lte => 1,
        _ => 0,
    }
}
impl PartialEq for Operator {
    fn eq(&self, other: &Operator) -> bool{
        prio(self) == prio(&other)
    }
}
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<Ordering>{
        Some(prio(self).cmp(&prio(&other)))
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),
    Variable(Vec<String>),
    Operator(Operator),
    RPar,
    LPar,
}
fn to_postfix(tokens: &Vec<Token>)-> Vec<Token>{
    use Token::*;
    let mut output:Vec<Token> = Vec::new();
    let mut stack:Vec<Token> = Vec::new();

    for token in tokens.iter(){
        match token {
            Value(v) => output.push(Value(v.clone())),
            Variable(v) => output.push(Variable(v.clone())),
            Operator(o) => {
                while let Some(t) = stack.last() {
                    match t {
                        LPar => break,
                        Operator(t) if t >= o => {
                            output.push(stack.pop().unwrap())
                        },
                        _ => break
                    }
                };
                stack.push(Operator(o.clone()));
            },
            LPar => stack.push(LPar),
            RPar => {
                let mut left_paren_found = false;
                while let Some(t) = stack.last() {
                    match t {
                        LPar => {
                            stack.pop();
                            left_paren_found = true;
                            break;
                        },
                        _ => {
                            output.push(stack.pop().unwrap());
                        }
                    };

                }
                if left_paren_found == false {
                    panic!("left parentheses not found");
                }
            },
        }
    }
    while let Some(t) = stack.pop(){
        match t {
            RPar | LPar => panic!("parentheses mismatched"),
            _ => {},
        }
        output.push(t);
    }
    output

}

fn operator(input: &str) -> IResult<&str, Operator>{
    preceded(sp, alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::Ne),
        map(tag(">"), |_| Operator::Gt),
        map(tag("<"), |_| Operator::Lt),
        map(tag(">="), |_| Operator::Gte),
        map(tag("<="), |_| Operator::Lte),
        map(tag("AND"), |_| Operator::And),
        map(tag("OR"), |_| Operator::Or),
     )))(input)
}
fn varialbe_key(input: &str) -> IResult<&str, String> {
    delimited(
        tag("["),
        snake_case,
        tag("]"),
    )(input)
}
fn variable(input: &str) -> IResult<&str, Vec<String>> {
    preceded(sp, many1(varialbe_key))(input)
}
fn token(input: &str) -> IResult<&str, Token> {
    preceded(sp, alt((
        map(value, Token::Value),
        map(operator, Token::Operator),
        map(tag("("), |_| Token::LPar),
        map(tag(")"), |_| Token::RPar),
        map(variable, Token::Variable),
    )))(input)
}
fn tokenize(input: &str) -> IResult<&str, Vec<Token>>{
    preceded(sp, many0(token))(input)
}


#[test]
fn test_expr () {
    let s = "1.0 == [var] AND 2 == 6";
    let mut expected = Vec::new();
    expected.push(Token::Value(Value::Number(1.0)));
    expected.push(Token::Operator(Operator::Eq));
    expected.push(Token::Variable(vec![String::from("var")]));
    expected.push(Token::Operator(Operator::And));
    expected.push(Token::Value(Value::Number(2.0)));
    expected.push(Token::Operator(Operator::Eq));
    expected.push(Token::Value(Value::Number(6.0)));
    assert_eq!(tokenize(s).unwrap().1, expected);
}

#[test]
fn test_rpn () {
    use Operator::*;
    let s = "(1.0 == 3.0 AND 2 == 6) OR 4 == 7";
    let tokens = tokenize(s).unwrap().1;
    let rpn = to_postfix(&tokens);
    let mut expected = Vec::new();
    expected.push(Token::Value(Value::Number(1.0)));
    expected.push(Token::Value(Value::Number(3.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Value(Value::Number(2.0)));
    expected.push(Token::Value(Value::Number(6.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Operator(And));
    expected.push(Token::Value(Value::Number(4.0)));
    expected.push(Token::Value(Value::Number(7.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Operator(Or));
    assert_eq!(rpn, expected);
}
#[test]
fn test_rpn_1 () {
    use Operator::*;
    let s = "1.0 == 3.0 AND (2 == 6 OR 4 == 7)";
    let tokens = tokenize(s).unwrap().1;
    let rpn = to_postfix(&tokens);
    let mut expected = Vec::new();
    expected.push(Token::Value(Value::Number(1.0)));
    expected.push(Token::Value(Value::Number(3.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Value(Value::Number(2.0)));
    expected.push(Token::Value(Value::Number(6.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Value(Value::Number(4.0)));
    expected.push(Token::Value(Value::Number(7.0)));
    expected.push(Token::Operator(Eq));
    expected.push(Token::Operator(Or));
    expected.push(Token::Operator(And));
    assert_eq!(rpn, expected);
}
#[test]
#[should_panic]
fn test_rpn_should_panic_right_paren () {
    let s = "1.0 == 3.0) AND (2 == 6 OR 4 == 7)";
    let tokens = tokenize(s).unwrap().1;
    let rpn = to_postfix(&tokens);

}
#[test]
#[should_panic]
fn test_rpn_should_panic_left_paren () {
    let s = "(1.0 == 3.0 AND (2 == 6 OR 4 == 7)";
    let tokens = tokenize(s).unwrap().1;
    let rpn = to_postfix(&tokens);

}

#[test]
fn test_op_orderign() {
    assert_eq!(Operator::And <= Operator::Eq, true);
    assert_eq!(Operator::Gt > Operator::Or, true);

}














