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
当出现了多个可能的类型时，我们就必须手动声明类型。类似地，当引用的生命周期可能以不同的方式相互关联时，我们就必须手动标注生命周期。Rust需要我们注明泛型生命周期参数之间的关系，来确保运行时实际使用的引用一定是有效的。

### 使用生命周期来避免悬垂引用

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

   // 当内部作用域结束时，尝试去打印出r所指向的值
   println!("r: {}", r);
}
```
```rust
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```
上面的错误提示信息指出，变量x的存活周期不够长。

正确的写法
```rust
    let x = 5;
    let r = &x;
    println!("r: {}", r);
```

### 函数中的泛型生命周期

返回两个字符串切片中较长的一个
```rust
src/main.rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

1. 不希望longest取得参数的所有权,可以接收字符串切片（也就是引用）作为参数
2. 这个函数既能处理String切片（也就是变量string1的类型），又能处理字符串字面量（也就是变量string2所存储的）。
```rust
src/main.rs
// panic !!!
// panic !!!
// panic !!!
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// panic 因为它不知道x与y的生命周期是如何与返回值的生命周期相关联的
// 所以需要添加一个泛型生命周期参数
```

### 生命周期标注语法
生命周期的标注并不会改变任何引用的生命周期长度。如同使用了泛型参数的函数可以接收任何类型一样，使用了泛型生命周期的函数也可以接收带有任何生命周期的引用。在不影响生命周期的前提下，标注本身会被用于描述多个引用生命周期之间的关系。

生命周期的标注的语法：
它们的参数名称必须以撇号（'）开头，且通常使用全小写字符。'a被大部分开发者选择作为默认使用的名称。

1. 引用 `&i32`
2. 拥有显示生命周期的引用 `&'a i32`
3. 拥有显示生命周期的可变引用 `&'a mut i32`

### 函数签名中的生命周期标注

参数与返回值中的所有引用都必须拥有相同的生命周期。
```rust
src/main.rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
函数所获取的两个字符串切片参数的存活时间，必须不短于给定的生命周期'a。这个函数签名同时也意味着，从这个函数返回的字符串切片也可以获得不短于'a的生命周期。

在函数签名中指定生命周期参数时，**并没有改变任何传入值或返回值的生命周期**。只是向借用检查器指出了一些用于检查非法调用的约束。

**泛型生命周期'a会被具体化为x与y两者中生命周期较短的那一个。因为将返回的引用也标记为了生命周期参数'a，所以返回的引用在具化后的生命周期范围内都是有效的。**

> 注意，longest函数本身并不需要知道x与y的具体存活时长，只要某些作用域可以被用来替换'a并满足约束就可以了。

> 函数所使用的生命周期可能在每次调用中都会发生变化。这也正是需要手动对生命周期进行标注的原因。

使用具有不同生命周期的String来调用longest函数
```rust
src/main.rs
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// 输出
// The longest string is long string is long。
```

尝试在string2离开作用域后使用result
```rust
src/main.rs
// panic !!!
// panic !!!
// panic !!!
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // 为了使println! 语句中的result是有效的，string2需要一直保持有效
    // 因为在函数参数与返回值中使用了同样的生命周期参数'a，
    println!("The longest string is {}", result);
}
```

### 深入理解生命周期

假如将longest函数的实现修改为返回第一个而不是最长的那个字符串切片参数，那么我们就无须再为y参数指定生命周期。
```rust
// 代码是可以通过编译的
src/main.rs
// y的生命周期与x和返回值的生命周期没有任何相互关系
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

无法通过编译的longest函数实现
```rust
src/main.rs
fn longest<'a>(x: &str, y: &str) -> &'a str {
    // 在于result在longest函数结束时就离开了作用域，并被清理
    // 无论我们怎么改变生命周期参数，都无法阻止悬垂引用的产生
    let result = String::from("really long string");
    result.as_str()
}
```

解决办法就是返回一个持有自身所有权的数据类型而不是引用, 将清理值的责任转移给函数调用者了。

### 结构体定义中的生命周期标注

结构体中持有了引用，所以它的定义中需要添加生命周期标注
```rust
struct ImportantExcerpt<'a> {
    // 这个标注意味着ImportantExcerpt实例的存活时间不能超过存储在part字段中的引用的存活时间。
    part: &'a str,
}

fn main() {
    let novel := String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".");
        .next()
        .expect("could not find a '.'");
    let i = ImportantExcerpt{part: first_sentence}
}
```
在ImportantExcerpt实例创建之前，novel中的数据就已经生成了，而且novel会在ImportantExcerpt离开作用域后才离开作用域，所以ImportantExcerpt实例中的引用总是有效的。

### 生命周期省略

参数和返回类型都是引用，这个函数依然没有使用生命周期标注
```rust
src/lib.rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

在Rust的早期版本（pre-1.0）中，这样的代码确实无法通过编译，因为每个引用都必须有一个显式的生命周期。当时的函数签名会被写为：

`fn first_word<'a>(s: &'a str) -> &'a str {}`
使借用检查器在这些情况下可以自动对生命周期进行推导而无须显式标注。

在没有显式标注的情况下，编译器目前使用了3种规则来计算引用的生命周期
1. 每一个引用参数都会拥有自己的生命周期参数。换句话说，单参数函数拥有一个生命周期参数：fn foo<'a>(x: &'a i32)；双参数函数拥有两个不同的生命周期参数：fn foo<'a, 'b>(x: &'a i32, y: &'b i32)；以此类推。
2. 当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数，例如fn foo<'a>(x: &'a i32) -> &'a i32。
3. 当拥有多个输入生命周期参数，而其中一个是&self或&mut self时，self的生命周期会被赋予给所有的输出生命周期参数。这条规则使方法更加易于阅读和编写，因为它省略了一些不必要的符号。

### 方法定义中的生命周期标注

生命周期省略规则在大部分情况下都可以帮我们免去方法签名中的生命周期标注。

声明在impl及类型名称之后的生命周期是不能省略的，但根据第一条省略规则，我们可以不用为方法中的self引用标注生命周期。
```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

应用第三条生命周期省略规则的例子：
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### 静态生命周期
一种特殊的生命周期'static，它表示整个程序的执行期。所有的字符串字面量都拥有'static生命周期。

可以像下面一样显式地把它们标注出来：
```rust
let s: &'static str = "I have a static lifetime.";
```
字符串的文本被直接存储在二进制程序中，并总是可用的。因此，所有字符串字面量的生命周期都是'static。

## 同时使用泛型参数、trait约束与生命周期

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 总结
泛型参数、trait与trait约束，以及泛型生命周期参数等概念