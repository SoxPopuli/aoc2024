use common::timed;

fn can_make(target: i64, numbers: &[i64]) -> bool {
    fn do_loop(target: i64, numbers: &[i64], acc: i64) -> bool {
        match numbers {
            [] => acc == target,

            [head, tail @ ..] => {
                do_loop(target, tail, acc + head) || do_loop(target, tail, acc * head)
            }
        }
    }

    do_loop(target, &numbers[1..], numbers[0])
}

fn concat(a: i64, b: i64) -> i64 {
    let b_places = (b as f64).log10().floor() + 1.0;
    let a = a * (10_i64.pow(b_places as u32));
    a + b
}

fn can_make_concat(target: i64, numbers: &[i64]) -> bool {
    fn do_loop(target: i64, numbers: &[i64], acc: i64) -> bool {
        match numbers {
            [] => acc == target,

            [head, tail @ ..] => {
                do_loop(target, tail, acc + head)
                    || do_loop(target, tail, acc * head)
                    || do_loop(target, tail, concat(acc, *head))
            }
        }
    }

    do_loop(target, &numbers[1..], numbers[0])
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(a, b)| {
            let target = a.parse().unwrap();
            let numbers = b.split_whitespace().filter_map(|x| x.parse().ok());

            (target, numbers.collect())
        })
        .collect()
}

fn get_calibration_result(
    input: Vec<(i64, Vec<i64>)>,
    filter_func: impl Fn(i64, &[i64]) -> bool,
) -> i64 {
    input
        .iter()
        .filter(|(target, numbers)| filter_func(*target, numbers))
        .map(|(target, _)| *target)
        .sum()
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, calibration_result) = timed(|| get_calibration_result(input.clone(), can_make));
    println!("Part 1: {calibration_result} in {}μs", time.as_micros());

    let (time, calibration_result) =
        timed(|| get_calibration_result(input.clone(), can_make_concat));
    println!("Part 2: {calibration_result} in {}ms", time.as_millis());
}

// Part 1: 1399219271639 in 913μs
// Part 2: 275791737999003 in 114ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_tests() {
        assert!(can_make(190, &[10, 19]));
        assert!(can_make(3267, &[81, 40, 27]));
        assert!(can_make(292, &[11, 6, 16, 20]));

        assert!(!can_make(20, &[5]));
        assert!(!can_make(100, &[10, 9]));
        assert!(!can_make(161011, &[16, 10, 13]));

        assert!(!can_make(83, &[17, 5]));
    }

    #[test]
    fn concat_tests() {
        assert_eq!(concat(15, 3), 153);
        assert_eq!(concat(1, 300), 1300);
    }

    #[test]
    fn part1() {
        let input = parse_input(include_str!("../example.txt"));
        assert_eq!(get_calibration_result(input, can_make), 3749);
    }

    #[test]
    fn part2() {
        let input = parse_input(include_str!("../example.txt"));
        assert_eq!(get_calibration_result(input, can_make_concat), 11387);
    }
}
