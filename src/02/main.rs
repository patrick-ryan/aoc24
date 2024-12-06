use aoc24::{self, parse_int};
use itertools::Itertools;
use num::abs;

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let mut direction = None;
        let mut prev = None;
        let mut valid = true;
        for n in line.split_whitespace() {
            let n_int = parse_int(n);
            if prev == None {
                prev = Some(n_int);
                continue;
            }
            let prev_int = prev.unwrap();

            if prev_int == n_int || abs(prev_int - n_int) > 3 {
                valid = false;
                break;
            }

            let cur_direction;
            if n_int > prev_int {
                cur_direction = 1;
            } else {
                cur_direction = -1;
            }
            match direction {
                Some(v) if (v == cur_direction) => (),
                Some(_) => {
                    valid = false;
                    break;
                }
                None => direction = Some(cur_direction),
            }
            prev = Some(n_int);
        }

        if valid {
            total += 1;
        }
    }

    println!("Total: {total}")
}

fn part2_first_attempt() {
    let lines = aoc24::parse_arg_file_lines();

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let mut direction = None;
        let mut prev = None;
        let mut valid = true;
        let mut tolerance = true;
        let ints: Vec<&str> =
            line.split_whitespace().collect_vec();
        let mut i = 0;
        while i < ints.len() {
            let n = ints[i];
            let n_int = parse_int(n);
            let mut level_removed = false;
            if prev != None {
                let prev_int = prev.unwrap();

                if prev_int == n_int || abs(prev_int - n_int) > 3
                {
                    if tolerance {
                        tolerance = false;
                        level_removed = true;
                    } else {
                        valid = false;
                        break;
                    }
                } else {
                    let cur_direction;
                    if n_int > prev_int {
                        cur_direction = 1;
                    } else {
                        cur_direction = -1;
                    }
                    match direction {
                        Some(v) if (v == cur_direction) => (),
                        Some(_) if tolerance => {
                            tolerance = false;
                            level_removed = true;
                        }
                        Some(_) => {
                            valid = false;
                            break;
                        }
                        None => direction = Some(cur_direction),
                    }
                }
            }

            if level_removed {
                if i >= 2 {
                    let new_prev = parse_int(ints[i - 2]);
                    prev = Some(new_prev);
                    direction = None;
                } else {
                    i += 1;
                    direction = None;
                }
            } else {
                prev = Some(n_int);
                i += 1;
            }
        }

        if valid {
            total += 1;
        } else {
            println!("no: {line}")
        }
    }

    println!("Total: {total}")
}

fn _is_valid(chars: &Vec<&str>) -> bool {
    let mut direction = None;
    let mut prev = None;
    let mut valid = true;
    for n in chars {
        let n_int = parse_int(n);
        if prev == None {
            prev = Some(n_int);
            continue;
        }
        let prev_int = prev.unwrap();

        if prev_int == n_int || abs(prev_int - n_int) > 3 {
            valid = false;
            break;
        }

        let cur_direction;
        if n_int > prev_int {
            cur_direction = 1;
        } else {
            cur_direction = -1;
        }
        match direction {
            Some(v) if (v == cur_direction) => (),
            Some(_) => {
                valid = false;
                break;
            }
            None => direction = Some(cur_direction),
        }
        prev = Some(n_int);
    }
    return valid;
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let mut total = 0;
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let chars = line.split_whitespace().collect_vec();
        let mut valid = _is_valid(&chars);

        if !valid {
            for char_i in 0..chars.len() {
                let mut new_chars = chars.clone();
                new_chars.remove(char_i);
                valid = _is_valid(&new_chars);
                if valid {
                    break;
                }
            }
        }

        if valid {
            total += 1;
        }
    }

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
