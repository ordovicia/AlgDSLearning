use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    for i in 1..11 {
        println!("{} x {} = {}", n, i, n * i)
    }
}
