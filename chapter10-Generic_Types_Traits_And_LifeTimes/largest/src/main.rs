fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // 这里需要Copy，因为比如参数全是String类型时，成员所有权是不能被移出的

    for &item in list.iter() { // 这里需要std::cmp::PartialOrd，因为并不是所有数据都是支持比较的
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}