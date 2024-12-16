use common::{timed, vectors, Grid, Pos};
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
    Box,
    LargeBoxLeft,
    LargeBoxRight,
    Robot,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => f.write_char('.'),
            Block::Wall => f.write_char('#'),
            Block::Box => f.write_char('O'),
            Block::Robot => f.write_char('@'),
            Block::LargeBoxLeft => f.write_char('['),
            Block::LargeBoxRight => f.write_char(']'),
        }
    }
}

fn extend_with<T>(mut v: Vec<T>, i: impl IntoIterator<Item = T>) -> Vec<T> {
    v.extend(i);
    v
}

fn remove_duplicates<T>(v: Vec<T>) -> Vec<T>
where
    T: Hash + Eq + Copy,
{
    let mut encountered = HashSet::<T>::new();

    v.into_iter()
        .filter(|x| {
            if encountered.contains(x) {
                false
            } else {
                encountered.insert(*x);
                true
            }
        })
        .collect()
}

#[derive(Debug, Clone, Default)]
struct Map {
    robot: Pos,
    grid: Grid<Block>,
}
impl Map {
    fn expand(&self) -> Self {
        use std::iter::repeat_n;

        let mut new_grid = vec![];
        let mut robot = Pos::default();

        for y in 0..self.grid.height {
            let mut line = vec![];
            for x in 0..self.grid.width {
                let elem = self.grid[(x, y)];
                match elem {
                    Block::Robot => {
                        line.push(Block::Robot);
                        robot = Pos {
                            x: line.len() as isize - 1,
                            y,
                        };
                        line.push(Block::Empty);
                    }
                    Block::Box => {
                        line.extend([Block::LargeBoxLeft, Block::LargeBoxRight]);
                    }
                    elem => line.extend(repeat_n(elem, 2)),
                }
            }
            new_grid.push(line);
        }

        Self {
            robot,
            grid: Grid::new(new_grid),
        }
    }

