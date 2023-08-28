use std::fs;

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
#[derive(Debug)]
struct FolderNode {
    pub name: String,
    pub files: Vec<AFile>,
    pub folders: Vec<FolderNode>,
}

impl<'a> FolderNode {
    pub fn new(name: &str) -> FolderNode {
        FolderNode {
            name: name.to_string(),
            files: vec![],
            folders: vec![],
        }
    }

    pub fn total_size_smalest(&self, space: u32, values: &'a mut Vec<u32>) -> &'a mut Vec<u32> {
        for child in self.folders.iter() {
            if child.total_size() >= space {
                values.push(child.total_size());
            }
            child.total_size_smalest(space, values);
        }
        return values;
    }

    pub fn total_size_limit(&self, limit: u32) -> u32 {
        let mut limit_size = 0;
        for child in self.folders.iter() {
            if child.total_size() > limit {
            } else {
                limit_size += child.total_size();
            }
            limit_size += child.total_size_limit(limit);
        }
        limit_size
    }

    pub fn total_size(&self) -> u32 {
        self.own_size() + self.child_size()
    }

    fn child_size(&self) -> u32 {
        let mut child_size = 0;
        for folder in self.folders.iter() {
            child_size += folder.total_size()
        }
        child_size
    }

    fn own_size(&self) -> u32 {
        self.files.iter().map(|file| file.size).sum()
    }

    pub fn add_folder(&mut self, name: &'a str) {
        self.folders.push(FolderNode::new(name));
    }

    pub fn add_files(&mut self, file: &AFile) {
        self.files.push(file.clone());
    }
}

fn build_dir<'a>(commands: Vec<CMD>) -> FolderNode {
    let mut root = FolderNode::new("phantom");
    root.add_folder("/");
    let mut cwd_path = vec![];
    for command in commands.iter() {
        match command {
            CMD::CD(cmd) => match cmd.arg.as_ref().expect("arg should be there").as_str() {
                ".." => cd_up(&mut cwd_path),
                folder_name => cd_into(&mut cwd_path, folder_name),
            },
            CMD::LS(cmd) => {
                let cwd = get_mut_cwd(&mut root, &cwd_path);
                ls(cwd, cmd)
            }
        }
    }
    root
}

fn cd_into<'a>(cwd: &mut Vec<&'a str>, folder_name: &'a str) {
    cwd.push(folder_name);
}

fn cd_up(cwd: &mut Vec<&str>) {
    cwd.pop();
}

fn ls(cwd: &mut FolderNode, cmd: &Command) {
    for item in cmd.output.as_ref().unwrap() {
        match item {
            LSOutput::FILE(file) => cwd.add_files(file),
            LSOutput::FOLDER(folder) => cwd.add_folder(folder.name.as_str()),
        }
    }
}
fn get_mut_cwd<'a>(root: &'a mut FolderNode, cwd_path: &Vec<&'a str>) -> &'a mut FolderNode {
    let mut current = root;
    for folder_name in cwd_path.iter() {
        current = current
            .folders
            .iter_mut()
            .filter(|child| {
                return child.name == folder_name.to_string();
            })
            .next()
            .expect("folder should exist");
    }
    return current;
}

#[derive(Debug)]
enum CMD {
    CD(Command),
    LS(Command),
}

#[derive(Debug)]
struct Command {
    arg: Option<String>,
    output: Option<Vec<LSOutput>>,
}

impl Command {
    fn from_lsoutput(input: Vec<LSOutput>) -> Self {
        Command {
            arg: None,
            output: Some(input),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AFile {
    size: u32,
    name: String,
}
impl AFile {
    fn new(size: u32, name: &str) -> Self {
        AFile {
            size,
            name: name.to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
struct AFolder {
    name: String,
}
impl AFolder {
    fn new(name: &str) -> Self {
        AFolder {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug)]
enum LSOutput {
    FILE(AFile),
    FOLDER(AFolder),
}

fn parse_cd(input: &str) -> IResult<&str, CMD> {
    let (rest, _) = tag("$ cd ")(input)?;
    map_res(alt((tag("/"), tag(".."), alpha1)), |new_arg: &str| {
        Ok::<CMD, ()>(CMD::CD(Command {
            arg: Some(new_arg.to_owned()),
            output: None,
        }))
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

fn parse_ls(input: &str) -> IResult<&str, CMD> {
    let (rest, _) = tag("$ ls\n")(input)?;
    let (rest, output) = separated_list1(newline, alt((folder_parser, file_parser)))(rest)?;
    return Ok((rest, CMD::LS(Command::from_lsoutput(output))));
}

pub fn advent7() -> String {
    let cmd_history = basic();

    let (rest, commands) =
        separated_list1(newline, alt((parse_ls, parse_cd)))(&cmd_history).unwrap();
    assert!(rest == "\n");

    let root = build_dir(commands);

    return String::from(root.total_size_limit(100000).to_string());
}

pub fn advent7_2() -> String {
    let cmd_history = basic();

    let (rest, commands) =
        separated_list1(newline, alt((parse_ls, parse_cd)))(&cmd_history).unwrap();
    assert!(rest == "\n");

    let root = build_dir(commands);

    let size_to_delete = 30000000 - (70000000 - root.total_size());
    let values = &mut vec![];
    let folder_min_size = root
        .total_size_smalest(size_to_delete, values)
        .iter()
        .min()
        .expect("there should be at least one value");

    return folder_min_size.to_string();
}
