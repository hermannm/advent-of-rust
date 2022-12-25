use advent_of_rust::day_25;

const INPUT: &str = include_str!("input.txt");

#[test]
fn puzzle() {
    let result = day_25::solve_puzzle(INPUT.trim_end());
    assert_eq!(result, Ok(String::from("2=-1=0")))
}
