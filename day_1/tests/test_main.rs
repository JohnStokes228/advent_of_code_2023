#[cfg(test)]
mod tests {
    use day_1_project::funcs::pointers;
    use day_1_project::funcs::replace_string_number;

    #[test]
    fn part_1_example_1() {
        assert_eq!(pointers("1abc2"), 12);
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(pointers("pqr3stu8vwx"), 38);
    }

    #[test]
    fn part_1_example_3() {
        assert_eq!(pointers("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(pointers("treb7uchet"), 77);
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(pointers(&replace_string_number("two1nine")), 29);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(pointers(&replace_string_number("eightwothree")), 83);
    }

    #[test]
    fn part_2_example_3() {
        assert_eq!(pointers(&replace_string_number("abcone2threexyz")), 13);
    }

    #[test]
    fn part_2_example_4() {
        assert_eq!(pointers(&replace_string_number("xtwone3four")), 24);
    }

    #[test]
    fn part_2_example_5() {
        assert_eq!(pointers(&replace_string_number("4nineeightseven2")), 42);
    }

    #[test]
    fn part_2_example_6() {
        assert_eq!(pointers(&replace_string_number("zoneight234")), 14);
    }

    #[test]
    fn part_2_example_7() {
        assert_eq!(pointers(&replace_string_number("7pqrstsixteen")), 76);
    }

    #[test]
    fn part_2_example_8() { // this one i added as it isnt in the actual provided examples :(
        assert_eq!(pointers(&replace_string_number("twone")), 21);
    }

}