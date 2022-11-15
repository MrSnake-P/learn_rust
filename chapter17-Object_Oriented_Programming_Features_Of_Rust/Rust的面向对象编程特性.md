## 面向对象语言的特性

包含以下这些特性：命名对象、封装及继承。

### 对象包含数据和行为

面向对象的程序由对象组成。对象包装了数据和操作这些数据的流程。这些流程通常被称作方法或操作。

### 封装实现细节

另外一个常常伴随着面向对象编程的思想便是封装 （encapsulation）：调用对象的外部代码无法直接访问对象内部的实现细节，而唯一可以与对象进行交互的方法便是通过它公开的接口。

```rust
src/lib.rs
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

src/lib.rs
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

结构体AveragedCollection封装了内部的实现细节，所以我们能够在未来轻松地改变数据结构等内部实现。

### 作为类型系统和代码共享机制的继承

继承（inheritance）机制使得对象可以沿用另一个对象的数据与行为，而无须重复定义代码。

默认trait方法来进行代码共享，与继承类似，任何实现trait中声明的方法的类型，都算实现了trait，正如子类覆盖父类中的方法一样。

多态 （polymorphism）：希望子类型能够被应用在一个需要父类型的地方。如果一些对象具有某些共同的特性，那么这些对象就可以在运行时相互替换使用。


> 多态

> 许多人将“多态”视作“继承”的同义词。但实际上多态是一个更为通用的概念，它指代所有能够适应多种数据类型的代码。对于继承概念而言，这些类型就是所谓的子类。

> 你可以在Rust中使用泛型来构建不同类型的抽象，并使用trait约束来决定类型必须提供的具体特性。这一技术有时也被称作限定参数化多态（bounded parametric polymorphism）。

继承的缺点：使用继承，会导致子类共享父类所有特性，而有些可能是不适用子类的方法，缺少一定的灵活性。

故使用trait来代替继承。

## 使用trait对象来存储不同类型的值

### 为共有行为定义一个trait

trait对象类似与其他语言中的对象，某种程度上组合了数据与行为。

但trait对象与传统对象不同的地方在于，无法为trait对象添加数据。由于trait对象被专门用于抽象某些共有行为，所以它没有其他语言中的对象那么通用。

```rust
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

```

### trait对象会执行动态派发

Rust编译器会在泛型使用trait约束时执行单态化：
编译器会为每一个具体类型生成对应泛型函数和泛型方法的非泛型实现，
并使用这些具体的类型来替换泛型参数。

**编译器能够在编译过程中确定你调用的具体方法。**

动态派发（dynamic dispatch）相对应，动态派发下的编译器无法在编译过程中确定你调用的究竟是哪一个方法。

> Rust必然会在我们使用trait对象时执行动态派发。因为编译器无法知晓所有能够用于trait对象的具体类型，所以它无法在编译时确定需要调用哪个类型的哪个具体方法。

### trait对象必须保证对象安全

trait的对象安全：

• 方法的返回类型不是Self。

• 方法中不包含任何泛型参数。

标准库中的Clone trait就是一个不符合对象安全的例子。Clone trait中的clone方法拥有这样的签名：

```rust
// clone方法的签名需要知道Self究竟代表了哪一种具体类型，
// 然后作为结果返回的类型。
pub trait Clone {
    fn clone(&self) -> Self;
}
```

## 实现一种面向对象的设计模式

状态模式（state pattern）是一种面向对象的设计模式，
它的关键特点是，一个值拥有的内部状态由数个状态对象（state object）表达而成，而值的行为则随着内部状态的改变而改变。

### 状态模式的权衡取舍

• 添加reject方法，它可以将文章的状态从PendingReview修改为Draft。

• 为了将文章状态修改为Published，用户需要调用两次approve。

• 用户只有在文章处于Draft状态时才能够修改文本内容（提示：将改变内容的职责从Post转移至状态对象）。.

缺点：
1. 因为状态实现了状态间的转移，所以某些状态之间是相互耦合的。

在PendingReview和Published之间添加一个Scheduled状态，那么我们就需要修改PendingReview中的代码来转移到Scheduled状态。

2. 我们需要重复实现一些代码逻辑。

如果让State trait的request_review和approve方法默认返回self；这样的代码违背了对象安全规则。

## 总结

由于Rust具有所有权等其他面向对象语言没有的特性，所以面向对象的模式仅仅是一种可用的选项，而并不总是最佳实践方式。