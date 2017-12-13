fn main(){
    println!("这是主函数");
    another_func();
    print(15);
    
    // let xx = 0.8 + 1; //  no implementation for `{float} + {integer}`
}

fn another_func(){
    println!("另一个函数");
}

// 函数的参数：必须加 类型
fn print(x:isize){
    println!("x = {}",x);
}


