use error::Error;

pub mod error;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vel {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Moon {
    pub pos: Pos,
    pub vel: Vel,
}

impl Moon {
    #[must_use] pub fn new(x: i64, y: i64, z: i64) -> Self {
        Moon {
            pos: Pos { x, y, z },
            vel: Vel { x: 0, y: 0, z: 0 },
        }
    }

    pub fn apply_vel(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

pub fn parse_init_moon(input: &str) -> Result<Moon, Error> {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let input = input.trim();
    let input = &input[1..input.len() - 1];
    let coords = input.split(',');

    for c in coords {
        let c = c.trim();
        let mut c = c.split('=');
        let axis = c.next().ok_or(Error::UnknownValue)?;
        let value = c.next().ok_or(Error::UnknownValue)?;

        let value = value.parse::<i64>()?;

        match axis {
            "x" => x = value,
            "y" => y = value,
            "z" => z = value,
            _ => panic!("unknown axis"),
        }
    }

    Ok(Moon::new(x, y, z))
}

fn apply_gravity(a: &mut Moon, b: &Moon) {
    if a.pos.x < b.pos.x {
        a.vel.x += 1;
    } else if a.pos.x > b.pos.x {
        a.vel.x -= 1;
    }

    if a.pos.y < b.pos.y {
        a.vel.y += 1;
    } else if a.pos.y > b.pos.y {
        a.vel.y -= 1;
    }

    if a.pos.z < b.pos.z {
        a.vel.z += 1;
    } else if a.pos.z > b.pos.z {
        a.vel.z -= 1;
    }
}

fn time_step(moons: &[Moon]) -> Vec<Moon> {
    moons
        .iter()
        .copied()
        .map(|mut nm| {
            moons.iter().for_each(|m| apply_gravity(&mut nm, m));

            nm.apply_vel();
            nm
        })
        .collect()
}

pub struct TimeStepIter {
    moons: Vec<Moon>,
}

impl TimeStepIter {
    #[must_use] pub fn new(moons: Vec<Moon>) -> Self {
        TimeStepIter { moons }
    }
}

impl Iterator for TimeStepIter {
    type Item = Vec<Moon>;

    fn next(&mut self) -> Option<Self::Item> {
        self.moons = time_step(&self.moons);
        Some(self.moons.clone())
    }
}

#[must_use] pub fn total_energy(moons: &[Moon]) -> i64 {
    moons
        .iter()
        .map(|m| {
            let pe = m.pos.x.abs() + m.pos.y.abs() + m.pos.z.abs();
            let ke = m.vel.x.abs() + m.vel.y.abs() + m.vel.z.abs();
            pe * ke
        })
        .sum()
}

#[must_use] pub fn find_cycle(axis_pos: Vec<i64>) -> i64 {
    let mut moons = axis_pos
        .into_iter()
        .map(|p| (p, 0))
        .collect::<Vec<(i64, i64)>>();
    let init_moons = moons.clone();

    let mut step = 1;

    loop {
        moons = moons
            .iter()
            .copied()
            .map(|mut nm| {
                for m in &moons {
                    if nm.0 < m.0 {
                        nm.1 += 1;
                    } else if nm.0 > m.0 {
                        nm.1 -= 1;
                    }
                }

                nm.0 += nm.1;
                nm
            })
            .collect();

        if moons == init_moons {
            break;
        }

        step += 1;
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_moon_1() {
        let moon = parse_init_moon("<x=2, y=-10, z=-7>").unwrap();
        assert_eq!(moon.pos.x, 2);
        assert_eq!(moon.pos.y, -10);
        assert_eq!(moon.pos.z, -7);
    }

    #[test]
    fn simulate_fn() {
        let moons = vec![
            parse_init_moon("<x=-1, y=0, z=2>").unwrap(),
            parse_init_moon("<x=2, y=-10, z=-7>").unwrap(),
            parse_init_moon("<x=4, y=-8, z=8>").unwrap(),
            parse_init_moon("<x=3, y=5, z=-1>").unwrap(),
        ];
        let new_moons = time_step(&moons);

        assert_eq!(new_moons[0].pos, Pos { x: 2, y: -1, z: 1 });
        assert_eq!(new_moons[0].vel, Vel { x: 3, y: -1, z: -1 });

        assert_eq!(new_moons[1].pos, Pos { x: 3, y: -7, z: -4 });
        assert_eq!(new_moons[1].vel, Vel { x: 1, y: 3, z: 3 });

        assert_eq!(new_moons[2].pos, Pos { x: 1, y: -7, z: 5 });
        assert_eq!(new_moons[2].vel, Vel { x: -3, y: 1, z: -3 });

        assert_eq!(new_moons[3].pos, Pos { x: 2, y: 2, z: 0 });
        assert_eq!(new_moons[3].vel, Vel { x: -1, y: -3, z: 1 });
    }

    #[test]
    fn simulate_iter() {
        let moons = vec![
            parse_init_moon("<x=-1, y=0, z=2>").unwrap(),
            parse_init_moon("<x=2, y=-10, z=-7>").unwrap(),
            parse_init_moon("<x=4, y=-8, z=8>").unwrap(),
            parse_init_moon("<x=3, y=5, z=-1>").unwrap(),
        ];
        let mut iter = TimeStepIter::new(moons);

        let new_moons = iter.next().unwrap();

        assert_eq!(new_moons[0].pos, Pos { x: 2, y: -1, z: 1 });
        assert_eq!(new_moons[0].vel, Vel { x: 3, y: -1, z: -1 });

        assert_eq!(new_moons[1].pos, Pos { x: 3, y: -7, z: -4 });
        assert_eq!(new_moons[1].vel, Vel { x: 1, y: 3, z: 3 });

        assert_eq!(new_moons[2].pos, Pos { x: 1, y: -7, z: 5 });
        assert_eq!(new_moons[2].vel, Vel { x: -3, y: 1, z: -3 });

        assert_eq!(new_moons[3].pos, Pos { x: 2, y: 2, z: 0 });
        assert_eq!(new_moons[3].vel, Vel { x: -1, y: -3, z: 1 });

        let mut iter = iter.skip(8);
        let new_moons = iter.next().unwrap();
        assert_eq!(total_energy(&new_moons), 179);
    }
}
