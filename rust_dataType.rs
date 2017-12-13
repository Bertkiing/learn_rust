fn main(){
    let number = 10;
    println!("number = {}",number);
    // 我们在使用无符号整数类型时,必须保证结果非负;
    // 所以在使用过程中，确保数值非负才使用无符号整数，otherwise,使用有符号的。
    // let first:u32 = 9-10;// attempt to subtract with overflow
    let first:i32 = 9-10;// correct
    println!("first = {}",first);
    
    //let discount = 0.8 -1; // no implementation for `{float} - {integer}`
    
    let discount : f64 = 0.8 - 1.0; //  -0.19999999999999996 , Why not 0.2 ?
    println!("discount = {}",discount);
    // for isReal, The Rust  will give an warning which variable `isReal` should have a snake case name such as `is_real`
    let isReal = true; // 所以我们在Rust编程中，避免使用驼峰命名，而使用_连接
    println!("The world is {}",isReal);
    
    let is_shown:bool = false;
    println!("The flower has shown, Right? {}",is_shown);
    // Rust中的字符类型,只能包含一个codepoint;
    //let hello = 'he'; // error: character literal may only contain one codepoint: 'he'
    let h:char = 'h';
    println!("The first alphabet of hello is {}",h);
    // Rust中的字符强大之处
    let name :char = '王';
    println!("我的姓氏 is {}!",name);
    
    
}