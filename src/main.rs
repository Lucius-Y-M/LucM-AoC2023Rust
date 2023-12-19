use aoc_2023::{day01, day02, day03, day04, day06, day07};







fn main() {

    /* d01 */
    // println!("Day 01 res 1 OR 2 = {}", d01());
    
    /* d02 */
    // println!("Day 02 Res 1 or 2 = {:?}", d02());

    /* d03 */
    // println!("Day 03 Res 1 or 2 = {:?}", day03::pt1());

    /* d04 */
    // println!("Day 03 Res 1 or 2 = {:?}", day04::pt2());


    /* d06 */

    // println!("Day 06 Res 1/2 = {:?}", day06::part2());

    /* d07 */
    println!("Day 07 Res 1 = {:?}", day07::part1());
    println!("Day 07 Res 2 = {:?}", day07::part2());
}













fn d02() -> u64 {
    let input = include_str!("day02.txt")
        .split("\n")
        .collect();

    // day02::pt1(input) as u64
    day02::pt2(input)
}

#[allow(dead_code)]
fn d01() -> usize {

    let input = include_str!("day01.txt")
        .split("\n")
        .collect();

    // day01::pt1(input)
    day01::pt2(input)
}
