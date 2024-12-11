#![feature(portable_simd)]
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day11;

fn time<F, R>(f: F) -> (Duration, R)
where
    F: Fn() -> R,
{
    let now = Instant::now();
    let result = f();
    (now.elapsed(), result)
}

macro_rules! day {
    ($mod:ident) => {
        let input = include_str!(concat!("../data/", stringify!($mod), ".txt"));
        let (dur, input) = time(|| $mod::parse_input(&input));
        println!(":: {}", stringify!($mod));
        println!("generator: {:?}", dur);
        println!("part1: {:?}", time(|| $mod::part1(&input)));
        println!("part2: {:?}", time(|| $mod::part2(&input)));
        println!();
    };
}

fn main() {
    let now = Instant::now();

    day!(day01);
    day!(day02);
    day!(day03);
    day!(day11);

    println!("total: {:?}", now.elapsed());
}
