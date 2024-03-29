use std::convert::TryFrom;
use std::io;

use error::Error;

pub mod error;

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

/// Reads in program input from stdin.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct StdInProgInput {}

impl Default for StdInProgInput {
    fn default() -> Self {
        Self::new()
    }
}

impl StdInProgInput {
    #[must_use] pub fn new() -> Self {
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
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct StdOutProgOutput {}

impl Default for StdOutProgOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl StdOutProgOutput {
    #[must_use] pub fn new() -> Self {
        StdOutProgOutput {}
    }
}

impl ProgOutput for StdOutProgOutput {
    fn write(&mut self, output: &str) -> Result<(), Error> {
        println!("{output}");
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
}

/// The operation as well as the parameter modes for operands.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    Add(ParamMode, ParamMode),
    Mul(ParamMode, ParamMode),
    Input,
    Output(ParamMode),
    JumpIfTrue(ParamMode, ParamMode),
    JumpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode),
    Equals(ParamMode, ParamMode),
    Halt,
}

fn param_mode(op: i64, param: u32) -> ParamMode {
    match (op % 10i64.pow(param + 3)) / 10i64.pow(param + 2) {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        _ => panic!("unexpected parameter mode"),
    }
}

fn decode_op_code(op: i64) -> OpCode {
    let op_code = op % 100;
    match op_code {
        1 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::Add(param_mode_0, param_mode_1)
        }
        2 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::Mul(param_mode_0, param_mode_1)
        }
        3 => {
            let param_mode_0 = param_mode(op, 0);
            assert_eq!(param_mode_0, ParamMode::Position);
            OpCode::Input
        }
        4 => {
            let param_mode_0 = param_mode(op, 0);
            OpCode::Output(param_mode_0)
        }
        5 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::JumpIfTrue(param_mode_0, param_mode_1)
        }
        6 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::JumpIfFalse(param_mode_0, param_mode_1)
        }
        7 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::LessThan(param_mode_0, param_mode_1)
        }
        8 => {
            let param_mode_0 = param_mode(op, 0);
            let param_mode_1 = param_mode(op, 1);
            OpCode::Equals(param_mode_0, param_mode_1)
        }
        99 => OpCode::Halt,
        _ => panic!("unexpected op"),
    }
}

fn get_operand(
    mem_state: &[i64],
    idx: usize,
    param_num: usize,
    param_mode: ParamMode,
) -> Result<i64, Error> {
    match param_mode {
        ParamMode::Position => {
            let index = usize::try_from(mem_state[idx + (param_num + 1)])?;
            Ok(mem_state[index])
        }
        ParamMode::Immediate => Ok(mem_state[idx + (param_num + 1)]),
    }
}

