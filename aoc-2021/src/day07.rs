pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn linear_cost(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn triangle_cost(a: u32, b: u32) -> u32 {
    let distance = linear_cost(a, b);
    distance * (distance + 1) / 2
}

fn get_fuel_consumption<F>(input: &[u32], position: u32, f: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    input.iter().map(|&x| f(x, position)).sum()
}

fn median(arr: &[u32]) -> u32 {
    let mut data = arr.to_vec();
    data.sort_unstable();
    data[data.len() / 2]
}

fn mean(arr: &[u32]) -> u32 {
    let sum: u32 = arr.iter().sum();
    sum / u32::try_from(arr.len()).unwrap()
}

pub fn part1(input: &[u32]) -> u32 {
    let target = median(input);
    get_fuel_consumption(input, target, linear_cost)
}

pub fn part2(input: &[u32]) -> u32 {
    let target = mean(input);
    get_fuel_consumption(input, target, triangle_cost)
}
