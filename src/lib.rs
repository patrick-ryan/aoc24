use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn test_lib() {
    println!("tested lib")
}

pub fn read_lines<P>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_lines(path: &Path) -> Lines<BufReader<File>> {
    if let Ok(lines) = read_lines(path) {
        return lines;
    } else {
        panic!();
    }
}

pub fn parse_arg_file_lines() -> Lines<BufReader<File>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Parsing {file_path}");

    // let path_buf =
    //     Path::new(file!()).parent().unwrap().join(file_path);

    return parse_lines(Path::new(file_path));
}

pub fn iter_lines_or_die(
    lines: Lines<BufReader<File>>,
) -> impl Iterator<Item = (usize, String)> {
    let it = lines
        .enumerate()
        .map(|(i, line_result)| {
            let line = line_result.ok().unwrap();
            return (i, line);
        })
        .into_iter();
    return it;
}

pub fn parse_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}
