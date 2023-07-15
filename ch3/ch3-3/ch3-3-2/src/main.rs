fn calculate(a: u32, b: u32) {
    println!("a is {:?}", a);
    println!("b is {:?}", b);

    // 在函数内部定义函数
    fn sum(a: u32, b: u32) -> u32 {
        a + b
    }

    let r = sum(a, b);
    println!("a + b is {:?}", r);
}

fn main() {
    let a = 1u32;
    let b = 2u32;
    calculate(a, b);
}
