use std::io;

fn main() -> io::Result<()> {
    const TOP_ELVES_COUNT: usize = 3;

    let mut top_calories = [0; TOP_ELVES_COUNT];

    let mut calories = 0;

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            let smallest_top_calories = top_calories
                .first_mut()
                .expect("top_calories should have at least one element");
            if *smallest_top_calories < calories {
                *smallest_top_calories = calories;
                top_calories.sort_unstable();
            }
            calories = 0;
        } else {
            let num = line
                .parse::<i32>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            calories += num;
        }
    }

    println!("{}", top_calories.iter().sum::<i32>());

    Ok(())
}
