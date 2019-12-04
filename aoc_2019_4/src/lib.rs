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
    let mut repeated_digit = false;

    for idx in 1..7 {
        let m = 10u32.pow(6 - idx);
        let cur_digit = remainder / m;
        if last_digit > cur_digit {
            return false;
        }
        if last_digit == cur_digit {
            repeated_digit = true;
        }
        last_digit = cur_digit;

        remainder = remainder % m;
    }

    repeated_digit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_possible_ex1() {
        assert!(is_possible_password(111111));
    }

    #[test]
    fn is_possible_ex2() {
        assert!(is_possible_password(111123));
    }

    #[test]
    fn is_possible_ex3() {
        assert!(!is_possible_password(223450));
    }

    #[test]
    fn is_possible_ex4() {
        assert!(!is_possible_password(123789));
    }
}
