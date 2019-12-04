use std::ops::RangeInclusive;

pub fn find_possible_passwords(rng: RangeInclusive<u32>) -> Vec<u32> {
    rng.filter(|p| is_possible_password(*p)).collect()
}

fn is_possible_password(p: u32) -> bool {
    if p < 100000 || p > 999999 {
        return false;
    }

    let mut remainder = p;
    let mut last_digit = 0;
    let mut repeated_digit_exactly_twice = false;
    let mut repeated_digit_counter: usize = 0;

    for idx in 1..7 {
        let m = 10u32.pow(6 - idx);
        let cur_digit = remainder / m;
        if last_digit > cur_digit {
            return false;
        }
        if last_digit == cur_digit {
            repeated_digit_counter += 1;
        } else {
            if repeated_digit_counter == 1 {
                repeated_digit_exactly_twice = true;
            }
            repeated_digit_counter = 0;
        }
        last_digit = cur_digit;

        remainder = remainder % m;
    }

    if repeated_digit_counter == 1 {
        repeated_digit_exactly_twice = true;
    }

    repeated_digit_exactly_twice
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_possible_ex1() {
        assert!(!is_possible_password(111111));
    }

    #[test]
    fn is_possible_ex2() {
        assert!(!is_possible_password(111123));
    }

    #[test]
    fn is_possible_ex3() {
        assert!(!is_possible_password(223450));
    }

    #[test]
    fn is_possible_ex4() {
        assert!(!is_possible_password(123789));
    }

    #[test]
    fn is_possible_ex5() {
        assert!(is_possible_password(112233));
    }

    #[test]
    fn is_possible_ex6() {
        assert!(!is_possible_password(123444));
    }

    #[test]
    fn is_possible_ex7() {
        assert!(is_possible_password(111122));
    }
}
