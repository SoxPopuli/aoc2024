use common::timed;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    // let trie = Trie::from_str(lines.next().unwrap());
    let towels = lines.next().unwrap().to_string();
    lines.next().unwrap();

    let designs = lines.map(|x| x.to_string()).collect();
    (
        towels
            .split(',')
            .map(|x| x.trim_ascii().to_owned())
            .collect(),
        designs,
    )
}

fn main() {
    let (towels, designs) = parse_input(&common::read_stdin());

    let (time, possible) = timed(|| {
        let regex = {
            let re = towels.join("|");

            let re = format!("^(?:{re})+$");
            regex::Regex::new(&re).unwrap()
        };

        designs.iter().filter(|x| regex.is_match(x)).count()
    });
    println!("Part 1: {possible} in {}ms", time.as_millis());

    let (time, combinations) = timed(|| count_possible_designs(&towels, &designs));
    println!("Part 2: {combinations} in {}ms", time.as_millis());
}

// Part 1: 306 in 8ms
// Part 2: 604622004681855 in 57ms

fn count_possible_designs(towels: &[String], designs: &[String]) -> usize {
    fn count_possible<'a>(
        towels: &[String],
        design: &'a str,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if let Some(cached) = cache.get(design) {
            *cached
        } else if design.is_empty() {
            1
        } else {
            let sum = towels
                .iter()
                .filter_map(|towel| design.strip_prefix(towel))
                .map(|p| count_possible(towels, p, cache))
                .sum();

            cache.insert(design, sum);
            sum
        }
    }

    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count_possible(towels, design.as_ref(), &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_prefixes_test() {
        let towels = [
            "r".to_owned(),
            "wr".to_owned(),
            "b".to_owned(),
            "g".to_owned(),
            "bwu".to_owned(),
            "rb".to_owned(),
            "gb".to_owned(),
            "br".to_owned(),
        ];

        let pattern = "brgr";
        assert_eq!(count_possible_designs(&towels, &[pattern.to_owned()]), 2);

        let pattern = "bbrgwb";
        assert_eq!(count_possible_designs(&towels, &[pattern.to_owned()]), 0);
    }
}
