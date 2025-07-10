fn main(){
    // mutable
    let mut s: &str = "hello, world";
    println!("{}", s);

    s = "test";
    println!("{}", s);

    // 整数型 符号付き 符号なし
    // 008-bit i8 u8
    // 浮動小数
    // 008-bit f8

    // constant
    const Y: i64 = 4;
    println!("{}", Y);
    println!("{}", (Y as f64));


    // list
    let list: [i64; 5] = [1, 2, 4, 5, 6];
    println!("{}", list[0]);
}