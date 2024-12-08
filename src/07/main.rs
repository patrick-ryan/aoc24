use core::panic;

use aoc24::{self, parse_int};
use itertools::Itertools;

fn get_string_permutations(
    symbols: Vec<&str>,
    len: usize,
) -> Vec<Vec<String>> {
    return (2..len).fold(
        symbols
            .iter()
            .cartesian_product(symbols.iter().cloned())
            .map(|t| vec![t.0.to_string(), t.1.to_string()])
            .collect_vec(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(symbols.iter().cloned())
                .map(|t| [t.0, vec![t.1.to_string()]].concat())
                .collect_vec()
        },
    );
}

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let (sum, str_values) =
            line.split(": ").collect_tuple().unwrap();

        let values =
            str_values.split(" ").map(parse_int).collect_vec();

        let symbols = vec!["+", "*"];
        let operators =
            get_string_permutations(symbols, values.len() - 1);

        for operator_combo in operators {
            let mut computed = values[0];
            for i in 1..values.len() {
                match operator_combo[i - 1].as_str() {
                    "+" => computed += values[i],
                    "*" => computed *= values[i],
                    _ => panic!(),
                }
            }

            if computed == parse_int(sum) {
                total += computed;
                break;
            }
        }
    }

    println!("Total: {total}")
}

fn part2() {
    // this is very slow, took almost a minute to run on input, but oh well.
    // possible that could do a binary search or something of the permutations
    // where the permutations are sorted somehow
    let lines = aoc24::parse_arg_file_lines();

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let (sum, str_values) =
            line.split(": ").collect_tuple().unwrap();

        let values =
            str_values.split(" ").map(parse_int).collect_vec();

        let symbols = vec!["+", "*", "||"];
        let operators =
            get_string_permutations(symbols, values.len() - 1);

        for operator_combo in operators {
            let mut computed = values[0];
            for i in 1..values.len() {
                match operator_combo[i - 1].as_str() {
                    "+" => computed += values[i],
                    "*" => computed *= values[i],
                    "||" => {
                        computed = parse_int(
                            format!("{}{}", computed, values[i])
                                .as_str(),
                        )
                    }
                    _ => panic!(),
                }
            }

            if computed == parse_int(sum) {
                total += computed;
                break;
            }
        }
    }

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
