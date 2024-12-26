use common::{iter::UniqueIter, timed};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
}

#[derive(Debug)]
struct Connections {
    connections: HashMap<String, HashSet<String>>,
}
impl Connections {
    fn new<S>(connections: impl IntoIterator<Item = (S, S)>) -> Self
    where
        S: Into<String>,
    {
        use std::collections::hash_map::Entry;

        let mut map = HashMap::<String, HashSet<String>>::new();
        for (a, b) in connections {
            let a: String = a.into();
            let b: String = b.into();

            match map.entry(a.clone()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(b.clone());
                }
                Entry::Vacant(e) => {
                    e.insert([b.clone()].into());
                }
            }

            match map.entry(b) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(a);
                }
                Entry::Vacant(e) => {
                    e.insert([a].into());
                }
            }
        }

        Self { connections: map }
    }

    fn get_connected(&self, computer: &str) -> Option<Vec<[String; 3]>> {
        let connections = self.connections.get(computer)?;
        let mut groups = vec![];
        let mut seen = HashSet::<_>::new();
        for a in connections {
            let a_conn = self.connections.get(a)?;

            for b in connections {
                if a == b {
                    continue;
                }

                let sorted = if a < b { (a, b) } else { (b, a) };

                if a_conn.contains(b) && !seen.contains(&sorted) {
                    seen.insert(sorted);
                    groups.push([computer.into(), a.clone(), b.clone()]);
                }
            }
        }

        Some(groups)
    }

    fn sets(&self) -> Vec<[String; 3]> {
        self.connections
            .keys()
            .filter_map(|pc| self.get_connected(pc))
            .flatten()
            .map(|mut x| {
                x.sort();
                x
            })
            .unique()
            .collect()
    }

    fn get_all_connected(&self, computer: &str) -> HashSet<String> {
        let conn = &self.connections[computer];

        let x = conn.iter().flat_map(|c| {
            self.connections[c].iter().filter(|c2| {
                if *c2 == computer {
                    true
                } else {
                    conn.contains(*c2)
                }
            })
        });

        x.unique().cloned().collect()
    }

    fn get_all_inter_connected(&self, computer: &str) -> HashSet<String> {
        let mut connected = self.get_all_connected(computer);
        let others = connected.iter().cloned().collect::<Vec<_>>();

        for c in others {
            connected = connected
                .intersection(&self.get_all_connected(&c))
                .cloned()
                .collect();
        }

        connected
    }
}

fn main() {
    let input = common::read_stdin();
    let input = parse_input(&input);

    let connections = Connections::new(input);

    let (time, t_sets) = timed(|| {
        connections
            .sets()
            .into_iter()
            .filter(|set| set.iter().any(|pc| pc.starts_with('t')))
            .collect::<Vec<_>>()
    });

    println!("Part 1: {} in {}ms", t_sets.len(), time.as_millis());

    let (time, most_connected) = timed(|| get_most_connected(&connections).join(","));
    println!("Part 2: {most_connected} in {}ms", time.as_millis());
}

// Part 1: 1119 in 68ms
// Part 2: av,fr,gj,hk,ii,je,jo,lq,ny,qd,uq,wq,xc in 222ms

fn get_most_connected(c: &Connections) -> Vec<String> {
    c.connections
        .keys()
        .map(|pc| c.get_all_inter_connected(pc))
        .map(|x| x.into_iter().collect::<Vec<_>>())
        .map(|mut x| {
            x.sort();
            x
        })
        .unique()
        .max_by_key(|v| v.len())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = parse_input(include_str!("../example.txt"));

        let c = Connections::new(input);

        let sets = c.sets();
        assert_eq!(sets.len(), 12);

        let ts = sets
            .iter()
            .filter(|set| set.iter().any(|pc| pc.starts_with('t')))
            .collect::<Vec<_>>();
        assert_eq!(ts.len(), 7);
    }

    #[test]
    fn test_2() {
        let input = parse_input(include_str!("../example.txt")).collect::<Vec<_>>();

        let c = Connections::new(input);
        assert_eq!(get_most_connected(&c), ["co", "de", "ka", "ta"]);
    }
}
