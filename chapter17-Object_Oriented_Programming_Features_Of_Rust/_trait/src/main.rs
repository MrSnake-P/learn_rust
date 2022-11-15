use _trait::{Screen, Button};
use _trait::Draw;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no")
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    // 鸭子类型
    // run方法的过程中并不需要知晓每个组件的具体类型，
    // 它仅仅调用了组件的draw方法，
    // 而不会去检查某个组件究竟是Button实例还是SelectBox实例
    screen.run();
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self.width);
        println!("{:?}", self.height);
        println!("{:?}", self.options);
    }
}