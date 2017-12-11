/*
* Rust 中的变量 
*
* Rust的变量是由关键字let来声明的，值得注意的是，Rust的变量默认是不可变的(immutable)的。
* 有时为了使变量可变，我们可以在变量名前面添加关键字mut，来使其可变。使用需要权衡利弊
*/
fn main() {
    println!("Hello, world!");
    let mut x = 5; // 需要注意的是：let 和 mut的位置不能颠倒，即
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
    
}