use std::io;

fn line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

#[derive(Debug)]
struct Queue {
    stack0: Vec<i32>,
    stack1: Vec<i32>,
}

enum Query {
    Push(i32),
    Pop,
    Print,
}

fn read_query() -> Query {
    let v = line()
        .clone()
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    match v[0] {
        1 => Query::Push(v[1]),
        2 => Query::Pop,
        3 => Query::Print,
        _ => unreachable!(),
    }
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            stack0: vec![],
            stack1: vec![],
        }
    }

    pub fn run_query(&mut self, query: &Query) {
        match *query {
            Query::Push(e) => self.push(e),
            Query::Pop => self.pop(),
            Query::Print => self.print(),
        }
    }

    fn push(&mut self, e: i32) {
        self.stack0.push(e);
    }

    fn pop(&mut self) {
        self.move_stack0_to_stack1_if_empty();
        self.stack1.pop();
    }

    fn print(&mut self) {
        self.move_stack0_to_stack1_if_empty();
        println!("{}", self.stack1[self.stack1.len() - 1]);
    }

    fn move_stack0_to_stack1_if_empty(&mut self) {
        if self.is_empty_stack1() {
            self.move_stack0_to_stack1();
        }
    }

    fn is_empty_stack1(&self) -> bool {
        self.stack1.is_empty()
    }

    fn move_stack0_to_stack1(&mut self) {
        while !self.stack0.is_empty() {
            self.stack1.push(self.stack0.pop().unwrap());
        }
        self.stack0.clear();
    }
}

fn main() {
    let mut queue = Queue::new();

    let n = line().parse::<i32>().unwrap();
    for _ in 0..n {
        queue.run_query(&read_query());
        // println!("{:?}", queue);
    }
}
