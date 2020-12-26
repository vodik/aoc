use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    warnings: HashSet<String>,
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(alpha1, Into::into)(input)
}

fn parse_food(input: &str) -> IResult<&str, Food> {
    map(
        separated_pair(
            separated_list1(space1, identifier),
            tag(" "),
            delimited(
                tag("(contains "),
                separated_list1(tag(", "), identifier),
                tag(")"),
            ),
        ),
        |(ingredients, warnings)| Food {
            ingredients: ingredients.into_iter().collect(),
            warnings: warnings.into_iter().collect(),
        },
    )(input)
}

fn parse_foods(input: &str) -> IResult<&str, Vec<Food>> {
    separated_list1(tag("\n"), parse_food)(input)
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Result<Vec<Food>, Error<String>> {
    match all_consuming(parse_foods)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn find_all_warnings(data: &[Food]) -> HashSet<String> {
    data.iter()
        .flat_map(|food| food.warnings.iter())
        .cloned()
        .collect()
}

#[aoc(day21, part1)]
fn part1(data: &[Food]) -> usize {
    let bad_ingredients = find_all_warnings(data)
        .into_iter()
        .filter_map(|warning| {
            let mut it = data.iter().filter(|food| food.warnings.contains(&warning));

            it.next().map(|first| {
                let acc = first.ingredients.clone();
                it.fold(acc, |acc, food| {
                    acc.intersection(&food.ingredients).cloned().collect()
                })
            })
        })
        .flatten()
        .collect::<HashSet<String>>();

    data.iter()
        .flat_map(|food| {
            food.ingredients
                .iter()
                .filter(|&ingredient| !bad_ingredients.contains(ingredient))
        })
        .count()
}

#[aoc(day21, part2)]
fn part2(data: &[Food]) -> String {
    let mut known = find_all_warnings(data)
        .into_iter()
        .filter_map(|warning| {
            let mut it = data.iter().filter(|food| food.warnings.contains(&warning));

            it.next()
                .map(|first| {
                    let acc = first.ingredients.clone();
                    it.fold(acc, |acc, food| {
                        acc.intersection(&food.ingredients).cloned().collect()
                    })
                })
                .map(move |candidates| (warning, candidates))
        })
        .collect::<HashMap<String, HashSet<String>>>();

    let mut warnings = BTreeMap::new();
    loop {
        let unique = known
            .drain_filter(|_, ingredients| ingredients.len() == 1)
            .flat_map(|(key, ingredients)| {
                ingredients
                    .into_iter()
                    .next()
                    .map(|ingredient| (key, ingredient))
            })
            .collect::<Vec<_>>();

        if unique.is_empty() {
            break;
        }

        for (_, value) in &unique {
            for set in known.values_mut() {
                set.remove(value);
            }
        }

        warnings.extend(unique);
    }

    warnings.into_values().collect::<Vec<_>>().join(",")
}
