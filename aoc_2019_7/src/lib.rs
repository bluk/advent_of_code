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
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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
pub fn run_prog<T, S>(mem_state: &mut [i64], input: &mut T, output: &mut S) -> Result<(), Error>
where
    T: ProgInput,
    S: ProgOutput,
{
    let mut idx = 0;
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
                output.write(&format!("{}", operand_0))?;
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
                mem_state[store_idx] = if operand_0 < operand_1 { 1 } else { 0 };
                idx += 4;
            }
            OpCode::Equals(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0)?;
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1)?;
                let store_idx = usize::try_from(mem_state[idx + 3])?;
                mem_state[store_idx] = if operand_0 == operand_1 { 1 } else { 0 };
                idx += 4;
            }

            OpCode::Halt => break,
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct StackProgInputOutput {
    data: Vec<String>,
}

impl StackProgInputOutput {
    pub fn new() -> Self {
        StackProgInputOutput { data: Vec::new() }
    }
}

impl ProgInput for StackProgInputOutput {
    fn read(&mut self) -> Result<String, Error> {
        Ok(self.data.pop().unwrap())
    }
}

impl ProgOutput for StackProgInputOutput {
    fn write(&mut self, output: &str) -> Result<(), Error> {
        self.data.push(output.to_string());
        Ok(())
    }
}

use std::ops::Range;

fn build_input(existing_input: &[i64], rng: Range<i64>, count: i64) -> Vec<Vec<i64>> {
    if count <= 0 {
        return vec![];
    }

    let inputs = rng
        .clone()
        .filter_map(|i| {
            if existing_input.contains(&i) {
                None
            } else {
                let mut input = Vec::from(existing_input);
                input.push(i);
                Some(input)
            }
        })
        .collect();

    if count <= 1 {
        inputs
    } else {
        inputs
            .into_iter()
            .map(|i| build_input(&i, rng.clone(), count - 1))
            .flatten()
            .collect()
    }
}

pub fn find_max_thrust_signal(init_mem_state: &[i64]) -> Result<Option<(Vec<i64>, i64)>, Error> {
    let mut max_result: Option<(Vec<i64>, i64)> = None;

    for inputs in build_input(&[], 0..5, 5) {
        if let Some(thrust_signal) = run_amplifiers(init_mem_state, &inputs)? {
            if let Some(exist_result) = max_result.as_ref() {
                if exist_result.1 < thrust_signal {
                    max_result = Some((inputs, thrust_signal));
                }
            } else {
                max_result = Some((inputs, thrust_signal));
            }
        }
    }

    Ok(max_result)
}

fn run_amplifiers(init_mem_state: &[i64], inputs: &[i64]) -> Result<Option<i64>, Error> {
    let mut stack = StackProgInputOutput::new();
    stack.write(&0.to_string())?;

    for input in inputs.iter() {
        stack.write(&input.to_string())?;

        let mut mem_state = Vec::<i64>::new();
        mem_state.extend_from_slice(&init_mem_state);

        let mut out_stack = StackProgInputOutput::new();

        run_prog(&mut *mem_state.clone(), &mut stack, &mut out_stack)?;

        stack = out_stack;
    }

    Ok(Some(stack.read()?.parse::<i64>()?))
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
        let mut mem_state = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            mem_state
        );
    }

    #[test]
    fn day2_ex2() {
        let mut mem_state = vec![1, 0, 0, 0, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 0, 0, 0, 99], mem_state);
    }

    #[test]
    fn day2_ex3() {
        let mut mem_state = vec![2, 3, 0, 3, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 3, 0, 6, 99], mem_state);
    }

    #[test]
    fn day2_ex4() {
        let mut mem_state = vec![2, 4, 4, 5, 99, 0];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], mem_state);
    }

    #[test]
    fn day2_ex5() {
        let mut mem_state = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex1() {
        let mut mem_state = vec![3, 0, 4, 0, 99];
        let mut test_output = TestOutput::new();
        let x = String::from("42");
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![x.clone()]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec![x], test_output.output);
        assert_eq!(vec![42, 0, 4, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex2() {
        let mut mem_state = vec![1002, 4, 3, 4, 33];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![]),
            &mut test_output,
        )
        .unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![1002, 4, 3, 4, 99], mem_state);
    }

    #[test]
    fn day5_ex3() {
        let mut mem_state = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], mem_state);
    }

    #[test]
    fn day5_ex5() {
        let mut mem_state = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8], mem_state);
    }

    #[test]
    fn day5_ex6() {
        let mut mem_state = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], mem_state);
    }

    #[test]
    fn day5_ex7() {
        let mut mem_state = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8], mem_state);
    }

    #[test]
    fn day5_ex8() {
        let mut mem_state = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 1, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex9() {
        let mut mem_state = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex10() {
        let mut mem_state = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("8")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["0"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 0, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex11() {
        let mut mem_state = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("7")]),
            &mut test_output,
        )
        .unwrap();

        assert_eq!(vec!["1"], test_output.output);
        assert_eq!(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99], mem_state);
    }

    #[test]
    fn day5_ex12() {
        let mut mem_state = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("0")]),
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
        let mut mem_state = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("1")]),
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
        let mut mem_state = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("0")]),
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
        let mut mem_state = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("1")]),
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
        let mut mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("7")]),
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
        let mut mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("8")]),
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
        let mut mem_state = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut test_output = TestOutput::new();
        run_prog(
            &mut mem_state,
            &mut TestInput::new(vec![String::from("9")]),
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

    #[test]
    fn test_build_input() {
        assert_eq!(
            vec![vec![0], vec![1], vec![2], vec![3], vec![4],],
            build_input(&[], 0..5, 1)
        );

        assert_eq!(
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![0, 4],
                vec![1, 0],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 0],
                vec![2, 1],
                vec![2, 3],
                vec![2, 4],
                vec![3, 0],
                vec![3, 1],
                vec![3, 2],
                vec![3, 4],
                vec![4, 0],
                vec![4, 1],
                vec![4, 2],
                vec![4, 3]
            ],
            build_input(&[], 0..5, 2)
        );
    }

    #[test]
    fn day7_ex1() {
        let mut mem_state = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let result = run_amplifiers(&mut mem_state, &[4, 3, 2, 1, 0]).unwrap();

        assert_eq!(result, Some(43210));
    }

    #[test]
    fn day7_ex2() {
        let mut mem_state = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let result = find_max_thrust_signal(&mut mem_state).unwrap().unwrap();

        assert_eq!(result.0, vec![4, 3, 2, 1, 0]);
        assert_eq!(result.1, 43210);
    }

    #[test]
    fn day7_ex3() {
        let mut mem_state = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let result = run_amplifiers(&mut mem_state, &[0, 1, 2, 3, 4]).unwrap();

        assert_eq!(result, Some(54321));
    }

    #[test]
    fn day7_ex4() {
        let mut mem_state = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let result = find_max_thrust_signal(&mut mem_state).unwrap().unwrap();

        assert_eq!(result.0, vec![0, 1, 2, 3, 4]);
        assert_eq!(result.1, 54321);
    }

    #[test]
    fn day7_ex5() {
        let mut mem_state = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let result = run_amplifiers(&mut mem_state, &[1, 0, 4, 3, 2]).unwrap();

        assert_eq!(result, Some(65210));
    }

    #[test]
    fn day7_ex6() {
        let mut mem_state = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let result = find_max_thrust_signal(&mut mem_state).unwrap().unwrap();

        assert_eq!(result.0, vec![1, 0, 4, 3, 2]);
        assert_eq!(result.1, 65210);
    }
}
