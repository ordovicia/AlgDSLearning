use std::io;

type CharNums = [i32; 26];

fn char_nums() -> CharNums {
    let l = line();
    let mut char_nums = [0; 26];
    for ch in l.chars() {
        char_nums[to_digit(ch) as usize] += 1;
    }
    char_nums
}

fn main() {
    let char_nums1 = char_nums();
    let char_nums2 = char_nums();

    let n = char_nums1
        .into_iter()
        .zip(char_nums2.into_iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum::<i32>();
    println!("{}", n);
}

/// 'a' -> 0, 'z' -> 25
fn to_digit(ch: char) -> i32 {
    ch.to_digit(36).unwrap() as i32 - 10
}

fn line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
