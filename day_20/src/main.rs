use common::{timed, Grid, Pos};
use pathfinding::directed::dijkstra::dijkstra;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Wall,
    Empty,
    Start,
    End,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Wall => f.write_char('#'),
            Self::Empty => f.write_char('.'),
            Self::Start => f.write_char('S'),
            Self::End => f.write_char('E'),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Map {
    start: Pos,
    end: Pos,
    grid: Grid<Block>,
}
impl Map {
    fn new(input: &str) -> Self {
        let mut start = Pos::default();
        let mut end = Pos::default();
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Block::Wall,
                        '.' => Block::Empty,
                        'S' => {
                            start = (x, y).into();
                            Block::Start
                        }
                        'E' => {
                            end = (x, y).into();
                            Block::End
                        }
                        x => panic!("Unexpected grid char: {x}"),
                    })
                    .collect()
            })
            .collect();

        Self {
            start,
            end,
            grid: Grid::new(grid),
        }
    }

    fn shortest_path(&self) -> Vec<Pos> {
        dijkstra(
            &self.start,
            |pos| {
                self.grid
                    .iter_adjacent_cardinal(*pos)
                    .filter(|(b, _)| matches!(b, Block::Empty | Block::End))
                    .map(|(_, p)| (p, 1))
            },
            |pos| *pos == self.end,
        )
        .unwrap()
        .0
    }

    #[allow(dead_code)]
    fn print_path(&self, path: &[Pos]) {
        let mut grid = self.grid.to_char_grid();
        for p in path {
            grid[*p] = 'O';
        }
        println!("{grid}");
    }

    fn find_long_shortcuts(
        &self,
        path_indices: &HashMap<Pos, usize>,
        pos: Pos,
        distance: u64,
        min_saved: u64,
    ) -> u64 {
        path_indices
            .iter()
            .filter(|(p, _)| p.manhattan_distance(&pos) <= distance as usize)
            .map(|(p, i)| {
                *i as isize - path_indices[&pos] as isize - p.manhattan_distance(&pos) as isize
            })
            .filter(|dist| *dist >= min_saved as isize)
            .count() as u64
    }
}

fn get_path_indices(path: &[Pos]) -> HashMap<Pos, usize> {
    path.iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_, _>>()
}

fn shortcuts(path_indices: &HashMap<Pos, usize>, path_pos: Pos, threshold: usize) -> u64 {
    let mut saved = 0;

    let dist = path_indices[&path_pos];
    for v in common::vectors::CARDINAL {
        let next = path_pos + v;
        if path_indices.get(&next).is_some() {
            continue;
        }

        let next_after = next + v;
        if let Some(next_dist) = path_indices.get(&next_after) {
            if *next_dist <= dist {
                continue;
            }

            let time_saved = next_dist - dist - 2;
            if time_saved >= threshold {
                saved += 1;
            }
        }
    }

    saved as u64
}

fn main() {
    let map = Map::new(&common::read_stdin());
    let shortest_path = map.shortest_path();
    let path_indices = get_path_indices(&shortest_path);

    let (time, cheats) = timed(|| {
        shortest_path
            .iter()
            .map(|p| shortcuts(&path_indices, *p, 100))
            .sum::<u64>()
    });
    println!("Part 1: {cheats} in {}ms", time.as_millis());

    let (time, long_cheats) = timed(|| {
        shortest_path
            .iter()
            .map(|p| map.find_long_shortcuts(&path_indices, *p, 20, 100))
            .sum::<u64>()
    });
    println!("Part 2: {long_cheats} in {}ms", time.as_millis());
}

// Part 1: 1448 in 2ms
// Part 2: 1017615 in 473ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let map = Map::new(
            "\
            ###############\n\
            #...#...#.....#\n\
            #.#.#.#.#.###.#\n\
            #S#...#.#.#...#\n\
            #######.#.#.###\n\
            #######.#.#...#\n\
            #######.#.###.#\n\
            ###..E#...#...#\n\
            ###.#######.###\n\
            #...###...#...#\n\
            #.#####.#.###.#\n\
            #.#...#.#.#...#\n\
            #.#.#.#.#.#.###\n\
            #...#...#...###\n\
            ###############\
        ",
        );

        let path = map.shortest_path();
        map.print_path(&path);
        assert_eq!(path.len() - 1, 84);

        let path_indices = get_path_indices(&path);
        let cheats = path
            .iter()
            .map(|p| shortcuts(&path_indices, *p, 64))
            .sum::<u64>();
        assert_eq!(cheats, 1);

        let long_cheats = path
            .iter()
            .map(|p| map.find_long_shortcuts(&path_indices, *p, 20, 76))
            .sum::<u64>();
        assert_eq!(long_cheats, 3);
    }
}
