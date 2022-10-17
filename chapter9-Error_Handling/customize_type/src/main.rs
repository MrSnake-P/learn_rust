fn main() {
    println!("Hello, world!");
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("must between 1 and 100{}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

## 总结
Rust中的错误处理功能被设计出来帮助我们编写更加健壮的代码。合理地搭配使用panic! 和Result可以让我们的代码在面对无法避免的错误时显得更加可靠。

