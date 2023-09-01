use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use crate::helper;

use self::parsing::ValueToken;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: RefCell<VecDeque<ValueToken>>,
    inspecting_count: RefCell<usize>,
    operation: (parsing::ArithmeticToken, parsing::ValueToken), // (instruction, argument(alpha, same))
    test: i64,                                                  // devisible argument
    bool_indices: (usize, usize),
}

pub struct Monkey2 {
    items: RefCell<VecDeque<HashMap<i64, ValueToken>>>,
    inspecting_count: RefCell<usize>,
    operation: (parsing::ArithmeticToken, parsing::ValueToken), // (instruction, argument(alpha, same))
    test: i64,                                                  // devisible argument
    bool_indices: (usize, usize),
}

mod parsing {
    use std::str::FromStr;

    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::character::complete::{alpha1, digit1, i64, multispace1, space1};
    use nom::combinator::map_res;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, tuple};
    use nom::IResult;

    use super::*;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ArithmeticToken {
        MULT,
        ADD,
    }
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ValueToken {
        CONST(i64),
        REF,
    }

    impl FromStr for ValueToken {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "OLD" => Ok(ValueToken::REF),
                dig => Ok(ValueToken::CONST(dig.parse::<i64>().unwrap())),
            }
        }
    }

    impl std::ops::Add for ValueToken {
        type Output = i64;

        fn add(self, rhs: Self) -> Self::Output {
            let val_l = match self {
                ValueToken::CONST(val) => val,
                ValueToken::REF => panic!("REF for lhs not allowed"),
            };
            let val_r = match rhs {
                ValueToken::CONST(val) => val,
                ValueToken::REF => val_l,
            };
            val_l + val_r
        }
    }
    impl std::ops::Mul for ValueToken {
        type Output = i64;

        fn mul(self, rhs: Self) -> Self::Output {
            let val_l = match self {
                ValueToken::CONST(val) => val,
                ValueToken::REF => panic!("REF not allowed on rhs"),
            };
            let val_r = match rhs {
                ValueToken::CONST(val) => val,
                ValueToken::REF => val_l,
            };
            val_l * val_r
        }
    }

    impl FromStr for ArithmeticToken {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "*" => Ok(ArithmeticToken::MULT),
                "+" => Ok(ArithmeticToken::ADD),
                &_ => panic!("not math"),
            }
        }
    }

    impl ArithmeticToken {
        pub fn apply(self, lhs: ValueToken, rhs: ValueToken) -> i64 {
            match self {
                ArithmeticToken::MULT => lhs * rhs,
                ArithmeticToken::ADD => lhs + rhs,
            }
        }
    }

    fn heading<'a>(input: &'a str) -> IResult<&str, i64> {
        delimited(tag("Monkey "), i64, tag(":\n"))(input)
    }

    fn items<'a>(input: &'a str) -> IResult<&'a str, RefCell<VecDeque<ValueToken>>> {
        let (input, _) = multispace1(input)?;
        map_res(
            delimited(
                tag("Starting items: "),
                separated_list1(tag(", "), i64),
                tag("\n"),
            ),
            |v| {
                Ok::<RefCell<VecDeque<ValueToken>>, nom::Err<nom::error::Error<&'a str>>>(
                    RefCell::new(v.iter().map(|i| ValueToken::CONST(i.clone())).collect()),
                )
            },
        )(input)
    }

    fn items2<'a>(input: &'a str) -> IResult<&'a str, RefCell<VecDeque<HashMap<i64, ValueToken>>>> {
        let module_test: [i64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
        let (input, _) = multispace1(input)?;
        let (output, items) = delimited(
            tag("Starting items: "),
            separated_list1(tag(", "), i64),
            tag("\n"),
        )(input)?;
        let worry_map: VecDeque<HashMap<i64, ValueToken>> = items
            .iter()
            .map(|i: &i64| {
                HashMap::from_iter(module_test.map(move |j| (j, ValueToken::CONST(i % j))))
            })
            .collect();

        //Vec<HashMap<u32, u32>>
        Ok((output, RefCell::from(worry_map)))
    }

    fn divisor<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, _) = multispace1(input)?;
        delimited(tag("Test: divisible by "), i64, tag("\n"))(input)
    }

    fn operation<'a>(input: &'a str) -> IResult<&'a str, (ArithmeticToken, ValueToken)> {
        let (input, _) = multispace1(input)?;
        map_res(
            delimited(
                tag("Operation: new = old "),
                tuple((alt((tag("*"), tag("+"))), space1, alt((alpha1, digit1)))),
                tag("\n"),
            ),
            |(arithmetic, _, constant)| {
                Ok::<(ArithmeticToken, ValueToken), nom::Err<nom::error::Error<&'a str>>>((
                    ArithmeticToken::from_str(arithmetic).unwrap(),
                    match constant {
                        "old" => ValueToken::REF,
                        c => ValueToken::from_str(c).unwrap(),
                    },
                ))
            },
        )(input)
    }

    fn predicate<'a>(input: &'a str) -> IResult<&'a str, (usize, usize)> {
        let (input, _) = multispace1(input)?;
        let (input, true_index) =
            delimited(tag("If true: throw to monkey "), i64, tag("\n"))(input)?;
        let (input, _) = multispace1(input)?;
        let (input, false_index) =
            delimited(tag("If false: throw to monkey "), i64, tag("\n"))(input)?;
        Ok((input, (true_index as usize, false_index as usize)))
    }

    fn parse_monkey<'a>(input: &'a str) -> IResult<&str, Monkey> {
        let (input, _) = heading(input)?;
        let (input, items) = items(input)?;
        let (input, operation) = operation(input)?;
        let (input, test) = divisor(input)?;
        let (output, indices) = predicate(input)?;
        Ok((
            output,
            Monkey {
                items,
                inspecting_count: RefCell::new(0),
                operation,
                test,
                bool_indices: indices,
            },
        ))
    }
    fn parse_monkey2<'a>(input: &'a str) -> IResult<&str, Monkey2> {
        let (input, _) = heading(input)?;
        let (input, items2) = items2(input)?;
        let (input, operation) = operation(input)?;
        let (input, test) = divisor(input)?;
        let (output, indices) = predicate(input)?;
        Ok((
            output,
            Monkey2 {
                items: items2,
                inspecting_count: RefCell::new(0),
                operation,
                test,
                bool_indices: indices,
            },
        ))
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&str, Vec<Monkey>> {
        separated_list1(newline, parse_monkey)(input)
    }

    pub fn parse2<'a>(input: &'a str) -> IResult<&str, Vec<Monkey2>> {
        separated_list1(newline, parse_monkey2)(input)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        static TEST_MONKEY: &str = "Monkey 1:
    Starting items: 78, 74, 88, 89, 50
    Operation: new = old * 11
    Test: divisible by 5
        If true: throw to monkey 3
        If false: throw to monkey 5
    ";

        #[test]
        fn parse_monkey_test() {
            let (out1, monkey_number) = heading(TEST_MONKEY).unwrap();
            let (out2, items) = items(out1).unwrap();
            let (out4, operation) = operation(out2).unwrap();
            let (out5, div) = divisor(out4).unwrap();
            let (_, indices) = predicate(out5).unwrap();

            assert_eq!(monkey_number, 1);
            assert_eq!(
                *items.borrow(),
                vec![
                    ValueToken::CONST(78),
                    ValueToken::CONST(74),
                    ValueToken::CONST(88),
                    ValueToken::CONST(89),
                    ValueToken::CONST(50)
                ]
            );
            assert_eq!(operation, (ArithmeticToken::MULT, ValueToken::CONST(11)));
            assert_eq!(div, 5);
            assert_eq!(indices, (3, 5));
        }
    }
}

