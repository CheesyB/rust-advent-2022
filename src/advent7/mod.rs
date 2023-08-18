use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;

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
enum CMD {
    CD,
    LS,
}

struct Command<'a> {
    cmd: CMD,
    arg: Option<&'a str>,
    output: Option<&'a str>,
}
fn tag(val: &str) -> impl Fn(VecDeque<&str>) -> bool {
    let match_tag = move |token: &VecDeque<&str>| &val == token.front().unwrap();
    return match_tag;
}

fn parse_cd_up_cmd(token: &mut VecDeque<&str>) {
    if tag("$")(token) {
        token.pop_front();
        let arg = token.pop_front().unwrap();
    }
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
