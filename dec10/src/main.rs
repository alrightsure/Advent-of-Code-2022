use std::fs;

fn get_lines_from_file() -> Vec<String> {
    const PATH_TO_FILE: &str = "./test.txt";
    let contents = fs::read_to_string(PATH_TO_FILE).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn main() {
    let lines = get_lines_from_file();
}

