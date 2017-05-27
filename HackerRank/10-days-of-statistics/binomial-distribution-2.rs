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

const REJECT_PROB: f64 = 0.12;
const PISTON_NUM: i32 = 10;

fn main() {
    let p = (0..3)
        .map(|n| binomial_prob(REJECT_PROB, PISTON_NUM, n))
        .collect::<Vec<_>>();
    println!("{:.3}", p[0] + p[1] + p[2]);
    println!("{:.3}", 1.0 - p[0] - p[1]);
}

fn combination(n: i32, x: i32) -> i32 {
    assert!(x >= 0 && x <= n);
    ((x + 1)..(n + 1)).product::<i32>() / (1..(n - x + 1)).product::<i32>()
}

fn pow(p: f64, n: i32) -> f64 {
    assert!(p >= 0.0 && p <= 1.0);
    assert!(n >= 0);
    let mut q = 1.0;
    for _ in 0..n {
        q *= p;
    }
    q
}

fn binomial_prob(p: f64, n: i32, x: i32) -> f64 {
    combination(n, x) as f64 * pow(p, x) * pow(1.0 - p, n - x)
}
