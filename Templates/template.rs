#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, str};

fn _get_line<Input: io::BufRead>(input: &mut Input) -> String {
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

macro_rules! get_line {
    ($buf: expr) => { _get_line($buf) };
    () => {{
        let stdin = io::stdin();
        let line = _get_line(&mut stdin.lock());
        line
    }}
}

fn _get_word<Input: io::Read>(input: &mut Input) -> String {
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf = Vec::with_capacity(16);
        loop {
            let res = input.read(&mut u8b);
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

macro_rules! get_word {
    ($buf: expr) => { _get_word($buf) };
    () => {{
        let stdin = io::stdin();
        let word = _get_word(&mut stdin.lock());
        word
    }}
}

fn get<T: str::FromStr>() -> T {
    get_word!().parse().ok().unwrap()
}

fn get_vec<T: str::FromStr>() -> Vec<T> {
    get_line!().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    //
}
