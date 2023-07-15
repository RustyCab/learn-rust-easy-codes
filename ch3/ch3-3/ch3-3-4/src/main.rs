fn sum(a: u32, b: u32) -> u32 {
    let r = a + b;
    return r; //可以加分号，也可以不加分号, 所以这行等价于“return r;”
}

fn main() {
    let a = 1u32;
    let b = 2u32;
    let c = sum(a, b);
    println!("c = {:?}", c);
}
