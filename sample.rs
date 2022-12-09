fn get_num_of_unique_coords(lines: &Vec<String>) -> usize {
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

    let unique_coords: Vec<String> = all_tail_coords.into_iter()
    .map(|c| c.x.to_string() + &c.y.to_string())
    .collect::<HashSet<String>>()
    .into_iter()
    .collect();

    unique_coords.len()
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
