use common::{timed, Tap};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}
impl Gate {
    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    a: String,
    b: String,
    gate: Gate,
    output: String,
}

#[derive(Debug, Clone)]
struct Device {
    inputs: HashMap<String, u8>,
    commands: Vec<Command>,
}
impl Device {
    fn new(input: &str) -> Self {
        let (inputs, commands) = input.split_once("\n\n").unwrap();

        let inputs = inputs
            .lines()
            .filter_map(|line| {
                let (variable, value) = line.trim_ascii().split_once(": ")?;
                Some((variable.into(), value.parse().unwrap()))
            })
            .collect();

        let commands = commands
            .lines()
            .filter_map(|line| {
                let mut line = line.trim_ascii().split_ascii_whitespace();
                let a = line.next()?;
                let gate = line.next()?;
                let b = line.next()?;
                let _ = line.next()?;
                let output = line.next()?;

                Some(Command {
                    a: a.into(),
                    b: b.into(),
                    output: output.into(),
                    gate: match gate {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        x => panic!("Unexpected gate: {x}"),
                    },
                })
            })
            .collect::<Vec<_>>();

        Device { inputs, commands }
    }

    fn run(mut self) -> Vec<(String, u8)> {
        let mut visited = HashSet::new();

        while visited.len() < self.commands.len() {
            for (i, c) in self.commands.iter().enumerate() {
                if visited.contains(&i) {
                    continue;
                } else if self.inputs.contains_key(&c.a) && self.inputs.contains_key(&c.b) {
                    visited.insert(i);

                    let a = self.inputs[&c.a];
                    let b = self.inputs[&c.b];
                    let val = c.gate.apply(a, b);

                    self.inputs
                        .entry(c.output.clone())
                        .and_modify(|e| *e = val)
                        .or_insert(val);
                }
            }
        }

        self.inputs.into_iter().collect()
    }

    fn get_swaps(&self) -> Vec<String> {
        let max_z = self
            .commands
            .iter()
            .filter(|x| x.output.starts_with('z'))
            .map(|x| &x.output)
            .max()
            .unwrap();
        let mut incorrect = HashSet::new();

        for cmd in &self.commands {
            #[allow(clippy::if_same_then_else)]
            if cmd.output.starts_with('z') && cmd.gate != Gate::Xor && cmd.output != *max_z {
                incorrect.insert(cmd.output.clone());
            } else if cmd.gate == Gate::Xor
                && !cmd.a.starts_with(['x', 'y', 'z'])
                && !cmd.b.starts_with(['x', 'y', 'z'])
                && !cmd.output.starts_with(['x', 'y', 'z'])
            {
                incorrect.insert(cmd.output.clone());
            } else if cmd.gate == Gate::And && (cmd.a != "x00" && cmd.b != "x00") {
                for c2 in &self.commands {
                    if (cmd.output == c2.a || cmd.output == c2.b) && c2.gate != Gate::Or {
                        incorrect.insert(cmd.output.clone());
                    }
                }
            } else if cmd.gate == Gate::Xor {
                for c2 in &self.commands {
                    if (cmd.output == c2.a || cmd.output == c2.b) && c2.gate == Gate::Or {
                        incorrect.insert(cmd.output.clone());
                    }
                }
            }
        }

        let mut incorrect = Vec::from_iter(incorrect);
        incorrect.sort();
        incorrect
    }
}

fn combine(data: &[(String, u8)], prefix: char) -> u64 {
    let data = data
        .iter()
        .filter(|x| x.0.starts_with(prefix))
        .collect::<Vec<_>>()
        .tap_mut(|x| x.sort_by_key(|x| &x.0));

    data.into_iter().fold(0, |acc, (name, val)| {
        let shift: u64 = name[1..].parse().unwrap();
        let val = (*val as u64) << shift;

        acc | val
    })
}

fn main() {
    let device = Device::new(&common::read_stdin());
    let (time, result) = timed(|| combine(&device.clone().run(), 'z'));
    println!("Part 1: {result} in {}μs", time.as_micros());

    let (time, swaps) = timed(|| device.get_swaps().join(","));
    println!("Part 2: {swaps} in {}μs", time.as_micros());
}

// Part 1: 47666458872582 in 481μs
// Part 2: dnt,gdf,gwc,jst,mcm,z05,z15,z30 in 297μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let device = Device::new(include_str!("../example.txt"));

        let initial = device.inputs.clone();
        let mut result = device
            .clone()
            .run()
            .into_iter()
            .filter(|x| !initial.contains_key(&x.0))
            .collect::<Vec<_>>();
        result.sort();
        assert_eq!(
            result,
            [
                ("bfw".to_string(), 1),
                ("bqk".to_string(), 1),
                ("djm".to_string(), 1),
                ("ffh".to_string(), 0),
                ("fgs".to_string(), 1),
                ("frj".to_string(), 1),
                ("fst".to_string(), 1),
                ("gnj".to_string(), 1),
                ("hwm".to_string(), 1),
                ("kjc".to_string(), 0),
                ("kpj".to_string(), 1),
                ("kwq".to_string(), 0),
                ("mjb".to_string(), 1),
                ("nrd".to_string(), 1),
                ("ntg".to_string(), 0),
                ("pbm".to_string(), 1),
                ("psh".to_string(), 1),
                ("qhw".to_string(), 1),
                ("rvg".to_string(), 0),
                ("tgd".to_string(), 0),
                ("tnw".to_string(), 1),
                ("vdt".to_string(), 1),
                ("wpb".to_string(), 0),
                ("z00".to_string(), 0),
                ("z01".to_string(), 0),
                ("z02".to_string(), 0),
                ("z03".to_string(), 1),
                ("z04".to_string(), 0),
                ("z05".to_string(), 1),
                ("z06".to_string(), 1),
                ("z07".to_string(), 1),
                ("z08".to_string(), 1),
                ("z09".to_string(), 1),
                ("z10".to_string(), 1),
                ("z11".to_string(), 0),
                ("z12".to_string(), 0),
            ]
        );

        assert_eq!(combine(&result, 'z'), 2024);
    }

    #[test]
    fn swap_test() {
        let input = "
            x00: 0
            x01: 1
            x02: 0
            x03: 1
            x04: 0
            x05: 1
            y00: 0
            y01: 0
            y02: 1
            y03: 1
            y04: 0
            y05: 1

            x00 AND y00 -> z05
            x01 AND y01 -> z02
            x02 AND y02 -> z01
            x03 AND y03 -> z03
            x04 AND y04 -> z04
            x05 AND y05 -> z00
        ";
        let device = Device::new(input);
        assert_eq!(device.get_swaps(), ["x".to_string()]);
    }
}
