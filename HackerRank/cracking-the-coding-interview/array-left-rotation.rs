use std::io;
use std::str::FromStr;
use std::fmt::Debug;

fn nums<T>() -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|e| e.parse::<T>().unwrap()).collect()
}

fn main() {
    let (n, d) = {
        let n = nums::<usize>();
        (n[0], n[1])
    };
        
    for e in nums::<i32>().into_iter().cycle().skip(d).take(n) {
        print!("{} ", e);
    }
}