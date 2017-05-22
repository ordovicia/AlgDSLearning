#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, str};
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

#[derive(Default)]
struct Cells {
    rows: i32,
    cols: i32,
    cells: [[bool; 10]; 10],
    visited: [[bool; 10]; 10],
}

fn main() {
    let mut cells = Cells::default();
    cells.rows = get::<i32>();
    cells.cols = get::<i32>();

    for r in 0..cells.rows {
        for c in 0..cells.cols {
            cells.cells[r as usize][c as usize] = get::<i32>() == 1;
        }
    }

    let mut max_region_size = 0;
    for r in 0..cells.rows {
        for c in 0..cells.cols {
            let region_size = dfs(&mut cells, r, c);
            if region_size > max_region_size {
                max_region_size = region_size;
            }
        }
    }

    println!("{}", max_region_size);
}

fn dfs(cells: &mut Cells, r: i32, c: i32) -> i32 {
    let ru = r as usize;
    let cu = c as usize;

    if (r < 0 || r >= cells.rows) || (c < 0 || c >= cells.cols) || cells.visited[ru][cu] ||
       cells.cells[ru][cu] == false {
        0
    } else {
        cells.visited[ru][cu] = true;
        1 + dfs(cells, r - 1, c - 1) + dfs(cells, r - 1, c) + dfs(cells, r - 1, c + 1) +
        dfs(cells, r, c - 1) + dfs(cells, r, c + 1) +
        dfs(cells, r + 1, c - 1) + dfs(cells, r + 1, c) + dfs(cells, r + 1, c + 1)
    }
}
