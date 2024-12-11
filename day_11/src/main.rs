use common::timed;
use std::collections::HashMap;

fn count_digits(x: u64) -> u64 {
    ((x as f64).log10().floor() + 1.0) as u64
}

fn split_number(x: u64) -> (u64, u64) {
    let digits = count_digits(x);
    let power = 10_u64.pow(digits as u32 / 2);

    let high = x / power;
    let low = x % power;

    (high, low)
}

type Memo = HashMap<(u8, u64), u64>;

fn blink(n: u8, stone: u64, memo: &mut Memo) -> u64 {
    let num_stones = if n == 0 {
        1
    } else if let Some(stored) = memo.get(&(n, stone)) {
        *stored
    } else if stone == 0 {
        blink(n - 1, 1, memo)
    } else if count_digits(stone) % 2 == 0 {
        let (high, low) = split_number(stone);
        blink(n - 1, high, memo) + blink(n - 1, low, memo)
    } else {
        blink(n - 1, stone * 2024, memo)
    };

    memo.insert((n, stone), num_stones);
    num_stones
}

fn blink_multiple(items: &[u64], n: u8, memo: &mut Memo) -> u64 {
    items.iter().fold(0, |acc, x| acc + blink(n, *x, memo))
}

fn main() {
    let input: Vec<u64> = common::read_stdin()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut memo = Memo::default();

    let (time, stones) = timed(|| blink_multiple(&input, 25, &mut memo));
    println!("Part 1: {} in {}ms", stones, time.as_millis());

    let (time, stones) = timed(|| blink_multiple(&input, 75, &mut memo));
    println!("Part 2: {} in {}ms", stones, time.as_millis());
}

// Part 1: 183435 in 1ms
// Part 2: 218279375708592 in 39ms

#[cfg(test)]
mod tests {
    use crate::{blink_multiple, split_number, Memo};

    #[test]
    fn split_tests() {
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(123456), (123, 456));
        assert_eq!(split_number(12345678), (1234, 5678));
    }

    #[test]
    fn blink_tests() {
        let initial = vec![125, 17];

        let mut memo = Memo::default();

        assert_eq!(blink_multiple(&initial, 6, &mut memo), 22);

        assert_eq!(blink_multiple(&initial, 25, &mut memo), 55312);
    }
}
