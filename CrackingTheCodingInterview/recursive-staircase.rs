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
    let mut steps_memo = [[0; 37]; 37];
    // steps_memo[t][n] = #ways to reach n'th step in time t
    for n in 1..4 {
        steps_memo[1][n] = 1;
    }

    for t in 2..37 {
        for n in t..(cmp::min(3 * t, 36) + 1) {
            steps_memo[t][n] = steps_memo[t - 1][n - 1] + steps_memo[t - 1][n - 2] +
                               (if n >= 3 { steps_memo[t - 1][n - 3] } else { 0 });
        }
    }

    let mut steps = [0; 37];
    for n in 1..37 {
        for t in (n / 3)..(n + 1) {
            steps[n] += steps_memo[t][n];
        }
        // println!("{}: {}", n, steps[n]);
    }

    let s = get::<i32>();
    for _ in 0..s {
        let n = get::<usize>();
        println!("{}", steps[n]);
    }
}
