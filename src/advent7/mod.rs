use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, newline, space1};
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

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

    pub fn build_tree(commands: Vec<Command>) -> Rc<RefCell<TreeNode>> {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        let mut current = Rc::clone(&root);
        for cmd in commands {
            match cmd.cmd {
                CMD::CD => TreeNode::handle_cd(cmd),
                CMD::LS => TreeNode::handle_ls(cmd),
            };
            if *c == '[' || c.is_numeric() {
                let child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    if c.is_numeric() {
                        mut_child.value = c.to_digit(10);
                    }
                }
                current = child;
            } else if *c == ',' || *c == ']' {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else {
                panic!("Unknown character: {}", c);
            }
        }
        return root;
    }

    fn handle_cd(cmd: Command) {}
    fn handle_ls(cmd: Command) {}

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

#[derive(Debug, PartialEq)]
enum CMD {
    CD,
    LS,
}

#[derive(Debug)]
struct Command<'a> {
    cmd: CMD,
    arg: Option<String>,
    output: Option<Vec<LSOutput<'a>>>,
}

impl<'c> Command<'c> {
    fn from_lsoutput(input: Vec<LSOutput<'c>>) -> Self {
        Command {
            cmd: CMD::LS,
            arg: None,
            output: Some(input),
        }
    }
}
#[derive(Debug)]
struct AFile<'a> {
    size: u32,
    name: &'a str,
}
impl<'b> AFile<'b> {
    fn new(size: u32, name: &'b str) -> Self {
        AFile { size, name }
    }
}
#[derive(Debug)]
struct AFolder<'a> {
    name: &'a str,
}
impl<'b> AFolder<'b> {
    fn new(name: &'b str) -> Self {
        AFolder { name }
    }
}

#[derive(Debug)]
enum LSOutput<'a> {
    FILE(AFile<'a>),
    FOLDER(AFolder<'a>),
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ cd ")(input)?;
    map_res(alt((tag("/"), tag(".."), alpha1)), |new_arg: &str| {
        Ok::<Command, ()>(Command {
            cmd: CMD::CD,
            arg: Some(new_arg.to_owned()),
            output: None,
        })
    })(rest)
}

fn file_parser(input: &str) -> IResult<&str, LSOutput> {
    let (rest, (size, name)) = separated_pair(
        map_res(digit1, str::parse::<u32>),
        space1,
        alt((recognize(separated_pair(alpha1, char('.'), alpha1)), alpha1)),
    )(input)?;
    return Ok((rest, LSOutput::FILE(AFile::new(size, name))));
}

fn folder_parser(input: &str) -> IResult<&str, LSOutput> {
    let (rest, (_, name)) = tuple((tag("dir "), alpha1::<&str, _>))(input)?;
    return Ok((rest, LSOutput::FOLDER(AFolder::new(name))));
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ ls\n")(input)?;
    let (rest, output) = separated_list1(newline, alt((folder_parser, file_parser)))(rest)?;
    return Ok((rest, Command::from_lsoutput(output)));
}

fn build_tree(commands: Vec<Command>) {
    let tree = TreeNode::new();
    for cmd in commands {}
}

pub fn advent7() -> String {
    let cmd_history = basic();

    let (rest, commands) =
        separated_list1(newline, alt((parse_ls, parse_cd)))(&cmd_history).unwrap();
    assert!(rest == "\n");

    //let root_node = TreeNode::build_tree(commands);

    dbg!(commands);
    return String::from("Hallo");
}
