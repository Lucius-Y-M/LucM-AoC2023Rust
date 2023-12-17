use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::BigUint;
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


// const DATA_TYPES: [&str; 8] = [
//     "seed",
//     "soil",
//     "fertilizer",
//     "water",
//     "light",
//     "temperature",
//     "humidity",
//     "location",
// ];

lazy_static!(
    static ref RE_TO: Regex = Regex::new(r"([a-z]*)-to-([a-z]*) .*").unwrap();
    // static ref RE_NUMS: Regex = Regex::new(r"([0-9]* {1})").unwrap();

    static ref MAPS_NAMES: [String; 7] = [
        String::from("seed-to-soil"),
        String::from("soil-to-fertilizer"),
        String::from("fertilizer-to-water"),
        String::from("water-to-light"),
        String::from("light-to-temperature"),
        String::from("temperature-to-humidity"),
        String::from("humidity-to-location")
    ];

);




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
    
    let mut all_maps: HashMap<_, HashMap<BigUint, BigUint>> = HashMap::from_iter(
        MAPS_NAMES.clone().into_iter()
        .map(|s| (s, HashMap::with_capacity(100)))
    );


    let mut curr_map_idx: Option<usize> = None;
    let _ = lines.iter()
        .filter_map(|line| {

            //// this is a TYPING line
            if let Some(cap) = RE_TO.captures(line) {
                let (prev, next) = (cap.get(1)?.as_str(), cap.get(2)?.as_str());
                let name = String::from_iter(prev.chars().chain("-to-".chars()).chain(next.chars()));
                
                curr_map_idx = match curr_map_idx {
                    Some(i) => Some(i+1),
                    None => Some(
                        MAPS_NAMES
                        .iter()
                        .enumerate()
                        .find(|(_, s)| s.as_bytes() == name.as_bytes())
                        .map(|(idx, _)| idx)?
                    ),
                };
                /* only 6 items listed */
                assert!(if let Some(ref i) = curr_map_idx {
                    *i < 7
                } else { true });

                Some(true)
            }
            //// this is a NUMBERS line
            // else if let Some(cap) = RE_NUMS.captures(line) {

            //     assert!(curr_map_idx.is_some());
            //     let curr_map = all_maps.get_mut(&MAPS_NAMES[curr_map_idx?])?;


            //     None
            // }
            //// something else
            else {
                println!("== Skipped line {}", line);
                None
            }
        })
        .count();


    Ok(())
}