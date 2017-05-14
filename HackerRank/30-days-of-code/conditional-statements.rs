use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    if n % 2 == 1 || (6 <= n && n <= 20) {
        println!("Weird");
    } else {
        println!("Not Weird");
    }
}
