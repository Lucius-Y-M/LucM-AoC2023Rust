use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use regex::Regex;



lazy_static!(

    static ref RE_ELIM: Regex = Regex::new(r"[A-Za-z]*:[ ]*(.*)").unwrap();
    static ref RE_NUM: Regex = Regex::new(r"([0-9]+)").unwrap();

);


pub fn part2() -> u128 {
    let nums = parse();
    assert_eq!( nums.len(), 2 );
    assert_eq!( nums[0].len(), nums[1].len() );

    let mut time_dist = nums
        .into_iter()
        .map(|line| {
            line
            .into_iter()
            .map(|i| i.to_string())
            .fold(String::with_capacity(20), |mut acc, next| {
                acc.extend(next.chars());
                acc
            })
            .parse::<u128>()
            .unwrap()
        })
        .collect_vec();

    assert!(time_dist.len() == 2);
    println!("curr td: {:?}", time_dist);

    let (dist, time) = (time_dist.pop().unwrap(), time_dist.pop().unwrap());
    
    get_ways_num(time, dist).unwrap()
        
}


pub fn part1() -> BigUint {

    let mut nums = parse();


    assert_eq!( nums.len(), 2 );
    assert_eq!( nums[0].len(), nums[1].len() );

    /* order: DISTANCE, TIME */
    let res = nums
        .pop().unwrap()
        .into_iter()
        .zip(
            nums
            .pop().unwrap()
            .into_iter()
        )
        .filter_map(|(dist, time)| {
            match get_ways_num(time, dist) {
                Some(x) => {
                    println!("curr x = {}", x);
                    Some(x)
                },
                None => {
                    eprintln!("ERROR beim get_ways_num with time = {} & dist = {}", time, dist);
                    None
                },
            }
        })
        .fold(BigUint::from(1u128), |curr, next| {
            curr * next
        });

    res

}


fn parse() -> Vec<Vec<u128>> {
    let lines = include_str!("day06.txt").split("\r\n").collect_vec();
    assert!(lines.len() == 2);
    let mut nums = lines
        .into_iter()
        .filter_map(|line| {
            let cap = RE_ELIM.captures(line)?.get(1)?.as_str();
            let nums = RE_NUM
                .find_iter(cap)
                .filter_map(|part| {
                    let p = part.as_str();
                    println!("cap'ed p = {}", p);

                    p.parse::<u128>().ok()
                })
                .collect_vec();
            Some(nums)
        })
        .collect_vec();

    nums
}


type Ways = u128;

#[inline]
fn get_ways_num(time: u128, target_dist: u128) -> Option<Ways> {

    let (left_incl, right_incl) = solve_eq(time, target_dist)?;

    println!("input: t = {}, d = {}, l&r={},{}", time, target_dist, left_incl, right_incl);

    let range_cnt = if time % 1 == 0 { right_incl + 1 - left_incl } else { right_incl - left_incl };

    Some(range_cnt)
}



/*
    consts A, B

    ALL ints x | x(A-x) > B
    find: x | x(A-x) = B

    x^2 - Ax + B = 0
    x = ( A (+-) sqrt(A^2 - 4 * 1 * B) ) / 2

    get x1, x2
    ALL ints: [ceil(x1), floor(x2)]
*/
#[inline]
fn solve_eq(a: u128, b: u128) -> Option<(u128, u128) /* least int, max int, for inclus range */> {

    let sqrt = ((a*a).checked_sub( 4 * b)? as f64).sqrt();

    let (r1, r2) = ( f64::ceil( (a as f64 - sqrt) / 2f64 ), f64::floor( (a as f64 + sqrt) / 2f64 ));
    // println!("r1 r2 before transf ={},{}", r1, r2);

    let r1 = if r1 < 0f64 { 0 } else {
        let r1 = r1 as u128;
        if r1 * (a - r1) == b { r1 + 1 }
        else  { r1 }
    };
    let r2 = {
        let r2 = r2 as u128;
        if r2 * (a - r2) == b { r2 - 1 }
        else  { r2 }
    };
    // println!("r1 r2 post transf ={},{}", r1, r2);

    /* smaller root may < 0, if so = 0 */
    Some((
        r1,
        r2
    ))
    // Some((r_1, r_2))

}



// #[inline]
// fn get_min_overtake_pair(n: u128) -> Option<(u128, u128)> {
//     let s = n.sqrt();
//     match Ord::cmp(&(s*s), &n) {
//         Ordering::Less => Some((s,s+1)),
//         Ordering::Equal => Some((s,s)),
//         Ordering::Greater => {
//             eprintln!("ERROR: sqrt^2 > orig");
//             None
//         },
//     }
// }

// #[inline]
// fn get_largest_mul(n: u128) -> BigUint {

//     let a = n >> 1;
//     let b = n - a;
//     BigUint::from( a * b )
// }