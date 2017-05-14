use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut n = buf.trim().parse::<u32>().unwrap();
    
    let mut max_cnt = 0;
    let mut cnt = 0;
    loop {
        if n == 0 {
            break;
        } 
    
        if n & 1u32 == 1 {
            cnt += 1;
            if max_cnt < cnt {
                max_cnt = cnt;
            }
        } else {
            cnt = 0;
        }
        
        n >>= 1;
    }
    
    println!("{}", max_cnt);
}