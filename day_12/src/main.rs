use common::{timed, vectors, Pos};
use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,

    data: Vec<Vec<char>>,
}
impl Grid {
    fn get(&self, Pos { x, y }: &Pos) -> &char {
        &self.data[*y as usize][*x as usize]
    }

    fn is_inside(&self, Pos { x, y }: &Pos) -> bool {
        let is_negative = *x < 0 || *y < 0;
        let is_outside = *x >= self.width || *y >= self.height;

        !is_outside && !is_negative
    }

    fn new(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

        Self {
            width: grid[0].len() as isize,
            height: grid.len() as isize,
            data: grid,
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&'_ char, Pos)> {
        self.data.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| (c, (x, y).into()))
        })
    }
}

fn get_area(c: char, pos: &Pos, grid: &Grid, visited: &mut HashSet<Pos>) -> HashSet<Pos> {
    let mut area = HashSet::<Pos>::default();

    visited.insert(*pos);
    area.insert(*pos);

    for vec in vectors::CARDINAL {
        let next = *pos + vec;
        if visited.contains(&next) || is_different(c, &next, grid) {
            continue;
        } else {
            visited.insert(next);
            area.insert(next);
            area.extend(get_area(c, &next, grid, visited));
        }
    }

    area
}

fn get_all_areas(grid: &Grid) -> Vec<(char, HashSet<Pos>)> {
    let mut visited = HashSet::new();
    let mut areas = vec![];

    for (c, pos) in grid.iter() {
        if !visited.contains(&pos) {
            areas.push((*c, get_area(*c, &pos, grid, &mut visited)));
        }
    }

    areas
}

fn is_different(c: char, pos: &Pos, grid: &Grid) -> bool {
    !grid.is_inside(pos) || *grid.get(pos) != c
}

fn get_perimeter(c: char, patch: &HashSet<Pos>, grid: &Grid) -> u32 {
    let mut perimeter = 0;

    for pos in patch {
        for vec in vectors::CARDINAL {
            let next = *pos + vec;

            if is_different(c, &next, grid) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn get_tangents(vec: Pos) -> (Pos, Pos) {
    match vec {
        vectors::UP | vectors::DOWN => (vectors::LEFT, vectors::RIGHT),
        vectors::RIGHT | vectors::LEFT => (vectors::UP, vectors::DOWN),
        _ => panic!("Unexpected vector {vec:?}"),
    }
}

fn get_sides(c: char, patch: &HashSet<Pos>, grid: &Grid) -> u32 {
    fn visit_direction(
        c: char,
        pos: &Pos,
        vec: &Pos,
        tangent: &Pos,
        grid: &Grid,
        visited: &mut HashSet<Pos>,
    ) {
        let mut cursor = *pos + *tangent;
        while grid.is_inside(&cursor) && *grid.get(&cursor) == c {
            let has_fence = is_different(c, &(cursor + *vec), grid);
            if has_fence {
                visited.insert(cursor);
                cursor = cursor + *tangent;
            } else {
                break;
            }
        }
    }

    fn get_sides_of_direction(c: char, patch: &HashSet<Pos>, grid: &Grid, vec: &Pos) -> u32 {
        let mut visited = HashSet::<Pos>::new();
        let mut sides = 0;
        for pos in patch {
            if visited.contains(pos) {
                continue;
            }
            visited.insert(*pos);

            let next = *pos + *vec;
            if is_different(c, &next, grid) {
                let (n, p) = get_tangents(*vec);
                visit_direction(c, pos, vec, &n, grid, &mut visited);
                visit_direction(c, pos, vec, &p, grid, &mut visited);
                sides += 1;
            }
        }

        sides
    }

    vectors::CARDINAL
        .iter()
        .map(|vec| get_sides_of_direction(c, patch, grid, vec))
        .sum()
}

fn get_total_price(patches: &[(char, HashSet<Pos>)], grid: &Grid) -> u64 {
    patches
        .iter()
        .map(|(c, patch)| {
            let perimeter = get_perimeter(*c, patch, grid);
            let area = patch.len();

            perimeter as u64 * area as u64
        })
        .sum()
}

fn get_total_price_with_discount(patches: &[(char, HashSet<Pos>)], grid: &Grid) -> u64 {
    patches
        .iter()
        .map(|(c, patch)| {
            let sides = get_sides(*c, patch, grid);
            let area = patch.len();

            sides as u64 * area as u64
        })
        .sum()
}

fn main() {
    let grid = Grid::new(&common::read_stdin());

    let areas = get_all_areas(&grid);
    let (time, price) = timed(|| get_total_price(&areas, &grid));
    println!("Part 1: {price} in {}μs", time.as_micros());

    let (time, price) = timed(|| get_total_price_with_discount(&areas, &grid));
    println!("Part 2: {price} in {}μs", time.as_micros());
}

// Part 1: 1361494 in 354μs
// Part 2: 830516 in 8494μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_area_test() {
        let grid = Grid::new(include_str!("../area_example.txt"));

        let area = get_area('A', &Pos { x: 0, y: 0 }, &grid, &mut HashSet::default());

        let a_areas = [
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 2, y: 0 },
            Pos { x: 3, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
            Pos { x: 2, y: 1 },
            Pos { x: 3, y: 1 },
        ]
        .into();

        assert_eq!(area, a_areas);

        let all = get_all_areas(&grid);
        assert_eq!(all[0].1, a_areas);
    }

    #[test]
    fn perimeter_test() {
        let grid = Grid::new(include_str!("../example.txt"));
        let areas = get_all_areas(&grid);

        let areas: Vec<_> = areas
            .iter()
            .map(|(c, patch)| {
                let perimeter = get_perimeter(*c, patch, &grid);
                let area = patch.len();

                (*c, area, perimeter)
            })
            .collect();

        assert_eq!(
            areas,
            [
                ('R', 12, 18),
                ('I', 4, 8),
                ('C', 14, 28),
                ('F', 10, 18),
                ('V', 13, 20),
                ('J', 11, 20),
                ('C', 1, 4),
                ('E', 13, 18),
                ('I', 14, 22),
                ('M', 5, 12),
                ('S', 3, 8),
            ]
        );
    }

    #[test]
    fn sides_test() {
        let grid = Grid::new(include_str!("../example.txt"));
        let areas = get_all_areas(&grid);

        let patches: Vec<_> = areas
            .iter()
            .map(|(c, patch)| {
                let sides = get_sides(*c, patch, &grid);
                let area = patch.len();

                (*c, area, sides)
            })
            .collect();

        assert_eq!(
            patches,
            [
                ('R', 12, 10),
                ('I', 4, 4),
                ('C', 14, 22),
                ('F', 10, 12),
                ('V', 13, 10),
                ('J', 11, 12),
                ('C', 1, 4),
                ('E', 13, 8),
                ('I', 14, 16),
                ('M', 5, 6),
                ('S', 3, 6),
            ]
        );

        assert_eq!(get_total_price_with_discount(&areas, &grid), 1206);
    }
}
