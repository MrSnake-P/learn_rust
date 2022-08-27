use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1-100的随即哦证书
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("secret number: {}", secret_number);

    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        // 定义可以修改的变量
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 隐藏机制，使用相同名的变量来隐藏就变量的值
        // 加上错误处理
        // match有OK和Err两个变体
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 展位符 {}
        println!("you guessed: {}", guess);

        // 匹配
        // 前面与后面作比较
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
