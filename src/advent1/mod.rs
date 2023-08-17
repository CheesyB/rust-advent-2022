use std::fs;

fn basic() -> Vec<i32> {
    let file_path = "./src/advent1/calories.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let nums: Vec<i32> = contents
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect();
    let mut grouped: Vec<i32> = vec![];
    let mut current_sum = 0;
    for elem in nums {
        if elem != 0 {
            current_sum += elem;
        } else {
            grouped.push(current_sum);
            current_sum = 0;
        }
    }
    return grouped;
}

pub fn advent1() -> i32 {
    let result = basic();
    return *result.iter().max().unwrap();
}

pub fn advent1_2() -> i32 {
    let result = basic();
    return result.iter().rev().take(3).sum::<i32>();
}
