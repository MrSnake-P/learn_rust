fn main() {
    let rect1 = (30, 50);

    println!("{}", area_dimensions(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", area_struct(&rect2));

    let rect3 = Rectangle {width: 30, height: 50};
    println!("{:?}", rect3);
    println!("{:#?}", rect3);
}

fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

### 方法
