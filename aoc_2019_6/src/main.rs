use std::io::{self};

use aoc_2019_6::{self, error::Error, OrbitMap};

fn main() -> Result<(), Error> {
    let mut orbit_map = OrbitMap::new();

    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }

        let (object_orbits, object) = aoc_2019_6::parse_orbit(&input);
        orbit_map.insert(object, object_orbits);
    }

    println!("Total Orbits: {:?}", orbit_map.total_orbits());

    Ok(())
}
