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
    let _ = get::<usize>();
    let k = get::<usize>();
    if k == 1 {
        println!("1");
        return;
    }

    let nums = gets::<usize>();

    let mut mod_cnt = vec![0; k];
    for n in nums {
        mod_cnt[n % k] += 1;
    }

    // k = 5, nums = 1, 2, 4, 6, 7, 8, 9, 11
    //  => mod_cnt = [0, 3, 1, 1, 2]
    //  => mod_cnt[0]?
    //      + max(mod_cnt[1], mod_cnt[4])
    //      + max(mod_cnt[2], mod_cnt[3])
    //
    // k = 6
    //  => mod_cnt[0]?
    //      + max(mod_cnt[1], mod_cnt[5])
    //      + max(mod_cnt[2], mod_cnt[4])

    let mut ans = 0;
    if mod_cnt[0] > 0 {
        ans += 1;
    }
    for d in 1..(k / 2) {
        ans += cmp::max(mod_cnt[d], mod_cnt[k - d]);
    }
    if k % 2 == 1 {
        ans += cmp::max(mod_cnt[k / 2], mod_cnt[k / 2 + 1]);
    } else {
        ans += 1;
    }

    println!("{}", ans);
}