fn print_monkies(monkies: &Vec<Monkey>) {
    for (count, mon) in monkies.iter().enumerate() {
        print!("{}", count);
        print_monkey(mon);
    }
    print!("\n")
}
fn print_monkey(mon: &Monkey) {
    print!(
        "Monkey: {:?}\n",
        mon.items
            .borrow()
            .iter()
            .map(|c| {
                match c {
                    ValueToken::CONST(val) => *val,
                    _ => panic!(),
                }
            })
            .collect::<Vec<i64>>()
    );
}

pub fn advent11_2() -> String {
    let content = helper::read_puzzle_input("./src/advent11/monkey-business.txt");
    let (rest, monkies) = parsing::parse2(&content).unwrap();
    println!("{:?}", rest);

    for i in 1..=10000 {
        println!("ROUND: {}", i);
        for (c, monkey) in monkies.iter().enumerate() {
            {
                while let Some(mut worry) = monkey.items.borrow_mut().pop_front() {
                    *monkey.inspecting_count.borrow_mut() += 1;

                    match monkey.operation.0 {
                        parsing::ArithmeticToken::MULT => {
                            worry = worry
                                .iter_mut()
                                .map(|(i, j)| {
                                    (*i, ValueToken::CONST((*j * monkey.operation.1) % *i))
                                })
                                .collect();
                            ()
                        }
                        parsing::ArithmeticToken::ADD => {
                            worry = worry
                                .iter_mut()
                                .map(|(i, j)| {
                                    (*i, ValueToken::CONST((*j + monkey.operation.1) % *i))
                                })
                                .collect();
                            ()
                        }
                    };
                    let case = match worry.get(&monkey.test).unwrap() {
                        ValueToken::CONST(val) => val.clone(),
                        ValueToken::REF => panic!("wrong"),
                    };

                    if case == 0 {
                        //println!("{c} From: {}, To: {} -> true", c, monkey.bool_indices.0);
                        monkies[monkey.bool_indices.0]
                            .items
                            .borrow_mut()
                            .push_back(worry);
                    } else {
                        //println!("{c} From: {}, To: {} -> false", c, monkey.bool_indices.0);
                        monkies[monkey.bool_indices.1]
                            .items
                            .borrow_mut()
                            .push_back(worry);
                    }
                }
                //print_monkies(&monkies)
            }
        }
    }
    let mut business: Vec<usize> = monkies
        .iter()
        .map(|m| m.inspecting_count.borrow().clone())
        .collect();
    business.sort();
    dbg!(&business);
    let mut iter = business.iter().rev().take(2);
    (iter.next().unwrap() * iter.next().unwrap()).to_string()
}

