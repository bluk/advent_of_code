use aoc_2019_4;

fn main() {
    let rng = 264360..=746325;

    let possible_passwords = aoc_2019_4::find_possible_passwords(rng);
    println!("{}", possible_passwords.len());
}
