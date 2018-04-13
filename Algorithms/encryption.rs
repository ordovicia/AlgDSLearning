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

fn solve() {
    let line = get_line();
    let len = line.len();
    let col_num = (len as f64).sqrt().ceil() as usize;
    let row_num = {
        let mut r = col_num;
        while r * col_num >= len {
            r -= 1;
        }
        r + 1
    };

    let bytes = line.as_bytes();
    let mut encrypt_idx = 0;
    while encrypt_idx < row_num * col_num {
        let line_idx = (encrypt_idx % row_num) * col_num + encrypt_idx / row_num;
        if line_idx >= len {
            print!(" ");
        } else {
            print!("{}", bytes[line_idx] as char);
            if encrypt_idx % row_num == row_num - 1 {
                print!(" ");
            }
        }
        encrypt_idx += 1;
    }
}
