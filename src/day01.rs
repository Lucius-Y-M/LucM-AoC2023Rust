use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;




pub fn pt1(input: Vec<&str>) -> usize {

    input
        .into_iter()
        .filter_map(|line| {
            let digits = line
                .bytes()
                .into_iter()
                .filter_map(|c| {
                    if (c >= b'0') & (c <= b'9') {
                        Some((c - b'0') as usize)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            if digits.len() == 0 { None }
            else {
                Some(
                    10 * *digits.first().unwrap()
                    + *digits.last().unwrap()
                )
            }
        })
        .sum()
}




lazy_static! {

    static ref OUR_MAP: HashMap<String, u8> = HashMap::from_iter([
        String::from("zero"),
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ].into_iter()
    .zip(0..=9)
    );
}

pub fn pt2(input: Vec<&str>) -> usize {


    
    input
        .into_iter()
        .filter_map(|line| {


            /* get direct digits */
            let digits = line
                .as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if (*c >= b'0') & (*c <= b'9') {
                        Some((i, (*c - b'0')))
                    } else {
                        None
                    }
                })
                .collect:: <Vec<_>>();

            /* get all words */
            let words = if line.len() >= 3 {
                let temp = OUR_MAP
                    .keys()
                    .into_iter()
                    .filter_map(|k| {
                        let idxs: Vec<_> = line
                            .match_indices(k)
                            .map(|(idx, word)| (idx, *OUR_MAP.get(word).unwrap()))
                            .collect();
                        match idxs.len() == 0 {
                            true => None,
                            false => Some(idxs),
                        }
                    })
                    .flatten()
                    .sorted_by_key(|(idx, _)| *idx)
                    .collect::<Vec<_>>();

                match temp.is_empty() {
                    true => None,
                    false => Some(temp),
                }
            } else {
                None
            };


            /* we KNOW there are at least digits, so check words first */
            if let Some(words) = words {

                let (w_first, w_last) = (*words.first().unwrap(), *words.last().unwrap());
                let (d_first, d_last) = (*digits.first().unwrap(), *digits.last().unwrap());

                let candidates = [w_first, w_last, d_first, d_last]
                    .into_iter()
                    .sorted_by_key(|(idx, _)| *idx)
                    .collect::<Vec<_>>();

                Some(
                    (10 * candidates.first().unwrap().1
                    + candidates.last().unwrap().1)
                    as usize
                )
            } else {
                if digits.len() == 0 { None }
                else {
                    Some(
                        (10 * digits.first().unwrap().1
                        + digits.last().unwrap().1)
                        as usize
                    )
                }
            }
        })
        .sum()
}