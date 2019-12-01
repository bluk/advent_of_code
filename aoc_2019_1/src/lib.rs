/// Calculate the amount of fuel required for a module of a specific mass.
///
/// ```
/// use aoc_2019_1::find_fuel_required;
///
/// assert_eq!(2, find_fuel_required(12));
/// ```
pub fn find_fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mass = 12;
        assert_eq!(2, find_fuel_required(mass));
    }

    #[test]
    fn ex2() {
        let mass = 14;
        assert_eq!(2, find_fuel_required(mass));
    }

    #[test]
    fn ex3() {
        let mass = 1969;
        assert_eq!(654, find_fuel_required(mass));
    }

    #[test]
    fn ex4() {
        let mass = 100_756;
        assert_eq!(33_583, find_fuel_required(mass));
    }
}
