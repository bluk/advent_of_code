use std::collections::VecDeque;
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct VecDequeProgInput {
    data: VecDeque<String>,
}

impl VecDequeProgInput {
    fn new() -> Self {
        VecDequeProgInput {
            data: VecDeque::new(),
        }
    }
}

impl ProgInput for VecDequeProgInput {
    fn read(&mut self) -> Result<String, Error> {
        if let Some(value) = self.data.pop_front() {
            Ok(value)
        } else {
            Err(Error::NoAvailableInput)
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct VecDequeProgOutput {
    data: VecDeque<String>,
}

impl VecDequeProgOutput {
    fn new() -> Self {
        VecDequeProgOutput {
            data: VecDeque::new(),
        }
    }
}

impl ProgOutput for VecDequeProgOutput {
    fn write(&mut self, output: &str) -> Result<(), Error> {
        self.data.push_back(output.to_string());
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
        if let Some(thrust_signal) = run_amplifiers_in_feedback_loop(init_mem_state, &inputs)? {
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

pub fn find_max_thrust_signal_in_feedback_loop(
    init_mem_state: &[i64],
) -> Result<Option<(Vec<i64>, i64)>, Error> {
    let mut max_result: Option<(Vec<i64>, i64)> = None;

    for inputs in build_input(&[], 5..10, 5) {
        if let Some(thrust_signal) = run_amplifiers_in_feedback_loop(init_mem_state, &inputs)? {
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

fn run_amplifiers_in_feedback_loop(
    init_mem_state: &[i64],
    inputs: &[i64],
) -> Result<Option<i64>, Error> {
    struct Amp {
        prog: Prog,
        prog_input: VecDequeProgInput,
    }

    let mut amps = Vec::<Amp>::with_capacity(inputs.len());
    for input in inputs {
        let mut mem_state = vec![0; init_mem_state.len()];
        mem_state.copy_from_slice(init_mem_state);

        let mut prog_input = VecDequeProgInput::new();
        prog_input.data.push_back(input.to_string());

        amps.push(Amp {
            prog: Prog::new(&mem_state),
            prog_input,
        });
    }

    amps[0].prog_input.data.push_back(0.to_string());

    let mut prog_output = VecDequeProgOutput::new();
    loop {
        for amp in &mut amps {
            prog_output.data.iter().for_each(|o| {
                amp.prog_input.data.push_back(o.to_string());
            });

            prog_output = VecDequeProgOutput::new();

            amp.prog.run(&mut amp.prog_input, &mut prog_output)?;
        }

        if amps[amps.len() - 1].prog.state == ProgState::Halt {
            assert!(amps.iter().all(|a| a.prog.state == ProgState::Halt));
            return Ok(Some(prog_output.data.pop_front().unwrap().parse::<i64>()?));
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl TryFrom<String> for Color {
    type Error = Error;

    fn try_from(other: String) -> Result<Self, Self::Error> {
        if other == "0" {
            Ok(Color::Black)
        } else if other == "1" {
            Ok(Color::White)
        } else {
            Err(Error::UnknownValue)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum TurnDir {
    Left,
    Right,
}

impl TryFrom<String> for TurnDir {
    type Error = Error;

    fn try_from(other: String) -> Result<Self, Self::Error> {
        if other == "0" {
            Ok(TurnDir::Left)
        } else if other == "1" {
            Ok(TurnDir::Right)
        } else {
            Err(Error::UnknownValue)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum RobotDir {
    Up,
    Down,
    Left,
    Right,
}

impl RobotDir {
    fn turn(&self, dir: TurnDir) -> RobotDir {
        match (self, dir) {
            (RobotDir::Up, TurnDir::Left) => RobotDir::Left,
            (RobotDir::Up, TurnDir::Right) => RobotDir::Right,
            (RobotDir::Left, TurnDir::Left) => RobotDir::Down,
            (RobotDir::Left, TurnDir::Right) => RobotDir::Up,
            (RobotDir::Down, TurnDir::Left) => RobotDir::Right,
            (RobotDir::Down, TurnDir::Right) => RobotDir::Left,
            (RobotDir::Right, TurnDir::Left) => RobotDir::Up,
            (RobotDir::Right, TurnDir::Right) => RobotDir::Down,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Panel {
    pos: Pos,
    color: Color,
}

pub fn paint_hull(mut prog: Prog) -> Result<Vec<Panel>, Error> {
    let mut robot_dir = RobotDir::Up;
    let mut robot_pos = Pos { x: 0, y: 0 };
    let mut panels = Vec::<Panel>::new();

    let mut input = VecDequeProgInput::new();
    let mut output = VecDequeProgOutput::new();

    loop {
        let index = panels.iter().position(|p| p.pos == robot_pos);
        let color = if let Some(index) = index {
            panels[index].color
        } else {
            Color::Black
        };

        input.data.push_back(match color {
            Color::Black => "0".to_string(),
            Color::White => "1".to_string(),
        });
        prog.run(&mut input, &mut output)?;

        let panel = if let Some(index) = index {
            panels.get_mut(index).unwrap()
        } else {
            let panel = Panel {
                pos: robot_pos,
                color: Color::Black,
            };
            panels.push(panel);
            panels.last_mut().unwrap()
        };

        let mut got_output = false;

        if let Some(v) = output.data.pop_front() {
            got_output = true;
            let color = Color::try_from(v)?;
            panel.color = color;
        } else {
            assert!(false);
        }

        if let Some(v) = output.data.pop_front() {
            assert!(got_output);
            let turn_dir = TurnDir::try_from(v)?;
            robot_dir = robot_dir.turn(turn_dir);
            match robot_dir {
                RobotDir::Up => robot_pos.y += 1,
                RobotDir::Left => robot_pos.x -= 1,
                RobotDir::Down => robot_pos.y -= 1,
                RobotDir::Right => robot_pos.x += 1,
            }
        } else {
            assert!(false);
        }

        match prog.state {
            ProgState::Halt => {
                assert!(got_output);
                break;
            }
            ProgState::NeedInput => assert!(got_output),
            ProgState::NotStarted => unreachable!(),
        }
    }

    assert!(input.data.is_empty());
    assert!(output.data.is_empty());

    Ok(panels)
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
        let result = run_amplifiers_in_feedback_loop(&mut mem_state, &[4, 3, 2, 1, 0]).unwrap();

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
        let result = run_amplifiers_in_feedback_loop(&mut mem_state, &[0, 1, 2, 3, 4]).unwrap();

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
        let result = run_amplifiers_in_feedback_loop(&mut mem_state, &[1, 0, 4, 3, 2]).unwrap();

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

    #[test]
    fn day7_ex7() {
        let mut mem_state = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let result = run_amplifiers_in_feedback_loop(&mut mem_state, &[9, 8, 7, 6, 5]).unwrap();

        assert_eq!(result, Some(139629729));
    }

    #[test]
    fn day7_ex8() {
        let mut mem_state = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let result = find_max_thrust_signal_in_feedback_loop(&mut mem_state)
            .unwrap()
            .unwrap();

        assert_eq!(result.0, vec![9, 8, 7, 6, 5]);
        assert_eq!(result.1, 139629729);
    }

    #[test]
    fn day7_ex9() {
        let mut mem_state = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let result = run_amplifiers_in_feedback_loop(&mut mem_state, &[9, 7, 8, 5, 6]).unwrap();

        assert_eq!(result, Some(18216));
    }

    #[test]
    fn day7_ex10() {
        let mut mem_state = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let result = find_max_thrust_signal_in_feedback_loop(&mut mem_state)
            .unwrap()
            .unwrap();

        assert_eq!(result.0, vec![9, 7, 8, 5, 6]);
        assert_eq!(result.1, 18216);
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
