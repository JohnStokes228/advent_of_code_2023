use regex::Regex;

/// return a boolean if the game represents a possible occurance of cubes
/// else, return false
/// sorry for how ugly this function is i really didnt want to have to resort to splitting
/// but regex alone i just couldnt land - didnt want to get too stuck on this otherwise fairly
/// basic task
fn is_valid(input_string: &str, substring: &str, max_balls: i32) -> bool {
    let re = Regex::new(&format!(r#"(\d+)\s*{}"#, substring)).unwrap();
    let cube_count: Vec<i32> = re.find_iter(input_string).map(|m| m.as_str().split_whitespace().next().unwrap().parse().unwrap()).collect();
    let leq_max_balls: bool = cube_count.iter().all(|&value| value <= max_balls);

    return leq_max_balls
}

/// convert false values to 0 so they dont affect the sum
/// otherwise, extract the game number and return that cast to i32
fn bool_to_game_numb(input_string: &str, validity: bool) -> i32 {
    if validity == false {
        return 0;
    }
    else {
        let re = Regex::new(r"\d+").unwrap();
    
        if let Some(captured) = re.find(input_string) {
            let result: i32 = captured.as_str().parse().unwrap();
            return result;
        } else {
            println!("No integer found in the string");
            return 0;
        }
    }
}

/// convert game string to valid int output
pub fn game_count(input_string: &str) -> i32 {
    let valid_blue = is_valid(input_string, "blue", 14);
    let valid_green = is_valid(input_string, "green", 13);
    let valid_red = is_valid(input_string, "red", 12);

    let validity = valid_blue && valid_green && valid_red;

    return bool_to_game_numb(input_string, validity)
}

pub fn game_power(input_string: &str) -> i32 {
    return 0;
}