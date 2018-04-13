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
    println!("{:.3}",
             160.0 +
             40.0 *
             (0..)
                 .map(|r| poisson_distri(0.88, r))
                 .take_while(|p| *p > 1e-6)
                 .zip(0..)
                 .map(|(p, i)| p * i as f64 * i as f64)
                 .sum::<f64>());
    println!("{:.3}",
             128.0 +
             40.0 *
             (0..)
                 .map(|r| poisson_distri(1.55, r))
                 .take_while(|p| *p > 1e-6)
                 .zip(0..)
                 .map(|(p, i)| p * i as f64 * i as f64)
                 .sum::<f64>());
}

fn poisson_distri(mean: f64, k: i32) -> f64 {
    mean.powi(k) * (-mean).exp() / (1..(k + 1)).product::<i32>() as f64
}
