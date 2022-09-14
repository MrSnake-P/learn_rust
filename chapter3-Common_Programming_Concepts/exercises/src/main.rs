fn main() {
    println!("Hello, world!");

    let temperature1: f64 = 10.0;
    let temperature2: f64 = 50.0;
    println!("{}", c_to_f(temperature1, true));
    println!("{}", c_to_f(temperature2, false));

    println!("{}", fibonacci_sequence(5));
}

// 摄氏度与华氏度互相转换
fn c_to_f(temperature: f64, is_c: bool) -> f64 {
    const FACTOR: f64 = 32.0;
    const FACTOR_DECIMAL: f64 = 1.8;
    // 摄氏度转为华氏度
    if is_c {
        temperature * FACTOR_DECIMAL + FACTOR
    } else {
        (temperature - FACTOR) / FACTOR_DECIMAL
    }
}

// 斐波那契数列
fn fibonacci_sequence(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 2;
    }

    return fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2);
}
