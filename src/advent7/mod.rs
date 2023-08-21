use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;
use regex::{self, RegexSet, Regex};

fn basic() -> String {
    let file_path = "./src/advent7/source_tree.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[derive(PartialEq)]
struct TreeNode {
    pub files: Vec<Option<u32>>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            files: vec![],
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    //pub fn calc_size(&self) todo!;

    // misses file
    pub fn print(&self) -> String {
        return String::from("[")
            + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
            + "]";
    }
}

struct Command {
    cmd: String,
    arg: Option<Vec<String>>,
    output: Option<Vec<String>>,
}


fn tag_contains(pattern: &str) -> impl for<'a> Fn(Vec<&'a str>,VecDeque<&'a str>) -> (Vec<&'a str>, VecDeque<&'a str>) {
    let matcher = Regex::new(pattern).unwrap();
    let match_tag = move |mut parsed: Vec<&str>, token: &VecDeque<&str>| {
        if matcher.is_match(token.front().unwrap()) {
            let matched = token.pop_front().unwrap();
            parsed.push(matched);
        }
        return (parsed, token);
    };
    return match_tag;
}


fn parse_cd<'a>(token: &'a mut VecDeque<&'a str>) -> (Option<&'a str>, VecDeque<&'a str>) {
    let (parsed, token) = tag_contains(r"\/|\.\.|[a-zA-Z]")(
     tag_contains(r"\cd")(
        tag_contains(r"\$")()));
    
    if cmd.is_some() && arg.is_some() {
        Some(Command {
            cmd: cmd.unwrap().to_owned(),
            arg: Some(arg.unwrap().to_owned()),
            output: None,
        })
    }
    let (folder, token) = tag_contains()
}

fn parse(mut token: VecDeque<&str>) -> Vec<Command> {
    return todo!();
}

pub fn advent7() -> String {
    let cmd_history = basic();
    let line_free_cmd_history = cmd_history.replace('\n', " ");
    let token: VecDeque<&str> = line_free_cmd_history.split_whitespace().collect();
    let commands: Vec<Command> = parse(token);

    return String::from("HAllo");
}
