use std::fs;

fn main() {
    const PATH_TO_FILE: &str = "./input.txt";
    
    let lines = get_lines_from_file(PATH_TO_FILE);
    let elves = get_all_elves(lines);

    println!("3 highest calorie elf total: {}", get_top_3_total_calories(elves));
}

fn get_lines_from_file(path_to_file: &str) -> Vec<String> {
    let contents = fs::read_to_string(path_to_file)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn get_all_elves(lines: Vec<String>) -> Vec<i32> {
    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf_index = 0;

    for line in lines{
        if line.is_empty() {
            current_elf_index += 1;
            continue;
        }
        
        let calories = line.parse::<i32>().unwrap();
        
        if elves.len() <= current_elf_index {
            elves.push(calories);
        } else {
            elves[current_elf_index] += calories;
        }
    }
    elves
}

fn get_top_3_total_calories(elves: Vec<i32>) -> i32 {
    let mut elves = elves;
    elves.sort();
    elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
}