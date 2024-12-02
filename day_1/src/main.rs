use std::io::Read as _;

fn sorted_difference(a: &[i32], b: &[i32]) -> i32 {
    fn sorted(x: &[i32]) -> Vec<i32> {
        let mut x = x.to_vec();
        x.sort();
        x
    }

    let a = sorted(a);
    let b = sorted(b);

    a.iter().zip(b).map(|(a, b)| a.abs_diff(b) as i32).sum()
}

fn similarity(a: &[i32], b: &[i32]) -> i32 {
    use std::collections::hash_map::HashMap;

    let mut occurences = HashMap::<_, _>::new();

    for x in b {
        occurences.entry(*x).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sum = 0;

    for x in a {
        let mult = occurences.get(x).cloned().unwrap_or(0);
        sum += x * mult;
    }

    sum
}

fn main() {
    let input = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let (a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();

            (a, b)
        })
        .unzip();

    let diff = sorted_difference(&a, &b);
    println!("Part 1: {}", diff);

    let similarity = similarity(&a, &b);
    println!("Part 2: {similarity}");
}

#[cfg(test)]
mod tests {
    use crate::{similarity, sorted_difference};

    #[test]
    fn part1() {
        let a = [3, 4, 2, 1, 3, 3];
        let b = [4, 3, 5, 3, 9, 3];

        assert_eq!(sorted_difference(&a, &b), 11)
    }

    #[test]
    fn part2() {
        let a = [3, 4, 2, 1, 3, 3];
        let b = [4, 3, 5, 3, 9, 3];

        assert_eq!(similarity(&a, &b), 31)
    }
}
