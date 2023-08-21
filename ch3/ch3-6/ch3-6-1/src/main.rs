fn f1(a: i32, b: i32) -> i32 {
    let c: i32 = 1;
    let r = a + b + c;
    r
}

fn f2(a: i32, b: i32) -> i32 {
    let c = a - b;
    let r = c % 2;
    r
}

fn main() {
    let a = 5i32;
    let b = 1i32;

    let _r = f1(a, b);
    let _r = f2(a, b);
}