pub fn advent11() -> String {
    let content = helper::read_puzzle_input("./src/advent11/monkey-business.txt");
    let (rest, monkies) = parsing::parse(&content).unwrap();
    println!("{:?}", rest);

    for i in 1..=20 {
        println!("ROUND: {}", i);
        for (c, monkey) in monkies.iter().enumerate() {
            {
                while let Some(lhs) = monkey.items.borrow_mut().pop_front() {
                    let mut worry_level = monkey.operation.0.apply(lhs, monkey.operation.1);
                    *monkey.inspecting_count.borrow_mut() += 1;

                    worry_level = (worry_level as f32 / 3.0).floor() as i64;
                    if worry_level % monkey.test == 0 {
                        //println!("{c} From: {}, To: {} -> true", c, monkey.bool_indices.0);
                        monkies[monkey.bool_indices.0]
                            .items
                            .borrow_mut()
                            .push_back(ValueToken::CONST(worry_level));
                    } else {
                        //println!("{c} From: {}, To: {} -> false", c, monkey.bool_indices.0);
                        monkies[monkey.bool_indices.1]
                            .items
                            .borrow_mut()
                            .push_back(ValueToken::CONST(worry_level));
                    }
                }
                print_monkies(&monkies)
            }
        }
    }
    let mut business: Vec<usize> = monkies
        .iter()
        .map(|m| m.inspecting_count.borrow().clone())
        .collect();
    business.sort();
    dbg!(&business);
    let mut iter = business.iter().rev().take(2);
    (iter.next().unwrap() * iter.next().unwrap()).to_string()
}
