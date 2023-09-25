use std::vec;

use crate::helper;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
enum Element<'a> {
    List(&'a str),
    Val(u8),
}

fn parse_level<'a>(input: &'a str) -> Vec<Element> {
    let mut char_peek = input.char_indices().peekable();
    let mut level: usize = 0;
    let mut nested_range_start: Option<usize> = None;
    let mut items = vec![];
    while let Some((pos, ch)) = char_peek.next() {
        match ch {
            '[' => {
                if level == 1 {
                    nested_range_start = Some(pos);
                }
                level += 1;
            }
            ']' => {
                level -= 1;
                if level == 1 {
                    items.push(Element::List(&input[nested_range_start.unwrap()..=pos]));
                }
            }
            ',' => (),
            ' ' => (),
            other => {
                if level == 1 {
                    let mut digit = other.to_string();
                    // if number is 10
                    if let Some(digi) = char_peek.peek().clone().unwrap().1.to_digit(10) {
                        let next_digit = char::from_digit(digi, 10).unwrap();
                        digit.push(next_digit);
                        char_peek.next(); //don't loop
                    }
                    let int = u8::from_str_radix(&digit, 10).unwrap();
                    items.push(Element::Val(int))
                }
            }
        }
    }
    items
}

fn start_compare_packets(left: &str, right: &str) -> bool {
    if let Some(res) = compare_packets(left, right) {
        return res;
    } else {
        return false;
    }
}

fn compare_packets(left: &str, right: &str) -> Option<bool> {
    let left_eles = parse_level(left);
    if left_eles.is_empty() {
        println!(" => true (Left RAN OUT FIRST)");
        return Some(true);
    }
    let right_eles = parse_level(right);
    let mut left_iter = left_eles.iter();
    let mut right_iter = right_eles.iter();
    while let Some(left_ele) = left_iter.next() {
        if let Some(right_ele) = right_iter.next() {
            match left_ele {
                Element::List(l_list) => match right_ele {
                    Element::List(r_list) => {
                        println!(" LIST:{} <> LIST:{}", l_list, r_list);
                        if let Some(cmp) = compare_packets(l_list, r_list) {
                            return Some(cmp);
                        }
                        continue;
                    }
                    Element::Val(r_int) => {
                        println!(" LIST:{} <> INT:{}", l_list, r_int);

                        if let Some(cmp) = compare_packets(l_list, &format!("[{r_int}]")) {
                            return Some(cmp);
                        }
                        continue;
                    }
                },
                Element::Val(l_int) => match right_ele {
                    Element::List(r_list) => {
                        println!(" INT:{} <> LIST:{}", l_int, r_list);
                        if let Some(cmp) = compare_packets(&format!("[{l_int}]"), r_list) {
                            return Some(cmp);
                        }
                        continue;
                    }
                    Element::Val(r_int) => {
                        if l_int == r_int {
                            println!(" INT:{} == INT:{}", l_int, r_int);
                            continue;
                        } else {
                            println!(" INT:{} < INT:{} => {}", l_int, r_int, l_int < r_int);
                            return Some(l_int < r_int);
                        }
                    }
                },
            }
        } else {
            //ran out of right values
            print!(" =>  false (RIGHT RAN OUT)\n");
            return Some(false);
        }
    }
    None
}

pub fn advent13() -> String {
    let content = helper::read_puzzle_input("./src/advent13/distress.txt");
    let mut packets_left = vec![];
    let mut packets_right = vec![];
    let mut raw = content.lines().peekable();
    while raw.peek().is_some() {
        packets_left.push(raw.next().unwrap());
        packets_right.push(raw.next().unwrap());
        raw.next(); // empty line
    }
    let packets: Vec<_> = packets_left.iter().zip(packets_right.iter()).collect();

    let mut summary = vec![];
    for (l, r) in packets {
        let val = start_compare_packets(l, r);
        println!();
        summary.push(val);
    }
    summary
        .iter()
        .enumerate()
        .fold(
            0,
            |acc: usize, (index, val)| if *val { acc + index + 1 } else { acc },
        )
        .to_string()
}

pub fn advent13_2() -> String {
    "hallo2".to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cmp1() {
        let result = start_compare_packets("[1,1,3,1,1]", "[1,1,5,1,1]");
        assert!(result);
    }
    #[test]
    fn test_cmp2() {
        let result = start_compare_packets("[[1],[2,3,4]]", "[[1],4]");
        assert!(result);
    }
    #[test]
    fn test_cmp3() {
        let result = start_compare_packets("[9]", "[[8,7,6]]");
        assert!(!result);
    }    }
    #[test]
    fn test_cmp4() {
              let result = start_compare_packets("[[4,4],4,4]", "[[4,4],4,4,4]");
              assert!(result);
    }    
    #[test]
     fn test_cmp5() {
              let result = start_compare_packets("[7,7,7,7]", "[7,[6],7,7]");
              assert!(!result);
    }
    #[test]
    fn test_cmp6() {
        let result = start_compare_packets("[]", "[3]");
        assert!(result);
    }
    #[test]
    fn test_cmp7() {
        let result = start_compare_packets("[[[]]]", "[[]]");
        assert!(!result);
    }
    #[test]
    fn test_cmp8() {
        let result =
            start_compare_packets("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(!result);
    }
    #[test]
    fn test_cmp9() {
        let result = start_compare_packets(
            "[[2,8]]",
            "[[[[2],[9,9,6],[1,8],[5,4,6,0,2]],[[5,9],[]],[7,[4,2,3,4],6]],[],[1,9,7,[6]]]",
        );
        assert!(result);
    }
    #[test]
    fn test_cmp10() {
        let result = start_compare_packets(
            "[[],[[1,[6,2,10]]],[],[[[7,9,2,8],[5,1,2],9],3,[10]]]",
            "[[[2]],[10,[10,6,8],8,[8,0,10,2],10]]",
        );
        assert!(result);
    }
        

    #[test]
    fn test_double_nesting() {
        let result = parse_level("[[4,[5,5],4],4,4]");
        dbg!(&result);
        assert_eq!(
            result,
            vec![
                Element::List("[4,[5,5],4]"),
                Element::Val(4),
                Element::Val(4)
            ]
        );
    }
    #[test]
    fn test_nesting() {
        let result = parse_level("[[4,4],4,4]");
        dbg!(&result);
        assert_eq!(
            result,
            vec![Element::List("[4,4]"), Element::Val(4), Element::Val(4)]
        );
    }
}
