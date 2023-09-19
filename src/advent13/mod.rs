use std::{cell::RefCell, fmt, ops::Deref, rc::Rc};

use crate::helper;
#[derive(Debug)]
enum Element {
    List(Vec<Rc<RefCell<Element>>>),
    Int(u8),
    EmptyList,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::List(list) => {
                write!(f, "[")?;
                for ele in list {
                    ele.borrow().fmt(f)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Element::Int(int) => write!(f, ",{}", int),
            Element::EmptyList => write!(f, "EMPTY"),
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Eq for Element {}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal)
        )
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Element::List(ref list) => match other {
                Element::List(ref other_list) => {
                    list.first().unwrap().cmp(other_list.first().unwrap())
                }
                Element::Int(other_int) => list
                Element::EmptyList => todo!(),
            },
            Element::Int(ref int) => match other {
                Element::List(ref other_list) => todo!(),
                Element::Int(ref other_int) => int.cmp(other_int),
                Element::EmptyList => todo!(),
            },
            Element::EmptyList => todo!(),
        }
    }
}

fn parse_packet2<'a>(input: &'a str) -> (Rc<RefCell<Element>>, Vec<Rc<RefCell<Element>>>) {

fn parse_packet<'a>(input: &'a str) -> (Rc<RefCell<Element>>, Vec<Rc<RefCell<Element>>>) {
    let mut peek = input.chars().peekable();
    let packet = Rc::new(RefCell::new(Element::List(vec![])));
    let mut parent_element = packet.clone();
    let mut current_element = packet.clone();
    let mut indices = vec![packet.clone()];
    peek.next(); // magic next ignores first nesting:)
    while let Some(char) = peek.next() {
        match char {
            '[' => {
                let nested = Rc::new(RefCell::new(Element::List(vec![])));
                match &mut *parent_element.deref().borrow_mut() {
                    Element::List(list) => {
                        if peek.peek().unwrap() == &']' {
                            let empty_list = Rc::new(RefCell::new(Element::EmptyList));
                            indices.push(nested.clone());
                            list.push(empty_list.clone());
                        } else {
                            indices.push(nested.clone());
                            list.push(nested.clone());
                        }
                    }
                    Element::Int(_) => panic!("some wrong parsing"),
                    Element::EmptyList => panic!("some wrong parsing"),
                }
                parent_element = current_element;
                current_element = nested
            }
            ']' => current_element = parent_element.clone(),
            val => match val {
                ',' => (),
                ' ' => (),
                other => {
                    let mut digit = other.to_string();
                    // if number is 10
                    if peek.peek().unwrap().to_digit(10).is_some() {
                        let next_digit =
                            char::from_digit(peek.peek().unwrap().to_digit(10).unwrap(), 10)
                                .unwrap();
                        digit.push(next_digit);
                        peek.next();
                    }
                    let int = u8::from_str_radix(&digit, 10).unwrap();
                    let ele = Rc::new(RefCell::new(Element::Int(int)));
                    indices.push(ele.clone());
                    match &mut *current_element.deref().borrow_mut() {
                        Element::List(list) => {
                            list.push(ele);
                        }
                        Element::Int(_) => panic!("some wrong parsing"),
                        Element::EmptyList => panic!("some wrong parsing"),
                    }
                }
            },
        }
    }
    (packet, indices)
}

//fn parse_unit<'a>ut: Rc<ROption<>}
pub fn advent13() -> String {
    let content = helper::read_puzzle_input("./src/advent13/distress_test.txt");
    let mut packets = vec![];
    let mut packets_iter = vec![];
    let mut raw = content.lines().peekable();
    while raw.peek().is_some() {
        let first = parse_packet(raw.next().unwrap());
        let second = parse_packet(raw.next().unwrap());
        packets.push((first.0, second.0));
        packets_iter.push((first.1, second.1));
        raw.next(); // empty line
    }
    packets
        .iter()
        .for_each(|p| print!("{}\n{}\n\n", p.0.borrow(), p.1.borrow()));

    dbg!(&packets_iter[0]);
    "hallo".to_string()
}

pub fn advent13_2() -> String {
    "hallo2".to_string()
}

#[cfg(test)]
mod tests {

    use crate::advent13::Element;

    //#[test]
    fn test1() {
        let input = "[[4,4],4,4]".to_string();
        let splitty: Vec<&str> = input.split('[').collect();
        dbg!(splitty);
        assert!(false);
    }

    #[test]
    fn test2() {
        let input = "[[]1,11,[4,4],4,4]".to_string();

        //dbg!(packet);
        assert!(false)
    }
}
