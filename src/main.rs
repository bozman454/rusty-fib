
use std::io;

fn fib_it(n: i32) -> Vec<i32>{
    let mut vec = vec![];
    for number in 0..n {
        if number == 0 {
            vec.push(number)
        }
        else if number == 1 {
            vec.push(number)
        }
        else{
            vec.push(vec[vec.len() - 1] + vec[vec.len() - 2])
        }
    }
    vec
}

fn main() {
    println!("Enter index...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    let n: i32 = input.trim().parse().unwrap();
    let fib_vec = fib_it(n);

    for i in &fib_vec{
        println!("{}", i)
    }
}   
