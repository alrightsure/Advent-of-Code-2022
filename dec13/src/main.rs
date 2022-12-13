use std::fs;

#[derive(Debug, Clone)]
struct Pair {
    packet1: Vec<Vec<i32>>,
    packet2: Vec<Vec<i32>>,
}

fn main() {
    let pairs = get_pairs_from_file();

    //let answer1 = solution1(pairs.clone());

    println!("{:?}", pairs);
}

fn get_pairs_from_file() -> Vec<Pair> {
    const PATH_TO_FILE: &str = "./test.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let mut pairs: Vec<Pair> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let mut line1 = lines[i].split(",").map(|s| s.to_string()).collect();
        let mut line2 = lines[i + 1].split(",").map(|s| s.to_string()).collect();

        let packet1 = convert_line_to_packet(&mut line1);
        let packet2 = convert_line_to_packet(&mut line2);

        pairs.push(Pair { packet1, packet2 });
        i += 3;
    }

    pairs
}

fn convert_line_to_packet(line: &mut Vec<String>) -> Vec<Vec<i32>> {
    let mut packet: Vec<Vec<i32>> = Vec::new();
    let mut data_index = 0;
    let line_length = line.len();

    // remove intiial brackets
    line[0].remove(0);
    line[line_length - 1].pop();

    for i in 0..line.len() {
        if i == 2 {
            println!("{:?}", line);
        }
        if line[i].contains("[") && line[i].contains("]") {
            let mut new_string = "".to_string();

            let mut value_saved = false;
            for (index, c) in line[i].chars().enumerate() {
                if c == '[' {
                    data_index += 1;
                } else if c == ']' {
                    data_index -= 1;
                } else {
                    new_string.push(c);
                }

                if value_saved == false && index + 1 < line[i].len() && line[i].chars().nth(index + 1).unwrap() == ']' {
                    while packet.len() <= data_index {
                        packet.push(Vec::new());
                    }

                    if new_string.is_empty() {
                        continue;
                    }
                    
                    packet[data_index].push(new_string.parse::<i32>().unwrap());
                    value_saved = true;
                }
            }
        } else {
            if line[i].contains("[") {
                let mut new_string = "".to_string();

                for c in line[i].chars() {
                    if c == '[' {
                        data_index += 1;
                    } else {
                        new_string.push(c);
                    }
                }

                line[i] = new_string;
            }
            if line[i].contains("]") {
                let mut new_string = "".to_string();

                for c in line[i].chars() {
                    if c == ']' {
                        data_index -= 1;
                    } else {
                        new_string.push(c);
                    }
                }

                line[i] = new_string;
            }

            while packet.len() <= data_index {
                packet.push(Vec::new());
            }

            if line[i].is_empty() {
                continue;
            }

            packet[data_index].push(line[i].parse::<i32>().unwrap());
        }
    }

    packet
}

fn solution1(pairs: Vec<Pair>) -> i32 {
    let mut sum_of_pairs_in_order = 0;
    for pair in pairs {
        let mut packet1 = pair.packet1;
        let mut packet2 = pair.packet2;

        if is_in_right_order(packet1, packet2) {
            sum_of_pairs_in_order += 1;
        }
    }

    sum_of_pairs_in_order
}

fn is_in_right_order(packet1: Vec<Vec<i32>>, packet2: Vec<Vec<i32>>) -> bool {
    println!("{:?}", packet1);
    println!("{:?}", packet2);

    false
}
