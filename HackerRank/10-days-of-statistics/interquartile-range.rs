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
    let _ = get::<usize>();

    let elements = gets::<i32>();
    let frequency = gets::<i32>();

    let freq_sum = frequency.iter().sum::<i32>() as usize;
    let mut s = Vec::with_capacity(freq_sum as usize);
    for (e, f) in elements.iter().zip(frequency.iter()) {
        for _ in 0..*f {
            s.push(*e);
        }
    }

    let (lower, _, upper) = quartiles(&mut s, freq_sum);
    println!("{:.1}", upper - lower);
}

fn quartiles(elements: &mut [i32], n: usize) -> (f64, f64, f64) {
    elements.sort();

    let lower;
    let upper;
    if n % 2 == 0 {
        lower = median(&elements[0..(n / 2)], n / 2);
        upper = median(&elements[(n / 2)..n], n / 2);
    } else {
        lower = median(&elements[0..(n / 2)], n / 2);
        upper = median(&elements[(n / 2 + 1)..n], n / 2);
    };

    (lower, median(&elements, n), upper)
}

fn median(elements: &[i32], n: usize) -> f64 {
    if n % 2 == 0 {
        (elements[n / 2 - 1] + elements[n / 2]) as f64 / 2.0
    } else {
        elements[n / 2] as f64
    }
}
