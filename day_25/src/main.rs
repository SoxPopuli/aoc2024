#[derive(Debug, Default)]
struct State {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}
impl State {
    fn new(input: &str) -> Self {
        fn add_key_or_lock(this: &mut State, input: &str) {
            let pins = input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut body = [0; 5];

            #[allow(clippy::needless_range_loop)]
            for x in 0..5 {
                for y in 1..=5 {
                    if pins[y][x] == '#' {
                        body[x] += 1;
                    }
                }
            }

            if pins[0].iter().all(|c| *c == '#') {
                this.locks.push(body);
            } else {
                this.keys.push(body)
            }
        }

        let mut this = Self::default();
        input
            .split("\n\n")
            .for_each(|x| add_key_or_lock(&mut this, x));

        this
    }

    fn find_matches(&self) -> Vec<([u8; 5], [u8; 5])> {
        fn has_overlap(lock: &[u8; 5], key: &[u8; 5]) -> bool {
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    return false;
                }
            }

            true
        }

        let mut matches = vec![];

        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                if has_overlap(lock, key) {
                    matches.push((*lock, *key));
                }
            }
        }

        matches
    }
}

fn main() {
    let state = State::new(&common::read_stdin());

    println!("Part 1: {}", state.find_matches().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let state = State::new(include_str!("../example.txt"));

        assert_eq!(
            state.keys,
            [[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1],]
        );

        assert_eq!(state.locks, [[0, 5, 3, 4, 3], [1, 2, 0, 5, 3],]);

        assert_eq!(
            state.find_matches(),
            [
                ([0, 5, 3, 4, 3], [3, 0, 2, 0, 1]),
                ([1, 2, 0, 5, 3], [4, 3, 4, 0, 2]),
                ([1, 2, 0, 5, 3], [3, 0, 2, 0, 1])
            ]
        );
    }
}
