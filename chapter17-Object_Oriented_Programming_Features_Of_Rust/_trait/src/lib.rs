pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw>来定义trait对象，
    // 它被用来代表所有被放置在Box中且实现了Draw trait的具体类型。
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // 逐一调用components中每个元素的draw方法
            component.draw();
        }
    }
}

pub struct Button {
    pub width: usize,
    pub height: usize,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制一个按钮
    }
}


// 带有trait约束的泛型参数来定义结构体
// 泛型参数一次只能被替代为一个具体的类型，
// 而trait对象则允许你在运行时填入多种不同的具体类型。
// 同时只能使用一种相同的类型的同质集合
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }