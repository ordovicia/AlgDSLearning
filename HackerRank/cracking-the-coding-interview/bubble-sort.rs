#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, str};
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

fn main() {
    let n = get::<usize>();
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(get::<i32>());
    }
    bubble_sort(&mut nums);
}

fn bubble_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    let mut swap_cnt = 0u32;

    for _ in 0..len {
        let mut swapped = false;
        for j in 0..(len - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
                swap_cnt += 1;
            }
        }

        if !swapped {
            break;
        }
    }

    println!("Array is sorted in {} swaps.", swap_cnt);
    println!("First Element: {}", nums[0]);
    println!("Last Element: {}", nums[nums.len() - 1]);
}
