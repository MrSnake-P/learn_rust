> 泛型是具体类型或其他属性的抽象替代。在编写代码时，我们可以直接描述泛型的行为，或者它与其他泛型产生的联系，而无须知晓它在编译和运行代码时采用的具体类型。

## 通过将代码提取为函数来减少重复工作

## 泛型数据类型

1. 在函数定义中
```rust
src/main.rs
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

暂时还无法通过编译，函数体中的相关语句需要比较类型T的值，这个操作只能被用于可排序的值类型。我们可以通过实现标准库中的std::cmp::PartialOrd trait来为类型实现比较功能。

2. 在结构体定义中
```rust
src/main.rs
struct Point<T>❶ {
    x: T,❷
    y: T,❸
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

存储了T类型值x与y的Point<T>结构体, 需要注意的是 x，y都是属于泛型T的，必须是相同类型，无法使用不同的值类型。
```rust
// panic
fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

*使用多个泛型参数*
```rust
src/main.rs
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

3. 在枚举定义中

```rust
enum Option<T> {
    Some(T),
    None,
}

// 多个泛型参数
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

4. 在方法定义中

结构体Point<T>实现名为x的方法，它会返回一个指向x字段中T类型值的引用

*必须紧跟着impl关键字声明T*
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

*指定具体的类型f32*
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

类型Point<f32>将会拥有一个名为distance_from_origin的方法，而其他的Point<T>实例则没有该方法的定义。

*接收另外一个Point作为参数* 
```rust
src/main.rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U>❶ Point<T, U> {
    fn mixup<V, W>❷(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
 ❸ let p1 = Point { x: 5, y: 10.4 };
 ❹ let p2 = Point { x: "Hello", y: 'c'};

 ❺ let p3 = p1.mixup(p2);

 ❻ println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### 泛型代码的性能问题

Rust会在编译时执行泛型代码的单态化（monomorphization）。单态化 是一个在编译期将泛型代码转换为特定代码的过程，它会将所有使用过的具体类型填入泛型参数从而得到有具体类型的代码。

## trait：定义共享行为
trait（特征）被用来向Rust编译器描述某些特定类型拥有的且能够被其他类型共享的功能，它使我们可以以一种抽象的方式来定义共享行为。我们还可以使用trait约束来将泛型参数指定为实现了某些特定行为的类型。

> trait与其他语言中常被称为接口（interface）的功能类似，但也不尽相同。

### 定义trait
当我们可以在不同的类型上调用相同的方法时，我们就称这些类型共享了相同的行为。

```rust
src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

一个trait可以包含多个方法：每个方法签名占据单独一行并以分号结尾。

### 为类型实现trait

```rust
src/lib.rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

方法调用
```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

> 假设这个lib.rs属于某个名为aggregator的库，当第三方开发者想要为他们自定义的结构体实现Summary trait并使用相关功能时，就必须将这个trait引入自己的作用域中。使用use aggregator::Summary;语句就可以完成引入操作，进而调用相关方法或为自定义类型实现Summary。

孤儿规则：不能为外部类型实现外部trait。

因为它的父类型没有定义在当前库中。这一规则也是程序一致性 （coherence）的组成部分，它确保了其他人所编写的内容不会破坏到你的代码，反之亦然。如果没有这条规则，那么两个库可以分别对相同的类型实现相同的trait，Rust将无法确定应该使用哪一个版本。

### 默认实现

```rust
src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String {
        // 定义一个默认的实现
        String::from("(Read more...)")
    }
}

// 指定一个空的impl代码块：
impl Summary for NewsArticle {}。

```

```rust
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

// 结果为：
// New article available! (Read more...)。
```

在默认实现中调用相同trait中的其他方法
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

### 使用trait作为参数
```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### trait约束
```rust
// 等价与上面
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```
impl Trait更适用于短小的示例，而trait约束则更适用于复杂情形

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {}
// 等价
pub fn notify<T: Summary>(item1: T, item2: T) {}
```

* 通过+语法来指定多个trait约束
```rust
pub fn notify(item: impl Summary + Display) {}
// 等价
pub fn notify<T: Summary + Display>(item: T) {}
```

* 使用where从句来简化trait约束
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// 等价
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}
```

### 返回实现了trait的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

*只能返回一个类型的trait*
```rust
// panic!!!
// panic!!!
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

### 使用trait约束来修复largest函数

largest函数可以被用于任何实现了PartialOrd与Copy这两个trait的泛型
```rust
src/main.rs
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // 这里需要Copy，因为比如参数全是String类型时，成员所有权是不能被移出的

    for &item in list.iter() { // 这里需要std::cmp::PartialOrd，因为并不是所有数据都是支持比较的
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
也可以用Clone来替换T trait约束中的Copy。当需要在largest函数中取得切片中某个值的所有权时，我们就可以使用克隆方法。
另一种可能的largest实现方式是返回切片中T值的引用。假如将返回类型从T修改为&T，并修改函数体使其返回一个引用，那么我们就不再需要Clone或Copy来进行trait约束了，同时可以避免执行堆分配操作。

### 使用trait约束来有条件地实现方法

只有在内部类型T实现了PartialOrd（用于比较）与 Display（用于打印）这两个trait的前提下，才会实现cmd_display方法。

根据泛型的trait约束来有条件地实现方法
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

为实现了某个trait的类型有条件地实现另一个trait。

例如，标准库对所有满足Display trait约束的类型实现了ToString trait。
```rust
impl<T: Display> ToString for T {
    // --略

--
}
```

> 借助于trait和trait约束，我们可以在使用泛型参数来消除重复代码的同时，向编译器指明自己希望泛型拥有的功能。并且在编译期就能发现，类型是否已经实现方法

## 使用生命周期保证引用的有效性