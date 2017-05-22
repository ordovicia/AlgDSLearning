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
    let d = get::<usize>();
    for _ in 0..d {
        solve();
    }
}

fn solve() {
    let _ = get::<usize>();
    let nums = get_line()
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let sorted = merge_sort(nums);
    println!("{}", sorted.1);
}

fn merge_sort(nums: Vec<i32>) -> (Vec<i32>, usize) {
    if nums.len() <= 1 {
        (nums, 0)
    } else {
        let (init, tail) = split(nums);
        let (init, init_inv) = merge_sort(init);
        let (tail, tail_inv) = merge_sort(tail);
        let (merged, merged_inv) = merge(init, tail);
        (merged, init_inv + tail_inv + merged_inv)
    }
}

fn split(mut nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let d = nums.len() / 2;
    let tail = nums.split_off(d);
    (nums, tail)
}

fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> (Vec<i32>, usize) {
    let (len1, len2) = (nums1.len(), nums2.len());
    let mut merged = Vec::with_capacity(len1 + len2);

    let mut i = 0;
    let mut j = 0;
    let mut inversion_cnt = 0;

    while i != len1 && j != len2 {
        if nums1[i] <= nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
            inversion_cnt += len1 - i;
        }
    }
    while i != len1 {
        merged.push(nums1[i]);
        i += 1;
    }
    while j != len2 {
        merged.push(nums2[j]);
        j += 1;
    }

    (merged, inversion_cnt)
}
