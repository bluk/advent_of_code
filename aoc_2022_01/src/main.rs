use std::io;

fn main() -> io::Result<()> {
    let mut max_calories = 0;

    let mut calories = 0;

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            if max_calories < calories {
                max_calories = calories;
            }
            calories = 0;
            continue;
        }

        let num = line
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        calories += num;
    }

    println!("{max_calories}");

    Ok(())
}
