// 文档注释也有文档行注释和文档块注释：
// （1）文档行注释，使用///;
// （2）文档块注释，使用/** ... */。

// ================================
// 下面是文档行注释
// ================================

/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// 下面是文档块注释

/** `add_two` 将指定值加2


\```
let arg = 5;
let answer = my_crate::add_two(arg);

assert_eq!(7, answer);
\```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn main() {
    let a: i32 = 1;
    let c = add_one(a);
    println!("a + 1 is {:?}", c);

    let d = add_two(a);
    println!("a + 2 is {:?}", d);
}
