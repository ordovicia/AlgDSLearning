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

fn median(nums: &[i32]) -> i32 {
    let l = nums.len();
    if l % 2 == 0 {
        (nums[l / 2 - 1] + nums[l / 2]) / 2
    } else {
        nums[l / 2]
    }
}

fn main() {
    let n = get::<usize>();
    let mut nums = gets::<i32>();
    nums.sort();

    let (lower, upper) = if n % 2 == 0 {
        (median(&nums[0..(n / 2)]), median(&nums[(n / 2)..n]))
    } else {
        (median(&nums[0..(n / 2)]), median(&nums[(n / 2 + 1)..n]))
    };

    println!("{}", lower);
    println!("{}", median(&nums));
    println!("{}", upper);
}
