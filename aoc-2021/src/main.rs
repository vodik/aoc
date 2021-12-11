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
mod day08;
mod day09;
mod day10;
mod day11;

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

    let input = fs::read_to_string("./data/day08.txt").unwrap();
    let input = day08::parse_input(&input);
    println!(":: day8");
    println!("part1: {:?}", time(|| day08::part1(&input)));
    println!("part2: {:?}", time(|| day08::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day09.txt").unwrap();
    let input = day09::parse_input(&input);
    println!(":: day9");
    println!("part1: {:?}", time(|| day09::part1(&input)));
    println!("part2: {:?}", time(|| day09::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day10.txt").unwrap();
    let input = day10::parse_input(&input);
    println!(":: day10");
    println!("part1: {:?}", time(|| day10::part1(&input)));
    println!("part2: {:?}", time(|| day10::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day11.txt").unwrap();
    let input = day11::parse_input(&input);
    println!(":: day11");
    println!("part1: {:?}", time(|| day11::part1(&input)));
    println!("part2: {:?}", time(|| day11::part2(&input)));
    println!();

    println!("total: {:?}", now.elapsed());
}
