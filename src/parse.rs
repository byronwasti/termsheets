use log::debug;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

#[derive(Debug)]
pub enum ParsingError {
    Bad,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Div,
    Mul,
    Sub,
    Add,
}

pub fn parse(source: &str) -> Result<((usize, usize), (usize, usize), Operation), ParsingError> {
    debug!("Source string: {}", source);
    let mut ident = LangParser::parse(Rule::ident, source).map_err(|_| ParsingError::Bad)?;
    let p = ident.next().unwrap();
    let c1 = convert_pair_to_cell_coord(p);

    let p = ident.next().unwrap();
    let op = convert_pair_to_operation(p);

    let p = ident.next().unwrap();
    let c2 = convert_pair_to_cell_coord(p);

    Ok((c1, c2, op))
}

pub fn convert_pair_to_cell_coord(pair: Pair<Rule>) -> (usize, usize) {
    let mut pair = pair.into_inner();
    let alpha = pair.next().unwrap().as_str();
    let number = pair.next().unwrap().as_str();
    let coord_0 = convert_alpha_str_to_num(alpha);
    let coord_1: usize = number.parse().unwrap();
    let coord_1 = coord_1;
    return (coord_0, coord_1);
}

pub fn convert_pair_to_operation(pair: Pair<Rule>) -> Operation {
    let mut pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::div => Operation::Div,
        Rule::mul => Operation::Mul,
        Rule::add => Operation::Add,
        Rule::sub => Operation::Sub,
        _ => unreachable!(),
    }
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
        let parsed = parse("=A2 - B3");
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.0, (0, 2));
        assert_eq!(parsed.1, (1, 3));
        assert_eq!(parsed.2, Operation::Sub);
    }

    #[test]
    fn test_convert() {
        let input = "AB";
        let out = convert_alpha_str_to_num(&input);
        assert_eq!(out, 27);
    }
}
