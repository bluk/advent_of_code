use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io;

use crate::error::Error;

/// Used to read input for the program.
///
/// Mainly used to allow easier testing.
pub trait ProgInput {
    fn read(&mut self) -> Result<String, Error>;
}

/// Used to write output from the program.
///
/// Mainly used to allow easier testing.
pub trait ProgOutput {
    fn write(&mut self, output: &str) -> Result<(), Error>;
}

impl ProgInput for VecDeque<String> {
    fn read(&mut self) -> Result<String, Error> {
        if let Some(value) = self.pop_front() {
            Ok(value)
        } else {
            Err(Error::NoAvailableInput)
        }
    }
}

impl ProgOutput for VecDeque<String> {
    fn write(&mut self, output: &str) -> Result<(), Error> {
        self.push_back(output.to_string());
        Ok(())
    }
}

/// Reads in program input from stdin.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct StdInProgInput {}

impl StdInProgInput {
    pub fn new() -> Self {
        StdInProgInput {}
    }
}

impl ProgInput for StdInProgInput {
    fn read(&mut self) -> Result<String, Error> {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input)?;
        Ok(input)
    }
}

/// Writes program output to stdout.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct StdOutProgOutput {}

impl StdOutProgOutput {
    pub fn new() -> Self {
        StdOutProgOutput {}
    }
}

impl ProgOutput for StdOutProgOutput {
    fn write(&mut self, output: &str) -> Result<(), Error> {
        println!("{}", output);
        Ok(())
    }
}

/// Parse a string into memory state.
pub fn parse_mem_state(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>())
        .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

/// The operation as well as the parameter modes for operands.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustsRelativeBase,
    Halt,
}

fn param_mode(param: u32, op: i64) -> ParamMode {
    match (op % 10i64.pow(param + 3)) / 10i64.pow(param + 2) {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => panic!("unexpected parameter mode"),
    }
}

