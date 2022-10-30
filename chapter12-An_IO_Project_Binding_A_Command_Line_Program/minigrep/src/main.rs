use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 当前执行的二进制文件名称
    // let file: &String = &args[0];

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    // println!("Seachering for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // 使用了if let而不是unwrap_or_else来检查run的返回值，
        // 因为run函数并不会返回一个需要进行unwrap的值，
        // 只关注产生错误时的情形
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

