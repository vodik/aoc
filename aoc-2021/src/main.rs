use std::{
    fs,
    time::{Duration, Instant},
};

mod day01;
mod day02;
mod day03;

fn time<F, R>(f: F) -> (Duration, R)
where
    F: Fn() -> R,
{
    let now = Instant::now();
    let result = f();
    (now.elapsed(), result)
}

fn main() {
    let input = fs::read_to_string("./data/day01.txt").unwrap();
    let input = day01::parse_input(&input);
    println!("a: {:?}", time(|| day01::part1(&input)));
    println!("b: {:?}", time(|| day01::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day02.txt").unwrap();
    let input = day02::parse_input(&input);
    println!("a: {:?}", time(|| day02::part1(&input)));
    println!("b: {:?}", time(|| day02::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day03.txt").unwrap();
    let input = day03::parse_input(&input);
    println!("a: {:?}", time(|| day03::part1(&input)));
    println!("b: {:?}", time(|| day03::part2(&input)));
}
