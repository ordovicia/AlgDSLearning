impl Person {
    fn new(initialAge: i32) -> Person {
        if initialAge < 0 {
            println!("Age is not valid, setting age to 0.");
            Person { age: 0 }
        } else {
            Person { age: initialAge }
        }
    }

    fn amIOld(&self) {
        if self.age < 13 {
            println!("You are young.");
        } else if self.age < 18 {
            println!("You are a teenager.");
        } else {
            println!("You are old.");
        }
    }

    fn yearPasses(&mut self) {
        self.age += 1;
    }
}
