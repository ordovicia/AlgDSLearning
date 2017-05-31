#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, mem, str};
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
        solve();
    }
}

fn print_bytes(bytes: &[u8]) {
    println!("{}", str::from_utf8(bytes).unwrap());
}

fn solve() {
    let line = get_line();
    let len = line.len();
    let mut bytes = line.into_bytes();

    // 0 1 2 5 3 3 0
    //     ^ ~~~~~~~
    //    piv suffix

    let mut dec_suffix_start = len - 1;
    while dec_suffix_start > 0 && bytes[dec_suffix_start - 1] >= bytes[dec_suffix_start] {
        dec_suffix_start -= 1;
    }
    if dec_suffix_start == 0 {
        println!("no answer");
        return;
    }
    let dec_suffix_start = dec_suffix_start;
    let pivot_idx = dec_suffix_start - 1;

    // 0 1 2 5 3 3 0
    //     ^     ^
    //    piv  g_suf

    let mut greater_suffix_ch_idx = len - 1;
    while bytes[greater_suffix_ch_idx] <= bytes[pivot_idx] {
        greater_suffix_ch_idx -= 1;
    }
    let greater_suffix_ch_idx = greater_suffix_ch_idx;

    bytes.swap(pivot_idx, greater_suffix_ch_idx);

    bytes[(pivot_idx + 1)..].reverse();
    print_bytes(&bytes);
}
