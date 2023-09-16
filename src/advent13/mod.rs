use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    fmt,
    rc::Rc,
};

use crate::helper;
#[derive(Debug, PartialEq)]
enum Element {
    List(Vec<Rc<RefCell<Element>>>),
    Int(u8),
    EmptyList,
}

struct Iter<T> {
    parent: T,
    current: T,
    current_index: Vec<usize>,
}

impl Iterator for Iter<Rc<RefCell<Element>>> {
    type Item = Rc<RefCell<Element>>;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp_current = self.current.clone();
        match &*tmp_current.borrow() {
            Element::List(list) => list
                .get(*self.current_index.last().unwrap())
                .map(|p| {
                    match &*p.borrow() {
                        Element::List(_) => {
                            self.current_index.push(0);
                        }
                        Element::Int(_) => (),
                        Element::EmptyList => (),
                    }
                    Some(p.clone())
                })
                .unwrap_or_else(|| {
                    // we ran out of elements in the list
                    self.current = self.parent;
                    self.current_index.pop();
                    Some(self.parent.clone())
                }),
            Element::EmptyList => {
                self.current = self.parent;
                self.current_index.pop();
                Some(Rc::new(RefCell::new(Element::EmptyList)))
            }
            Element::Int(int) => {
                *self.current_index.last().unwrap() += 1;
                Some(self.current.clone())
            }
        }
    }
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

fn parse_packet<'a>(input: &'a str) -> Rc<RefCell<Element>> {
    let mut peek = input.chars().peekable();
    let packet = Rc::new(RefCell::new(Element::List(vec![])));
    let mut parent_element = packet.clone();
    let mut current_element = packet.clone();
    peek.next(); // magic next ignores first nesting:)
    while let Some(char) = peek.next() {
        match char {
            '[' => {
                let nested = Rc::new(RefCell::new(Element::List(vec![])));
                match &mut *parent_element.borrow_mut() {
                    Element::List(list) => {
                        if peek.peek().unwrap() == &']' {
                            list.push(Rc::new(RefCell::new(Element::EmptyList)));
                        } else {
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
                    match &mut *current_element.borrow_mut() {
                        Element::List(list) => {
                            list.push(ele);
                        }
                        Element::Int(_) => panic!("some wrong parsing"),
                    }
                }
            },
        }
    }
    packet
}

//fn parse_unit<'a>ut: Rc<ROption<>}
fn advent13() -> String {
    let content = helper::read_puzzle_input("./src/advent13/distress_test.txt");
    let mut packets = vec![];
    let mut raw = content.lines().peekable();
    while raw.peek().is_some() {
        packets.push((
            parse_packet(raw.next().unwrap()),
            parse_packet(raw.next().unwrap()),
        ));
        raw.next(); // empty line
    }
    packets
        .iter()
        .for_each(|p| print!("{}\n{}\n\n", p.0.borrow(), p.1.borrow()));

    let first_packet = Iter {
        parent: packets[0].0.clone(),
        current: packets[0].0.clone(),
        current_index: vec![0],
    };
    let second_packet = Iter {
        parent: packets[0].1.clone(),
        current: packets[0].1.clone(),
        current_index: vec![0],
    };

    print!("{}", &*packets[0].0.borrow());
    for (i, ele) in first_packet.enumerate() {
        print!("{}: {}", i, &*ele.borrow());
    }

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