fn decode_op_code(op: i64) -> OpCode {
    let op_code = op % 100;
    match op_code {
        1 => OpCode::Add,
        2 => OpCode::Mul,
        3 => OpCode::Input,
        4 => OpCode::Output,
        5 => OpCode::JumpIfTrue,
        6 => OpCode::JumpIfFalse,
        7 => OpCode::LessThan,
        8 => OpCode::Equals,
        9 => OpCode::AdjustsRelativeBase,
        99 => OpCode::Halt,
        _ => panic!("unexpected op"),
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ProgState {
    NotStarted,
    Halt,
    NeedInput,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Prog {
    mem_state: Vec<i64>,
    pc: usize,
    relative_base: isize,
    state: ProgState,
}

impl Prog {
    pub fn new(init_mem_state: &[i64]) -> Self {
        let mut mem_state = vec![0; init_mem_state.len()];
        mem_state.copy_from_slice(init_mem_state);

        Prog {
            mem_state,
            pc: 0,
            relative_base: 0,
            state: ProgState::NotStarted,
        }
    }

    pub fn state(&self) -> ProgState {
        self.state
    }
}

impl Prog {
    fn get_operand(&mut self, param_num: usize, op_code: i64) -> Result<i64, Error> {
        match param_mode(u32::try_from(param_num)?, op_code) {
            ParamMode::Position => {
                let index = usize::try_from(self.mem_state[self.pc + (param_num + 1)])?;
                if index >= self.mem_state.len() {
                    self.mem_state.resize(index + 1, 0);
                }
                Ok(self.mem_state[index])
            }
            ParamMode::Immediate => Ok(self.mem_state[self.pc + (param_num + 1)]),
            ParamMode::Relative => {
                let index = usize::try_from(
                    isize::try_from(self.mem_state[self.pc + (param_num + 1)])?
                        + self.relative_base,
                )?;
                if index >= self.mem_state.len() {
                    self.mem_state.resize(index + 1, 0);
                }
                Ok(self.mem_state[index])
            }
        }
    }

    fn store_value(&mut self, value: i64, param_num: usize, op_code: i64) -> Result<(), Error> {
        match param_mode(u32::try_from(param_num)?, op_code) {
            ParamMode::Position => {
                let index = usize::try_from(self.mem_state[self.pc + (param_num + 1)])?;
                if index >= self.mem_state.len() {
                    self.mem_state.resize(index + 1, 0);
                }
                self.mem_state[index] = value;
                Ok(())
            }
            ParamMode::Immediate => unreachable!(),
            ParamMode::Relative => {
                let index = usize::try_from(
                    isize::try_from(self.mem_state[self.pc + (param_num + 1)])?
                        + self.relative_base,
                )?;
                if index >= self.mem_state.len() {
                    self.mem_state.resize(index + 1, 0);
                }
                self.mem_state[index] = value;
                Ok(())
            }
        }
    }

    /// Runs a program given an initial memory state.
    pub fn run<T, S>(&mut self, input: &mut T, output: &mut S) -> Result<(), Error>
    where
        T: ProgInput,
        S: ProgOutput,
    {
        loop {
            let op_code = self.mem_state[self.pc];
            match decode_op_code(op_code) {
                OpCode::Add => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    let operand_1 = self.get_operand(1, op_code)?;
                    self.store_value(operand_0 + operand_1, 2, op_code)?;
                    self.pc += 4;
                }
                OpCode::Mul => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    let operand_1 = self.get_operand(1, op_code)?;
                    self.store_value(operand_0 * operand_1, 2, op_code)?;
                    self.pc += 4;
                }
                OpCode::Input => {
                    let input = match input.read() {
                        Ok(v) => v,
                        Err(Error::NoAvailableInput) => {
                            self.state = ProgState::NeedInput;
                            return Ok(());
                        }
                        Err(e) => return Err(e),
                    };
                    let input = input.trim().parse::<i64>()?;

                    self.store_value(input, 0, op_code)?;
                    self.pc += 2;
                }
                OpCode::Output => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    output.write(&format!("{}", operand_0))?;
                    self.pc += 2;
                }
                OpCode::JumpIfTrue => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    if operand_0 != 0 {
                        let operand_1 = self.get_operand(1, op_code)?;
                        self.pc = usize::try_from(operand_1)?;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    if operand_0 == 0 {
                        let operand_1 = self.get_operand(1, op_code)?;
                        self.pc = usize::try_from(operand_1)?;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::LessThan => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    let operand_1 = self.get_operand(1, op_code)?;
                    self.store_value(if operand_0 < operand_1 { 1 } else { 0 }, 2, op_code)?;
                    self.pc += 4;
                }
                OpCode::Equals => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    let operand_1 = self.get_operand(1, op_code)?;
                    self.store_value(if operand_0 == operand_1 { 1 } else { 0 }, 2, op_code)?;
                    self.pc += 4;
                }
                OpCode::AdjustsRelativeBase => {
                    let operand_0 = self.get_operand(0, op_code)?;
                    self.relative_base =
                        isize::try_from(i64::try_from(self.relative_base)? + operand_0)?;
                    self.pc += 2;
                }
                OpCode::Halt => {
                    self.state = ProgState::Halt;
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn param_mode_0() {
        assert_eq!(param_mode(0, 101), ParamMode::Immediate);
        assert_eq!(param_mode(0, 1), ParamMode::Position);
        assert_eq!(param_mode(0, 201), ParamMode::Relative);
    }

    #[test]
    fn param_mode_1() {
        assert_eq!(param_mode(1, 1101), ParamMode::Immediate);
        assert_eq!(param_mode(1, 1001), ParamMode::Immediate);
        assert_eq!(param_mode(1, 101), ParamMode::Position);
        assert_eq!(param_mode(1, 1), ParamMode::Position);
        assert_eq!(param_mode(1, 2101), ParamMode::Relative);
        assert_eq!(param_mode(1, 2001), ParamMode::Relative);
    }

    #[test]
    fn param_mode_2() {
        assert_eq!(param_mode(2, 1101), ParamMode::Position);
        assert_eq!(param_mode(2, 1001), ParamMode::Position);
        assert_eq!(param_mode(2, 101), ParamMode::Position);
        assert_eq!(param_mode(2, 1), ParamMode::Position);
        assert_eq!(param_mode(2, 20001), ParamMode::Relative);
        assert_eq!(param_mode(2, 21001), ParamMode::Relative);
        assert_eq!(param_mode(2, 22001), ParamMode::Relative);
    }

    fn run_prog_no_input_or_output(mem_state: &[i64]) -> Prog {
        let mut test_output = TestOutput::new();
        let mut prog = Prog::new(&mem_state);
        prog.run(&mut TestInput::new(vec![]), &mut test_output)
            .unwrap();
        assert_eq!(ProgState::Halt, prog.state);
        assert!(test_output.output.is_empty());
        prog
    }

    #[test]
    fn day2_ex1() {
        let mem_state = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let prog = run_prog_no_input_or_output(&mem_state);

        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            prog.mem_state
        );
    }

    #[test]
    fn day2_ex2() {
        let mem_state = vec![1, 0, 0, 0, 99];
        let prog = run_prog_no_input_or_output(&mem_state);
        assert_eq!(vec![2, 0, 0, 0, 99], prog.mem_state);
    }

    #[test]
    fn day2_ex3() {
        let mem_state = vec![2, 3, 0, 3, 99];
        let prog = run_prog_no_input_or_output(&mem_state);
        assert_eq!(vec![2, 3, 0, 6, 99], prog.mem_state);
    }

    #[test]
    fn day2_ex4() {
        let mem_state = vec![2, 4, 4, 5, 99, 0];
        let prog = run_prog_no_input_or_output(&mem_state);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], prog.mem_state);
    }

    #[test]
    fn day2_ex5() {
        let mem_state = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let prog = run_prog_no_input_or_output(&mem_state);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex1() {
        let mem_state = vec![3, 0, 4, 0, 99];
        let mut test_output = TestOutput::new();
        let x = String::from("42");

        let mut prog = Prog::new(&mem_state);
        prog.run(&mut TestInput::new(vec![x.clone()]), &mut test_output)
            .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec![x], test_output.output);
        assert_eq!(vec![42, 0, 4, 0, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex2() {
        let mem_state = vec![1002, 4, 3, 4, 33];

        let prog = run_prog_no_input_or_output(&mem_state);

        assert_eq!(vec![1002, 4, 3, 4, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex3() {
        let mem_state = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], prog.mem_state);
    }

    #[test]
    fn day5_ex5() {
        let mem_state = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8], prog.mem_state);
    }

    #[test]
    fn day5_ex6() {
        let mem_state = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], prog.mem_state);
    }

    #[test]
    fn day5_ex7() {
        let mem_state = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8], prog.mem_state);
    }

    #[test]
    fn day5_ex8() {
        let mem_state = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 1, 8, 3, 4, 3, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex9() {
        let mem_state = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex10() {
        let mem_state = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 0, 8, 3, 4, 3, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex11() {
        let mem_state = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99], prog.mem_state);
    }

    #[test]
    fn day5_ex12() {
        let mem_state = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("0")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex13() {
        let mem_state = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("1")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex14() {
        let mem_state = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("0")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(
            vec![3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex15() {
        let mem_state = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("1")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(
            vec![3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex16() {
        let mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["999"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 7, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex17() {
        let mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1000"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 1000, 8, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101,
                1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            prog.mem_state
        );
    }

    #[test]
    fn day5_ex18() {
        let mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(
            &mut TestInput::new(vec![String::from("9")]),
            &mut test_output,
        )
        .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec!["1001"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 1001, 9, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101,
                1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            prog.mem_state
        );
    }

    #[test]
    fn day9_ex1() {
        let mem_state = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(&mut TestInput::new(vec![]), &mut test_output)
            .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        let expected_output: Vec<String> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        assert_eq!(expected_output, test_output.output);
    }

    #[test]
    fn day9_ex2() {
        let mem_state = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(&mut TestInput::new(vec![]), &mut test_output)
            .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec![1219_0706_3239_6864i64.to_string()], test_output.output);
    }

    #[test]
    fn day9_ex3() {
        let mem_state = vec![104, 1125899906842624, 99];
        let mut test_output = TestOutput::new();

        let mut prog = Prog::new(&mem_state);
        prog.run(&mut TestInput::new(vec![]), &mut test_output)
            .unwrap();
        assert_eq!(ProgState::Halt, prog.state);

        assert_eq!(vec![1125899906842624i64.to_string()], test_output.output);
    }
}
