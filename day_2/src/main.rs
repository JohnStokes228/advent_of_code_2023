use std::fs::File;
use std::io::{self, BufRead};
use day_2::funcs::game_count;
use day_2::funcs::game_power;

fn main() {
    let file_path = "src/inputs.txt";
    let file = File::open(&file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut results_sum = Vec::new();
    let mut results_power = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result returned by lines()
        let result = game_count(&line);
        let power = game_power(&line);

        results_sum.push(result);
        results_power.push(power);
    }

    let sum: i32 = results_sum.iter().sum();
    let power_sum: i32 = results_power.iter().sum();

    println!("Sum: {}", sum);
    println!("Sum power: {}", power_sum);


}