fn sum(a: u32, b: u32) -> u32 {
    println!("a is {:?}", a);
    println!("b is {:?}", b);

    a + b // 注意，是没有加分号的
}

fn main() {
    let a = 1u32;
    let b = 2u32;
    let c = sum(a, b);
    println!("c = {:?}", c);
}
