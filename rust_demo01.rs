fn main() {
    let count = 5.0;
    println!("count = {}!",count);
    // count +=1 ; // error: re-assignment of immutable variable
    
    let mut number : u32 =10;
    number +=1;// Rust 没有 ++ 操作符
    println!("number = {}!",number);
}