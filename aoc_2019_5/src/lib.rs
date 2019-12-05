use std::io;

use error::Error;

pub mod error;

pub trait ProgInput {
    fn read(&mut self) -> Result<String, Error>;
}

pub trait ProgOutput {
    fn write(&mut self, output: &str) -> Result<(), Error>;
}

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

pub fn parse_input(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    Add(ParamMode, ParamMode),
    Mul(ParamMode, ParamMode),
    Input,
    Output(ParamMode),
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
        99 => OpCode::Halt,
        _ => panic!("unexpected op"),
    }
}

fn get_operand(values: &[i64], idx: usize, param_num: usize, param_mode: ParamMode) -> i64 {
    match param_mode {
        ParamMode::Position => values[values[idx + (param_num + 1)] as usize],
        ParamMode::Immediate => values[idx + (param_num + 1)],
    }
}

pub fn run_prog<T, S>(prog: &[i64], mut input: T, output: &mut S) -> Result<Vec<i64>, Error>
where
    T: ProgInput,
    S: ProgOutput,
{
    let mut idx = 0;
    let mut mem_state = Vec::with_capacity(prog.len());
    mem_state.extend_from_slice(prog);
    loop {
        match decode_op_code(mem_state[idx]) {
            OpCode::Add(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0);
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1);
                let store_idx = mem_state[idx + 3] as usize;
                mem_state[store_idx] = operand_0 + operand_1;
                idx += 4;
            }
            OpCode::Mul(param_mode_0, param_mode_1) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0);
                let operand_1 = get_operand(&mem_state, idx, 1, param_mode_1);
                let store_idx = mem_state[idx + 3] as usize;
                mem_state[store_idx] = operand_0 * operand_1;
                idx += 4;
            }
            OpCode::Input => {
                let input = input.read()?;
                let input = input.trim().parse::<i64>()?;

                let store_idx = mem_state[idx + 1] as usize;
                mem_state[store_idx] = input;
                idx += 2;
            }
            OpCode::Output(param_mode_0) => {
                let operand_0 = get_operand(&mem_state, idx, 0, param_mode_0);
                output.write(&format!("{}", operand_0))?;
                idx += 2;
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
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            mem_state
        );
    }

    #[test]
    fn day2_ex2() {
        let input = vec![1, 0, 0, 0, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 0, 0, 0, 99], mem_state);
    }

    #[test]
    fn day2_ex3() {
        let input = vec![2, 3, 0, 3, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 3, 0, 6, 99], mem_state);
    }

    #[test]
    fn day2_ex4() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], mem_state);
    }

    #[test]
    fn day2_ex5() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex1() {
        let input = vec![3, 0, 4, 0, 99];
        let mut test_output = TestOutput::new();
        let x = String::from("42");
        let mem_state =
            run_prog(&input, TestInput::new(vec![x.clone()]), &mut test_output).unwrap();

        assert_eq!(vec![x], test_output.output);
        assert_eq!(vec![42, 0, 4, 0, 99], mem_state);
    }

    #[test]
    fn day5_ex2() {
        let input = vec![1002, 4, 3, 4, 33];
        let mut test_output = TestOutput::new();
        let mem_state = run_prog(&input, TestInput::new(vec![]), &mut test_output).unwrap();

        assert!(test_output.output.is_empty());
        assert_eq!(vec![1002, 4, 3, 4, 99], mem_state);
    }
}
