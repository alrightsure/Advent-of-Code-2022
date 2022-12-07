use std::fs;

#[derive(Debug)]
struct Directory {
    name: String,
    parent_name: String,
    size: i32,
}

fn main() {
    let lines = get_lines_from_file();
    let file_system = get_file_system_from_lines(lines);

    println!("{:?}", file_system);

    let answer1 = solution1(&file_system);

    println!("The answer to question 1 is {}", answer1);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn get_file_system_from_lines(lines: Vec<String>) -> Vec<Directory> {
    let mut file_system: Vec<Directory> = vec![Directory {name: "".to_string(), parent_name: "_".to_string(), size: 0}];
    let mut current_directory_index: usize = 0;

    for line in lines {
        if line.starts_with("$") {
            if line.contains("cd") {
                let line_split: Vec<&str> = line.split(" ").collect();
                let destination = line_split[2];

                if destination == ".." {
                    if current_directory_index > 0 {
                        current_directory_index = file_system.iter().position(|dir| dir.name == file_system[current_directory_index].parent_name).unwrap();
                    }
                } else {
                    let mut parent_name = "";
                    if current_directory_index > 0 {
                        parent_name = &file_system[current_directory_index].name;
                    }

                    if !file_system.iter().any(|d| d.name == destination) {
                        file_system.push(Directory { name: destination.to_string(), parent_name: parent_name.to_string(), size: 0 });
                    }

                    current_directory_index = file_system.iter().position(|dir| dir.name == destination).unwrap();
                }
            }
        } else {
            if line.as_bytes()[0].is_ascii_digit() {
                let line_split: Vec<&str> = line.split(" ").collect();
                let file_size = line_split[0];

                file_system[current_directory_index].size += file_size.parse::<i32>().unwrap();
            }
        }
    }

    file_system
}

fn solution1(file_system: &Vec<Directory>) -> i32 {
    let mut sum = 0;
    
    for dir in file_system {
        let total_dir_size = get_directory_size(file_system, dir.name.to_string());

        if total_dir_size <= 100000 {
            sum += total_dir_size;
        }
    }

    sum
}

fn get_directory_size(file_system: &Vec<Directory>, dir_name: String) -> i32 {
    let mut sum_size = file_system.iter().find(|d| d.name == dir_name).unwrap().size;
    
    for dir in file_system {
        if dir.parent_name == dir_name {
            sum_size += get_directory_size(file_system, dir.name.to_string());
        }
    }

    sum_size
}