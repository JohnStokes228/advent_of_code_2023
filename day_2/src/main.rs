use std::fs::File;
use std::io::{self, BufRead};
use day_2::funcs::game_count;

fn main() {
    let file_path = "src/inputs.txt";
    let file = File::open(&file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut results = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result returned by lines()
        let result = game_count(&line);

        results.push(result);
    }

    let sum: i32 = results.iter().sum();

    println!("Sum: {}", sum);
}