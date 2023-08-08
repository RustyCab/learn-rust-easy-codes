fn main() {
    // // 1.while条件循环执行代码，当条件不满足时结束循环:
    // let mut cnt = 0u32;
    // while cnt < 10 {
    //     println!("cnt = {:?}", cnt);
    //     cnt += 1;
    // }

    // 2.在while循环中也可以使用break和continue:
    let mut x = 0;
    while x < 10 {
        x += 1;
        if x % 2 == 0 {
            continue; // 跳到下一轮循环
        }
        if x > 8 {
            break; // 提前结束
        }
        println!("{}", x);
    }
}