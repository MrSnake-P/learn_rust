fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());

    println!("{}", rect.can_hold(&rect1));
    println!("{}", rect.can_hold(&rect2));

    // 关联函数
    let sq = Rectangle::square(32);
    println!("{}", sq.area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

// 是否是只读的（&self），是否需要修改数据（&mut self）
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
