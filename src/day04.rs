use std::collections::HashSet;
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};



#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ParseOutput {
    game_idx_info: i32,
    winning_nums: HashSet<i32>,
    bought_nums: HashSet<i32>,
}



pub fn pt2() -> Result<BigUint, ()> {

    let parse_out = parse()?;

    // let bigint_0 = 0.to_biguint().unwrap();
    let bigint_1 = 1.to_biguint().unwrap();    

    let mut card_counts = vec![ bigint_1.clone() ; parse_out.len()];

    parse_out
        .into_iter()
        .enumerate()
        .for_each(|(idx, output)| {
            
            let count = output.winning_nums
                .intersection(&output.bought_nums)
                .count();
            /*
                Add [ curr_card_num ]
                to the next [ actual_won_nums ] card(s)
             */
            let curr_card_cnt = card_counts[idx].clone();
            card_counts[idx + 1..=idx + count]
                .iter_mut()
                .for_each(|cnt| {
                    *cnt += curr_card_cnt.clone();
                });

        });
        
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