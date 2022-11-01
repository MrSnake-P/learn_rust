pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

### 生成其他迭代器的方法

Iterator trait还定义了另外一些被称为迭代器适配器 （iterator adaptor）的方法，
这些方法可以使你将已有的迭代器转换成其他不同类型的迭代器。
可以链式地调用多个迭代器适配器完成一些复杂的操作，同时保持代码易于阅读。
为所有的迭代器都是惰性的，所以必须调用一个消耗适配器的方法才能从迭代器适配器中获得结果。

调用map方法创建新迭代器，接着再调用collect方法将其消耗掉并得到一个动态数组
```rust
src/main.rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

### 使用闭包捕获环境

迭代器的filter方法会接收一个闭包作为参数，这个闭包会在遍历迭代器中的元素时返回一个布尔值，
而每次遍历的元素只有在闭包返回true时才会被包含在filter生成的新迭代器中。

传入一个捕获了变量shoe_size的闭包来使用filter方法
```rust
src/lib.rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 调用了into_iter来创建可以获取动态数组所有权的迭代器
    shoes.into_iter()
       .filter(|s| s.size == shoe_size)
       .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

调用了into_iter来创建可以获取动态数组所有权的迭代器

### 使用Iterator trait来创建自定义迭代器

只需要提供一个next方法的定义即可实现Iterator trait

```rust
src/lib.rs
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 为Counter结构体实现Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

#### 使用Counter迭代器的next方法

```rust
src/lib.rs
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

#### 使用其他的Iterator trait方法

将一个Counter实例产生的值与另一个Counter实例跳过首元素后的值一一配对，
接着将配对的两个值相乘，最后再对乘积中能被3整除的那些数字求和。
```rust
src/lib.rs
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

> 注意，zip方法只会产生4对值，它在两个迭代器中的任意一个返回None时结束迭代，所以理论上的第五对值(5, None)永远不会被生成出来。
因为我们指定了next方法的具体行为，而标准库又对其他调用next的方法提供了默认实现，所以我们能够合法地使用所有这些方法。

## 改进I/O项目

### 使用迭代器代替clone

```rust
src/lib.rs
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

改进后


new函数并不持有args参数内元素的所有权，
获得的仅仅是一个String序列的切片。
为了返回Config实例的所有权，必须要克隆Config的query字段和filename字段中的值，
只有这样Config才能拥有这些值的所有权。

只要Config::new能够获取迭代器的所有权，我们就可以将迭代器产生的String值移动到Config中，而无须调用clone进行二次分配。

直接使用返回的迭代器


```rust
impl Config {
    // env::args函数的返回值其实就是一个迭代器！
    // 获得了args的所有权并会在函数体中通过迭代来改变它，
    // 所以需要在args参数前指定mut关键字来使其可变。
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // --略
    }
```

#### 使用Iterator trait方法来替代索引

```rust
src/lib.rs
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // env::args的返回值的第一个值是程序本身的名称
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

### 使用迭代器适配器让代码更加清晰

使用迭代器适配器实现search函数

```rust
src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```
优化后
```rust
src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

## 比较循环和迭代器的性能

遍历了coefficients中所有的12个元素，并用zip方法将其与buffer的前12个值一一配对。
接着，将每一对数值相乘并对所有得到的乘积求和，最后将总和向右移qlp_shift位得到结果。
```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```
因为Rust知道这里会迭代12次，所以它直接“展开”（unroll）了循环。
展开 是一种优化策略，它通过将循环代码展开成若干份重复的代码来消除循环控制语句带来的性能开销。
这样能让所有coefficients中的值都存储在寄存器中，进而使得对它们的访问变得异常快速。
同时，我们也就无须在运行时浪费时间对数组访问进行边界检查了。

## 总结

闭包和迭代器的实现保证了运行时性能不会受到影响。