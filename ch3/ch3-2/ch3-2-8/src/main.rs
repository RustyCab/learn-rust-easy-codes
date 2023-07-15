// 元组
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // 接下来你可以使用x、y、z
    println!("x = {:?}, y = {:?}, z = {:?}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let first = x.0;
    let second = x.1;
    let third = x.2;

    let x: () = (); // 将值()保存到类型为()的变量x中
}
