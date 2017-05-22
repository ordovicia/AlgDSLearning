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
    let t = get::<i32>();
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let money = get::<i32>();
    let n = get::<usize>();
    let mut cost_map = collections::HashMap::new();
    for i in 0..n {
        let cost = get::<i32>();
        let complement_cost = money - cost;
        if let Some(j) = cost_map.get(&complement_cost) {
            println!("{} {}", j + 1, i + 1);
        }
        cost_map.insert(cost, i);
    }
}
