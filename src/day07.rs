use std::{collections::HashMap, cmp::Ordering};

use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use regex::Regex;



const CNT: usize = 13;

/* for part 1 */
// const CARDS: [char; CNT] = [
//     'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
// ];

/* for part 2 */
const CARDS: [char; CNT] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
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



pub fn part1() -> BigUint {

    let mut parsed = parse();

    /* here: weakest first, to help count upward */
    parsed.sort_unstable_by(|h1, h2| h2.cmp(h1));

    // for l in parsed.iter() {
    //     println!("After sorting, hands: {:?}", l);
    // }

    parsed.into_iter()
    .enumerate()
    .map(|(idx, hand)| {
        // start from 1
        let idx = idx + 1;        

        // dbg!(
            BigUint::from(idx * hand.bid)
        // )
    })
    .fold(BigUint::from(0u8), |mut acc, next| {
        acc += next;
        acc
    })
}

pub fn part2() -> BigUint {

    let mut parsed = parse();

    /* here: weakest first, to help count upward */
    parsed.sort_unstable_by(|h1, h2| h2.cmp(h1));

    // for l in parsed.iter() {
    //     println!("After sorting, hands: {:?}", l);
    // }

    parsed
    .into_iter()
    .enumerate()
    .map(|(idx, hand)| {
        // start from 1
        let idx = idx + 1;        

        // dbg!(
            BigUint::from(idx * hand.bid)
        // )
    })
    .fold(BigUint::from(0u8), |mut acc, next| {
        acc += next;
        acc
    })
}


// #[derive(Debug, Clone, Eq)]
#[derive(Debug, Clone)]
pub enum HandType {
    FiveAKind(char),
    FourAKind((char, [char;1])), /* 4+1 */
    FullHouse((char, char)), /* 3+2 */
    ThreeAKind((char, [char;2])), /* 3, 1+1 */
    TwoPairs((char, char, [char;1])), /* 2+2, 1 */
    OnePair((char, [char;3])), /* 2, 1+1+1 */
    High([char; 5])
}

    impl Ord for HandType {
        fn cmp(&self, other: &Self) -> Ordering {
            // Assign a unique numeric value to each variant
            fn variant_rank(hand_type: &HandType) -> usize {
                match hand_type {
                    /* the lower, the better, so 0 > 6 */
                    HandType::FiveAKind(_) => 0,
                    HandType::FourAKind(_) => 1,
                    HandType::FullHouse(_) => 2,
                    HandType::ThreeAKind(_) => 3,
                    HandType::TwoPairs(_) => 4,
                    HandType::OnePair(_) => 5,
                    HandType::High(_) => 6,
                }
            }

            variant_rank(self).cmp(&variant_rank(other))
        }
    }

    impl PartialOrd for HandType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // Implement PartialEq and Eq based on Ord
    impl PartialEq for HandType {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == std::cmp::Ordering::Equal
        }
    }

    impl Eq for HandType {}



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<char>,
    cards_sorted: Vec<char>, /* biggest first */
    cards_counts: Vec<(char, usize)>,
    bid: usize,

    handtype: HandType,
    handtype_joker: Option<HandType> /* only if J exists */

}


