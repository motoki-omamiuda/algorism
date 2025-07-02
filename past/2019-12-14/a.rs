use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    if s.chars().all(|c| c.is_ascii_digit()) {
        let num: i32 = s.parse().unwrap();
        println!("{}", num * 2);
    } else {
        println!("error");
    }
}
