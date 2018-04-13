use std::io;
use std::str::FromStr;

fn read<T: FromStr>() -> Result<T, T::Err> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>()
}

fn main() {
    let cost = read::<f64>().unwrap();
    let tip = read::<f64>().unwrap();
    let tax = read::<f64>().unwrap();
    let total = cost + (cost * tip / 100.0) + (cost * tax / 100.0);
    println!("The total meal cost is {} dollars.", total.round());
}
