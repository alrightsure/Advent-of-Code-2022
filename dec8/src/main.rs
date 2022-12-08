use std::fs;

#[derive(Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

fn main() {
    let lines = get_lines_from_file();
    let trees = get_trees_from_lines(lines);

    let answer1 = get_visible_trees(&trees);
    let answer2 = get_best_scenic_score(&trees);

    println!("The answer to question 1 is {}", answer1);
    println!("The answer to question 2 is {}", answer2);
}

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./input.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn get_trees_from_lines(lines: Vec<String>) -> Vec<Tree> {
    let mut trees: Vec<Tree> = vec![];
    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let height = line.chars().nth(x).unwrap().to_digit(10).unwrap();
            trees.push(Tree { x, y, height });
        }
    }

    trees
}

fn get_visible_trees(trees: &Vec<Tree>) -> i32 {
    let mut visible_trees = 0;
    for tree in trees {
        if check_tree_visible(tree, trees) {
            visible_trees += 1;
        }
    }

    visible_trees
}

fn get_best_scenic_score(trees: &Vec<Tree>) -> i32 {
    let mut scenic_scores: Vec<i32> = vec![];

    for tree in trees {
        let below_trees: Vec<&Tree> = trees
            .iter()
            .filter(|t| t.x == tree.x && t.y > tree.y)
            .collect();
        let above_trees: Vec<&Tree> = trees
            .iter()
            .filter(|t| t.x == tree.x && t.y < tree.y)
            .rev().collect();
        let left_trees: Vec<&Tree> = trees
            .iter()
            .filter(|t| t.y == tree.y && t.x < tree.x)
            .rev().collect();
        let right_trees: Vec<&Tree> = trees
            .iter()
            .filter(|t| t.y == tree.y && t.x > tree.x)
            .collect();

        let mut score = get_scenic_score_direction(tree, below_trees);
        score *= get_scenic_score_direction(tree, above_trees);
        score *= get_scenic_score_direction(tree, left_trees);
        score *= get_scenic_score_direction(tree, right_trees);
        
        scenic_scores.push(score);
    }

    *scenic_scores.iter().max().unwrap()
}

fn check_tree_visible(check_tree: &Tree, trees: &Vec<Tree>) -> bool {
    let below_trees: Vec<&Tree> = trees
        .iter()
        .filter(|t| t.x == check_tree.x && t.y > check_tree.y)
        .collect();
    let above_trees: Vec<&Tree> = trees
        .iter()
        .filter(|t| t.x == check_tree.x && t.y < check_tree.y)
        .collect();
    let left_trees: Vec<&Tree> = trees
        .iter()
        .filter(|t| t.y == check_tree.y && t.x < check_tree.x)
        .collect();
    let right_trees: Vec<&Tree> = trees
        .iter()
        .filter(|t| t.y == check_tree.y && t.x > check_tree.x)
        .collect();

    if below_trees.len() == 0
        || above_trees.len() == 0
        || left_trees.len() == 0
        || right_trees.len() == 0
    {
        return true;
    }

    if check_visibility_direction(check_tree, below_trees) {
        return true;
    }
    if check_visibility_direction(check_tree, above_trees) {
        return true;
    }
    if check_visibility_direction(check_tree, left_trees) {
        return true;
    }

    if check_visibility_direction(check_tree, right_trees) {
        return true;
    }

    false
}

fn check_visibility_direction(check_tree: &Tree, direction_trees: Vec<&Tree>) -> bool {
    let mut visible = true;

    for tree in direction_trees {
        if tree.height >= check_tree.height {
            visible = false;
        }
    }

    visible
}

fn get_scenic_score_direction(check_tree: &Tree, direction_trees: Vec<&Tree>) -> i32 {
    let mut score = 0;

    for tree in direction_trees {
        if tree.height < check_tree.height {
            score += 1;
        }

        if tree.height >= check_tree.height {
            score += 1;
            break;
        }
    }

    if check_tree.x == 2 && check_tree.y == 1 {
        println!("{}", score);
    }

    score
}