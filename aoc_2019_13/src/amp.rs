use std::collections::VecDeque;
use std::ops::Range;

use crate::error::Error;
use crate::intcode::{Prog, ProgState};

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
        prog_input: VecDeque<String>,
    }

    let mut amps = Vec::<Amp>::with_capacity(inputs.len());
    for input in inputs {
        let mut mem_state = vec![0; init_mem_state.len()];
        mem_state.copy_from_slice(init_mem_state);

        let mut prog_input = VecDeque::new();
        prog_input.push_back(input.to_string());

        amps.push(Amp {
            prog: Prog::new(&mem_state),
            prog_input,
        });
    }

    amps[0].prog_input.push_back(0.to_string());

    let mut prog_output = VecDeque::<String>::new();
    loop {
        for amp in &mut amps {
            prog_output.iter().for_each(|o| {
                amp.prog_input.push_back(o.to_string());
            });

            prog_output = VecDeque::<String>::new();

            amp.prog.run(&mut amp.prog_input, &mut prog_output)?;
        }

        if amps[amps.len() - 1].prog.state() == ProgState::Halt {
            assert!(amps.iter().all(|a| a.prog.state() == ProgState::Halt));
            return Ok(Some(prog_output.pop_front().unwrap().parse::<i64>()?));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
