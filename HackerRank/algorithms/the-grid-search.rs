#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, str};
use std::io::Read;

#[allow(dead_code)]
fn get_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn get_word() -> String {
    let mut stdin = io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T>() -> T
    where T: str::FromStr
{
    get_word().parse().ok().unwrap()
}

#[allow(dead_code)]
fn gets<T>() -> Vec<T>
    where T: str::FromStr
{
    get_line()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let t = get::<i32>();
    for _ in 0..t {
        let haystack = get_grid();
        let needle = get_grid();
        let mut candidate_pos = vec![];
        for r in 0..(haystack.len() - needle.len() + 1) {
            for c in 0..(haystack[0].len() - needle[0].len() + 1) {
                if same_row(&haystack, r, c, &needle[0]) {
                    candidate_pos.push((r, c));
                }
            }
        }

        if candidate_pos
               .iter()
               .any(|&(r, c)| {
                        (1..needle.len()).all(|p| same_row(&haystack, r + p, c, &needle[p]))
                    }) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

type Row = Vec<u32>;
type Grid = Vec<Row>;

fn get_grid() -> Grid {
    let r = get::<usize>();
    let c = get::<usize>();
    let mut grid = vec![vec![0; c]; r];
    for i in 0..r {
        let nums = get_line()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        grid[i] = nums;
    }
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", grid[r][c]);
        }
        println!();
    }
}

fn same_row(haystack: &Grid, r: usize, c: usize, needle: &Row) -> bool {
    haystack[r]
        .iter()
        .skip(c)
        .zip(needle.iter())
        .all(|(h, n)| h == n)
}
