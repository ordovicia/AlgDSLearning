// use std::{io, fmt, iter};
use std::io;
use std::collections::HashMap;

fn split_first_char(s: &str) -> (char, &str) {
    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    let remain_str = chars.as_str();
    (first_char, remain_str)
}

struct Trie {
    children: HashMap<char, Box<Trie>>,
    prefix_cnt: usize,
}

/*
impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fn fmt_r(trie: &Trie, indent: usize, f: &mut fmt::Formatter) {
            let padding = iter::repeat(' ').take(indent).collect::<String>();
            for (ch, child) in &trie.children {
                writeln!(f, "{}- {} ({})", padding, ch, child.prefix_cnt).unwrap();
                fmt_r(&*child, indent + 2, f);
            }
        };

        fmt_r(self, 0, f);
        Ok(())
    }
}
*/

impl Trie {
    fn new() -> Trie {
        Trie {
            children: HashMap::new(),
            prefix_cnt: 0usize,
        }
    }

    fn insert(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }

        let (ch, rem_str) = split_first_char(s);
        let entry = self.children.entry(ch).or_insert(Box::new(Trie::new()));
        entry.prefix_cnt += 1;
        entry.insert(rem_str);
    }

    fn find(&self, s: &str) -> usize {
        let mut node = self;
        for ch in s.chars() {
            match node.children.get(&ch) {
                Some(child) => {
                    node = &*child;
                }
                None => {
                    return 0usize;
                }
            }
        }
        node.prefix_cnt
    }
}

fn line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn main() {
    let n = line().parse::<i32>().unwrap();
    let mut trie = Trie::new();
    for _ in 0..n {
        let (query, oprand) = {
            let l = line();
            let mut iter = l.split_whitespace();
            let q = iter.next().unwrap().to_owned();
            let o = iter.next().unwrap().to_owned();
            (q, o)
        };
        match query.as_ref() {
            "add" => {
                trie.insert(&oprand);
                // println!("{}", trie);
            }
            "find" => {
                println!("{}", trie.find(&oprand));
            }
            _ => unreachable!(),
        }
    }
}
