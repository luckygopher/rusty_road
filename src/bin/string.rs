fn main() {
    // 通过字符串字面值创建可变字符串，在heap上分配内存
    let mut str1 = String::from("hello");

    str1.push_str(" world");

    println!("{}",str1);

    // str1在离开其作用域后失效，发生了值的移动，编译时此处会报错
    let str2 = str1;
    println!("{},{}", str1, str2);


    // 简单的值被压到了stack中，不需要堆上分配内存（可理解为不需要指针二次索引）
    let x = 5;
    let y = x;
    println!("{},{}",x, y)
}