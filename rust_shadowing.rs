fn main() {
    let apples = 5;
    println!("The number of apples = {}!",apples);
    println!("Give you an apple");
    let apples = apples + 1 ; //注意，这里的apples操作 不能简写为 apples += 1;
    /** 我们知道在Rust中，变量是不可更改的(immutable),但是上述情况我们发现apples，
    我们没有使用mut关键字,也改变了apples变量
    这种现象在Rust中称为“隐藏(Shadowing)”,
    即：新定义的变量会隐藏掉之前的同名变量   
    其实这里let关键字就是再造一个变量apples，而不是复用旧的apples。
    */
    println!("Now the number of apples = {}",apples);
    
    // Shadowing的使用场景
    let discount : f32 = 0.8;
    println!("专卖店打折{}!",discount);
    let discount : i32 = 800;
    println!("折扣价是：{}",discount);
     /** 有木有很骚气,不用去理会Rust变量不可变的规则，
    我们使用一个变量discount来表示两种不同类型的值啦。
    此时你也许会想到mut关键字,但是它只能改变变量的值，而不能改变变量的类型，
    so mut关键字对此种操作无能为力. */
   
   // doc comments must come before what they document, otherwise， an error occurs.
   println!("Rust的文档注释必须在它要注释的前面");
}