use common::timed;
use std::fmt::Display;

use common::Pos;
use pathfinding::directed::astar::astar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Safe,
    Corrupted,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Safe => f.write_char('.'),
            Self::Corrupted => f.write_char('#'),
        }
    }
}

type Grid = common::Grid<Block>;

fn add_bytes_to_grid(mut grid: Grid, incoming: &[Pos], bytes: usize) -> Grid {
    (0..bytes).for_each(|i| {
        let next = incoming[i];
        grid[next] = Block::Corrupted;
    });

    grid
}

fn find_path(grid: &Grid) -> Option<Vec<Pos>> {
    let start = Pos { x: 0, y: 0 };
    let end = Pos {
        x: grid.width - 1,
        y: grid.height - 1,
    };

    fn heuristic(a: Pos, b: Pos) -> u64 {
        a.distance(&b) as u64
    }

    fn successors(grid: &Grid, node: Pos) -> Vec<(Pos, u64)> {
        grid.iter_adjacent_cardinal(node)
            .filter(|(b, _)| matches!(b, Block::Safe))
            .map(|(_, p)| (p, 1))
            .collect()
    }

    astar(
        &start,
        |state| successors(grid, *state),
        |state| heuristic(*state, end),
        |state| *state == end,
    )
    .map(|x| x.0)
}

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();

            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()).into()
        })
        .collect()
}

fn get_first_blocking(mut grid: Grid, incoming: &[Pos]) -> Pos {
    fn find_path_bfs(grid: &Grid) -> Option<Vec<Pos>> {
        use pathfinding::directed::bfs::bfs;

        let start = Pos { x: 0, y: 0 };
        let end = Pos {
            x: grid.width - 1,
            y: grid.height - 1,
        };

        fn successors(grid: &Grid, node: Pos) -> Vec<Pos> {
            grid.iter_adjacent_cardinal(node)
                .filter(|(b, _)| matches!(b, Block::Safe))
                .map(|(_, p)| p)
                .collect()
        }

        bfs(&start, |x| successors(grid, *x), |x| *x == end)
    }

    for p in incoming {
        grid[*p] = Block::Corrupted;

        if find_path_bfs(&grid).is_none() {
            return *p;
        }
    }

    panic!("all clear")
}

fn main() {
    let grid = make_grid(71, 71);
    let incoming = parse_input(&common::read_stdin());

    let (time, (blocked_grid, path)) = timed(|| {
        let blocked_grid = add_bytes_to_grid(grid, &incoming, 1024);
        let path = find_path(&blocked_grid).unwrap();

        (blocked_grid, path)
    });

    println!("Part 1: {} in {}μs", path.len() - 1, time.as_micros());

    let (time, first_blocking) = timed(|| get_first_blocking(blocked_grid, &incoming[1024..]));
    println!(
        "Part 2: {},{} in {}ms",
        first_blocking.x,
        first_blocking.y,
        time.as_millis()
    );
}

// Part 1: 308 in 1327μs
// Part 2: 46,28 in 819ms

fn make_grid(width: usize, height: usize) -> Grid {
    Grid {
        width: width as isize,
        height: height as isize,
        data: (0..height)
            .map(|_| (0..width).map(|_| Block::Safe).collect())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path_grid(grid: &Grid, path: &[Pos]) -> String {
        let mut grid = common::Grid {
            width: grid.width,
            height: grid.height,
            data: grid
                .data
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|x| x.to_string().chars().next().unwrap())
                        .collect()
                })
                .collect(),
        };

        for p in path {
            grid[*p] = 'O';
        }

        grid.to_string()
    }

    #[test]
    fn find_path_test() {
        let grid = make_grid(7, 7);
        let incoming: [Pos; 25] = [
            (5, 4).into(),
            (4, 2).into(),
            (4, 5).into(),
            (3, 0).into(),
            (2, 1).into(),
            (6, 3).into(),
            (2, 4).into(),
            (1, 5).into(),
            (0, 6).into(),
            (3, 3).into(),
            (2, 6).into(),
            (5, 1).into(),
            (1, 2).into(),
            (5, 5).into(),
            (2, 5).into(),
            (6, 5).into(),
            (1, 4).into(),
            (0, 4).into(),
            (6, 4).into(),
            (1, 1).into(),
            (6, 1).into(),
            (1, 0).into(),
            (0, 5).into(),
            (1, 6).into(),
            (2, 0).into(),
        ];

        let grid = add_bytes_to_grid(grid, &incoming, 12);
        println!("{}", grid);
        println!();

        let path = find_path(&grid).unwrap();
        let path_string = get_path_grid(&grid, &path);

        let expected_path = "\
            OO.#OOO\n\
            .O#OO#O\n\
            .OOO#OO\n\
            ...#OO#\n\
            ..#OO#.\n\
            .#.O#..\n\
            #.#OOOO\
        ";

        assert_eq!(path_string, expected_path);
        assert_eq!(path.len() - 1, 22);

        assert_eq!(
            get_first_blocking(grid, &incoming[12..]),
            Pos { x: 6, y: 1 }
        );
    }
}
