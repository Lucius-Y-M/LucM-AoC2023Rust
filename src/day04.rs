use std::{io, collections::HashSet, ops::Add};
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};



#[derive(Debug, Default)]
pub struct ParseOutput {
    game_idx_info: i32,
    winning_nums: HashSet<i32>,
    bought_nums: HashSet<i32>,
}

pub fn pt2() -> Result<BigUint, ()> {

    let parse_out = parse()?;

    let mut card_counts = vec![1.to_biguint().unwrap() ; parse_out.len()];

    parse_out
            .into_iter()
            .enumerate()
            /* curr count, CURR pow of 2 */
            .fold((0, 0), |mut acc, (idx, next)| {
                // let idx = idx + 1;
                acc.0 += idx as i128;
                
                let count = next.winning_nums
                    .intersection(&next.bought_nums)
                    .count();
    
                if count > 0 {
                    acc.1 += 1;
                    card_counts[idx+1..=idx+count]
                        .iter_mut()
                        .for_each(|cnt| {*cnt += 2.to_biguint().unwrap().pow(acc.1 - 1);})
                }
                // println!("curr counts {:?}; curr acc {:?}",
                //     card_counts,
                //     acc
                // );
    
                acc
            });
    println!("curr counts: {:?}", card_counts);
    Ok(card_counts.into_iter().sum())
}


pub fn pt1() -> Result<i32, ()> {

    let parse_out = parse()?;

    Ok(parse_out
            .into_iter()
            .filter_map(|p| {
                let count = p.winning_nums.intersection(&p.bought_nums).count();
                
                if count == 0 { return None; }

                let res = 2i32.pow(count as u32 - 1);
                // println!("{}", res);
                Some(res)
            })
            .sum()
    )

}



fn parse() -> Result<Vec<ParseOutput>, ()> {

    // let input = io::read_to_string(io::stdin()).map_err(|s| {
    //     println!("Reading file failed, reason: {}", s.to_string());
    // })?;

    let input = include_str!("day04.txt");

    let lines = input
        .lines()
        .filter_map(|line| {
            let (game_idx, game_info) = line.split_once(':')?;
            let (win_nums, own_nums) = game_info.split_once('|')?;

            let (game_idx, win_nums, own_nums) = (
                parse_line_for_nums_whitespace(game_idx).into_iter().last()?,
                parse_line_for_nums_whitespace(win_nums),
                parse_line_for_nums_whitespace(own_nums)
            );

            Some(ParseOutput {
                game_idx_info: game_idx,
                winning_nums: win_nums,
                bought_nums: own_nums,
            })
            
        }).collect_vec();

    Ok(lines)
}


fn parse_line_for_nums_whitespace(line: &str) -> HashSet<i32> {
    line
        .split_whitespace()
        .filter_map(|part| {
            match part.parse::<i32>() {
                Ok(p) => Some(p),
                Err(_) => {
                    part
                    .as_bytes()
                    .into_iter()
                    .filter_map(|ch| {
                        if (*ch >= b'0') & (*ch <= b'9') {
                            Some(*ch as char)
                        } else {
                            None
                        }
                    })
                    .collect::<String>()
                    .parse::<i32>()
                    .ok()
                },
            }
        })
        .collect()
}