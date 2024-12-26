use common::{timed, iter::PairsIter};
use std::{
    fmt::{Display, Write},
    iter::{from_fn, repeat_n},
};

#[derive(Debug, Clone)]
struct ExpandedDiskMap(Vec<Option<u64>>);
impl Display for ExpandedDiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.0.iter() {
            match ch {
                Some(x) => f.write_str(&x.to_string())?,
                None => f.write_char('.')?,
            };
        }
        Ok(())
    }
}

impl ExpandedDiskMap {
    fn new(input: &str) -> Self {
        let pairs = input
            .chars()
            .filter_map(|x| x.to_digit(10).map(|x| x as usize))
            .pairs();

        let mut map = vec![];

        for (id, (len, empty)) in pairs.enumerate() {
            map.extend(repeat_n(Some(id as u64), len));

            if let Some(empty) = empty {
                map.extend(repeat_n(None, empty))
            }
        }

        ExpandedDiskMap(map)
    }

    fn shrink(mut self) -> Self {
        let mut head = 0;
        let mut tail = self.0.len() - 1;

        while head <= tail {
            match self.0[head] {
                None => match self.0[tail] {
                    None => {
                        tail -= 1;
                        continue;
                    }
                    _ => {
                        self.0.swap(head, tail);
                        head += 1;
                    }
                },
                _ => {
                    head += 1;
                }
            }
        }

        self
    }

    fn shrink_whole_files(mut self) -> Self {
        #[derive(Debug)]
        struct Window {
            start: usize,
            end: usize,
        }
        impl Window {
            fn len(&self) -> usize {
                self.end - self.start + 1
            }
        }

        // get indices of contiguous similar values
        fn get_window(ExpandedDiskMap(data): &ExpandedDiskMap, i: usize) -> Window {
            let value = data[i];

            let mut start = i as isize;
            let mut end = i as isize;

            while start > 0 && data[(start - 1) as usize] == value {
                start -= 1;
            }

            while end < data.len() as isize - 1 && data[(end + 1) as usize] == value {
                end += 1;
            }

            Window {
                start: start as usize,
                end: end as usize,
            }
        }

        fn get_similar(data: &ExpandedDiskMap) -> Vec<Window> {
            let mut index = 0;
            let iter = from_fn(|| {
                if index >= data.0.len() {
                    return None;
                }
                let window = get_window(data, index);
                index = window.end + 1;

                Some(window)
            });

            iter.filter(|x| data.0[x.start].is_some()).collect()
        }

        fn leftmost_empty_of_size_n(data: &ExpandedDiskMap, size: usize) -> Option<Window> {
            for (i, x) in data.0.iter().enumerate() {
                match x {
                    Some(_) => continue,
                    None => {
                        let window = get_window(data, i);

                        if window.len() >= size {
                            return Some(window);
                        }
                    }
                }
            }

            None
        }

        let similar = get_similar(&self);

        for s in similar.iter().rev() {
            if let Some(empty) = leftmost_empty_of_size_n(&self, s.len()) {
                if s.start <= empty.start {
                    continue;
                }

                for i in 0..s.len() {
                    self.0.swap(s.start + i, empty.start + i);
                }
            }
        }

        self
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| (i, x)))
            .map(|(i, x)| i as u64 * x)
            .sum::<u64>()
    }
}

fn main() {
    let input = common::read_stdin();

    let map = ExpandedDiskMap::new(&input);
    let (time, checksum) = timed(|| map.clone().shrink().checksum());
    println!("Part 1: {checksum} in {}μs", time.as_micros());

    let (time, checksum) = timed(|| map.shrink_whole_files().checksum());
    println!("Part 2: {checksum} in {}ms", time.as_millis());
}

// Part 1: 6262891638328 in 2011μs
// Part 2: 6287317016845 in 310ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_test() {
        assert_eq!(ExpandedDiskMap::new("12345").to_string(), "0..111....22222");

        assert_eq!(
            ExpandedDiskMap::new("2333133121414131402").to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn shrink_test() {
        assert_eq!(
            ExpandedDiskMap::new("12345").shrink().to_string(),
            "022111222......"
        );

        assert_eq!(
            ExpandedDiskMap::new("2333133121414131402")
                .shrink()
                .to_string(),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn shrink_whole_test() {
        assert_eq!(
            ExpandedDiskMap::new("2333133121414131402")
                .shrink_whole_files()
                .to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn checksum_test() {
        assert_eq!(
            ExpandedDiskMap::new("2333133121414131402")
                .shrink()
                .checksum(),
            1928,
        );

        assert_eq!(
            ExpandedDiskMap::new("2333133121414131402")
                .shrink_whole_files()
                .checksum(),
            2858,
        );
    }
}
