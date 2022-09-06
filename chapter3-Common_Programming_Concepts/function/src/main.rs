fn main() { 
    println!("Hello, world!"); 
 
    another_function(5, 6); 

    println!("The value of x is: {}", five());

    println!("The value of x is: {}", plus_one(1));
} 
 
// 必须显式地声明每个参数的类型
fn another_function(x: i32, y: i32) { 
    println!("The value is: {}", x); 
    println!("The value is: {}", y); 
} 

fn five() -> i32 { 
    5 
} 

fn plus_one(x: i32) -> i32 { 
    // 末尾不能加上分号
    // panic！ x + 1；
    // 加上分号
    // 默认会返回一个空元组
    // （）会导致类型不匹配
    x + 1 
} 