    fn run(mut self, cmd: Command) -> Self {
        fn find_last_touching_box(grid: &Grid<Block>, pos: Pos, vec: Pos) -> Pos {
            let next = pos + vec;
            match grid.get(&next) {
                Some(Block::Box) => find_last_touching_box(grid, next, vec),
                _ => pos,
            }
        }

        fn get_large_boxes_in_dir(
            grid: &Grid<Block>,
            left_pos: Pos,
            right_pos: Pos,
            vec: Pos,
        ) -> Vec<(Pos, Pos)> {
            fn match_vertical(grid: &Grid<Block>, pos: Pos, vec: Pos) -> Vec<(Pos, Pos)> {
                match grid.get(&(pos + vec)) {
                    Some(Block::LargeBoxLeft) => {
                        let left_pos = pos + vec;
                        let right_pos = left_pos + vectors::RIGHT;

                        extend_with(
                            vec![(left_pos, right_pos)],
                            get_large_boxes_in_dir(grid, left_pos, right_pos, vec),
                        )
                    }
                    Some(Block::LargeBoxRight) => {
                        let right_pos = pos + vec;
                        let left_pos = right_pos + vectors::LEFT;
                        extend_with(
                            vec![(left_pos, right_pos)],
                            get_large_boxes_in_dir(grid, left_pos, right_pos, vec),
                        )
                    }
                    _ => vec![],
                }
            }

            match vec {
                vectors::LEFT => {
                    let next_pos = left_pos + vec;
                    if let Some(Block::LargeBoxRight) = grid.get(&next_pos) {
                        extend_with(
                            vec![(next_pos + vec, next_pos)],
                            get_large_boxes_in_dir(grid, next_pos + vec, next_pos, vec),
                        )
                    } else {
                        vec![]
                    }
                }
                vectors::RIGHT => {
                    let next_pos = right_pos + vec;
                    if let Some(Block::LargeBoxLeft) = grid.get(&next_pos) {
                        extend_with(
                            vec![(next_pos, next_pos + vec)],
                            get_large_boxes_in_dir(grid, next_pos, next_pos + vec, vec),
                        )
                    } else {
                        vec![]
                    }
                }

                vectors::UP => {
                    let mut v = extend_with(
                        match_vertical(grid, left_pos, vec),
                        match_vertical(grid, right_pos, vec),
                    );
                    v.sort_by(|(_, a), (_, b)| b.y.cmp(&a.y));
                    v
                }
                vectors::DOWN => {
                    let mut v = extend_with(
                        match_vertical(grid, left_pos, vec),
                        match_vertical(grid, right_pos, vec),
                    );
                    v.sort_by(|(_, a), (_, b)| a.y.cmp(&b.y));
                    v
                }

                _ => panic!(),
            }
        }

        fn advance_robot(mut map: Map, next: Pos) -> Map {
            map.grid.swap(map.robot, next);
            Map {
                robot: next,
                grid: map.grid,
            }
        }

        fn can_move_large_box(grid: &Grid<Block>, left_pos: Pos, right_pos: Pos, vec: Pos) -> bool {
            fn is_moveable(grid: &Grid<Block>, pos: Pos) -> bool {
                matches!(
                    grid.get(&pos),
                    Some(Block::Empty | Block::LargeBoxRight | Block::LargeBoxLeft)
                )
            }

            match vec {
                vectors::LEFT => is_moveable(grid, left_pos + vec),
                vectors::RIGHT => is_moveable(grid, right_pos + vec),
                vectors::DOWN | vectors::UP => {
                    is_moveable(grid, left_pos + vec) && is_moveable(grid, right_pos + vec)
                }
                _ => panic!(),
            }
        }

        fn move_large_box(grid: &mut Grid<Block>, left_pos: Pos, right_pos: Pos, vec: Pos) {
            match vec {
                vectors::LEFT => {
                    grid.swap(left_pos + vec, left_pos);
                    grid.swap(right_pos, left_pos);
                }
                vectors::RIGHT => {
                    grid.swap(right_pos + vec, right_pos);
                    grid.swap(left_pos, right_pos);
                }
                vectors::UP | vectors::DOWN => {
                    grid.swap(left_pos, left_pos + vec);
                    grid.swap(right_pos, right_pos + vec);
                }
                _ => panic!(),
            }
        }

        let vec = match cmd {
            Command::Up => vectors::UP,
            Command::Down => vectors::DOWN,
            Command::Left => vectors::LEFT,
            Command::Right => vectors::RIGHT,
        };

        let next_pos = self.robot + vec;

        fn handle_large_box(
            mut map: Map,
            next_pos: Pos,
            left_pos: Pos,
            right_pos: Pos,
            vec: Pos,
        ) -> Map {
            let boxes = get_large_boxes_in_dir(&map.grid, left_pos, right_pos, vec);
            let boxes = extend_with(vec![(left_pos, right_pos)], boxes);
            let boxes = remove_duplicates(boxes);

            if boxes
                .iter()
                .rev()
                .all(|(left, right)| can_move_large_box(&map.grid, *left, *right, vec))
            {
                for b in boxes.iter().rev() {
                    move_large_box(&mut map.grid, b.0, b.1, vec);
                }
                advance_robot(map, next_pos)
            } else {
                map
            }
        }

        match self.grid.get(&next_pos) {
            Some(Block::Wall) => self,
            Some(Block::Box) => {
                let last_touching = find_last_touching_box(&self.grid, next_pos, vec);
                if let Some(Block::Empty) = self.grid.get(&(last_touching + vec)) {
                    self.grid.swap(last_touching, last_touching + vec);
                    self.grid.swap(next_pos, last_touching);

                    advance_robot(self, next_pos)
                } else {
                    self
                }
            }
            Some(Block::LargeBoxLeft) => {
                let right_pos = next_pos + vectors::RIGHT;
                handle_large_box(self, next_pos, next_pos, right_pos, vec)
            }
            Some(Block::LargeBoxRight) => {
                let left_pos = next_pos + vectors::LEFT;
                handle_large_box(self, next_pos, left_pos, next_pos, vec)
            }
            Some(Block::Empty) => advance_robot(self, next_pos),
            Some(Block::Robot) => panic!("multiple robots?\n{}", self.grid),
            None => panic!("out of bounds: {next_pos}"),
        }
    }

    fn run_all(self, commands: &[Command]) -> Self {
        commands.iter().fold(self, |map: Self, cmd| map.run(*cmd))
    }

    fn get_box_coords(&self) -> i64 {
        self.grid
            .iter()
            .filter(|(b, _)| matches!(b, Block::Box | Block::LargeBoxLeft))
            .map(|(_, p)| (100 * p.y + p.x) as i64)
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

fn parse_map(input: &str) -> Map {
    let mut robot = Pos::default();

    let data: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Block::Wall,
                    'O' => Block::Box,
                    '@' => {
                        robot = (x, y).into();
                        Block::Robot
                    }
                    '[' => Block::LargeBoxLeft,
                    ']' => Block::LargeBoxRight,
                    _ => Block::Empty,
                })
                .collect()
        })
        .collect();

    Map {
        robot,
        grid: Grid::new(data),
    }
}

