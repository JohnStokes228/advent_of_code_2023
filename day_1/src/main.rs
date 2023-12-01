/// use a two pointer algorithm to extract the desired two digit number
/// code is likely super clunky as ive never written with rust before
/// so expect some bastardisation of rust x python here...
fn pointers(input_string: &str)  -> i32 {
    let (mut i, mut j) = (0, input_string.len() - 1);
    let mut condition_met: bool = false;

    while i <= j && !condition_met {
        let i_numeric: bool = input_string.chars().nth(i).unwrap().is_numeric();
        let j_numeric: bool = input_string.chars().nth(j).unwrap().is_numeric();

        if i_numeric && j_numeric {
            condition_met = true;
        } 
        if !i_numeric { // if i isnt numeric, iterate towards j
            i += 1;
        }
        if !j_numeric { // if j isnt numeric, iterate towards i
            j -= 1;
        }
    }

    if let (Some(char_i), Some(char_j)) = (input_string.chars().nth(i), input_string.chars().nth(j)) {
        let solution = format!("{}{}", char_i, char_j);
        println!("sol={}", solution);
        match solution.parse::<i32>() {
            Ok(number) => return number,
            Err(e) => {
                println!("Error parsing integer: {}", e);
                // Return some default value or handle the error accordingly
                return 0;
            }
        }
    } else {
        // Return some default value or handle the case where i or j are out of bounds
        return 0;
    }
}

fn main() {
    println!("Hello, world!");
}
