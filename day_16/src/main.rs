use common::{ vectors, timed };
use std::{fmt::Display, hash::Hash};

type HashSet<T> = std::collections::HashSet<T>;

use common::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    Wall,
    Start,
    End,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Empty => f.write_char('.'),
            Self::Wall => f.write_char('#'),
            Self::Start => f.write_char('S'),
            Self::End => f.write_char('E'),
        }
    }
}

type Grid = common::Grid<Block>;

#[derive(Debug)]
struct Map {
    start: Pos,
    end: Pos,

    grid: Grid,
}

fn parse_map(input: &str) -> Map {
    let mut start = Pos::default();
    let mut end = Pos::default();

    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
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

                    c => panic!("Unrecognised character: {c}"),
                })
                .collect()
        })
        .collect();

    Map {
        start,
        end,
        grid: Grid::new(grid),
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North,
    South,
    #[default]
    East,
    West,
}
impl Heading {
    fn num_rotations(&self, other: &Self) -> u8 {
        if self == other {
            return 0;
        }

        match self {
            Self::North | Self::South => match other {
                Self::East | Self::West => 1,
                _ => 2,
            },
            Self::East | Self::West => match other {
                Self::North | Self::South => 1,
                _ => 2,
            },
        }
    }

    fn from_pos(start: Pos, end: Pos) -> Self {
        let vec = end - start;
        match vec {
            vectors::UP => Heading::North,
            vectors::LEFT => Heading::West,
            vectors::RIGHT => Heading::East,
            vectors::DOWN => Heading::South,
            x => panic!("Unexpected vector: {x}"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Vector {
    pos: Pos,
    heading: Heading,
}
impl Hash for Vector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state)
    }
}

fn find_paths_a_star(m @ Map { start, end, .. }: &Map) -> Vec<Vec<Vector>> {
    fn get_successors(current: Vector, map: &Map) -> Vec<(Vector, u64)> {
        let mut succ = vec![];

        for (_, p) in map
            .grid
            .iter_adjacent_cardinal(current.pos)
            .filter(|(b, _)| matches!(b, Block::Empty | Block::End))
        {
            let heading = Heading::from_pos(current.pos, p);
            let num_rotations = current.heading.num_rotations(&heading);
            let score = current.pos.distance(&p) as u64 + (1000 * num_rotations as u64);

            succ.push((Vector {
                pos: p,
                heading
            }, score))
        }

        succ
    }

    fn heuristic(end: &Pos, node: &Pos) -> u64 {
        node.distance(end) as u64
    }

    let results = pathfinding::directed::astar::astar_bag(
        &Vector {
            pos: *start,
            heading: Heading::East,
        },
        |state| get_successors(*state, m),
        |state| heuristic(end, &state.pos),
        |x| x.pos == *end,
    );

    match results {
        Some((paths, ..)) => paths.into_iter().collect(),
        None => panic!("no paths found"),
    }

    // fn reconstruct_path(came_from: &HashMap<Vector, Vector>, mut current: Vector) -> Vec<Vector> {
    //     let mut total_path = vec![current];

    //     while came_from.contains_key(&current) {
    //         current = came_from[&current];
    //         total_path.push(current);
    //     }

    //     total_path.reverse();
    //     total_path
    // }

    // fn min_score_node(set: &HashSet<Vector>, score: &HashMap<Vector, f64>) -> Vector {
    //     let mut min_node = Vector::default();
    //     let mut min_score = f64::MAX;
    //     for v in set.iter() {
    //         if score[v] < min_score {
    //             min_node = *v;
    //             min_score = score[v];
    //         }
    //     }

    //     min_node
    // }

    // let v = Vector {
    //     pos: *start,
    //     heading: Heading::East,
    // };
    // let mut open_set = HashSet::from_iter([v]);

    // let mut came_from = HashMap::<Vector, Vector>::new();
    // let mut g_score = HashMap::from_iter([(v, 0.0)]);
    // let mut f_score = HashMap::from_iter([(v, heuristic(end, start))]);

    // let mut paths = vec![];

    // while !open_set.is_empty() {
    //     let current = min_score_node(&open_set, &f_score);
    //     if current.pos == *end {
    //         paths.push(reconstruct_path(&came_from, current));
    //     }

    //     open_set.remove(&current);
    //     grid.iter_adjacent_cardinal(current.pos)
    //         .filter(|(b, _)| matches!(b, Block::Empty | Block::End))
    //         .for_each(|(b, p)| {
    //             let heading = Heading::from_pos(current.pos, p);
    //             let num_rotations = current.heading.num_rotations(&heading);

    //             let v = Vector { pos: p, heading };

    //             let current_g_score = g_score.get(&current).unwrap_or(&f64::MAX);
    //             let possible_g_score = current_g_score
    //                 + current.pos.distance(&p)
    //                 + (1000 * num_rotations as u64) as f64;

    //             if possible_g_score < *g_score.get(&v).unwrap_or(&f64::MAX) {
    //                 came_from.insert(v, current);
    //                 g_score.insert(v, possible_g_score);
    //                 f_score.insert(v, possible_g_score + heuristic(end, &p));
    //                 if !open_set.contains(&v) {
    //                     open_set.insert(v);
    //                 }
    //             }
    //         })
    // }

    // paths
}

fn count_path(path: &[Vector]) -> u64 {
    let mut total = 0;

    for i in 1..path.len() {
        let prev = path[i - 1].heading;
        let cur = path[i].heading;

        total += 1000 * prev.num_rotations(&cur) as u64;
    }

    total + path.len() as u64 - 1
}

fn count_tiles<'a, I, U>(paths: I) -> usize 
    where 
        I: Iterator<Item = U>,
        U: IntoIterator<Item = &'a Vector>
{
    let positions: HashSet<_> = paths
        .flatten()
        .map(|x| x.pos)
        .collect();
    
    positions.len()
}

#[allow(unused)]
fn print_paths(map: &Map, paths: &Vec<Vec<Vector>>) {
    let mut grid = common::Grid {
        height: map.grid.height,
        width: map.grid.width,
        data: map
            .grid
            .data
            .iter()
            .map(|v| {
                v.iter()
                    .map(|x| x.to_string().chars().next().unwrap())
                    .collect()
            })
            .collect(),
    };

    for path in paths {
        for p in path.iter() {
            grid[p.pos] = match p.heading {
                Heading::North => '^',
                Heading::South => 'v',
                Heading::East => '>',
                Heading::West => '<',
            }
        }
    }

    println!("{grid}")
}

fn main() {
    let map = parse_map(&common::read_stdin());
    let (time, path) = timed(|| find_paths_a_star(&map));
    let distance = count_path(&path[0]);
    println!("Part 1: {distance} in {}ms", time.as_millis());

    let (time, tiles) = timed(|| count_tiles(path.iter()));
    println!("Part 2: {tiles} in {}ms", time.as_millis());
}

// Part 1: 90460 in 44ms
// Part 2: 575 in 17ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_test() {
        let input = "\
            ###############\n\
            #.......#....E#\n\
            #.#.###.#.###.#\n\
            #.....#.#...#.#\n\
            #.###.#####.#.#\n\
            #.#.#.......#.#\n\
            #.#.#####.###.#\n\
            #...........#.#\n\
            ###.#.#####.#.#\n\
            #...#.....#.#.#\n\
            #.#.#.###.#.#.#\n\
            #.....#...#.#.#\n\
            #.###.#.#.#.#.#\n\
            #S..#.....#...#\n\
            ###############\
        ";

        let map = parse_map(input);
        assert_eq!(map.grid.to_string(), input);
        let paths = find_paths_a_star(&map);
        assert_eq!(count_path(&paths[0]), 7036);

        let input = "\
            #################\n\
            #...#...#...#..E#\n\
            #.#.#.#.#.#.#.#.#\n\
            #.#.#.#...#...#.#\n\
            #.#.#.#.###.#.#.#\n\
            #...#.#.#.....#.#\n\
            #.#.#.#.#.#####.#\n\
            #.#...#.#.#.....#\n\
            #.#.#####.#.###.#\n\
            #.#.#.......#...#\n\
            #.#.###.#####.###\n\
            #.#.#...#.....#.#\n\
            #.#.#.#####.###.#\n\
            #.#.#.........#.#\n\
            #.#.#.#########.#\n\
            #S#.............#\n\
            #################\
        ";

        let map = parse_map(input);
        assert_eq!(map.grid.to_string(), input);
        let paths = &find_paths_a_star(&map);
        print_paths(&map, paths);
        assert_eq!(count_path(&paths[0]), 11048);

        assert_eq!(count_path(&paths[0]), count_path(&paths[1]));

        assert_eq!(count_tiles(paths.iter()), 64);
    }
}
