use common::timed;
use std::{
    io::Read,
    iter::{from_fn, Peekable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mul {
    a: i32,
    b: i32,
    enabled: bool,
}
impl Mul {
    fn mul(&self) -> i32 {
        self.a * self.b
    }
}

fn tokenize_part_1(input: &str) -> Option<Vec<Mul>> {
    let mut chars = input.chars().peekable();
    let mut ops = vec![];

    'outer: while let Some(ch) = chars.next() {
        if ch == 'm' {
            if chars.next_if(|c| *c == 'u').is_none()
                || chars.next_if(|c| *c == 'l').is_none()
                || chars.next_if(|c| *c == '(').is_none()
            {
                continue;
            }

            let mut numbers = vec![];

            loop {
                match chars.peek() {
                    Some('0'..='9') => {
                        numbers.push(parse_number(&mut chars));
                    }
                    Some(',') => {
                        chars.next();
                        continue;
                    }
                    Some(')') => {
                        chars.next();
                        break;
                    }
                    Some(_x) => continue 'outer, //panic!("Unexpected character in mul: {x}"),
                    None => continue 'outer,     //panic!("Unexpected end of stream in mul"),
                }
            }

            ops.push(Mul {
                a: numbers[0],
                b: numbers[1],
                enabled: true,
            });
        }
    }

    Some(ops)
}

fn tokenize_part_2(input: &str) -> Option<Vec<Mul>> {
    let mut chars = input.chars().peekable();
    let mut ops = vec![];
    let mut enabled = true;

    'outer: while let Some(ch) = chars.next() {
        match ch {
            'm' => {
                if chars.next_if(|c| *c == 'u').is_none()
                    || chars.next_if(|c| *c == 'l').is_none()
                    || chars.next_if(|c| *c == '(').is_none()
                {
                    continue;
                }

                let mut numbers = vec![];

                loop {
                    match chars.peek() {
                        Some('0'..='9') => {
                            numbers.push(parse_number(&mut chars));
                        }
                        Some(',') => {
                            chars.next();
                            continue;
                        }
                        Some(')') => {
                            chars.next();
                            break;
                        }
                        Some(_x) => continue 'outer, //panic!("Unexpected character in mul: {x}"),
                        None => continue 'outer,     //panic!("Unexpected end of stream in mul"),
                    }
                }

                ops.push(Mul {
                    a: numbers[0],
                    b: numbers[1],
                    enabled,
                });
            }

            'd' => {
                fn is_parens(chars: &mut Peekable<impl Iterator<Item = char>>) -> bool {
                    chars.next_if_eq(&'(').is_some() && chars.next_if_eq(&')').is_some()
                }

                if chars.next_if_eq(&'o').is_none() {
                    continue;
                } else if is_parens(&mut chars) {
                    enabled = true;
                    continue;
                } else if chars.next_if_eq(&'n').is_none()
                    || chars.next_if_eq(&'\'').is_none()
                    || chars.next_if_eq(&'t').is_none()
                {
                    continue;
                } else if is_parens(&mut chars) {
                    enabled = false;
                    continue;
                }

                enabled = false;
            }

            _ => {}
        }
    }

    Some(ops)
}

fn parse_number(iter: &mut Peekable<impl Iterator<Item = char>>) -> i32 {
    let mut total = 0;

    from_fn(|| iter.by_ref().next_if(|c| c.is_ascii_digit())).for_each(|x| {
        let value = x as u32 - '0' as u32;

        total *= 10;

        total += value;
    });

    total as i32
}

fn parse_instructions(ops: &[Mul]) -> i32 {
    ops.iter().filter(|o| o.enabled).map(|o| o.mul()).sum()
}

fn main() {
    let input = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let (time, result) = timed(|| {
        let tokens = tokenize_part_1(&input).unwrap();
        parse_instructions(&tokens)
    });
    println!("Part 1: {result} in {}μs", time.as_micros());

    let (time, result) = timed(|| {
        let tokens = tokenize_part_2(&input).unwrap();
        parse_instructions(&tokens)
    });
    println!("Part 2: {result} in {}μs", time.as_micros());
}

// Part 1: 174103751 in 70μs
// Part 2: 100411201 in 69μs

#[cfg(test)]
mod tests {
    use crate::{parse_instructions, tokenize_part_1};

    #[test]
    fn part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let tokens = tokenize_part_1(input).unwrap();
        println!("{tokens:#?}");
        let result = parse_instructions(&tokens);

        assert_eq!(result, 161);
    }
}