fn parse_commands(commands: &str) -> Vec<Command> {
    commands
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Command::Up),
            'v' => Some(Command::Down),
            '<' => Some(Command::Left),
            '>' => Some(Command::Right),
            _ => None,
        })
        .collect()
}

fn parse_input(input: &str) -> (Map, Vec<Command>) {
    let (map, commands) = input.split_once("\n\n").unwrap();
    (parse_map(map), parse_commands(commands))
}

fn main() {
    let (map, commands) = parse_input(&common::read_stdin());

    let (time, small) = timed(|| map.clone().run_all(&commands));
    println!(
        "Part 1: {} in {}μs",
        small.get_box_coords(),
        time.as_micros()
    );

    let (time, large) = timed(|| map.expand().run_all(&commands));
    println!(
        "Part 2: {} in {}μs",
        large.get_box_coords(),
        time.as_micros()
    );
}

// Part 1: 1413675 in 644μs
// Part 2: 1399772 in 2862μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let (map, commands) = parse_input(include_str!("../example.txt"));
        let map = map.run_all(&commands);
        assert_eq!(
            format!("{}", map.grid),
            include_str!("../example_answer.txt").trim_ascii()
        );
        assert_eq!(map.get_box_coords(), 2028);

        let (map, commands) = parse_input(include_str!("../example_large.txt"));
        let map = map.run_all(&commands);
        assert_eq!(
            format!("{}", map.grid),
            include_str!("../example_large_answer.txt").trim_ascii()
        );
        assert_eq!(map.get_box_coords(), 10092);
    }

    #[test]
    fn expand_test() {
        let input = "\
            #######\n\
            #...#.#\n\
            #.....#\n\
            #..OO@#\n\
            #..O..#\n\
            #.....#\n\
            #######\
        ";

        let expected = "\
            ##############\n\
            ##......##..##\n\
            ##..........##\n\
            ##....[][]@.##\n\
            ##....[]....##\n\
            ##..........##\n\
            ##############\
        ";

        let map = parse_map(input);
        let expanded = map.expand();

        assert_eq!(format!("{}", expanded.grid), expected);
        assert_eq!(expanded.robot, Pos { x: 10, y: 3 });

        let commands = parse_commands("<vv<<^^<<^^");
        let expanded = expanded.run_all(&commands);

        assert_eq!(
            format!("{}", expanded.grid),
            "\
            ##############\n\
            ##...[].##..##\n\
            ##...@.[]...##\n\
            ##....[]....##\n\
            ##..........##\n\
            ##..........##\n\
            ##############\
        "
        );
    }

    #[test]
    fn move_large_test() {
        let input = "\
            ###################\n\
            ##...............##\n\
            ##.....[]........##\n\
            ##....[].........##\n\
            ##...[][]........##\n\
            ##..[][].........##\n\
            ##...[]..........##\n\
            ##....@..........##\n\
            ##...............##\n\
            ###################\
        ";

        let expected = "\
            ###################\n\
            ##.....[]........##\n\
            ##....[].........##\n\
            ##...[][]........##\n\
            ##..[][].........##\n\
            ##...[]..........##\n\
            ##....@..........##\n\
            ##...............##\n\
            ##...............##\n\
            ###################\
        ";

        let map = parse_map(input).run(Command::Up);
        assert_eq!(map.grid.to_string(), expected);

        let input = "\
            ###################\n\
            ##.....@.........##\n\
            ##.....[]........##\n\
            ##....[].........##\n\
            ##...[][]........##\n\
            ##..[][].........##\n\
            ##...[]..........##\n\
            ##...............##\n\
            ##...............##\n\
            ###################\
        ";

        let expected = "\
            ###################\n\
            ##...............##\n\
            ##.....@.........##\n\
            ##.....[]........##\n\
            ##....[].........##\n\
            ##...[][]........##\n\
            ##..[][].........##\n\
            ##...[]..........##\n\
            ##...............##\n\
            ###################\
        ";

        let map = parse_map(input).run(Command::Down);
        assert_eq!(map.grid.to_string(), expected);
    }

    #[test]
    fn count_large_test() {
        let input = "\
            ####################\n\
            ##[].......[].[][]##\n\
            ##[]...........[].##\n\
            ##[]........[][][]##\n\
            ##[]......[]....[]##\n\
            ##..##......[]....##\n\
            ##..[]............##\n\
            ##..@......[].[][]##\n\
            ##......[][]..[]..##\n\
            ####################\
        ";

        assert_eq!(parse_map(input).get_box_coords(), 9021)
    }
}
