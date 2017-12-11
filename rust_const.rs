/*
* Rust 中的常量 
*
* Rust的常量是由关键字const来声明的
* 
* Rust中常量 and 变量之间的区别：
* 1. 常量就是不能变，绝不允许对常量使用关键字mut;
* 2. 声明常量的关键字是const，且必须声明值的数据类型;
* 3. 常量可以在任何作用域中声明;
* 4. Rust的常量命名规范:下划线分割大写字母
*/
fn main() {
    println!("Hello, world!");
    const MATH_PI: f32 = 3.1415926; // const 不能使用mut关键字
   println!("The value of MATH_PI is: {}",MATH_PI);
   let x = 5;
   //println!(MATH_PI); println!()不能直接输出变量or常量；
}