#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

fn main() {
    // 1.绑定
    // 绑定就是将值和变量关联的过程。变量的绑定可以使用let关键字。如下：
    let a = 1; // 将1绑定到变量a
    let mut b = 2; // 将2绑定到变量b
    let some_number = Some(2); // 将Some(2)绑定到some_number

    // 2.Rust中变量分为不可变变量和可变变量。不可变变量不能对其进行二次绑定，可变变量可以对其进行二次绑定。
    // (1)不可变变量定义方式如下：
    let a: u32 = 1; // 将1绑定到a这个变量
    let b = 0u32;
    let c = 1; // 定义时不指定类型，可以自动类型推导

    // 对不可变变量二次绑定一个值会报错：
    let a: u32 = 1; // 将1绑定到变量a，a为不可变变量，
                    // a = 2; // 此行打开编译会错误，a是不可变的变量，不能进行二次绑定

    // (2)可变变量定义方式如下：
    let mut a: u32 = 1; // 通过mut关键字定义可变变量
    a = 2; // 将2绑定到变量a，编译正确，因为a是可变变量，可以进行二次绑定
    let mut b = 2;
    b = 3;

    println!("Hello, world!");
}