use std::{cmp::Ordering, collections::HashSet};

use aoc24::{self, parse_int};
use itertools::Itertools;

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut is_updates_section = false;
    let mut rules = HashSet::new();
    let mut updates = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        if line == "" {
            is_updates_section = true;
            continue;
        }

        if is_updates_section {
            updates.push(
                line.split(',')
                    .map(|s| s.to_string())
                    .collect_vec(),
            );
        } else {
            rules.insert(line.to_string());
        }
    }

    let mut total = 0;
    for update in updates {
        let mut valid = true;

        for i in 0..update.len() {
            for j in i..update.len() {
                // check if violation exists -> a later element is supposed to be before this element
                let test_rule =
                    format!("{}|{}", update[j], update[i]);
                if rules.contains(&test_rule) {
                    valid = false;
                    break;
                }
            }
            if valid == false {
                break;
            }
        }

        if valid {
            total += parse_int(&update[(update.len() - 1) / 2]);
        }
    }

    println!("Total: {total}")
}

fn sort_update(
    update: &Vec<String>,
    rules: &HashSet<String>,
) -> Vec<String> {
    // sort by comparator rules, could have just used this in part 1...
    let mut res = update.clone();
    res.sort_unstable_by(|a, b| {
        let test_rule = format!("{}|{}", b, a);
        if rules.contains(&test_rule) {
            return Ordering::Greater;
        }
        return Ordering::Less;
    });
    return res;
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let mut is_updates_section = false;
    let mut rules = HashSet::new();
    let mut updates = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        if line == "" {
            is_updates_section = true;
            continue;
        }

        if is_updates_section {
            updates.push(
                line.split(',')
                    .map(|s| s.to_string())
                    .collect_vec(),
            );
        } else {
            rules.insert(line.to_string());
        }
    }

    let mut total = 0;
    for update in updates {
        let corrected = sort_update(&update, &rules);
        if corrected != update {
            total +=
                parse_int(&corrected[(corrected.len() - 1) / 2]);
        }
    }

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
