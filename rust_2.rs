use std::io;

fn read_i32() -> i32{
    let mut number: String = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read int32 in a line");
    number.trim().parse::<i32>().expect("Not i32")
}

fn read_vec_i32() -> Vec<i32>{
    let mut numbers: String = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");
    numbers
        .trim()
        .split_whitespace()
        .map(|i: &str| i.parse().unwrap())
        .collect()
}
fn main() {
    let tests = read_i32();
    for test in 0..tests{
        let arr = read_vec_i32();
        let n = arr[0];
        let k = arr[1];
        let mut ans = 0;
        let a = read_vec_i32();
        let ans = a.contains(&k);
        if ans {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    // println!("Hello, world!");
}
