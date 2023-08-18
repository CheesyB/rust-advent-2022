use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Section {
    start: i32,
    end: i32,
}

impl FromStr for Section {
    type Err = ();

    fn from_str(input: &str) -> Result<Section, Self::Err> {
        let mut intermediate = input
            .split("-")
            .map(|number| number.parse::<i32>().unwrap());
        return Ok(Section {
            start: intermediate.next().unwrap(),
            end: intermediate.next().unwrap(),
        });
    }
}

fn range_overlap(first: &Section, second: &Section) -> bool {
    if first.end >= second.start && first.start <= second.end {
        return true;
    } else {
        return false;
    }
}

fn range_contained(first: Section, second: Section) -> bool {
    if (first.start <= second.start && first.end >= second.end)
        || first.start >= second.start && first.end <= second.end
    {
        return true;
    } else {
        return false;
    }
}

fn basic() -> String {
    let file_path = "./src/advent4/sections.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

pub fn advent4_2() -> String {
    let content = basic();
    let mut count: i32 = 0;
    for line in content.lines() {
        let mut ranges = line.split(',');
        let start_range = Section::from_str(ranges.next().unwrap()).unwrap();
        let end_range = Section::from_str(ranges.next().unwrap()).unwrap();
        if range_overlap(&start_range, &end_range) {
            count += 1;
        }
    }

    return count.to_string();
}

pub fn advent4() -> String {
    let content = basic();
    let mut count: i32 = 0;
    for line in content.lines() {
        let mut ranges = line.split(',');
        let start_range = Section::from_str(ranges.next().unwrap()).unwrap();
        let end_range = Section::from_str(ranges.next().unwrap()).unwrap();
        if range_contained(start_range, end_range) {
            count += 1;
        }
    }
    return count.to_string();
}
