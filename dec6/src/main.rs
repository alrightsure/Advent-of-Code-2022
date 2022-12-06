use itertools::Itertools;
use std::fs;

fn main() {
    let line = get_line_from_file();

    let answer1 = find_index_of_unique_chars(&line, 4);
    let answer2 = find_index_of_unique_chars(&line, 14);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is {}", answer2);
}

fn get_line_from_file() -> String {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    contents
}

fn find_index_of_unique_chars(line: &String, num_of_unique_chars_required: usize) -> usize {
    let line_chars: Vec<char> = line.chars().collect();
    let mut return_value: usize = 0;

    for i in 0..line_chars.len() {
        if i >= num_of_unique_chars_required - 1 {
            let mut unique_chars: Vec<char> = Vec::new();
            for num in 0..num_of_unique_chars_required {
                unique_chars.push(line_chars[i - num]);
            }

            unique_chars = unique_chars.into_iter().rev().unique().collect();

            if unique_chars.len() == num_of_unique_chars_required {
                return_value = i + 1;
                break;
            }
        }
    }

    return_value
}
