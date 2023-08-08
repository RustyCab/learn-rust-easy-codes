fn main() {
    // // 1.一直循环打印 again!
    // loop {
    //     println!("again!");
    // }

    // // 2.使用break终止循环:
    // let mut counter = 0;
    // loop {
    //     println!("counter = {:?}", counter);
    //     counter += 1;

    //     if counter == 10 {
    //         break; // 将终止循环
    //     }
    // }

    // // break也可以返回值
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    // 3.使用continue可以直接跳到下一轮循环:
    let mut x = 0;
    // 此循环将只打印10以内的奇数
    loop {
        x += 1;
        if x == 10 {
            break;
        }
        if x % 2 == 0 {
            continue; // 将直接跳到下一轮循环
        }
        println!("{}", x);
    }
}