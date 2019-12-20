use std::collections::HashMap;

pub mod error;

use error::Error;

type Reactions = HashMap<String, (u64, HashMap<String, u64>)>;

pub fn parse_line(s: &str) -> Result<(String, u64, HashMap<String, u64>), Error> {
    let mut chems = s.trim().split("=>");

    let mut input = HashMap::new();
    let input_chems = chems.next().unwrap().trim().split(",");
    for ic in input_chems {
        let mut ic = ic.trim().split(" ");
        let quantity = ic.next().unwrap();
        let quantity = quantity.parse::<u64>()?;
        let chem = ic.next().unwrap().to_string();
        input.insert(chem, quantity);
        assert!(ic.next().is_none());
    }

    let mut output_chem = chems.next().unwrap().trim().split(" ");
    let quantity = output_chem.next().unwrap();
    let quantity = quantity.parse::<u64>()?;
    let chem = output_chem.next().unwrap();
    assert!(output_chem.next().is_none());

    Ok((chem.to_string(), quantity, input))
}

pub fn parse_reactions(input: &str) -> Result<Reactions, Error> {
    let mut reactions = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (output_chem, output_qty, input_chems) = parse_line(&line)?;
        reactions.insert(output_chem, (output_qty, input_chems));
    }

    Ok(reactions)
}

pub fn find_ore_for_fuel(reactions: &Reactions) -> Result<u64, Error> {
    let mut unused_chems = HashMap::new();
    let mut used_chems = HashMap::new();

    create_chemical(reactions, "FUEL", &mut unused_chems, &mut used_chems)?;
    Ok(*used_chems.get("ORE").unwrap_or(&0) + *unused_chems.get("ORE").unwrap_or(&0))
}

fn create_chemical(
    reactions: &Reactions,
    chemical: &str,
    unused_chems: &mut HashMap<String, u64>,
    used_chems: &mut HashMap<String, u64>,
) -> Result<(), Error> {
    let (output_qty, input_chems) = reactions.get(chemical).expect("chemical should be known");

    for (ic, ic_qty) in input_chems {
        if ic == "ORE" {
            *used_chems.entry(ic.to_string()).or_insert(0) += ic_qty;
            continue;
        }

        loop {
            if let Some(existing_qty) = unused_chems.get(ic) {
                if existing_qty >= ic_qty {
                    break;
                }
            }
            create_chemical(reactions, ic, unused_chems, used_chems)?;
        }

        if let Some(existing_qty) = unused_chems.get_mut(ic) {
            *existing_qty -= ic_qty;
        }
        *used_chems.entry(ic.to_string()).or_insert(0) += ic_qty;
    }

    *unused_chems.entry(chemical.to_string()).or_insert(0) += output_qty;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_single_input_chem() {
        let (output_chem, output_qty, input_chems) = parse_line("10 ORE => 10 A").unwrap();
        assert_eq!("A", output_chem);
        assert_eq!(10, output_qty);
        let mut expected_input_chems = HashMap::new();
        expected_input_chems.insert(String::from("ORE"), 10);
        assert_eq!(expected_input_chems, input_chems);
    }

    #[test]
    fn test_parse_multiple_input_chems() {
        let (output_chem, output_qty, input_chems) = parse_line("7 A, 1 E => 1 FUEL").unwrap();
        assert_eq!("FUEL", output_chem);
        assert_eq!(1, output_qty);
        let mut expected_input_chems = HashMap::new();
        expected_input_chems.insert(String::from("A"), 7);
        expected_input_chems.insert(String::from("E"), 1);
        assert_eq!(expected_input_chems, input_chems);
    }

    #[test]
    fn ex1() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
        ";

        let reactions = parse_reactions(input).unwrap();
        assert_eq!(31, find_ore_for_fuel(&reactions).unwrap());
    }

    #[test]
    fn ex2() {
        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ";

        let reactions = parse_reactions(input).unwrap();
        assert_eq!(13312, find_ore_for_fuel(&reactions).unwrap());
    }

    #[test]
    fn ex3() {
        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
        ";

        let reactions = parse_reactions(input).unwrap();
        assert_eq!(180697, find_ore_for_fuel(&reactions).unwrap());
    }

    #[test]
    fn ex4() {
        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
        ";

        let reactions = parse_reactions(input).unwrap();
        assert_eq!(2210736, find_ore_for_fuel(&reactions).unwrap());
    }
}
