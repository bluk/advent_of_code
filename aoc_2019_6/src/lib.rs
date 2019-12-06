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

    pub fn orbits_for(&self, object: &str) -> Option<usize> {
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
}
