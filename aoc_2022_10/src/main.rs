use std::io;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn num_cycles(self) -> u64 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Cpu {
    reg_x: i64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self { reg_x: 1 }
    }
}

fn parse_line(line: &str) -> nom::IResult<&str, Instruction> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, opt, recognize},
        sequence::{preceded, tuple},
    };

    alt((
        map(tag("noop"), |_| Instruction::Noop),
        map(
            preceded(
                tag("addx "),
                map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse),
            ),
            Instruction::Addx,
        ),
    ))(line)
}

fn main() -> io::Result<()> {
    let answer = itertools::process_results(io::stdin().lines(), |lines| {
        let mut cpu = Cpu::default();
        let mut cycle = 0;
        let mut signal_sum = 0;

        for line in lines {
            let (remaining, inst) = parse_line(&line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid instruction"))?;
            assert_eq!("", remaining);

            for _ in 0..inst.num_cycles() {
                cycle += 1;

                if (cycle == 20) || ((60..=220).contains(&cycle) && (cycle - 20) % 40 == 0) {
                    signal_sum += cycle * cpu.reg_x;
                }
            }

            match inst {
                Instruction::Noop => {}
                Instruction::Addx(signal) => cpu.reg_x += signal,
            }
        }

        Ok::<_, io::Error>(signal_sum)
    })??;

    println!("{answer}");

    Ok(())
}
