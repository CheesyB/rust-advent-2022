use std::vec;

use crate::helper;

use nom::{self, character::complete::newline, multi::separated_list1, IResult};

#[derive(Debug)]
pub struct Monkey<'a> {
    items: Vec<i32>,
    operation: (&'a str, i32), // (instruction, argument(alpha, same))
    test: i32,                 // devisible argument
    monkey_index_true: usize,
    monkey_index_false: usize,
}

mod parsing {
    use std::num::ParseIntError;
    use std::str::FromStr;

    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1, digit1, i32, space1};
    use nom::combinator::{map_res, recognize};
    use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};

    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        CONST(i32),
        OLD(Option<i32>),
        MULT,
        ADD,
    }

    impl FromStr for Token {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "OLD" => Ok(Token::OLD(None)),
                "*" => Ok(Token::MULT),
                "+" => Ok(Token::ADD),
                dig => Ok(Token::CONST(dig.parse::<i32>().unwrap())),
            }
        }
    }

    fn to_int(input: &str) -> Result<i32, ParseIntError> {
        i32::from_str_radix(input, 10)
    }

    fn heading<'a>(input: &'a str) -> IResult<&str, i32> {
        let result = delimited(tag("Monkey "), digit1, tag(":\n"))(input)?;
        Ok((result.0, i32::from_str_radix(result.1, 10).unwrap()))
    }

    fn items<'a>(input: &'a str) -> IResult<&'a str, Vec<i32>> {
        let result = delimited(
            tag("Starting items: "),
            separated_list1(tag(", "), digit1),
            tag("\n"),
        )(input)?;
        Ok((
            result.0,
            result.1.iter().map(|d| d.parse::<i32>().unwrap()).collect(),
        ))
    }

    fn divisor<'a>(input: &'a str) -> IResult<&'a str, Token> {
        map_res(
            delimited(tag("Test: divisible by "), i32, tag("\n")),
            |int| Ok::<Token, nom::Err<nom::error::Error<&'a str>>>(Token::CONST(int)),
        )(input)
    }

    fn operation<'a>(input: &'a str) -> IResult<&'a str, (Token, Token)> {
        map_res(
            delimited(
                tag("Operation: new = old "),
                tuple((alt((tag("*"), tag("+"))), space1, alt((alpha1, digit1)))),
                tag("\n"),
            ),
            |(arithmetic, _, constant)| {
                Ok::<(Token, Token), nom::Err<nom::error::Error<&'a str>>>((
                    Token::from_str(arithmetic).unwrap(),
                    Token::from_str(constant).unwrap(),
                ))
            },
        )(input)
    }

    fn predicate<'a>(input: &'a str) -> IResult<&'a str, i32> {

    fn parse_monkey<'a>(input: &'a str) -> IResult<&str, Monkey<'a>> {
        let monkey = Monkey {
            items: vec![],
            operation: ("-", 20),
            test: 19,
            monkey_index_false: 0,
            monkey_index_true: 1,
        };
        Ok(("rest", monkey))
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&str, Vec<Monkey<'a>>> {
        separated_list1(newline, parse_monkey)(input)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        static TEST_MONKEY: &str = "Monkey 1:
Starting items: 78, 74, 88, 89, 50
Operation: new = old * 11
Test: divisible by 5
    If true: throw to monkey 3
    If false: throw to monkey 5";

        #[test]
        fn parse_headline() {
            let (out1, monkey_number) = heading(TEST_MONKEY).unwrap();
            let (out2, items) = items(out1).unwrap();
            let (out4, operation) = operation(out2).unwrap();
            let (out5, div) = divisor(out4).unwrap();

            assert_eq!(monkey_number, 1);
            assert_eq!(items, vec![78, 74, 88, 89, 50]);
            assert_eq!(operation, (Token::MULT, Token::CONST(11)));
            assert_eq!(div, Token::CONST(5));
        }
    }
}

pub fn advent11() -> String {
    let content = helper::read_puzzle_input("./src/advent10/signal.txt");
    let monkies = parsing::parse(&content);
    println!("{:?}", monkies);

    "hallo".to_string()
}
