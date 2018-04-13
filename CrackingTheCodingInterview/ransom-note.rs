use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn main() {
    // let (magazine_num, ransom_num) = {
    let (_, _) = {
        let l = line();
        let v = l.split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut magazine_words_cnt = HashMap::new();
    for word in line().split_whitespace() {
        let e = magazine_words_cnt.entry(word.to_owned()).or_insert(0);
        *e += 1;
    }

    macro_rules! fail {
        () => { println!("No"); return; }
    }

    for word in line().split_whitespace() {
        match magazine_words_cnt.entry(word.to_owned()) {
            Entry::Occupied(mut e) => {
                if e.get() == &0 {
                    fail!();
                } else {
                    *e.get_mut() -= 1;
                }
            }
            _ => {
                fail!();
            }
        }
    }

    println!("Yes");
}
