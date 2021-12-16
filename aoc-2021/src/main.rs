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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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

    let input = fs::read_to_string("./data/day12.txt").unwrap();
    let input = day12::parse_input(&input);
    println!(":: day12");
    println!("part1: {:?}", time(|| day12::part1(&input)));
    println!("part2: {:?}", time(|| day12::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day13.txt").unwrap();
    let input = day13::parse_input(&input);
    println!(":: day13");
    println!("part1: {:?}", time(|| day13::part1(&input)));
    println!("part2: {:?}", time(|| day13::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day14.txt").unwrap();
    let input = day14::parse_input(&input);
    println!(":: day14");
    println!("part1: {:?}", time(|| day14::part1(&input)));
    println!("part2: {:?}", time(|| day14::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day15.txt").unwrap();
    let input = day15::parse_input(&input);
    println!(":: day15");
    println!("part1: {:?}", time(|| day15::part1(&input)));
    println!("part2: {:?}", time(|| day15::part2(&input)));
    println!();

    let input = fs::read_to_string("./data/day16.txt").unwrap();
    let input = day16::parse_input(&input);
    println!(":: day16");
    println!("part1: {:?}", time(|| day16::part1(&input)));
    println!("part2: {:?}", time(|| day16::part2(&input)));
    println!();

    println!("total: {:?}", now.elapsed());
}
