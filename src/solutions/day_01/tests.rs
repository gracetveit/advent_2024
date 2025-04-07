#[test]
fn test_example() {
    use super::solution::day_01_solution;
    assert_eq!(day_01_solution("inputs/day_01_test.txt"), 11);
}

#[test]
fn test_example_02() {
    use super::solution::part_two;
    assert_eq!(part_two("inputs/day_01_test.txt"), 31);
}
