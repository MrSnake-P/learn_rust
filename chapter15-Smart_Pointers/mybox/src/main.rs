use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *y会被Rust隐式地展开为：
    // *(y.deref())
    assert_eq!(5, *y);
}

// 拥有T类型单元素的元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 在没有Deref trait的情形下，
// 编译器只能对&形式的常规引用执行解引用操作。
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}