/// Runs a program given an initial memory state.
pub fn run_prog<T, S>(
    init_mem_state: &[i64],
    mut input: T,
    output: &mut S,
) -> Result<Vec<i64>, Error>
where
    T: ProgInput,
    S: ProgOutput,
{
    let mut idx = 0;
    let mut mem_state = Vec::with_capacity(init_mem_state.len());
    mem_state.extend_from_slice(init_mem_state);
    loop {
        match decode_op_code(mem_state[idx]) {
            OpCode::Add(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                let store_idx = usize::try_from(mem_state[idx + 3])?;
                mem_state[store_idx] = operand_0 + operand_1;
                idx += 4;
            }
            OpCode::Mul(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                let store_idx = usize::try_from(mem_state[idx + 3])?;
                mem_state[store_idx] = operand_0 * operand_1;
                idx += 4;
            }
            OpCode::Input => {
                let input = input.read()?;
                let input = input.trim().parse::<i64>()?;

                let store_idx = usize::try_from(mem_state[idx + 1])?;
                mem_state[store_idx] = input;
                idx += 2;
            }
            OpCode::Output(param_mode_0) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                output.write(&format!("{operand_0}"))?;
                idx += 2;
            }
            OpCode::JumpIfTrue(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                if operand_0 != 0 {
                    let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                    idx = usize::try_from(operand_1)?;
                } else {
                    idx += 3;
                }
            }
            OpCode::JumpIfFalse(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                if operand_0 == 0 {
                    let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                    idx = usize::try_from(operand_1)?;
                } else {
                    idx += 3;
                }
            }
            OpCode::LessThan(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                let store_idx = usize::try_from(mem_state[idx + 3])?;
                mem_state[store_idx] = i64::from(operand_0 < operand_1);
                idx += 4;
            }
            OpCode::Equals(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                let store_idx = usize::try_from(mem_state[idx + 3])?;
                mem_state[store_idx] = i64::from(operand_0 == operand_1);
                idx += 4;
            }

            OpCode::Halt => break,
        }
    }

    Ok(mem_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    struct TestInput {
        input: Vec<String>,
    }

    impl TestInput {
        fn new(mut input: Vec<String>) -> Self {
            input.reverse();
            TestInput { input }
        }
    }

    impl ProgInput for TestInput {
        fn read(&mut self) -> Result<String, Error> {
            if let Some(input) = self.input.pop() {
                Ok(input)
            } else {
                Err(Error::IoErr(io::Error::from(io::ErrorKind::UnexpectedEof)))
            }
        }
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    struct TestOutput {
        output: Vec<String>,
    }

    impl TestOutput {
        fn new() -> Self {
            TestOutput { output: Vec::new() }
        }
    }

    impl ProgOutput for TestOutput {
        fn write(&mut self, output: &str) -> Result<(), Error> {
            self.output.push(output.to_string());
            Ok(())
        }
    }

    #[test]
    fn param_mode_0() {
        assert_eq!(param_mode(101, 0), ParamMode::Immediate);
        assert_eq!(param_mode(1, 0), ParamMode::Position);
    }

    #[test]
    fn param_mode_1() {
        assert_eq!(param_mode(1101, 1), ParamMode::Immediate);
        assert_eq!(param_mode(1001, 1), ParamMode::Immediate);
        assert_eq!(param_mode(101, 1), ParamMode::Position);
        assert_eq!(param_mode(1, 1), ParamMode::Position);
    }

    #[test]
    fn day2_ex1() {
        let prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            mem_state
        );
    }

    #[test]
    fn day2_ex2() {
        let prog = vec![1, 0, 0, 0, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 0, 0, 0, 99], mem_state);
    }

    #[test]
    fn day2_ex3() {
        let prog = vec![2, 3, 0, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 3, 0, 6, 99], mem_state);
    }

    #[test]
    fn day2_ex4() {
        let prog = vec![2, 4, 4, 5, 99, 0];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], mem_state);
    }

    #[test]
    fn day2_ex5() {
        let prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex1() {
        let prog = vec![3, 0, 4, 0, 99];
        let mut test_output = TestOutput::new();
        let x = String::from("42");
        let mem_state = run_prog(&prog, TestInput::new(vec![x.clone()]), &mut test_output).unwrap();

        assert_eq!(vec![x], test_output.output);
        assert_eq!(vec![42, 0, 4, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex2() {
        let prog = vec![1002, 4, 3, 4, 33];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&prog, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![1002, 4, 3, 4, 99], mem_state);
    }

    #[test]
    fn day5_ex3() {
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], mem_state);
    }

    #[test]
    fn day5_ex5() {
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8], mem_state);
    }

    #[test]
    fn day5_ex6() {
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], mem_state);
    }

    #[test]
    fn day5_ex7() {
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8], mem_state);
    }

    #[test]
    fn day5_ex8() {
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 1, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex9() {
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex10() {
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 0, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex11() {
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex12() {
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("0")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9],
            mem_state
        );
    }

    #[test]
    fn day5_ex13() {
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("1")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9],
            mem_state
        );
    }

    #[test]
    fn day5_ex14() {
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("0")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(
            vec![3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0],
            mem_state
        );
    }

    #[test]
    fn day5_ex15() {
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("1")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(
            vec![3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            mem_state
        );
    }

    #[test]
    fn day5_ex16() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["999"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 7, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            mem_state
        );
    }

    #[test]
    fn day5_ex17() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1000"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 1000, 8, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101,
                1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            mem_state
        );
    }

    #[test]
    fn day5_ex18() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(
            &prog,
            TestInput::new(vec![String::from("9")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1001"], test_output.output);
        assert_eq!(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 1001, 9, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101,
                1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            mem_state
        );
    }
}
