use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    
    let mut phonebook = HashMap::new();
    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let s = buf.trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
        phonebook.insert(s[0].clone(), s[1].clone());
    }
    
    buf.clear();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    for name in buf.lines() {
        let name = name.trim();
        match phonebook.get(name) {
            Some(number) => { println!("{}={}", name, number); },
            None => { println!("Not found"); }
        }
    }
}