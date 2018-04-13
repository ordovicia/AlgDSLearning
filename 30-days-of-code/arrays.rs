use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let v = buf.trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for e in v.iter().rev() {
        print!("{} ", e)
    }
}
