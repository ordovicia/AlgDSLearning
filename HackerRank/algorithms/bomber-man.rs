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
    solve();
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<i32>>,
    row: usize,
    col: usize,
}

fn solve() {
    let row = get::<usize>();
    let col = get::<usize>();
    let n = get::<i32>();

    if n == 1 {
        for _ in 0..row {
            println!("{}", get_line());
        }
        return;
    } else if n % 2 == 0 {
        for _ in 0..row {
            for _ in 0..col {
                print!("O");
            }
            println!();
        }
        return;
    }

    let mut grid = vec![vec![0; col + 2]; row + 2];
    for r in 1..(row + 1) {
        let mut col = vec![0];
        col.append(&mut get_line()
                            .chars()
                            .map(|c| match c {
                                     '.' => 0i32,
                                     'O' => 1i32,
                                     _ => unreachable!(),
                                 })
                            .collect());
        col.append(&mut vec![0]);
        grid[r] = col;
    }

    let grid_t0 = Grid { grid, row, col };
    let grid_t3 = flip(&dilate(&grid_t0));
    let grid_t5 = flip(&dilate(&grid_t3));

    if (n - 1) % 4 == 2 {
        // 3, 7, 11, ..
        print_grid_ans(&grid_t3);
    } else {
        // 5, 9, 14, ...
        assert!((n - 1) % 4 == 0);
        print_grid_ans(&grid_t5);
    }
}

fn dilate(grid: &Grid) -> Grid {
    let mut dilated = grid.clone();
    let g = &grid.grid;
    for r in 1..(grid.row + 1) {
        for c in 1..(grid.col + 1) {
            if g[r][c] > 0 || g[r - 1][c] > 0 || g[r + 1][c] > 0 || g[r][c - 1] > 0 ||
               g[r][c + 1] > 0 {
                dilated.grid[r][c] = 1;
            }
        }
    }
    dilated
}

fn flip(grid: &Grid) -> Grid {
    let mut flipped = grid.clone();
    for r in 1..(grid.row + 1) {
        for c in 1..(grid.col + 1) {
            flipped.grid[r][c] = (grid.grid[r][c] + 1) % 2;
        }
    }
    flipped
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for r in 1..(grid.row + 1) {
        for c in 1..(grid.col + 1) {
            print!("{}", grid.grid[r][c]);
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_grid_ans(grid: &Grid) {
    for r in 1..(grid.row + 1) {
        for c in 1..(grid.col + 1) {
            print!("{}", if grid.grid[r][c] == 0 { '.' } else { 'O' });
        }
        println!();
    }
}
