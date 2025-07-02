use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i64> = a_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 1..n{
        if a[i] == a[i - 1]{
            println!("stay");
        }else if a[i] > a[i - 1]{
            println!("up {}", a[i] - a[i -1]);
        }else{
            println!("down {}", a[i - 1] - a[i]);
        }
    }
}
