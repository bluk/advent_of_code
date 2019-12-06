use std::collections::HashMap;

pub mod error;

pub struct OrbitMap {
    map: HashMap<String, String>,
}

impl OrbitMap {
    pub fn new() -> Self {
        OrbitMap {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, object: &str, object_orbits: &str) {
        self.map
            .insert(object.to_string(), object_orbits.to_string());
    }

    fn orbits_for(&self, object: &str) -> Option<usize> {
        let mut orbits = 0;
        let mut cur_obj = object;
        while let Some(object_orbits) = self.map.get(cur_obj) {
            orbits += 1;
            cur_obj = object_orbits;
        }

        if orbits == 0 {
            None
        } else {
            Some(orbits)
        }
    }

    pub fn total_orbits(&self) -> Option<usize> {
        let total_orbits: usize = self.map.keys().filter_map(|k| self.orbits_for(k)).sum();
        if total_orbits == 0 {
            None
        } else {
            Some(total_orbits)
        }
    }

    fn orbits_to_com_for(&self, object: &str) -> Option<Vec<&str>> {
        let mut objects_orbited: Vec<&str> = Vec::new();

        let mut cur_obj = object;
        while let Some(object_orbits) = self.map.get(cur_obj) {
            objects_orbited.push(object_orbits);
            cur_obj = object_orbits;
        }

        if objects_orbited.is_empty() {
            None
        } else {
            Some(objects_orbited)
        }
    }

    pub fn orbital_transfers_between(&self, you: &str, san: &str) -> Option<usize> {
        if let Some(you_orbits_to_com) = self.orbits_to_com_for(you) {
            if let Some(san_orbits_to_com) = self.orbits_to_com_for(san) {
                let common: usize = you_orbits_to_com
                    .iter()
                    .rev()
                    .zip(san_orbits_to_com.iter().rev())
                    .map(|(y, s)| if y == s { 1 } else { 0 })
                    .sum();

                let transfers =
                    (you_orbits_to_com.len() - common) + (san_orbits_to_com.len() - common);

                return Some(transfers);
            }
        }

        None
    }
}

/// Parse an orbit of a planet.
pub fn parse_orbit(input: &str) -> (&str, &str) {
    let input = input.trim().split(')').collect::<Vec<&str>>();
    assert_eq!(input.len(), 2);
    (input[0], input[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_orbit() {
        let (object_orbits, object) = parse_orbit("COM)B");
        assert_eq!(object_orbits, "COM");
        assert_eq!(object, "B");
    }

    #[test]
    fn ex1() {
        let mut orbit_map = OrbitMap::new();

        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
        ";

        input
            .trim()
            .split('\n')
            .map(|s| parse_orbit(s))
            .for_each(|(object_orbits, object)| orbit_map.insert(object, object_orbits));

        assert_eq!(orbit_map.orbits_for("D"), Some(3));
        assert_eq!(orbit_map.orbits_for("L"), Some(7));
        assert_eq!(orbit_map.orbits_for("COM"), None);
        assert_eq!(orbit_map.total_orbits(), Some(42));
    }

    #[test]
    fn ex2() {
        let mut orbit_map = OrbitMap::new();

        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

        input
            .trim()
            .split('\n')
            .map(|s| parse_orbit(s))
            .for_each(|(object_orbits, object)| orbit_map.insert(object, object_orbits));

        assert_eq!(orbit_map.orbital_transfers_between("YOU", "SAN"), Some(4));
    }
}
