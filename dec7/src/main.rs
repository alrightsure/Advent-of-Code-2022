use std::fs;

struct Directory {
    id: i32,
    parent_id: Option<i32>,
    size: i32,
}

fn main() {
    let lines = get_lines_from_file();
    let file_system = get_file_system_from_lines(lines);

    let answer1 = solution1(&file_system);
    let answer2 = solution2(&file_system);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn get_file_system_from_lines(lines: Vec<String>) -> Vec<Directory> {
    let mut file_system: Vec<Directory> = vec![Directory {id: 0, parent_id: None, size: 0}];
    let mut current_directory_id: i32 = 0;
    let mut last_used_id = 0;

    for line in lines {
        if line.starts_with("$") {
            if line.contains("cd") {
                let line_split: Vec<&str> = line.split(" ").collect();
                let destination = line_split[2];

                if destination == ".." {
                        current_directory_id = file_system.iter().find(|d| d.id == current_directory_id).unwrap().parent_id.unwrap();
                } else {
                    let mut parent_id = 0;
                    if current_directory_id > 0 {
                        parent_id = file_system.iter().find(|d| d.id == current_directory_id).unwrap().id;
                    }

                    last_used_id += 1;
                    file_system.push(Directory { id: last_used_id, parent_id: Some(parent_id), size: 0 });
                    current_directory_id = last_used_id;
                }
            }
        } else {
            if line.as_bytes()[0].is_ascii_digit() {
                let line_split: Vec<&str> = line.split(" ").collect();
                let file_size = line_split[0];
                let current_directory = file_system.iter_mut().find(|d| d.id == current_directory_id).unwrap();

                current_directory.size += file_size.parse::<i32>().unwrap();
            }
        }
    }

    file_system
}

fn get_directory_size(file_system: &Vec<Directory>, dir_id: i32) -> i32 {
    let mut sum_size = file_system.iter().find(|d| d.id == dir_id).unwrap().size;
    
    for dir in file_system {
        if dir.parent_id == Some(dir_id) {
            sum_size += get_directory_size(file_system, dir.id);
        }
    }

    sum_size
}

fn solution1(file_system: &Vec<Directory>) -> i32 {
    let mut sum = 0;
    
    for dir in file_system {
        let total_dir_size = get_directory_size(file_system, dir.id);

        if total_dir_size <= 100000 {
            sum += total_dir_size;
        }
    }

    sum
}

fn solution2(file_system: &Vec<Directory>) -> i32 {
    const TOTAL_SPACE: i32 = 70000000;
    const NEEDED_SPACE: i32 = 30000000;
    
    let used_space = get_directory_size(file_system, 0);
    let free_space = TOTAL_SPACE - used_space;
    let mut smallest_option = 0;

    for dir in file_system {
        let total_dir_size = get_directory_size(file_system, dir.id);

        if free_space + total_dir_size >= NEEDED_SPACE {
            if smallest_option == 0 {
                smallest_option = total_dir_size;
            } else {
                if total_dir_size < smallest_option {
                    smallest_option = total_dir_size;
                }
            }
        }
    }
    
    smallest_option
}