/* impl Hand */


    /* part1 */
    #[allow(dead_code)]
    fn cmp_impl_pt1(own: &Hand, other: &Hand) -> Ordering {
        match own.handtype.cmp(&other.handtype) {
                
            Ordering::Equal => {
                match own.cards == other.cards {
                    true => Ordering::Equal,
                    false => {
                        for (c1, c2) in own.cards.iter().zip(other.cards.iter()) {
                            match Ord::cmp(&CARDS_RANKINGS[c1], &CARDS_RANKINGS[c2]) {
                                Ordering::Equal => continue,
                                uneq => return uneq,
                            }
                        }

                        Ordering::Equal
                    },
                }    
            },

            other => other
        }
    }

    #[allow(dead_code)]
    fn cmp_impl_pt2(own: &Hand, other: &Hand) -> Ordering {

        /* get strongest hand, if exists */
        let own_hand = if let Some(ref jok) = own.handtype_joker {
            jok
        } else {
            &own.handtype
        };
        // println!("own hand = {:?}", own_hand);

        let other_hand = if let Some(ref jok) = other.handtype_joker {
            jok
        } else {
            &other.handtype
        };
        // println!("other hand = {:?}", other_hand);

        match own_hand.cmp(&other_hand) {
            
            /* this means the two hands are equal, so we no longer count J as wildcard */
            Ordering::Equal => {
                match own.cards == other.cards {
                    true => Ordering::Equal,
                    false => {
                        for (c1, c2) in own.cards.iter().zip(other.cards.iter()) {
                            match Ord::cmp(&CARDS_RANKINGS[c1], &CARDS_RANKINGS[c2]) {
                                Ordering::Equal => continue,
                                uneq => return uneq,
                            }
                        }

                        Ordering::Equal
                    },
                }    
            },

            other => other
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            // cmp_impl_pt1(self, other) /* part1 */
            cmp_impl_pt2(self, other) /* part2 */
        }
    }




    
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }
    
    impl Eq for Hand {}
    



fn parse() -> Vec<Hand> {

    let input = include_str!("day07.txt").lines().collect_vec();

    let parsed = input
    .into_iter()
    .filter_map(|line|/* get: cards, bid */ {
        let q = RE_PARSE.captures(line)?;

        // q.iter().for_each(|p| {println!("cap part = {:?}", p);});

        let (cards, bid) = (
            q.get(1)?.as_str().chars().collect_vec(),
            q.get(2)?.as_str().parse::<usize>().ok()?
        );

        let mut cards_sorted = cards.clone();

        /* biggest first */
        cards_sorted.sort_unstable_by(|c1, c2| CARDS_RANKINGS[c1].cmp(&CARDS_RANKINGS[c2]));


        let (cards_counts, handtype, handtype_joker) = {

            let mut v: Vec<(char, usize)> = Vec::with_capacity(5);



            let mut prev = *cards_sorted.get(0)?;
            v.push((prev, 1));
            for &c in cards_sorted.iter().skip(1) {
                if prev == c {
                    v.last_mut()?.1 += 1;
                } else {
                    prev = c;
                    v.push((c, 1));
                }
            }

            // highest freq first
            v.sort_unstable_by_key(|c_c| -(c_c.1 as i8));

            let get_handtype = |vec: &Vec<(char, usize)>| -> Option<HandType> {

                let first = *vec.first()?;
                let handtype: HandType;
                match vec {
                    _ if vec.len() == 1 => { handtype = HandType::FiveAKind(first.0)},
                    _ if vec.len() == 2 => {
                        if first.1 == 4 { handtype = HandType::FourAKind((first.0, [vec[1].0])); }
                        else { handtype = HandType::FullHouse((first.0, vec[1].0)); }
                    },
                    _ if vec.len() == 3 => {
                        if first.1 == 3 { handtype = HandType::ThreeAKind((first.0, [vec[1].0, vec[2].0])); }
                        else { handtype = HandType::TwoPairs((first.0, vec[1].0, [vec[2].0])); }
                    },

                    _ if vec.len() == 4 => {
                        handtype = HandType::OnePair((first.0, vec[1..].iter().map(|c_c| c_c.0).collect_vec().try_into().ok()?));
                    }
                    
                    _ => {
                        handtype = HandType::High(vec.iter().map(|s| s.0).collect_vec().try_into().ok()?);
                    }

                }
                Some(handtype)
            };

            let handtype = get_handtype(&v)?;



            

            /* for part 2 */
            let mut handtype_joker = None;
            if let Some((idx, _)) = v.iter().enumerate().find(|(_, s)| s.0 == 'J') {
                let mut vc = v.clone();                
                let j_cnt = vc.swap_remove(idx).1;
                vc.first_mut()?.1 += j_cnt;


                // println!("curr vc = {:?}", vc);
                handtype_joker = Some(get_handtype(&vc))?;                
            }



            (v, handtype, handtype_joker)
        };
        
        Some(Hand {
            cards,
            cards_sorted,
            cards_counts,
            bid,
            handtype,
            handtype_joker
        })
    })
    .collect_vec();

    parsed

}