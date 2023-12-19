use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use regex::Regex;



const CNT: usize = 13;
const CARDS: [char; CNT] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
];

lazy_static!(
    // k: char, v: ranking (LOWER is better)
    static ref CARDS_RANKINGS: HashMap<char, usize> = HashMap::from_iter(
        CARDS
        .iter()
        .enumerate()
        .map(|(idx, &ch)| (ch, idx))
    );
    static ref RE_PARSE: Regex = Regex::new(r"([0-9A-Z]+) ([0-9]+)").unwrap();

);



fn part1() -> BigUint {



    BigUint::from(1u32)
}




#[allow(dead_code)]
pub struct Hand {
    cards: Vec<char>,
    cards_counts: Vec<(char, usize)>
}

fn parse() -> Vec<Hand> {

    let input = include_str!("day07.txt").lines().collect_vec();

    //// BUFFER:
    let mut cnt_bnf: HashMap<char, usize> = HashMap::from_iter(
        CARDS
        .iter()
        .map(|&c| (c, 0))
    );

    let parsed = input
    .into_iter()
    .filter_map(|line|/* get: cards, bid */ {
        let q = RE_PARSE.captures(line)?;
        let (mut cards, num) = (
            q.get(1)?.as_str().chars().collect_vec(),
            q.get(2)?.as_str().parse::<usize>().ok()?
        );

        cards.sort_unstable_by(|c1, c2| CARDS_RANKINGS[c2].cmp(&CARDS_RANKINGS[c1]));

        let cards_counts = {

            let mut v: Vec<(char, usize)> = Vec::with_capacity(5);

            let mut prev = None;
            for &c in cards.iter() {
                if let Some(prev) = prev {
                    if prev == c {
                        v.last_mut()?.1 += 1;
                    } else {
                        v.push((c, 1));
                    }
                } else {
                    prev = Some(c);
                    v.push((c, 1));
                }
            }

            v
        };
        
        Some(Hand {
            cards,
            cards_counts,
        })
    })
    .collect_vec();

    parsed

}