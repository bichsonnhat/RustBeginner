use std::io;

fn read_i32() -> i32{
    let mut line: String = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Read int32 in a line : Error");
    line.trim().parse::<i32>().expect("Not int32")
}

fn read_vec_i32() -> Vec<i32>{
    let mut line: String = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Read i32 array in a line : Error");
    line
        .trim()
        .split_whitespace()
        .map(|i: &str| i.parse().unwrap())
        .collect()
}
fn main() {
    let tests = read_i32();
    for test in 0..tests{
        let n = read_i32();
        let mut j = 1;
        for i in 0..n{
            print!("{j} ");
            j += 2;
        }
        println!();
    }
}
