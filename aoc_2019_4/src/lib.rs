use std::ops::RangeInclusive;

#[must_use] pub fn find_possible_passwords(rng: RangeInclusive<u32>) -> Vec<u32> {
    rng.filter(|p| is_possible_password(*p)).collect()
}

fn is_possible_password(p: u32) -> bool {
    if !(100_000..=999_999).contains(&p) {
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

        remainder %= m;
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
        assert!(!is_possible_password(111_111));
    }

    #[test]
    fn is_possible_ex2() {
        assert!(!is_possible_password(111_123));
    }

    #[test]
    fn is_possible_ex3() {
        assert!(!is_possible_password(223_450));
    }

    #[test]
    fn is_possible_ex4() {
        assert!(!is_possible_password(123_789));
    }

    #[test]
    fn is_possible_ex5() {
        assert!(is_possible_password(112_233));
    }

    #[test]
    fn is_possible_ex6() {
        assert!(!is_possible_password(123_444));
    }

    #[test]
    fn is_possible_ex7() {
        assert!(is_possible_password(111_122));
    }
}
