use aoc24::{self};
use itertools::{enumerate, Itertools};

fn transform_coordinates(
    i: usize,
    j: usize,
    di: i32,
    dj: i32,
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    // Ensure the transformed coordinates stay within bounds
    let ni = i as i32 + di;
    let nj = j as i32 + dj;

    if ni >= 0 && nj >= 0 {
        let (ni, nj) = (ni as usize, nj as usize);
        if nj < grid.len() && ni < grid[0].len() {
            return Some((ni, nj));
        }
    }
    None // Out of bounds
}

fn search_char(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    chars: Vec<char>,
    direction: Option<(i32, i32)>,
) -> i32 {
    match chars.as_slice() {
        [c, rest @ ..] => {
            let mut sum = 0;
            if grid[j][i] == *c {
                let adds;
                match direction {
                    Some(d) => adds = [d].to_vec(),
                    None => {
                        // if no direction defined, search every direction
                        adds = [
                            (1, 0),
                            (1, -1),
                            (0, -1),
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ]
                        .to_vec()
                    }
                }
                for (di, dj) in adds {
                    if let Some((ni, nj)) =
                        transform_coordinates(i, j, di, dj, grid)
                    {
                        sum += search_char(
                            grid,
                            ni,
                            nj,
                            rest.to_vec(),
                            Some((di, dj)),
                        );
                    } else {
                        // at edge of grid
                        match rest {
                            [_, ..] => (),
                            // made it to end of word
                            [] => sum += 1,
                        }
                    }
                }
            }
            return sum;
        }
        // made it to end of word
        [] => return 1,
    }
}

fn part1() {
    let lines = aoc24::parse_arg_file_lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let row = line.chars().collect_vec();
        grid.push(row);
    }

    let mut total = 0;
    for (i, row) in enumerate(&grid) {
        for (j, _c) in enumerate(row) {
            let res = search_char(
                &grid,
                i,
                j,
                ['X', 'M', 'A', 'S'].to_vec(),
                None,
            );

            total += res;
        }
    }

    println!("Total: {total}")
}

fn get_cross_coordinates(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> Option<[((usize, usize), (i32, i32)); 2]> {
    // get the top corner coordinates for the lower-right bounding box, with direction
    if i + 2 >= grid[0].len() || j + 2 >= grid.len() {
        return None;
    } else {
        let res = [
            // top-left, towards bottom-right
            ((i, j), (1, 1)),
            // top-right, towards bottom-left (y increases going down)
            ((i + 2, j), (-1, 1)),
        ];
        return Some(res);
    }
}

fn part2() {
    let lines = aoc24::parse_arg_file_lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_i, line) in aoc24::iter_lines_or_die(lines) {
        let row = line.chars().collect_vec();
        grid.push(row);
    }

    let mut total = 0;
    for (i, row) in enumerate(&grid) {
        for (j, _c) in enumerate(row) {
            let maybe_diagonals =
                get_cross_coordinates(&grid, i, j);

            if let Some(diagonals) = maybe_diagonals {
                let mut valid = true;
                for ((x, y), direction) in diagonals {
                    let res = search_char(
                        &grid,
                        x,
                        y,
                        ['M', 'A', 'S'].to_vec(),
                        Some(direction),
                    ) + search_char(
                        &grid,
                        x,
                        y,
                        ['S', 'A', 'M'].to_vec(),
                        Some(direction),
                    );

                    if res == 0 {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    total += 1;
                }
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
