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
    let mut sum = 0;
    let mut heap = collections::BinaryHeap::new();
    let mut cnt = collections::HashMap::new();

    let mut mode_cnt = 0;
    let mut mode = 0;

    let n = get::<i32>();
    for _ in 0..n {
        let x = get::<i32>();
        sum += x;
        heap.push(x);
        let entry = cnt.entry(x).or_insert(0);
        *entry += 1;
        if *entry > mode_cnt {
            mode_cnt = *entry;
            mode = x;
        } else if *entry == mode_cnt && x < mode {
            mode = x;
        }
    }

    let mean = sum as f64 / n as f64;
    println!("{:.1}", mean);

    if n % 2 == 0 {
        for _ in 0..(n / 2 - 1) {
            heap.pop();
        }
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        println!("{:.1}", (a + b) as f64 / 2.0);
    } else {
        for _ in 0..(n / 2) {
            heap.pop();
        }
        println!("{:.1}", heap.pop().unwrap() as f64 / 2.0);
    }

    println!("{}", mode);
}
