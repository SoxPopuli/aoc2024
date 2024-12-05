use common::{read_stdin, timed};
use std::collections::{HashMap, HashSet};

type Requirements = HashMap<i32, HashSet<i32>>;
type Update = Vec<i32>;

#[derive(Debug)]
struct Input {
    requirements: Requirements,
    updates: Vec<Update>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let mut requirements = HashMap::new();
    let mut updates = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (x, y) = line.split_once('|').unwrap();

        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();

        requirements
            .entry(x)
            .and_modify(|set: &mut HashSet<i32>| {
                set.insert(y);
            })
            .or_insert(HashSet::from_iter([y]));
    }

    for line in lines {
        let update = line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        updates.push(update);
    }

    Input {
        requirements,
        updates,
    }
}

fn is_correctly_ordered(requirements: &Requirements, update: &Update) -> bool {
    let mut encountered = HashSet::new();

    for x in update {
        match requirements.get(x) {
            None => {}
            Some(reqs) => {
                for r in reqs {
                    if encountered.contains(r) {
                        return false;
                    }
                }
            }
        };

        encountered.insert(*x);
    }

    true
}

fn get_middle(update: &Update) -> i32 {
    let middle = update.len() / 2;
    update[middle]
}

fn get_ordered_middles(input: &Input) -> Vec<i32> {
    input
        .updates
        .iter()
        .filter(|update| is_correctly_ordered(&input.requirements, update))
        .map(get_middle)
        .collect()
}

fn reorder_incorrect(requirements: &Requirements, update: &Update) -> Update {
    let mut update = update.clone();

    update.sort_by(|a, b| {
        let req = requirements.get(a).map(|s| s.contains(b)).unwrap_or(false);
        if req {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    update
}

fn get_unordered_middles(input: &Input) -> Vec<i32> {
    input
        .updates
        .iter()
        .filter(|update| !is_correctly_ordered(&input.requirements, update))
        .map(|update| reorder_incorrect(&input.requirements, update))
        .map(|x| get_middle(&x))
        .collect()
}

fn main() {
    let input = parse_input(&read_stdin());

    let (time, result) = timed(|| get_ordered_middles(&input).into_iter().sum::<i32>());
    println!("Part 1: {result} in {}μs", time.as_micros());

    let (time, result) = timed(|| get_unordered_middles(&input).into_iter().sum::<i32>());
    println!("Part 2: {result} in {}μs", time.as_micros());
}

// Part 1: 5713 in 993μs
// Part 2: 5180 in 1492μs

#[cfg(test)]
mod tests {
    use crate::{
        get_middle, get_ordered_middles, is_correctly_ordered, parse_input, reorder_incorrect,
    };

    #[test]
    fn ordering_test() {
        let input = parse_input(include_str!("../example.txt"));

        assert!(!is_correctly_ordered(
            &input.requirements,
            &vec![61, 13, 29]
        ));
    }

    #[test]
    fn part_1() {
        let input = parse_input(include_str!("../example.txt"));

        let ordered: Vec<_> = input
            .updates
            .iter()
            .filter(|update| is_correctly_ordered(&input.requirements, update))
            .collect();

        let expected: [&[i32]; 3] = [&[75, 47, 61, 53, 29], &[97, 61, 53, 29, 13], &[75, 29, 13]];

        assert_eq!(ordered, expected, "ordered");

        let middles: Vec<_> = ordered.iter().map(|x| get_middle(x)).collect();

        assert_eq!(middles, [61, 53, 29], "middles");
        assert_eq!(get_ordered_middles(&input), [61, 53, 29], "middles");
    }

    #[test]
    fn resort_test() {
        let input = parse_input(include_str!("../example.txt"));
        let update = vec![75, 97, 47, 61, 53];
        let expected = vec![97, 75, 47, 61, 53];

        assert_eq!(reorder_incorrect(&input.requirements, &update), expected);

        assert_eq!(
            reorder_incorrect(&input.requirements, &vec![61, 13, 29]),
            [61, 29, 13]
        );

        assert_eq!(
            reorder_incorrect(&input.requirements, &vec![97, 13, 75, 29, 47]),
            [97, 75, 47, 29, 13]
        );
    }
}
