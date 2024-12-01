use aoc24;

use itertools::{multizip, Itertools};

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut left_ints = Vec::new();
    let mut right_ints = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let (left, right) =
            line.split_whitespace().collect_tuple().unwrap();

        left_ints.push(aoc24::parse_int(left));
        right_ints.push(aoc24::parse_int(right));
    }

    left_ints.sort();
    right_ints.sort();

    let total: i64 = multizip((left_ints, right_ints))
        .map(|(x, y)| num::abs(x - y))
        .sum();

    println!("Total: {total}")
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let mut left_ints = Vec::new();
    let mut right_ints = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let (left, right) =
            line.split_whitespace().collect_tuple().unwrap();

        left_ints.push(aoc24::parse_int(left));
        right_ints.push(aoc24::parse_int(right));
    }

    let total: i64 = left_ints
        .iter()
        .map(|x| {
            x * right_ints.iter().filter(|y| *y == x).count()
                as i64
        })
        .sum();

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
