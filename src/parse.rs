use log::debug;
use std::fmt;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use crate::data::Data;
use crate::position::CellPos;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

#[derive(Debug)]
pub enum LangError {
    ParseError,
    OpError,
    CellError,
    RefError,
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LangError::*;
        let val = match self {
            ParseError => "#PARSE_ERR",
            OpError => "#OP_ERR",
            CellError => "#CELL_ERR",
            RefError => "#REF_ERR",
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Div,
    Mul,
    Sub,
    Add,
}

pub fn parse(source: &str, data: &Data) -> Result<i32, LangError> {
    debug!("Source string: {}", source);
    let mut ident = LangParser::parse(Rule::ident, source).map_err(|_| LangError::ParseError)?;
    handle_expression(ident.next().unwrap().into_inner(), data)
}

pub fn handle_expression(mut pairs: Pairs<Rule>, data: &Data) -> Result<i32, LangError> {
    let op = pairs.next().unwrap();
    debug!("{}", &op);
    let op = match op.as_str() {
        "add" => Ok(Operation::Add),
        "sub" => Ok(Operation::Sub),
        "div" => Ok(Operation::Div),
        "mul" => Ok(Operation::Mul),
        n => {
            debug!("Invalid Op: {}", n);
            Err(LangError::OpError)
        }
    }?;

    let mut vals = Vec::new();
    for (idx, term) in pairs.enumerate() {

        let val = match term.as_rule() {
            Rule::cell => {
                let coord = convert_pair_to_cell_coord(term);
                let val = data.get(coord).ok_or(LangError::RefError)?;
                val.parse::<i32>().map_err(|_| LangError::RefError)?
            }
            Rule::int => term.as_str().parse::<i32>().map_err(|_| LangError::CellError)?,
            Rule::expr => handle_expression(term.into_inner(), data)?,
            n => {
                debug!("Unexpected Rule: {:?}", n);
                return Err(LangError::ParseError);
            }
            
        };
        vals.push(val);
    }

    if (op == Operation::Sub || op == Operation::Div) && vals.len() > 2 {
        debug!("Too many values");
        return Err(LangError::OpError);
    }

    let out = match op {
        Operation::Add => vals.iter().sum(),
        Operation::Mul => vals.iter().product(),
        Operation::Sub => vals[0] - vals[1],
        Operation::Div => vals[0] / vals[1],
    };

    Ok(out)
}

pub fn convert_pair_to_cell_coord(pair: Pair<Rule>) -> CellPos {
    let mut pair = pair.into_inner();
    let alpha = pair.next().unwrap().as_str();
    let number = pair.next().unwrap().as_str();
    let coord_0 = convert_alpha_str_to_num(alpha);
    let coord_1: usize = number.parse().unwrap();
    let coord_1 = coord_1;
    return CellPos::new(coord_0, coord_1);
}

pub fn convert_alpha_str_to_num(alpha: &str) -> usize {
    let mut acc = 0;
    for (i, c) in alpha.chars().enumerate() {
        let digit = c.to_digit(26).unwrap() as usize;
        let digit = digit - 10;
        acc += digit + (i * 26);
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "=add 2 (sub (mul 2 2) 3)";
        let val = parse(input, &Data::new()).unwrap();
        assert_eq!(val, 3);
    }

    #[test]
    fn test_convert() {
        let input = "AB";
        let out = convert_alpha_str_to_num(&input);
        assert_eq!(out, 27);
    }
}
