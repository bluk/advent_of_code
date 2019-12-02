pub fn parse_input(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input
        .split(',')
        .map(|s| s.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
}

pub fn run_prog(input: &[usize]) -> Vec<usize> {
    let mut idx = 0;
    let mut output = Vec::with_capacity(input.len());
    output.extend_from_slice(input);
    loop {
        match output[idx] {
            1 => {
                let operand_1 = output[output[idx + 1]];
                let operand_2 = output[output[idx + 2]];
                let store_idx = output[idx + 3];
                output[store_idx] = operand_1 + operand_2;
                idx += 4;
            }
            2 => {
                let operand_1 = output[output[idx + 1]];
                let operand_2 = output[output[idx + 2]];
                let store_idx = output[idx + 3];
                output[store_idx] = operand_1 * operand_2;
                idx += 4;
            }
            99 => break,
            _ => panic!("Unexpected operation"),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output = run_prog(&input);

        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], output);
    }

    #[test]
    fn ex2() {
        let input = vec![1, 0, 0, 0, 99];
        let output = run_prog(&input);

        assert_eq!(vec![2, 0, 0, 0, 99], output);
    }

    #[test]
    fn ex3() {
        let input = vec![2, 3, 0, 3, 99];
        let output = run_prog(&input);

        assert_eq!(vec![2, 3, 0, 6, 99], output);
    }

    #[test]
    fn ex4() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = run_prog(&input);

        assert_eq!(vec![2, 4, 4, 5, 99, 9801], output);
    }

    #[test]
    fn ex5() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = run_prog(&input);

        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], output);
    }
}
