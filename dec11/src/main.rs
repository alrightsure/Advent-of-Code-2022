use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i128>,
    operation_char: char,
    operation_num: Option<i128>,
    test_num: i128,
    true_monkey: usize,
    false_monkey: usize,
    inpsection_count: i128
}

impl Monkey {
    fn inspect(&mut self, item_index: usize) {
        match self.operation_char {
            '+' => self.items[item_index] = if self.operation_num.is_some() { &self.items[item_index] + self.operation_num.as_ref().unwrap() } else {&self.items[item_index] + &self.items[item_index]},
            '-' => self.items[item_index] = if self.operation_num.is_some() { &self.items[item_index] - self.operation_num.as_ref().unwrap() } else {&self.items[item_index] - &self.items[item_index]},
            '*' => self.items[item_index] = if self.operation_num.is_some() { &self.items[item_index] * self.operation_num.as_ref().unwrap() } else {&self.items[item_index] * &self.items[item_index]},
            '/' => self.items[item_index] = if self.operation_num.is_some() { &self.items[item_index] / self.operation_num.as_ref().unwrap() } else {&self.items[item_index] / &self.items[item_index]},
            _ => println!("UNKOWN OPERATION CHAR PRESENT")
        }
        self.inpsection_count += 1;
    }

    fn exeprience_relief(&mut self, item_index: usize) {
        self.items[item_index] = &self.items[item_index] / 3;
    }

    fn solution2_relief(&mut self, item_index: usize, divisor: i128) {
        self.items[item_index] = &self.items[item_index] % divisor;
    }
}

fn main() {
    let monkeys = get_monkeys_from_file();

    let answer1 = solution1(monkeys.clone());
    let answer2 = solution2(monkeys);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is {}", answer2);
}

fn get_monkeys_from_file() -> Vec<Monkey> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    
    let mut current_monkey: usize = 0;
    let mut all_monkeys: Vec<Monkey> = vec![];
    for mut line in lines {
        line = line.trim().to_string();

        if line.contains("Monkey") {
            let line_vec: Vec<&str> = line.split(" ").into_iter().collect();
            let monkey_info: Vec<&str> = line_vec[1].split(":").into_iter().collect();
            current_monkey = monkey_info[0].parse::<usize>().unwrap();
            all_monkeys.push(Monkey {items: vec![], operation_char: '_', operation_num: None, test_num: 0, true_monkey: 0, false_monkey: 0, inpsection_count: 0});
        }

        if line.contains("Starting items") {
            let line_vec: Vec<&str> = line.split(":").into_iter().collect();
            let item_numbers: Vec<&str> = line_vec[1].split(",").into_iter().collect();
            
            for mut item in item_numbers {
                item = item.trim();
                all_monkeys[current_monkey].items.push(item.parse::<i128>().unwrap());
            }
        }

        if line.contains("Operation") {
            let line_vec: Vec<&str> = line.split(" ").into_iter().collect();
            all_monkeys[current_monkey].operation_char = line_vec[4].parse::<char>().unwrap();

            let num = line_vec[5].parse::<i128>();
            all_monkeys[current_monkey].operation_num = num.ok();
        }

        if line.contains("Test") {
            let line_vec: Vec<&str> = line.split(" ").into_iter().collect();

            all_monkeys[current_monkey].test_num = line_vec[3].parse::<i128>().unwrap();
        }

        if line.contains("true") {
            let line_vec: Vec<&str> = line.split(" ").into_iter().collect();

            all_monkeys[current_monkey].true_monkey = line_vec[5].parse::<usize>().unwrap();
        }

        if line.contains("false") {
            let line_vec: Vec<&str> = line.split(" ").into_iter().collect();

            all_monkeys[current_monkey].false_monkey = line_vec[5].parse::<usize>().unwrap();
        }
    }

    all_monkeys
}

fn solution1(mut monkeys: Vec<Monkey>) -> i128 {
    const TOTAL_ROUNDS: i32 = 20;

    for _round_num in 0..TOTAL_ROUNDS {
        for monkey_index in 0..monkeys.len() {
            for _item_index in 0..monkeys[monkey_index].items.len() {
                monkeys[monkey_index].inspect(0);
                monkeys[monkey_index].exeprience_relief(0);

                let item_to_throw = monkeys[monkey_index].items[0].clone();
                let item_num = &monkeys[monkey_index].items[0];
                let test_num = &monkeys[monkey_index].test_num;
                if item_num % test_num == 0 {
                    let true_index = monkeys[monkey_index].true_monkey;
                    monkeys[true_index].items.push(item_to_throw);
                } else {
                    let false_index = monkeys[monkey_index].false_monkey;
                    monkeys[false_index].items.push(item_to_throw);
                }

                monkeys[monkey_index].items.remove(0);
            }
        }
    }

    let mut inspection_counts: Vec<i128> = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inpsection_count);
    }
    inspection_counts.sort();
    inspection_counts.reverse();

    inspection_counts[0] * inspection_counts[1]
}

fn solution2(mut monkeys: Vec<Monkey>) -> i128 {
    const TOTAL_ROUNDS: i32 = 10000;
    let mut divisor = 1;
    let mut monkey_length = 0;

    for monkey in monkeys.clone() {
        monkey_length += 1;
        divisor *= monkey.test_num;
    }

    for _round_num in 0..TOTAL_ROUNDS {
        println!("ROUND: {}", _round_num);
        for monkey_index in 0..monkey_length {
            for _item_index in 0..monkeys[monkey_index].items.len() {
                monkeys[monkey_index].inspect(0);
                monkeys[monkey_index].solution2_relief(0, divisor);

                let item_to_throw = monkeys[monkey_index].items[0].clone();
                let item_num = &monkeys[monkey_index].items[0];
                let test_num = &monkeys[monkey_index].test_num;

                if item_num % test_num == 0 {
                    let true_index = monkeys[monkey_index].true_monkey;
                    monkeys[true_index].items.push(item_to_throw);
                } else {
                    let false_index = monkeys[monkey_index].false_monkey;
                    monkeys[false_index].items.push(item_to_throw);
                }

                monkeys[monkey_index].items.remove(0);
            }
        }
    }

    let mut inspection_counts: Vec<i128> = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inpsection_count);
    }
    inspection_counts.sort();
    inspection_counts.reverse();

    inspection_counts[0] * inspection_counts[1]
}
