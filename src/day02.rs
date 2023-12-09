use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE: Regex = Regex::new(r"(([0-9]+) ([a-z]+))[,]?").expect("REGEX PARSING ERROR");
);

pub fn pt1(input: Vec<&str>) -> u32 {

    input
        .into_iter()
        .filter_map(|line| {
            
            match parse(line) {
                Ok(data) => {
                    /*
                        12 red cubes, 13 green cubes, and 14 blue cubes.
                    */
                    if data.1.iter().any(|bs| bs.red > 12 || bs.green > 13 || bs.blue > 14) {
                        None
                    } else {
                        Some(data.0 as u32)
                    }                    
                },
                Err(err) => {
                    println!("{}", err);
                    None
                },
            }
        })
        .sum()
}


pub fn pt2(input: Vec<&str>) -> u64 {
    input
        .into_iter()
        .filter_map(|line| parse(line).ok())
        .map(|data| {
            //// find min that is NOT 0
            let calc = data.1
                .into_iter()
                .fold([0, 0, 0], |mut acc, next| {
                    if next.blue != 0 && next.blue > acc[0] {
                        acc[0] = next.blue;
                    }
                    if next.green != 0 && next.green > acc[1] {
                        acc[1] = next.green;
                    }
                    if next.red != 0 && next.red > acc[2] {
                        acc[2] = next.red;
                    };
                    println!("acc = {:?}", acc);
                    acc
                });
            
            let res: u64 = calc.into_iter().fold(1, |prev, next| prev * (next as u64));
            println!("curr res = {}", res);
            res
        })
        .sum()
}





#[derive(Debug, Copy, Clone)]
enum Ball {
    Red(u8),
    Blue(u8),
    Green(u8),
}
#[derive(Debug, Copy, Clone, Default)]
pub struct BallSet {
    red: u8,
    blue: u8,
    green: u8,
}

const ALLOWED: [&str; 3] = ["red", "blue", "green"];
/*
    Game 71: 6 blue, 3 red, 12 green; 2 red, 8 green, 3 blue; 8 green, 8 blue; 7 blue, 1 red, 9 green; 2 green, 4 blue, 1 red; 3 red, 7 blue, 8 green
*/
//// This gives each "set" of balls in a game
pub struct GameData(usize, Vec<BallSet>);
fn parse(line: &str) -> Result<GameData, &str> {


    let (game, sets) = line
        .split_once(":")
        .ok_or("Splitting line failed!")?;

    let game = game
        .split_once(" ")
        .ok_or("Splitting First part of line failed!")?
        .1
        .parse::<usize>()
        .or(Err("Game No. is not valid!"))?;

    let sets: Vec<BallSet> = sets
        .split(";")
        .filter_map(|balls| {
            /*
                parse the numbers of balls
                into enum BALL
            */
            let mut balls_and_nums = RE
                .captures_iter(balls)
                .filter_map(|c| {
                    let b_num = c.get(2)
                        .map_or(None, |c| {
                            c.as_str().parse::<u8>().ok()
                        })?;
                    let b_name = c.get(3)
                        .map_or(None, |c| {
                            let c = c.as_str();
                            match ALLOWED.contains(&c) {
                                true => Some(c),
                                false => {
                                    println!("!! Unknown ball type {}", c);
                                    None
                                },
                            }
                        })?;
                    
                    match b_name {
                        // RED
                        _ if b_name == ALLOWED[0] => {
                            Some(Ball::Red(b_num))
                        },
                        // BLUE
                        _ if b_name == ALLOWED[1] => {
                            Some(Ball::Blue(b_num))
                        },
                        // GREEN
                        _ if b_name == ALLOWED[2] => {
                            Some(Ball::Green(b_num))
                        },
                        _ => {
                            println!("!! ball name = {}, unexpected", b_name);
                            None
                        }
                    }
                    
                })
                .collect_vec();
            println!("{b:?}", b=balls_and_nums);

            match (1..=3).contains(&balls_and_nums.len()) {
                true => {

                    let mut bs = BallSet::default();

                    while let Some(b_n_num) = balls_and_nums.pop() {
                        match b_n_num {
                            Ball::Red(n) => bs.red = n,
                            Ball::Blue(n) => bs.blue = n,
                            Ball::Green(n) => bs.green = n,
                        }
                    }

                    Some(bs)
                },
                false => {
                    println!("!! FAILED! Expected num of ball types: 1..=3, actual: {}", balls_and_nums.len());
                    None
                },
            }
        })
        .collect();


    Ok(GameData(game, sets))
}