### Rust数据类型
> Rust是静态类型(statically typed)语言,即：在编译时就必须知道所有变量的类型;

Rust的内建类型：

1. 标量(scalar)类型;
2. 复合(compound)类型; 

#### 下面具体讨论一下数据类型

标量(scalar)类型:代表一个单独的值。
 1. 整形 
    - 有符号整数
    - 无符号整数
 2. 浮点型
 3. 布尔型
 4. 字符型

 ##### 整形
 长度 | 有符号 | 无符号
---|---|---
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
arch   | isize| usize