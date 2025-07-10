fn main(){
    let num: i32 = sub(32);
    println!("{}", num)
}

fn sub(number: i32)->i32{
    number + 2
}