// 在Rust中，注释分为三类：
// (1) 代码注释，用于说明某一块代码功能，读者往往是同一项目的协作者；
// (2) 文档注释，支持markdown，对项目、公共API等进行描述，同时还能提供示例代码，读者是想要了解你项目的人；
// (3) 包和模块注释，文档注释的一种，用于说明当前包和模块的功能，方便用户迅速了解项目。

// ===================================
// 本示例演示代码注释
// ===================================
// 代码注释有两种：
// （1）行注释，使用//；
// （2）块注释，使用/* ... */。


/*
 * 块注释：
 * 函数名：sum
 * 参数：a，b
 * 返回值类型：u32
 */
fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let a: u32 = 1;
    let b: u32 = 1;
    // 行注释：调用sum函数计算a+b的和
    let c = sum(a, b);
    println!("a + b is {:?}", c);
}
