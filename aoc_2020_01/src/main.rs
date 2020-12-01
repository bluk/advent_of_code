use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    ParseError,
    InvalidArg,
}

fn find_combination_sum<T>(input: &[T], k: usize, target_sum: T) -> Result<Option<Vec<T>>, Error>
where
    T: std::iter::Sum,
    T: std::cmp::PartialEq,
    T: Copy,
{
    if k == 0 {
        return Err(Error::InvalidArg);
    }

    if input.len() < k {
        return Err(Error::InvalidArg);
    }

    let mut indices: Vec<usize> = (0..k).collect();

    loop {
        // println!("{:?}", &indices);
        let sum: T = indices.iter().map(|i| input[*i]).sum();
        if sum == target_sum {
            return Ok(Some(indices.iter().map(|i| input[*i]).collect()));
        }

        // From the indices right to left, increment the rightmost index value which is not already
        // at its maximum value.
        //
        // Assuming 4 input elements.
        //
        // If the indexes are: [0, 1, 2], then the next iteration should be [0, 1, 3].
        //
        // If the indexes are: [0, 1, 3], then the next iteration should be [0, 2, 3].
        //
        // If the indexes are: [0, 2, 3], then the next iteration should be [1, 2, 3].
        let mut indices_index = indices.len() - 1;
        loop {
            if indices[indices_index] < input.len() - 1 - (indices.len() - 1 - indices_index) {
                indices[indices_index] += 1;

                // Reset all the indexes to the "right" to a value greater than any on the "left".
                let mut val = indices[indices_index];
                loop {
                    indices_index += 1;
                    if indices_index >= indices.len() {
                        break;
                    }
                    val += 1;
                    indices[indices_index] = val;
                }
                break;
            } else {
                if indices_index == 0 {
                    return Ok(None);
                }

                indices_index -= 1;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse())
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|_| Error::ParseError)?;

    let values = find_combination_sum(&input, 2, 2020)
        .expect("No invalid arguments")
        .expect("An answer to exist");
    println!("First Part Answer: {}", values.iter().product::<u32>());

    let values = find_combination_sum(&input, 3, 2020)
        .expect("No invalid arguments")
        .expect("An answer to exist");
    println!("Second Part Answer: {}", values.iter().product::<u32>());

    Ok(())
}
