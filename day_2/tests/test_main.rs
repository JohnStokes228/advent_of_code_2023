#[cfg(test)]
mod tests {
    use day_2::funcs::is_valid;

    #[test]
    fn part_1_example_1() {
        assert_eq!(is_valid("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 1);
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(is_valid("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 2);
    }

    #[test]
    fn part_1_example_3() { // itll return 0 if its not valid so the sum isnt affected and the type is consistent
        assert_eq!(is_valid("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 0);
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(is_valid("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 0);
    }

    #[test]
    fn part_1_example_5() {
        assert_eq!(is_valid("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 5);
    }
}