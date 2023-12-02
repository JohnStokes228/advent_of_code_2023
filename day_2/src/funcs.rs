use regex::Regex;

/// return a boolean if the game represents a possible occurance of cubes
/// else, return false
fn is_valid(input_string: &str, substring: &str) -> bool {
    let re = Regex::new(&format!(r#"(?i){}\s*(\d+)"#, substring)).unwrap();
    let cube_count: Vec<&str> = re.find_iter(input_string).map(|m| m.as_str()).collect();
    // if any is > x return false, else return true
    return false
}

/// convert false values to 0 so they dont affect the sum
/// otherwise, extract the game number and return that cast to i32
fn bool_to_game_numb(input_string: &str, validity: bool) -> i32 {
    if validity == false {
        return 0;
    }
    else {
        return 1;
    }
}

/// convert game string to valid int output
pub fn game_count(input_string: &str) -> i32 {
    let valid_blue = is_valid(input_string, "blue");
    let valid_green = is_valid(input_string, "green");
    let valid_red = is_valid(input_string, "red");

    let validity = valid_blue && valid_green && valid_red;

    return bool_to_game_numb(input_string, validity)
}