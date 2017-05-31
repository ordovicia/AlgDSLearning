use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        buf = String::from(buf.trim());
        let s = buf.len();
        let mut str0 = String::with_capacity(s);
        let mut str1 = String::with_capacity(s);
        for (i, c) in buf.chars().enumerate() {
            if i % 2 == 0 {
                str0.push(c);
            } else {
                str1.push(c);
            }
        }
        println!("{} {}", str0, str1);
    }
}
