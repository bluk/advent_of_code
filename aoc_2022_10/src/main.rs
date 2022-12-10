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
    itertools::process_results(io::stdin().lines(), |lines| {
        let mut cpu = Cpu::default();
        let mut pos = 0;

        for line in lines {
            let (remaining, inst) = parse_line(&line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid instruction"))?;
            assert_eq!("", remaining);

            for _ in 0..inst.num_cycles() {
                if (cpu.reg_x - 1..=cpu.reg_x + 1).contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }

                pos += 1;
                if pos % 40 == 0 {
                    println!();
                    pos = 0;
                }
            }

            match inst {
                Instruction::Noop => {}
                Instruction::Addx(signal) => cpu.reg_x += signal,
            }
        }

        Ok::<_, io::Error>(())
    })??;

    Ok(())
}
