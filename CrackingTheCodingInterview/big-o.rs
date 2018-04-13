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

fn main() {
    let p = get::<i32>();
    for _ in 0..p {
        let n = get::<i32>();
        if is_prime(n) {
            println!("Prime");
        } else {
            println!("Not prime");
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt().floor() as i32;
    for i in 2..(sqrt_n + 1) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
