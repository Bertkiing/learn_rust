fn main(){
    // Rust中的复合数据类型

    //  note: to avoid this warning, consider using `_names` instead
    // 避免提示：warning: unused variable:,我们可以考虑使用_做变量名的前缀
    let _name = "bertking";
    println!("元组");
    // Rust中的元组，可以不同类型
   let tuple = (1,0.8,"king",'王');
   println!("第一种：模式匹配取值");
   let (number,discount,name,surname) = tuple;
    println!("number = {}!",number);
    println!("discount = {}!",discount);
    println!("name = {}!",name);
    println!("surname = {}!",surname);
   
   println!("第二种：直接索引取值");
   println!("first = {}",tuple.0);
   println!("second = {}",tuple.1);
   println!("third = {}",tuple.2);
   println!("fourth = {}",tuple.3);
   
   println!();
   // Rust中的数组，必须相同类型
   let array = [1,2,3,4,5];
   println!("数组取值");
   // 数组访问数据是按下标来取值的
   println!("first element = {}",array[0]);
   println!("second element = {}",array[1]);
   println!("third element = {}",array[2]);
   println!("fourth element = {}",array[3]);
}