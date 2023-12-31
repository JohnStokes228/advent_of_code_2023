use std::fs::File;
use std::io::{self, BufRead};
use day_1_project::funcs::pointers;
use day_1_project::funcs::replace_string_number;

/// runs only part 2 as part 1 is just a gimpy version of the same
fn main() {
    let file_path = "src/inputs.txt";
    let file = File::open(&file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut results = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result returned by lines()
        let result = pointers(&replace_string_number(&line));

        results.push(result);
    }

    let sum: i32 = results.iter().sum();

    println!("Sum: {}", sum);
}
