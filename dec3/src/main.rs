use std::fs;

fn main() {
    let lines = get_lines_from_file();

    let answer1 = solution1(lines);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(lines: Vec<String>) -> i32 {
    for line in lines {
        let length = line.chars().count();
        let middle_point = length / 2;

        let compartment1 = &line[0..middle_point];
        let compartment2 = &line[middle_point..length];

        println!("Compartment 1: {}", compartment1);
        println!("Compartment 2: {}", compartment2);
    }
    
    1
}