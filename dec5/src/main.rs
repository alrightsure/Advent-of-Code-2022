use std::fs;

struct InstructionLine {
    quantity: usize,
    from_stack_index: usize,
    to_stack_index: usize
}

fn main() {
    let lines = get_lines_from_file();
    let crate_stacks = get_initial_stacks(&lines);
    let instructions = get_instruction_lines(&lines);

    let answer1 = solution1(crate_stacks.clone(), &instructions);
    let answer2 = solution2(crate_stacks.clone(), &instructions);

    println!("The solution to question 1 is {}", answer1);
    println!("The solution to question 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(mut crate_stacks: Vec<Vec<char>>, instructions: &Vec<InstructionLine>) -> String {
    for instruction in instructions {
        let mut i = 0;
        while i < instruction.quantity {
            let popped_value = crate_stacks[instruction.from_stack_index].pop().unwrap();
            crate_stacks[instruction.to_stack_index].push(popped_value);
            i += 1;
        }
    }

    get_answer_string(crate_stacks)
}

fn solution2(mut crate_stacks: Vec<Vec<char>>, instructions: &Vec<InstructionLine>) -> String {
    for instruction in instructions {
        let mut popped_values:Vec<char> = Vec::new();

        for _i in 0..instruction.quantity {
            popped_values.push(crate_stacks[instruction.from_stack_index].pop().unwrap());
        }

        popped_values = popped_values.iter().copied().rev().collect();
        
        for popped_value in popped_values {
            crate_stacks[instruction.to_stack_index].push(popped_value);
        }
    }

    get_answer_string(crate_stacks)
}

fn get_initial_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut crate_stacks: Vec<Vec<char>> = Vec::with_capacity(9);

    for _i in 0..9 {
        crate_stacks.push(Vec::new());
    }

    const SPACES_OF_SEPARATION: usize = 4;
    for i in 0..8 {
        for stack_count in 0..9 {
            let current_crate = lines[i].as_bytes()[(stack_count * SPACES_OF_SEPARATION) + 1] as char;

            if !current_crate.is_whitespace() {
                crate_stacks[stack_count].push(current_crate);
            }
        }
    }

    for i in 0..9 {
        crate_stacks[i] = crate_stacks[i].iter().copied().rev().collect();
    }

    print!("Stacks: {:?}", crate_stacks);

    crate_stacks
}

fn get_instruction_lines(lines: &Vec<String>) -> Vec<InstructionLine> {
    let mut instruction_lines: Vec<InstructionLine> = Vec::new();
    let mut in_instructions = false;
    for line in lines {
        for c in line.chars() {
            if c.is_numeric() {
                in_instructions = true;
            }
        }

        if in_instructions && line.starts_with("move") {
            let line = line.replace("move", "");
            let line = line.replace("from", "");
            let line = line.replace("to", "");
            let instruction_vec: Vec<&str> = line.split("  ").collect();
            
            instruction_lines.push(InstructionLine {
                quantity: instruction_vec[0].trim().parse::<usize>().unwrap(),
                from_stack_index: instruction_vec[1].parse::<usize>().unwrap() - 1,
                to_stack_index: instruction_vec[2].parse::<usize>().unwrap() - 1,
            });
        }
    }

    instruction_lines
}

fn get_answer_string(crate_stacks: Vec<Vec<char>>) -> String {
    let mut answer = String::from("");
    for mut stack in crate_stacks {
        answer.push(stack.pop().unwrap());
    }

    answer
}