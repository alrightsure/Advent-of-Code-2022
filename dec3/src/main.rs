use std::{fs, collections::HashSet};


fn main() {
    let lines = get_lines_from_file();

    let answer1 = solution1(&lines);
    let answer2 = solution2(&lines);

    println!("The answer to question 1 is {}", answer1);
    println!("The anwer to question 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(lines: &Vec<String>) -> i32 {
    let mut total_common_priority = 0;
    for line in lines {
        let length = line.chars().count();
        let middle_point = length / 2;

        let compartment1 = &line[0..middle_point];
        let compartment2 = &line[middle_point..length];

        let common_chars = find_common_char_per_compartment(compartment1, compartment2);

        for c in common_chars {
            total_common_priority += get_item_priority(c);
        }
    }
    
    total_common_priority
}

fn solution2(lines: &Vec<String>) -> i32 {
    const LINES_PER_SET: usize = 3;
    let mut line_count:usize = 0;
    let mut total_badge_priority: i32 = 0;

    while line_count < lines.len() {
        let common_chars = find_common_char_per_group(lines[line_count..line_count + LINES_PER_SET].to_vec());
        
        for c in common_chars {
            total_badge_priority += get_item_priority(c);
        }

        line_count += LINES_PER_SET;
    }

    total_badge_priority
}

fn find_common_char_per_compartment(compartment1: &str, compartment2: &str) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for c in compartment1.chars() {
        if compartment2.contains(c) && !common_chars.contains(&c) {
            common_chars.push(c);
        }
    }
    
    common_chars
}

fn find_common_char_per_group(group: Vec<String>) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();

    for line in group {
        if common_chars.is_empty() {
            common_chars = line.chars().collect();
        } else {
            let current_common_chars: HashSet<char> = common_chars.into_iter().collect();
            let line_chars: HashSet<char> = line.clone().chars().into_iter().collect();

            common_chars = current_common_chars.intersection(&line_chars).map(|i| *i).collect::<Vec<_>>();
        }
    }

    common_chars
}

fn get_item_priority(item: char) -> i32 {
    let lowercase_item = item.to_ascii_lowercase();
    let mut priority = match lowercase_item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };

    if item.is_uppercase() {
        priority += 26;
    }

    priority
}