use std::fmt::Display;

use common::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
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
        println!("{}", map.grid);
        panic!()
    }
}
