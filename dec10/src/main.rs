use std::fs;

fn main() {
    let lines = get_lines_from_file();
    let answer1 = solution1(&lines);
    let answer2 = solution2(&lines);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is \n{}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(lines: &Vec<String>) -> i32 {
    let mut x_register = 1;
    let mut cycle_count = 0;
    let mut signal_stength = 0;

    for line in lines {
        let line_vec: Vec<&str> = line.split(" ").into_iter().collect();
        add_cycle_and_check_signal(&mut cycle_count, &mut signal_stength, &x_register);

         if line_vec[0] == "addx" {
            add_cycle_and_check_signal(&mut cycle_count, &mut signal_stength, &x_register);
            x_register += line_vec[1].parse::<i32>().unwrap();
        }
    }

    signal_stength
}

fn add_cycle_and_check_signal(cycle_count: &mut i32, signal_stength: &mut i32, current_x_value: &i32) {
    *cycle_count += 1;

    if *cycle_count == 20 || *cycle_count == 60 || *cycle_count == 100 || *cycle_count == 140 || *cycle_count == 180 || *cycle_count == 220 {
        *signal_stength += *cycle_count * current_x_value;
    }
}

fn solution2(lines: &Vec<String>) -> String {
    let mut x_register = 1;
    let mut cycle_count = 1;
    let mut console_output = "".to_string();

    for line in lines {
        let line_vec: Vec<&str> = line.split(" ").into_iter().collect();
        console_output.push_str(&add_cycle_and_get_pixel(&mut cycle_count, &x_register));

         if line_vec[0] == "addx" {
            console_output.push_str(&add_cycle_and_get_pixel(&mut cycle_count, &x_register));
            x_register += line_vec[1].parse::<i32>().unwrap();
        }
    }

    console_output
}

fn add_cycle_and_get_pixel(cycle_count: &mut i32, current_x_value: &i32) -> String {
    let mut output = "".to_string();
    let current_coords = get_coords_from_cycle_count(cycle_count);

    if &current_coords == current_x_value|| current_coords == *current_x_value - 1|| current_coords == *current_x_value + 1 {
        output.push_str("#");
    } else {
        output.push_str(".");
    }

    if current_coords == 39 {
        output.push_str("\n");
    }
    
    *cycle_count += 1;
    output
}

fn get_coords_from_cycle_count(cycle_count: &i32) -> i32 {
    if *cycle_count < 41 {
        return *cycle_count - 1;
    }

    if *cycle_count > 40 && *cycle_count < 81 {
        return cycle_count - 41;
    }

    if *cycle_count > 80 && *cycle_count < 121 {
        return cycle_count - 81;
    }

    if *cycle_count > 120 && *cycle_count < 161 {
        return  cycle_count - 121;
    }

    if *cycle_count > 160 && *cycle_count < 201 {
        return  cycle_count - 161;
    }

    return cycle_count - 201;
}