use std::collections::HashMap;
use std::fs;

#[derive(PartialEq)]
enum PossibleMoves {
    Rock,
    Paper,
    Scissors,
    NONE,
}

impl Copy for PossibleMoves {}

impl Clone for PossibleMoves {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(PartialEq)]
enum PossibleGoals {
    Win,
    Lose,
    Draw,
    NONE,
}

impl Copy for PossibleGoals {}

impl Clone for PossibleGoals {
    fn clone(&self) -> Self {
        *self
    }
}

struct GameRound {
    opponent_move: PossibleMoves,
    player_goal: PossibleGoals,
}

fn main() {
    const PATH_TO_FILE: &str = "./input.txt";

    let lines = get_lines_from_file(PATH_TO_FILE);
    let rounds = convert_lines_to_plays(lines);
    let player_score = calculate_player_score(rounds);

    println!("Total Player Score: {}", player_score);
}

fn get_lines_from_file(path_to_file: &str) -> Vec<String> {
    let contents = fs::read_to_string(path_to_file).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn convert_lines_to_plays(lines: Vec<String>) -> Vec<GameRound> {
    let move_rules = HashMap::from([
        ("A", PossibleMoves::Rock),
        ("B", PossibleMoves::Paper),
        ("C", PossibleMoves::Scissors),
    ]);
    let goal_rules = HashMap::from([
        ("X", PossibleGoals::Lose),
        ("Y", PossibleGoals::Draw),
        ("Z", PossibleGoals::Win),
    ]);
    let mut rounds: Vec<GameRound> = Vec::new();

    for line in lines {
        let split = line.split(" ");
        let split_line = split.collect::<Vec<&str>>();
        let mut opponent_move = &PossibleMoves::NONE;
        let mut player_goal = &PossibleGoals::NONE;

        match move_rules.get(split_line[0]) {
            Some(value) => opponent_move = value,
            _ => println!("Faled to find move in rules"),
        }

        match goal_rules.get(split_line[1]) {
            Some(value) => player_goal = value,
            _ => println!("Failed to find move in rules"),
        }

        rounds.push(GameRound {
            opponent_move: opponent_move.clone(),
            player_goal: player_goal.clone(),
        });
    }

    rounds
}

fn calculate_player_score(rounds: Vec<GameRound>) -> i32 {
    const DRAW_POINTS: i32 = 3;
    const WIN_POINTS: i32 = 6;

    let mut player_score = 0;
    for round in rounds {
        if round.player_goal != PossibleGoals::Lose {
            if round.player_goal == PossibleGoals::Draw {
                player_score += DRAW_POINTS;

                player_score += match round.opponent_move {
                    PossibleMoves::Rock => 1,
                    PossibleMoves::Paper => 2,
                    PossibleMoves::Scissors => 3,
                    PossibleMoves::NONE => 0,
                };
            }

            if round.player_goal == PossibleGoals::Win {
                player_score += WIN_POINTS;

                player_score += match round.opponent_move {
                    PossibleMoves::Rock => 2,
                    PossibleMoves::Paper => 3,
                    PossibleMoves::Scissors => 1,
                    PossibleMoves::NONE => 0,
                };
            }
        } else {
            player_score += match round.opponent_move {
                PossibleMoves::Rock => 3,
                PossibleMoves::Paper => 1,
                PossibleMoves::Scissors => 2,
                PossibleMoves::NONE => 0,
            };
        }
    }

    player_score
}
