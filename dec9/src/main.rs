use std::fs;

#[derive(Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

fn main() {
    let lines = get_lines_from_file();

    let answer1 = solution1(&lines);
    let answer2 = solution2(&lines);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn solution1(lines: &Vec<String>) -> usize {
    let mut all_tail_coords: Vec<Coordinate> = vec![];
    let mut head_coords = Coordinate {x: 0, y: 0};
    let mut tail_coords = Coordinate {x: 0, y: 0};

    for line in lines {
        let line_split: Vec<&str> = line.split(" ").collect();
        let movement_amount: i32 = line_split[1].parse().unwrap();

        for _n in 0..movement_amount {
            match line_split[0] {
                "R" => head_coords.x += 1,
                "L" => head_coords.x -= 1,
                "U" => head_coords.y += 1,
                "D" => head_coords.y -= 1,
                _ => println!("UNKNOWN CHARACTER")
            }
            all_tail_coords.push(move_tail(&head_coords, &mut tail_coords));
        }
    }

    let mut unique_tail_coords: Vec<Coordinate> = vec![];
    for coord in all_tail_coords {
        let mut true_match = false;
        for u_coord in &unique_tail_coords {
            if coord.x == u_coord.x && coord.y == u_coord.y {
                true_match = true;
            }
        }

        if !true_match {
            unique_tail_coords.push(coord);
        }
    }

    unique_tail_coords.len()
}

fn solution2(lines: &Vec<String>) -> usize {
    let mut all_tail_coords: Vec<Coordinate> = vec![];
    let mut snake: Vec<Coordinate> = vec![];

    for _n in 0..10 {
        snake.push(Coordinate { x: 0, y: 0 });
    }

    for line in lines {
        let line_split: Vec<&str> = line.split(" ").collect();
        let movement_amount: i32 = line_split[1].parse().unwrap();

        for _n in 0..movement_amount {
            match line_split[0] {
                "R" => snake[0].x += 1,
                "L" => snake[0].x -= 1,
                "U" => snake[0].y += 1,
                "D" => snake[0].y -= 1,
                _ => println!("UNKNOWN CHARACTER")
            }

            for n in 0..10 {
                if n > 0 {
                    let output = move_tail(&snake[n - 1].clone(), &mut snake[n]);
                    if n == 9 {
                        all_tail_coords.push(output);
                    }
                }
            }
        }
    }

    let mut unique_tail_coords: Vec<Coordinate> = vec![];
    for coord in all_tail_coords {
        let mut true_match = false;
        for u_coord in &unique_tail_coords {
            if coord.x == u_coord.x && coord.y == u_coord.y {
                true_match = true;
            }
        }

        if !true_match {
            unique_tail_coords.push(coord);
        }
    }

    unique_tail_coords.len()
}

fn move_tail(head_coords: &Coordinate, tail_coords: &mut Coordinate) -> Coordinate {
    if head_coords.x != tail_coords.x && head_coords.y != tail_coords.y && (i32::abs(head_coords.x - tail_coords.x) > 1 || i32::abs(head_coords.y - tail_coords.y) > 1) {
        if head_coords.x > tail_coords.x {
            tail_coords.x += 1;
        } else {
            tail_coords.x -= 1;
        }

        if head_coords.y > tail_coords.y {
            tail_coords.y += 1;
        } else {
            tail_coords.y -= 1;
        }
    } else {    
        if i32::abs(head_coords.x - tail_coords.x) > 1 {
            if head_coords.x > tail_coords.x {
                tail_coords.x += 1;
            } else {
                tail_coords.x -= 1;
            }
        }

        if i32::abs(head_coords.y - tail_coords.y) > 1 {
            if head_coords.y > tail_coords.y {
                tail_coords.y += 1;
            } else {
                tail_coords.y -= 1;
            }
        }
    }

    tail_coords.clone()
}