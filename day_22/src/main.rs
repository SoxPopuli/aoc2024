use common::timed;
use std::collections::{HashMap, HashSet};

fn mix(secret: i64, value: i64) -> i64 {
    value ^ secret
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn mix_prune(secret: i64, value: i64) -> i64 {
    prune(mix(secret, value))
}

#[derive(Debug, Default)]
struct NumberGenerator(HashMap<i64, i64>);
impl NumberGenerator {
    fn new() -> Self {
        Self::default()
    }

    fn generate(&mut self, number: i64) -> i64 {
        if let Some(cached) = self.0.get(&number) {
            *cached
        } else {
            let a = mix_prune(number, number * 64);
            let b = mix_prune(a, a / 32);
            let c = mix_prune(b, b * 2048);

            self.0.insert(number, c);
            c
        }
    }

    fn generate_n(&mut self, starting_number: i64, n: usize) -> i64 {
        (0..n).fold(starting_number, |acc, _| self.generate(acc))
    }

    fn generate_n_iter(
        &mut self,
        starting_number: i64,
        n: usize,
    ) -> impl Iterator<Item = i64> + use<'_> {
        (0..n).scan(starting_number, |acc, _| {
            let val = self.generate(*acc);
            *acc = val;
            Some(val)
        })
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn price(number: i64) -> i64 {
    number % 10
}

fn get_numbers(input: i64, rng: &mut NumberGenerator) -> Vec<i64> {
    rng.generate_n_iter(input, 2000).collect::<Vec<_>>()
}

fn get_changes(numbers: &[i64]) -> Vec<i64> {
    let mut changes = vec![0];
    for i in 1..numbers.len() {
        let change = price(numbers[i]) - price(numbers[i - 1]);
        changes.push(change);
    }
    changes
}

fn find_first_occurence(changes: &[i64], sequence: &[i64; 4]) -> Option<usize> {
    (3..changes.len()).find(|&i| {
        changes[i] == sequence[3]
            && changes[i - 1] == sequence[2]
            && changes[i - 2] == sequence[1]
            && changes[i - 3] == sequence[0]
    })
}

fn find_best_sequence(numbers: &[Vec<i64>], changes: &[Vec<i64>]) -> ([i64; 4], i64) {
    let mut seq_values = HashMap::new();

    for monkey in 0..numbers.len() {
        let changes = &changes[monkey];
        let numbers = &numbers[monkey];
        let mut visited = HashSet::new();

        #[allow(clippy::needless_range_loop)]
        for i in 3..changes.len() {
            let seq = [changes[i - 3], changes[i - 2], changes[i - 1], changes[i]];
            if visited.contains(&seq) {
                continue;
            }

            let price = price(numbers[i]);
            seq_values
                .entry(seq)
                .and_modify(|value| {
                    *value += price;
                })
                .or_insert(price);

            visited.insert(seq);
        }
    }

    seq_values
        .into_iter()
        .max_by_key(|(_, price)| *price)
        .unwrap()
}

fn get_most_bananas(input: &[i64], rng: &mut NumberGenerator) -> i64 {
    let numbers = input
        .iter()
        .map(|x| get_numbers(*x, rng))
        .collect::<Vec<_>>();
    let changes = numbers.iter().map(|n| get_changes(n)).collect::<Vec<_>>();
    let (sequence, _) = find_best_sequence(&numbers, &changes);
    numbers
        .iter()
        .zip(changes.iter())
        .filter_map(|(n, c)| {
            let idx = find_first_occurence(c, &sequence);
            idx.map(|i| price(n[i]))
        })
        .sum::<i64>()
}

fn main() {
    let input = parse_input(&common::read_stdin());
    let mut rng = NumberGenerator::new();
    let (time, sum) = timed(|| input.iter().map(|x| rng.generate_n(*x, 2000)).sum::<i64>());

    println!("Part 1: {sum} in {}ms", time.as_millis());

    let (time, bananas) = timed(|| get_most_bananas(&input, &mut rng));
    println!("Part 2: {bananas} in {}ms", time.as_millis());
}

// Part 1: 15335183969 in 801ms
// Part 2: 1696 in 2447ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_test() {
        let secret = 123;
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        let mut rng = NumberGenerator::new();
        let actual = rng
            .generate_n_iter(secret, expected.len())
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);

        assert_eq!(rng.generate_n(secret, 10), 5908254);
    }

    #[test]
    fn input_test() {
        let input = parse_input(
            "\
            1\n\
            10\n\
            100\n\
            2024\
        ",
        );

        assert_eq!(input, [1, 10, 100, 2024]);

        let mut rng = NumberGenerator::new();
        let expected = [8685429, 4700978, 15273692, 8667524];
        let actual = input
            .iter()
            .map(|x| rng.generate_n(*x, 2000))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
        assert_eq!(actual.iter().sum::<i64>(), 37327623);

        let (seq, _) = {
            let numbers = [1, 2, 3, 2024].map(|x| get_numbers(x, &mut rng));
            let changes = numbers.iter().map(|n| get_changes(n)).collect::<Vec<_>>();
            find_best_sequence(&numbers, &changes)
        };

        let sequence = [-2, 1, -1, 3];
        assert_eq!(seq, sequence);

        let first_buyer = rng.generate_n_iter(1, 2000).collect::<Vec<_>>();
        let first_buyer_prices = get_changes(&first_buyer);
        let a = find_first_occurence(&first_buyer_prices, &sequence).unwrap();
        assert_eq!(price(first_buyer[a]), 7);

        let second_buyer = rng.generate_n_iter(2, 2000).collect::<Vec<_>>();
        let second_buyer_prices = get_changes(&second_buyer);
        let b = find_first_occurence(&second_buyer_prices, &sequence).unwrap();
        assert_eq!(price(second_buyer[b]), 7);

        let third_buyer = rng.generate_n_iter(3, 2000).collect::<Vec<_>>();
        let third_buyer_prices = get_changes(&third_buyer);
        let c = find_first_occurence(&third_buyer_prices, &sequence);
        assert_eq!(c, None);

        assert_eq!(get_most_bananas(&[1, 2, 3, 2024], &mut rng), 23);
    }

    #[test]
    fn changes_test() {
        let numbers = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
        ];
        let changes = get_changes(&numbers);
        assert_eq!(changes, [0, -3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }
}
