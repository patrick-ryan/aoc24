use aoc24::{self, parse_int};
use itertools::Itertools;

use regex::Regex;

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let regex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)")
        .expect("Invalid regex");

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        for rmatch in regex.captures_iter(&line) {
            let (_, [group]) = rmatch.extract();

            let (n1, n2) =
                group.split(',').collect_tuple().unwrap();

            total += parse_int(n1) * parse_int(n2);
        }
    }

    println!("Total: {total}")
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let regex = Regex::new(
        r"mul\((\d{1,3},\d{1,3})\)|(don't\(\))|(do\(\))",
    )
    .expect("Invalid regex");

    let mut total = 0;
    let mut skip = false;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        for rmatch in regex.captures_iter(&line) {
            let (_, [group]) = rmatch.extract();

            match group {
                "don't()" => {
                    skip = true;
                    continue;
                }
                "do()" => {
                    skip = false;
                    continue;
                }
                _ if skip => continue,
                _ => (),
            }

            let (n1, n2) =
                group.split(',').collect_tuple().unwrap();

            total += parse_int(n1) * parse_int(n2);
        }
    }

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
