#![allow(unused_variables)]
#![allow(unused_assignments)]

// 常量是绑定到一个名称不允许改变的值，定义方式如下：
const HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() {
    println!("seconds = {:?}", HOUR_IN_SECONDS);

    #[allow(dead_code)]
    // 不允许对常量使用mut关键字，它总是不可变的，定义时必须显式地标注类型；
    const A: u32 = 1; // 编译正确
                      // const B = 2u32;   // 编译错误
                      // const C = 2;      // 编译错误

    // 常量可以在任何作用域声明，包括全局作用域；
    const HOUR_IN_MINS: u32 = 60;
    println!("mins = {:?}", HOUR_IN_MINS);

    // 常量只能被设置为常量表达式，不能是在运行时计算出来的值。
    let a: u32 = 1;
    let b: u32 = 2;
    // const D: u32 = a + b; // 编译错误
}