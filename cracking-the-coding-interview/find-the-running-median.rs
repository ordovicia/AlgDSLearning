use std::io;
use std::cmp::{self, Ordering};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
struct i32Inv {
    v: i32,
}

impl cmp::Ord for i32Inv {
    fn cmp(&self, other: &Self) -> Ordering {
        other.v.cmp(&self.v)
    }
}

impl cmp::PartialOrd for i32Inv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.v.cmp(&self.v))
    }
}

impl std::convert::From<i32> for i32Inv {
    fn from(e: i32) -> Self {
        i32Inv { v: e }
    }
}

#[derive(Default, Debug)]
struct Heap {
    low_heap: BinaryHeap<i32>, // lower-half, max-heap
    high_heap: BinaryHeap<i32Inv>, // higher-half, min-heap
}

impl Heap {
    pub fn push(&mut self, e: i32) {
        match (self.low_heap.len(), self.high_heap.len()) {
            (0, 0) => {
                self.low_heap.push(e);
            }
            (0, _) => {
                self.low_heap.push(e);
                self.swap_if_unordered();
            }
            (_, 0) => {
                self.high_heap.push(i32Inv::from(e));
                self.swap_if_unordered();
            }
            (low_size, high_size) if low_size <= high_size => {
                self.low_heap.push(e);
                self.swap_if_unordered();
            }
            _ => {
                self.high_heap.push(i32Inv::from(e));
                self.swap_if_unordered();
            }
        }
    }

    fn swap_if_unordered(&mut self) {
        assert!(self.low_heap.len() > 0 && self.high_heap.len() > 0);

        let low = self.low_heap.pop().unwrap();
        let high = self.high_heap.pop().unwrap();
        if low < high.v {
            self.low_heap.push(low);
            self.high_heap.push(high);
        } else {
            self.low_heap.push(high.v);
            self.high_heap.push(i32Inv::from(low));
        }
    }

    pub fn median(&self) -> f64 {
        let (low_size, high_size) = (self.low_heap.len(), self.high_heap.len());
        let total_size = low_size + high_size;
        if total_size % 2 == 1 {
            if low_size < high_size {
                self.high_heap.peek().unwrap().v as f64
            } else {
                *self.low_heap.peek().unwrap() as f64
            }
        } else {
            assert!(low_size > 0 && high_size > 0);
            (*self.low_heap.peek().unwrap() + self.high_heap.peek().unwrap().v) as f64 / 2.0
        }
    }
}

fn main() {
    let n = read_i32();
    let mut heap = Heap::default();

    for _ in 0..n {
        heap.push(read_i32());
        println!("{:.1}", heap.median());
    }
}

fn read_i32() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap()
}
