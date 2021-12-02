use std::fs;

mod day01;
mod day02;

fn main() {
    let input = fs::read_to_string("./data/day01.txt").unwrap();
    let input = day01::parse_input(&input);
    println!("a: {}", day01::part1(&input));
    println!("b: {}", day01::part2(&input));

    println!();

    let input = fs::read_to_string("./data/day02.txt").unwrap();
    let input = day02::parse_input(&input);
    println!("a: {}", day02::part1(&input));
    println!("b: {}", day02::part2(&input));
}
