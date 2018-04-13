use std::io;

fn line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn main() {
    let n = line().parse::<i32>().unwrap();
    for _ in 0..n {
        if is_balanced(&line()) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn is_balanced(line: &str) -> bool {
    let mut stack = Vec::new();
    for ch in line.chars() {
        if is_open(ch) {
            stack.push(matching_bracket(ch));
        } else {
            if stack.is_empty() || ch != stack.pop().unwrap() {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn matching_bracket(ch: char) -> char {
    match ch {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => unreachable!(),
    }
}

fn is_open(ch: char) -> bool {
    match ch {
        '{' | '[' | '(' => true,
        _ => false,
    }
}
