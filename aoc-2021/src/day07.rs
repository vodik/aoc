pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn linear_fuel_consumption(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn triangle_fuel_consumption(a: u32, b: u32) -> u32 {
    let distance = linear_fuel_consumption(a, b);
    distance * (distance + 1) / 2
}

fn project_fuel_consumption<F>(input: &[u32], position: u32, f: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    input.iter().map(|&x| f(x, position)).sum()
}

pub fn part1(input: &[u32]) -> u32 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    (min..max)
        .map(|position| project_fuel_consumption(input, position, linear_fuel_consumption))
        .min()
        .unwrap()
}

pub fn part2(input: &[u32]) -> u32 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    (min..max)
        .map(|position| project_fuel_consumption(input, position, triangle_fuel_consumption))
        .min()
        .unwrap()
}
