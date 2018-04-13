use std::io;

fn main() {
    let mut numbers = [[0; 6]; 6];
    let mut buf = String::new();
    for i in 0..6 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for (j, e) in buf.trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .enumerate() {
            numbers[i][j] = e;
        }
    }

    let numbers = numbers;
    let mut max_hourglass = <i32>::min_value();
    for i in 0..4 {
        for j in 0..4 {
            let hourglass = numbers[i][j] + numbers[i][j + 1] + numbers[i][j + 2] +
                            numbers[i + 1][j + 1] +
                            numbers[i + 2][j] + numbers[i + 2][j + 1] +
                            numbers[i + 2][j + 2];
            if max_hourglass < hourglass {
                max_hourglass = hourglass;
            }
        }
    }

    println!("{}", max_hourglass);
}
