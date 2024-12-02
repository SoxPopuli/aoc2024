use common::{timed, timed_repeated};
use std::io::Read as _;

fn is_safe_dampened(seq: &[i32]) -> bool {
    if is_safe(seq) {
        true
    } else {
        (0..seq.len()).any(|i| {
            let mut seq = seq.to_vec();
            seq.remove(i);
            is_safe(&seq)
        })
    }
}

fn is_safe(seq: &[i32]) -> bool {
    let mut last_change = i32::MIN;

    fn is_error(diff: i32, last_change: i32) -> bool {
        const SIGN_WIDTH: usize = i32::BITS as usize - 1;

        let sign_changed =
            last_change != i32::MIN && (last_change >> SIGN_WIDTH) != (diff >> SIGN_WIDTH);

        diff == 0 || diff.abs() > 3 || sign_changed
    }

    for i in 1..seq.len() {
        let diff = seq[i] - seq[i - 1];

        if is_error(diff, last_change) {
            return false;
        }

        last_change = diff;
    }

    true
}

fn main() {
    let input = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let (time_to_parse, lines) = timed(|| {
        input
            .lines()
            .map(|line| {
                let parts = line.split_whitespace();
                parts.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });
    println!("Time to parse: {}μs", time_to_parse.as_micros());

    let (time, safe_count) = timed_repeated(10, || lines.iter().filter(|x| is_safe(x)).count());
    println!("Part 1: {safe_count} in {}μs", time.as_micros());

    let (time, safe_count) =
        timed_repeated(10, || lines.iter().filter(|x| is_safe_dampened(x)).count());
    println!("Part 2: {safe_count} in {}μs", time.as_micros());
}

// Time to parse: 289μs
// Part 1: 390 in 14μs
// Part 2: 439 in 156μs

#[cfg(test)]
mod tests {
    use crate::{is_safe, is_safe_dampened};

    #[test]
    fn part1() {
        let x = [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ];

        (0..x.len()).for_each(|i| {
            let state = if is_safe(&x[i]) { "Safe" } else { "Unsafe" };
            println!("{i}: {state}");
        });

        let count = x.into_iter().filter(|x| is_safe(x)).count();

        assert_eq!(count, 2);
    }

    #[test]
    fn part2() {
        let x = [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ];

        let count = x.into_iter().filter(|x| is_safe_dampened(x)).count();
        assert_eq!(count, 4);
    }
}
