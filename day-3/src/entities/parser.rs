use crate::entities::mul_expression::{Instruction, MulExpression};
use crate::entities::parser::ParsingError::InvalidFileSyntax;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Parser)]
#[grammar = "entities/syntax.pest"]
pub struct AOCParser();

pub fn parse_stdin(input: &str) -> Result<Vec<MulExpression>, ParsingError> {
    let parsed_file = match AOCParser::parse(Rule::file, input) {
        Ok(data) => data,
        Err(err) => return Err(InvalidFileSyntax(err.to_string())),
    }
    .next()
    .unwrap();

    let mut vec_mul_expressions: Vec<MulExpression> = Vec::new();

    for instruction in parsed_file.into_inner() {
        if instruction.as_rule() == Rule::mul_expression {
            let (left_number, right_number) = parse_mul_expression(instruction);
            vec_mul_expressions.push(MulExpression::new(
                left_number,
                right_number,
                Instruction::Do,
            ));
        } else {
            vec_mul_expressions.append(&mut parse_instruction(instruction))
        }
    }

    Ok(vec_mul_expressions)
}

fn parse_instruction(parse_instruction: Pair<Rule>) -> Vec<MulExpression> {
    let instruction = match parse_instruction.as_rule() {
        Rule::dont => Instruction::Dont,
        Rule::doit => Instruction::Do,
        _ => panic!("Invalid rule, expected dont or do"),
    };

    let parse_mul_instructions = parse_instruction.into_inner();

    let mut vec_mul_expressions = Vec::new();

    for mul_instruction in parse_mul_instructions {
        let (left_number, right_number) = parse_mul_expression(mul_instruction);
        vec_mul_expressions.push(MulExpression::new(left_number, right_number, instruction));
    }

    vec_mul_expressions
}

fn parse_mul_expression(mul_expression: Pair<Rule>) -> (usize, usize) {
    match mul_expression.as_rule() {
        Rule::mul_expression => {
            let mut mul_expression_inner = mul_expression.into_inner();

            let left_number = parse_number(&mut mul_expression_inner);
            let right_number = parse_number(&mut mul_expression_inner);

            (left_number, right_number)
        }

        _ => panic!("Invalid rule, expected mul_expression"),
    }
}

fn parse_number(mul_expression_inner: &mut Pairs<Rule>) -> usize {
    mul_expression_inner
        .next()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("could not parse number")
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidFileSyntax(String),
    // InvalidData(&'a str),
}
impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidFileSyntax(err) => write!(f, "Invalid file syntax: {}", err),
        }
    }
}

impl Error for ParsingError {}
