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
    let (returned_day, returned_month, returned_year) = {
        let returned = gets::<i32>();
        (returned[0], returned[1], returned[2])
    };
    let (due_day, due_month, due_year) = {
        let due = gets::<i32>();
        (due[0], due[1], due[2])
    };

    let mut fine = 0;
    if returned_year > due_year {
        fine = 10000;
    } else if returned_month > due_month && returned_year == due_year {
        let late_months = returned_month - due_month;
        fine = 500 * late_months;
    } else if returned_day > due_day && returned_month == due_month && returned_year == due_year {
        let late_days = returned_day - due_day;
        fine = 15 * late_days;
    }

    println!("{}", fine);
}
