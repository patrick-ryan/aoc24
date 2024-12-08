use std::collections::HashSet;

use aoc24::{self};
use itertools::Itertools;

fn get_path(
    grid: &Vec<Vec<char>>,
    path_start: &(usize, usize),
    direction: &(i32, i32),
) -> (Vec<(usize, usize)>, bool) {
    let mut path = Vec::new();
    let mut pos = *path_start;
    loop {
        if grid[pos.1][pos.0] == '#' {
            return (path, false);
        } else {
            path.push(pos);
            match direction {
                (0, -1) if pos.1 == 0 => return (path, true),
                (1, 0) if pos.0 == grid[0].len() - 1 => {
                    return (path, true)
                }
                (0, 1) if pos.1 == grid.len() - 1 => {
                    return (path, true)
                }
                (-1, 0) if pos.0 == 0 => return (path, true),
                _ => {
                    pos = (
                        (pos.0 as i32 + direction.0) as usize,
                        (pos.1 as i32 + direction.1) as usize,
                    )
                }
            }
        }
    }
}

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut maybe_start = None;
    for (i, line) in aoc24::iter_lines_or_die(lines) {
        let row = line.chars().collect_vec();

        if let Some(s) = row.iter().position(|&x| x == '^') {
            maybe_start = Some((s, i));
        }

        grid.push(row);
    }

    let mut visited = HashSet::new();
    let mut path_start = maybe_start.unwrap();
    let mut direction = (0, -1);
    loop {
        let (path, is_end) =
            get_path(&grid, &path_start, &direction);
        path_start = path.last().unwrap().to_owned();
        visited.extend(path);

        if is_end {
            break;
        }

        match direction {
            (0, -1) => direction = (1, 0),
            (1, 0) => direction = (0, 1),
            (0, 1) => direction = (-1, 0),
            (-1, 0) => direction = (0, -1),
            _ => panic!(),
        }
    }

    let total = visited.len();
    println!("Total: {total}")
}

fn extend_path(
    grid: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    direction: &(i32, i32),
) -> Vec<(usize, usize)> {
    let mut new_path = Vec::new();
    new_path.extend(get_path(grid, &path[0], &direction).0);
    let rev_direction = (direction.0 * -1, direction.1 * -1);
    new_path.extend(get_path(grid, &path[0], &rev_direction).0);

    new_path.sort_unstable();
    if direction.0 < 0 || direction.1 < 0 {
        new_path.reverse();
    }
    new_path.dedup_by(|a, b| a == b);

    return new_path;
}

fn get_loop_or_end(
    grid: &Vec<Vec<char>>,
    path_start: &(usize, usize),
    direction: &(i32, i32),
) -> bool {
    let mut current_direction = *direction;
    let mut current_path_start = *path_start;
    let mut visited = HashSet::new();
    loop {
        let (path, is_end) =
            get_path(&grid, &current_path_start, &current_direction);

        let ext_path = extend_path(grid, &path, &current_direction);

        let check = (ext_path, current_direction);
        if visited.contains(&check) {
            return true;
        } else {
            visited.insert(check);
        }

        if is_end {
            return false;
        }

        current_path_start = path.last().unwrap().to_owned();

        match current_direction {
            (0, -1) => current_direction = (1, 0),
            (1, 0) => current_direction = (0, 1),
            (0, 1) => current_direction = (-1, 0),
            (-1, 0) => current_direction = (0, -1),
            _ => panic!(),
        }
    }
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut maybe_start = None;
    for (i, line) in aoc24::iter_lines_or_die(lines) {
        let row = line.chars().collect_vec();

        if let Some(s) = row.iter().position(|&x| x == '^') {
            maybe_start = Some((s, i));
        }

        grid.push(row);
    }

    let guard_start = maybe_start.unwrap();
    let mut path_start = guard_start.clone();
    let mut direction = (0, -1);
    let mut total = 0;
    let mut obstacle_cache = HashSet::new();
    loop {
        let (path, is_end) =
            get_path(&grid, &path_start, &direction);

        if path.len() > 1 {
            // loop over each next point on path, find out of the orthogonal path from each point
            // causes a loop to occur (or ends without loop); a loop occurs if it overlaps with a
            // previous path
            for i in 1..path.len() {
                let added_obstacle = (
                    ((path_start.0 as i32)
                        + (direction.0 * (i as i32)))
                        as usize,
                    ((path_start.1 as i32)
                        + (direction.1 * (i as i32)))
                        as usize,
                );

                if added_obstacle != guard_start
                    && !obstacle_cache.contains(&added_obstacle)
                {
                    grid[added_obstacle.1][added_obstacle.0] = '#';

                    let is_loop = get_loop_or_end(
                        &grid,
                        &path_start,
                        &direction,
                    );
                    if is_loop {
                        total += 1;
                    }

                    grid[added_obstacle.1][added_obstacle.0] = '.';

                    obstacle_cache.insert(added_obstacle);
                }
            }
        }

        if is_end {
            break;
        }

        path_start = path.last().unwrap().to_owned();

        match direction {
            (0, -1) => direction = (1, 0),
            (1, 0) => direction = (0, 1),
            (0, 1) => direction = (-1, 0),
            (-1, 0) => direction = (0, -1),
            _ => panic!(),
        }
    }

    println!("Total: {total}")
}

fn main() {
    println!("Starting");
    part1();
    part2();
}
