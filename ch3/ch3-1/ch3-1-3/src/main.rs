#![allow(unused_variables)]
#![allow(unused_assignments)]

// rust中可以定义一个与之前的变量同名的变量，这称之为第一个变量被第二个变量隐藏。
fn main() {
    let a: u32 = 1; // 这个变量a被下面的a隐藏掉了
    let a: u32 = 2; // 定义了一个新的变量，这个变量也叫作a
    println!("a: {:?}", a); // 输出结果为2

    let mut b: u32 = 1; // 定义可变变量b
    b = 2; // 对b的值进行的修改
    println!("b: {:?}", b); // 输出结果为2

    let c = 5;
    {
        let c = 3; // 在当前的花括号作用域内，对之前的c进行隐藏
        println!("c: {:?}", c); // 输出结果为3
    }
    println!("c: {:?}", c); // 输出结果为5
}