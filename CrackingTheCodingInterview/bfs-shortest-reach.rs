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

#[derive(Default, Clone)]
struct Node {
    connected: Vec<usize>,
    reached: bool,
    distance: i32,
}

fn main() {
    let q = get::<usize>();
    for _ in 0..q {
        let n = get::<usize>();
        let mut nodes = vec![Node::default(); n];

        let e = get::<usize>();
        for _ in 0..e {
            let u = get::<usize>() - 1;
            let v = get::<usize>() - 1;
            nodes[u].connected.push(v);
            nodes[v].connected.push(u);
        }

        let s = get::<usize>() - 1;

        let mut search_queue = collections::VecDeque::with_capacity(n);
        search_queue.push_back(s);

        while !search_queue.is_empty() {
            let u = search_queue.pop_front().unwrap();
            for v in nodes[u].connected.clone() {
                if !nodes[v].reached {
                    nodes[v].reached = true;
                    nodes[v].distance = nodes[u].distance + 6;
                    search_queue.push_back(v);
                }
            }
        }

        for i in 0..n {
            if i == s {
                continue;
            }

            let dist = nodes[i].distance;
            print!("{} ", if dist > 0 { dist } else { -1 });
        }
        println!();
    }
}
