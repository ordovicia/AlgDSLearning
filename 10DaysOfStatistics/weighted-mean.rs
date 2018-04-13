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
    let _ = get::<i32>();
    let nums = gets::<i32>();
    let weights = gets::<i32>();
    let mut sum = 0;
    let mut weight_sum = 0;
    for (n, w) in nums.iter().zip(weights.iter()) {
        sum += n * w;
        weight_sum += *w;
    }
    println!("{:.1}", sum as f64 / weight_sum as f64);
}
