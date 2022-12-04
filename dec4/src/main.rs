use std::fs;

fn main() {
    let lines = get_lines_from_file();

    let answer1 = solution1(&lines);
    let answer2 = solution2(&lines);

    println!("The answer to problem 1 is {}", answer1);
    println!("The answer to problem 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(lines: &Vec<String>) -> i32 {
    let mut number_of_intersecting_pairs = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();
        let elf_1_sections = get_sections_from_range(ranges[0]);
        let elf_2_sections = get_sections_from_range(ranges[1]);

        if (elf_1_sections.iter().all(|item| elf_2_sections.contains(item))) || (elf_2_sections.iter().all(|item| elf_1_sections.contains(item))) {
            number_of_intersecting_pairs += 1;
        }
    }

    number_of_intersecting_pairs
}

fn solution2(lines: &Vec<String>) -> i32 {
    let mut number_of_overlapping_pairs = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();
        let elf_1_sections = get_sections_from_range(ranges[0]);
        let elf_2_sections = get_sections_from_range(ranges[1]);

        if (elf_1_sections.iter().any(|item| elf_2_sections.contains(item))) || (elf_2_sections.iter().any(|item| elf_1_sections.contains(item))) {
            number_of_overlapping_pairs += 1;
        }
    }

    number_of_overlapping_pairs
}

fn get_sections_from_range(range: &str) -> Vec<i32> {
    let input_numbers: Vec<&str> = range.split('-').collect();
    let number1 = input_numbers[0].parse::<i32>().unwrap();
    let number2 = input_numbers[1].parse::<i32>().unwrap();
    let mut output_range: Vec<i32> = Vec::new();

    for n in number1 .. number2 + 1 {
        output_range.push(n);
    }

    output_range
}