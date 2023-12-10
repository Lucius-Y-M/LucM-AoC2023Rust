use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::{BigUint, BigInt};
use regex::Regex;






#[derive(Debug)]
#[allow(dead_code)]
struct SeedPlan {
    seed: BigUint,
    soil: BigUint,
    fertilizer: BigUint,
    water: BigUint,
    light: BigUint,
    temperature: BigUint,
    humidity: BigUint,
    location: BigUint,
}


const DATA_TYPES: [&str; 8] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

lazy_static!(
    static ref RE_TO: Regex = Regex::new(r"([a-z]*)-to-([a-z]*) .*").unwrap();
);


macro_rules! var_name_to_type {

    //// meant for set
    // ($var_name: ident, $type_a: ident, $capac: expr) => {
    //     {
    //         HashSet<$type_a>::with_capacity($capac)
    //     }
    // };
    //// meant for map
    ($type_a: ident, $type_b: ident, $capac: expr) => {
        {
            HashMap<$type_a, $type_b>::with_capacity($capac)
        }
    };
}


fn parse() -> Result<(), ()> {


    let input = include_str!("day05_ex.txt");
    let mut lines = input.lines().collect_vec();

    let seed_nums = lines
        .first().ok_or(())?
        .split_once(":")
        .ok_or_else(|| { println!("failed to split"); () })?.0
        .split(" ")
        .filter_map(|par| {
            BigUint::try_from(par.parse::<i128>().ok()?).ok()
        })
        .collect_vec();

    

    let mut seed_to_soil: HashMap<_, _>;
    let mut soil_to_fertilizer: HashMap<_, _>;
    let mut water_to_light: HashMap<_, _>;
    let mut light_to_temperature: HashMap<_, _>;
    let mut temperature_to_humidity: HashMap<_, _>;
    let mut humidity_to_location: HashMap<_, _>;
    
    let mut curr;
    lines.iter()
        .filter_map(|line| {
            let cap = RE_TO.captures(line);

            //// this is a TYPING line
            if let Some(cap) = cap {
                let (prev, next) = (cap.get(1)?.as_str(), cap.get(2)?.as_str());
                let name = String::from_iter(prev.chars().chain("_to_".chars()).chain(next.chars()));
                curr = var_name_to_type!(name, BigUint, BigUint);

                None
            }
            //// this is a NUMBERS line
            else {
                Some(vec![35])
            }
        });



    Ok(())
}