use common::timed;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: f64,
    y: f64,
}
impl std::ops::Mul<f64> for Pos {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::Add<f64> for Pos {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[derive(Debug)]
struct Machine {
    a_incr: Pos,
    b_incr: Pos,
    target: Pos,
}

fn get_presses(
    Machine {
        a_incr,
        b_incr,
        target,
    }: &Machine,
) -> (f64, f64) {
    // A: X+94, Y+34
    // B: X+22, Y+67
    // 94x + 22y = 8400
    // 34x + 67y = 5400

    let a = a_incr.x;
    let b = b_incr.x;
    let c = target.x;

    let d = a_incr.y;
    let e = b_incr.y;
    let f = target.y;

    let x = (c * e - b * f) / (a * e - b * d);
    let y = (c - a * x) / b;
    (x, y)
}

fn validate((a_presses, b_presses): (f64, f64)) -> bool {
    // Solution is correct if num of presses is an integer
    a_presses == a_presses.floor() && b_presses == b_presses.floor()
}

fn get_token_cost((a, b): (f64, f64)) -> f64 {
    (a * 3.0) + b
}

fn parse_input(input: &str) -> Vec<Machine> {
    fn read_line(line: &str) -> Pos {
        fn read_part(part: &str) -> f64 {
            part.chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        }

        let (x, y) = line.split_once(',').unwrap();

        let x = read_part(x);
        let y = read_part(y);
        Pos { x, y }
    }

    let mut machines = vec![];

    let mut lines = input.lines();
    while let Some(a_line) = lines.next() {
        let b_line = lines.next().unwrap();
        let target_line = lines.next().unwrap();

        let a_incr = read_line(a_line);
        let b_incr = read_line(b_line);
        let target = read_line(target_line);

        machines.push(Machine {
            a_incr,
            b_incr,
            target,
        });

        if lines.next().is_none() {
            break;
        }
    }

    machines
}

fn main() {
    let machines = parse_input(&common::read_stdin());

    let (time, tokens) = timed(|| {
        machines
            .iter()
            .map(get_presses)
            .filter(|x| validate(*x))
            .map(get_token_cost)
            .sum::<f64>()
    });
    println!("Part 1: {tokens} in {}μs", time.as_micros());

    const TARGET_MODIFIER: f64 = 10_000_000_000_000.0;

    let (time, tokens) = timed(|| {
        machines
            .iter()
            .map(|m| Machine {
                target: m.target + TARGET_MODIFIER,
                ..*m
            })
            .map(|m| get_presses(&m))
            .filter(|x| validate(*x))
            .map(get_token_cost)
            .sum::<f64>()
    });
    println!("Part 2: {tokens} in {}μs", time.as_micros());
}

// Part 1: 33481 in 5μs
// Part 2: 92572057880885 in 2μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a_incr = Pos { x: 94.0, y: 34.0 };
        let b_incr = Pos { x: 22.0, y: 67.0 };
        let target = Pos {
            x: 8400.0,
            y: 5400.0,
        };

        let presses = get_presses(&Machine {
            a_incr,
            b_incr,
            target,
        });
        assert_eq!(presses, (80.0, 40.0));
        assert!(validate(presses));
        assert_eq!(get_token_cost(presses), 280.0);

        let a_incr = Pos { x: 26.0, y: 66.0 };
        let b_incr = Pos { x: 67.0, y: 21.0 };
        let target = Pos {
            x: 12748.0,
            y: 12176.0,
        };

        let presses = get_presses(&Machine {
            a_incr,
            b_incr,
            target,
        });
        assert!(!validate(presses))
    }
}
