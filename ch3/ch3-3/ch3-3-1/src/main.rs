// 一个没有参数，也没有返回值的函数
fn print_line() {
    println!("++++++++++++++++");
}

// 一个有参数，没有返回值的函数
fn print(x: u32) {
    println!("result is {:?}", x);
}

// 一个有参数，也有返回值的函数
fn sum(a: u32, b: u32) -> u32 {
    a + b
}

// main函数是rust程序的入口函数
fn main() {
    print_line();
    let a = 1u32;
    let b = 2u32;
    let r = sum(a, b);
    print(r);
}
