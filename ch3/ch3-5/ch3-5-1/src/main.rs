#![allow(clippy::needless_bool)]

// Rust中的控制流结构主要包括：
//      if条件判断；
//      loop循环；
//      while循环；
//      for .. in 循环。
fn main() {
    // 1.if执行条件判断
    let a = 2u32;

    if a > 5u32 {
        println!("a > 5");
    } else {
        println!("a <= 5");
    }

    // 2.if - else if处理多重条件
    let a = 1u32;

    if a > 5u32 {
        println!("a > 5");
    } else if a > 4u32 {
        println!("a > 4");
    } else if a > 3u32 {
        println!("a > 3");
    } else if a > 2u32 {
        println!("a > 2");
    } else if a > 1u32 {
        println!("a > 1");
    } else {
        println!("a = 1");
    }

    // 3.在let语句中使用if
    let a = 3u32;

    let a_bigger_than_two: bool = if a > 2u32 { true } else { false };

    if a_bigger_than_two {
        println!("a > 2");
    } else {
        println!("a <= 2");
    }
}