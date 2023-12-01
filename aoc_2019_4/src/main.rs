

fn main() {
    let rng = 264_360..=746_325;

    let possible_passwords = aoc_2019_4::find_possible_passwords(rng);
    println!("{}", possible_passwords.len());
}
