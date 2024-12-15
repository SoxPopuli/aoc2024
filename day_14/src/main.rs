use common::timed;
use std::collections::HashSet;

use common::Pos;

#[derive(Debug, Clone)]
struct Robot {
    position: Pos,
    velocity: Pos,
}

#[derive(Debug, Clone)]
struct Grid {
    width: isize,
    height: isize,

    robots: Vec<Robot>,
}
impl Grid {
    fn wrap_position(&self, pos: &Pos) -> Pos {
        Pos {
            x: pos.x.rem_euclid(self.width),
            y: pos.y.rem_euclid(self.height),
        }
    }

    fn step_robot(&self, robot: &Robot, steps: isize) -> Robot {
        let new_pos = robot.position + (robot.velocity * steps);
        let new_pos = self.wrap_position(&new_pos);

        Robot {
            position: new_pos,
            ..*robot
        }
    }

    fn simulate(&self, steps: isize) -> Self {
        let robots = self
            .robots
            .iter()
            .map(|r| self.step_robot(r, steps))
            .collect();

        Self {
            robots,
            ..self.clone()
        }
    }

    fn count_quadrants(&self) -> i32 {
        use std::cmp::Ordering;

        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        let x_middle = self.width / 2;
        let y_middle = self.height / 2;

        for r in &self.robots {
            let x = r.position.x.cmp(&x_middle);
            let y = r.position.y.cmp(&y_middle);

            match (x, y) {
                (Ordering::Equal, _) | (_, Ordering::Equal) => continue,

                (Ordering::Less, Ordering::Less) => top_left += 1,
                (Ordering::Greater, Ordering::Less) => top_right += 1,
                (Ordering::Less, Ordering::Greater) => bottom_left += 1,
                (Ordering::Greater, Ordering::Greater) => bottom_right += 1,
            }
        }

        top_left * top_right * bottom_left * bottom_right
    }

    fn is_tree(&self) -> bool {
        let mut positions = HashSet::<Pos>::new();

        for r in &self.robots {
            if positions.contains(&r.position) {
                return false;
            }

            positions.insert(r.position);
        }

        true
    }
}

fn parse_positions(input: &str) -> Vec<Robot> {
    fn parse_line(line: &str) -> Robot {
        let mut parts = line.split([',', ' ']).map(|x| {
            x.chars()
                .filter(|x| x.is_ascii_digit() || *x == '-')
                .collect::<String>()
                .parse()
                .unwrap()
        });

        Robot {
            position: Pos {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
            },
            velocity: Pos {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
            },
        }
    }

    input.lines().map(parse_line).collect()
}

fn main() {
    let robots = parse_positions(&common::read_stdin());

    let mut grid = Grid {
        width: 101,
        height: 103,
        robots,
    };

    let (time, safety) = timed(|| grid.simulate(100).count_quadrants());
    println!("Part 1: {safety} in {}μs", time.as_micros());

    let (time, iterations) = timed(|| {
        let mut iterations = 0;
        while !grid.is_tree() {
            grid = grid.simulate(1);
            iterations += 1;
        }
        iterations
    });
    println!("Part 2: {iterations} in {}ms", time.as_millis());
}

// Part 1: 230436441 in 62μs
// Part 2: 8270 in 456ms

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Grid {
            width: 11,
            height: 7,
            robots: parse_positions(include_str!("../example.txt")),
        };

        let simulated = input.simulate(100);
        assert_eq!(simulated.count_quadrants(), 12);
    }
}
