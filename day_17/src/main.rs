use common::timed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    Register(Register),
    Reserved,
}
impl Operand {
    fn combo(&self, registers: &Registers) -> u64 {
        match self {
            Self::Literal(x) => *x as u64,
            Self::Register(r) => match r {
                Register::A => registers.a,
                Register::B => registers.b,
                Register::C => registers.c,
            },
            Self::Reserved => panic!("reserved operand"),
        }
    }
}

fn operand(x: u8) -> Operand {
    x.into()
}

impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        match value {
            x @ 0..=3 => Self::Literal(x),
            4 => Self::Register(Register::A),
            5 => Self::Register(Register::B),
            6 => Self::Register(Register::C),
            7 => Self::Reserved,

            x => panic!("Unexpected operand: {x}"),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Adv, // A := A / 2^x (shift right?)
    Bxl, // B := B xor x
    Bst, // B := x % 8
    Jnz, // jump not equal, using a as test
    Bxc, // B := B ^ C
    Out, // return (x % 8)
    Bdv, // B := A / 2^x
    Cdv, // C := A / 2^x
}
impl Opcode {
    fn run(&self, output: &mut Vec<u8>, pc: &mut usize, registers: &mut Registers, op: u8) -> bool {
        match self {
            Self::Adv => {
                registers.a >>= operand(op).combo(registers);
            }
            Self::Bxl => {
                registers.b ^= op as u64;
            }
            Self::Bst => {
                registers.b = operand(op).combo(registers) % 8;
            }
            Self::Jnz => {
                if registers.a != 0 {
                    *pc = op as usize;
                    return false;
                }
            }
            Self::Bxc => {
                registers.b ^= registers.c;
            }
            Self::Out => {
                let val = operand(op).combo(registers) % 8;
                output.push(val as u8);
            }
            Self::Bdv => {
                registers.b = registers.a >> operand(op).combo(registers);
            }
            Self::Cdv => {
                registers.c = registers.a >> operand(op).combo(registers);
            }
        }

        true
    }
}
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        use Opcode::*;
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,

            x => panic!("Unexpected opcode: {x}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Machine {
    pc: usize,
    output: Vec<u8>,
    registers: Registers,
}
impl Machine {
    fn with_registers(registers: Registers) -> Self {
        Self {
            registers,
            ..Default::default()
        }
    }

    fn step(mut self, program: &[u8]) -> Self {
        let opcode_idx = program[self.pc];
        let operand = program[self.pc + 1];

        let opcode = Opcode::from(opcode_idx);
        // let operand = Operand::from(operand_idx);

        // println!("{opcode_idx}: {opcode:?}, {operand_idx}:{operand:?}");

        if opcode.run(&mut self.output, &mut self.pc, &mut self.registers, operand) {
            self.pc += 2;
        }

        self
    }

    fn run(mut self, program: &[u8]) -> Self {
        let mut loop_count = 0;
        while self.pc < program.len() {
            self = self.step(program);
            loop_count += 1;
            if loop_count == 1_000_000 {
                panic!("inf loop");
            }
        }

        self
    }
}

fn parse_input(input: &str) -> (Machine, Vec<u8>) {
    let mut lines = input.lines();

    fn parse_register(line: &str) -> u64 {
        line.split_once(':')
            .unwrap()
            .1
            .trim_ascii()
            .parse()
            .unwrap()
    }

    fn parse_program(line: &str) -> Vec<u8> {
        line.split_once(':')
            .unwrap()
            .1
            .trim_ascii()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());
    lines.next().unwrap();
    let program = parse_program(lines.next().unwrap());

    (Machine::with_registers(Registers { a, b, c }), program)
}

fn join_output(output: &[u8]) -> String {
    output
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |mut s, x| {
            if s.is_empty() {
                x
            } else {
                s.push(',');
                s.push_str(&x);
                s
            }
        })
}

fn main() {
    let (machine, program) = parse_input(&common::read_stdin());

    let (time, output) = timed(|| {
        let Machine { output, .. } = machine.run(&program);
        join_output(&output)
    });
    println!("Part 1: {output} in {}μs", time.as_micros());

    let (time, value) = timed(|| find_output(&program));
    println!("Part 2: {value} in {}μs", time.as_micros());
}

// Part 1: 6,4,6,0,4,5,7,2,7 in 2μs
// Part 2: 164541160582845 in 19μs

fn find_output(expected: &[u8]) -> u64 {
    let mut a = 0;
    let mut shift = 0;

    'out: loop {
        for i in 0..=7 {
            let a_val = a + i;
            let output = part2(a_val);

            if output.is_empty() {
                continue;
            } else if output == expected {
                return a_val;
            } else if output[output.len() - shift - 1] == expected[expected.len() - shift - 1] {
                a += i;
                a <<= 3;
                shift += 1;
                // println!("{a_val}: {output:?}");
                continue 'out;
            }
        }

        // Doesn't work for first value 
        // So brute force last leg
        for i in 0..=1000 {
            let a_val = a + i;
            let output = part2(a_val);
            if output == expected {
                // println!("{a_val}: {output:?}");
                return a_val;
            }
        }

        panic!("none found")
    }
}

fn part2(a: u64) -> Vec<u8> {
    // Bst: [4] Register(A)
    // Bxl: [1] Literal(1)
    // Cdv: [5] Register(B)
    // Bxl: [5] Register(B)
    // Bxc: [0] Literal(0)
    // Adv: [3] Literal(3)
    // Out: [5] Register(B)
    // Jnz: [0] Literal(0)

    let mut registers = Registers {
        a,
        ..Default::default()
    };
    let mut output = vec![];

    while registers.a != 0 {
        registers.b = registers.a & 0b111; // last 3 A bits
        registers.b ^= 1; // flip last bit
        registers.c = registers.a >> registers.b;
        registers.b ^= 5;
        registers.b ^= registers.c;
        registers.a >>= 3;
        output.push((registers.b & 0b111) as u8);
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn program_test() {
        let m = Machine::with_registers(Registers {
            c: 9,
            ..Default::default()
        });
        let m = m.run(&[2, 6]);
        assert_eq!(m.registers.b, 1);

        let m = Machine::with_registers(Registers {
            a: 10,
            ..Default::default()
        });
        let m = m.run(&[5, 0, 5, 1, 5, 4]);
        assert_eq!(m.output, [0, 1, 2]);

        let m = Machine::with_registers(Registers {
            a: 2024,
            ..Default::default()
        });
        let m = m.run(&[0, 1, 5, 4, 3, 0]);
        assert_eq!(m.output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(m.registers.a, 0);

        let m = Machine::with_registers(Registers {
            b: 29,
            ..Default::default()
        })
        .run(&[1, 7]);
        assert_eq!(m.registers.b, 26);

        let m = Machine::with_registers(Registers {
            b: 2024,
            c: 43690,
            ..Default::default()
        })
        .run(&[4, 0]);
        assert_eq!(m.registers.b, 44354);

        let m = Machine::with_registers(Registers {
            a: 729,
            ..Default::default()
        })
        .run(&[0, 1, 5, 4, 3, 0]);
        assert_eq!(m.output, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0])
    }
}
