use common::{read_stdin, timed};

trait Vec2d<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T>;
}
impl<T> Vec2d<T> for Vec<Vec<T>> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.as_slice().get(y).and_then(|row| row.as_slice().get(x))
    }
}

type Array = Vec<Vec<char>>;

fn string_to_array(input: &str) -> Array {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn new_pos(x: usize, y: usize, delta_x: isize, delta_y: isize) -> (usize, usize) {
    let new_x = x as isize + delta_x;
    let new_y = y as isize + delta_y;
    (new_x as usize, new_y as usize)
}

fn search_all_directions(data: &Array, x: usize, y: usize) -> i32 {
    let vectors = [
        // Right
        (1, 0),
        // Up
        (0, -1),
        // Down
        (0, 1),
        // Left
        (-1, 0),
        // Up-Left
        (-1, -1),
        // Up-Right
        (1, -1),
        // Down-Left
        (-1, 1),
        // Down-Right
        (1, 1),
    ];

    let mut count = 0;

    for (delta_x, delta_y) in vectors {
        let m_pos = new_pos(x, y, delta_x, delta_y);
        let a_pos = new_pos(x, y, delta_x * 2, delta_y * 2);
        let s_pos = new_pos(x, y, delta_x * 3, delta_y * 3);
        if data.get(m_pos.0, m_pos.1) == Some(&'M')
            && data.get(a_pos.0, a_pos.1) == Some(&'A')
            && data.get(s_pos.0, s_pos.1) == Some(&'S')
        {
            count += 1;
        }
    }

    count
}

fn find_matches(data: &Array) -> i32 {
    let height = data.len();
    let width = data[0].len();

    let mut count = 0;

    for col in 0..width {
        for row in 0..height {
            if data.get(col, row) == Some(&'X') {
                let matches = search_all_directions(data, col, row);
                count += matches;
            }
        }
    }

    count
}

fn search_all_directions_2(data: &Array, x: usize, y: usize) -> bool {
    struct Diagonals {
        up_left: char,
        up_right: char,
        down_left: char,
        down_right: char,
    }

    fn get_diagonals(data: &Array, x: usize, y: usize) -> Option<Diagonals> {
        let up_left = new_pos(x, y, -1, -1);
        let up_right = new_pos(x, y, 1, -1);
        let down_left = new_pos(x, y, -1, 1);
        let down_right = new_pos(x, y, 1, 1);

        Some(Diagonals {
            up_left: data.get(up_left.0, up_left.1).cloned()?,
            up_right: data.get(up_right.0, up_right.1).cloned()?,
            down_left: data.get(down_left.0, down_left.1).cloned()?,
            down_right: data.get(down_right.0, down_right.1).cloned()?,
        })
    }

    fn has_down_right_diagonal(diagonals: &Diagonals) -> bool {
        (diagonals.up_left == 'M' && diagonals.down_right == 'S')
            || (diagonals.up_left == 'S' && diagonals.down_right == 'M')
    }

    fn has_up_right_diagonal(diagonals: &Diagonals) -> bool {
        (diagonals.down_left == 'M' && diagonals.up_right == 'S')
            || (diagonals.down_left == 'S' && diagonals.up_right == 'M')
    }

    let diagonals = get_diagonals(data, x, y);
    if let Some(diagonals) = diagonals {
        if has_up_right_diagonal(&diagonals) && has_down_right_diagonal(&diagonals) {
            return true;
        }
    }

    false
}

fn find_matches_2(data: &Array) -> i32 {
    let height = data.len();
    let width = data[0].len();

    let mut count = 0;

    for col in 0..width {
        for row in 0..height {
            if data.get(col, row) == Some(&'A') && search_all_directions_2(data, col, row) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = read_stdin();
    let array = string_to_array(&input);

    let (time, ranges) = timed(|| find_matches(&array));
    println!("Part 1: {} in {}μs", ranges, time.as_micros());

    let (time, matches) = timed(|| find_matches_2(&array));
    println!("Part 2: {matches} in {}μs", time.as_micros());
}

// Part 1: 2547 in 430μs
// Part 2: 1939 in 159μs

#[cfg(test)]
mod tests {
    use crate::{find_matches, find_matches_2, string_to_array, Vec2d};

    #[test]
    fn two_dimensional_array_test() {
        let input = "abc\ndef\nghi";
        let array = string_to_array(input);

        assert_eq!(array[1][0], 'd');
        assert_eq!(array.get(0, 1), Some(&'d'));

        assert_eq!(array, [['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i'],]);
    }

    #[test]
    fn part1() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        let array = string_to_array(input);
        let matches = find_matches(&array);

        assert_eq!(matches, 18);
    }

    #[test]
    fn part2() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        let array = string_to_array(input);
        let matches = find_matches_2(&array);

        assert_eq!(matches, 9);
    }
}
