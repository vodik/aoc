#![feature(drain_filter)]

use std::{
    fs,
    time::{Duration, Instant},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn time<F, R>(f: F) -> (Duration, R)
where
    F: Fn() -> R,
{
    let now = Instant::now();
    let result = f();
    (now.elapsed(), result)
}

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("./data/day01.txt").unwrap();
    let input = day01::parse_input(&input);
    println!(":: day1");
    println!("part1: {:?}", time(|| day01::part1(&input)));
    println!("part2: {:?}", time(|| day01::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day02.txt").unwrap();
    let input = day02::parse_input(&input);
    println!(":: day2");
    println!("part1: {:?}", time(|| day02::part1(&input)));
    println!("part2: {:?}", time(|| day02::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day03.txt").unwrap();
    let input = day03::parse_input(&input);
    println!(":: day3");
    println!("part1: {:?}", time(|| day03::part1(&input)));
    println!("part2: {:?}", time(|| day03::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day04.txt").unwrap();
    let input = day04::parse_input(&input);
    println!(":: day4");
    println!("part1: {:?}", time(|| day04::part1(&input)));
    println!("part2: {:?}", time(|| day04::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day05.txt").unwrap();
    let input = day05::parse_input(&input);
    println!(":: day5");
    println!("part1: {:?}", time(|| day05::part1(&input)));
    println!("part2: {:?}", time(|| day05::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day06.txt").unwrap();
    let input = day06::parse_input(&input);
    println!(":: day6");
    println!("part1: {:?}", time(|| day06::part1(&input)));
    println!("part2: {:?}", time(|| day06::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day07.txt").unwrap();
    let input = day07::parse_input(&input);
    println!(":: day7");
    println!("part1: {:?}", time(|| day07::part1(&input)));
    println!("part2: {:?}", time(|| day07::part2(&input)));
    println!();

    println!("total: {:?}", now.elapsed());
}
