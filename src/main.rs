use aoc_2023::day01;







fn main() {

    /* d01 */
    println!("Day 01 res 1 OR 2 = {}", d01());
}


fn d01() -> usize {

    let input = include_str!("day01.txt")
        .split("\n")
        .collect();

    /* 1 or 2 */
    // day01::pt1(input)
    day01::pt2(input)
}
