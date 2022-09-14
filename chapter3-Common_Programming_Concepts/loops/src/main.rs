fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result); // 20

    for_iter();
    for_iter_rev()
}

fn for_iter() {
    // 左闭右开
    for i in 0..10+1 {
        println!("{}", i);
    }
}

fn for_iter_rev() {
    // 反转range生成的序列
    for i in (0..10).rev() {
        println!("{}", i);
    }
}
