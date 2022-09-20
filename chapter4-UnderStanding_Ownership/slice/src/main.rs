fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 索引5会被绑定到变量word上

    s.clear(); // 这里的clear方法会清空当前字符串，使之变为""
               // 虽然word依然拥有5这个值，但因为我们用于搜索的字符串发生了改变，
               // 所以这个索引也就没有任何意义了，word到这里便失去了有效性
    println!("{}", word);

    // 切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("{}", hello);

    // 传入字符串切片
    let my_string = String::from("hello world");
    let word = common_first_word(&my_string[..]);

    let mysql_string_literal = "hello world";
    let word = common_first_word(&mysql_string_literal[..]);

    let word = common_first_word(mysql_string_literal);
}

fn first_word(s: &String) -> usize {
    // 将String转换为字节数组
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // 检查String中的字节是否为空格
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn new_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// 可以传入字符串切片， 也可以传入字面量字符串
fn common_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
