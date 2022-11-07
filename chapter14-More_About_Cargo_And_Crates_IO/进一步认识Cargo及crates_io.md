• 使用发布配置来定制构建。

• 将代码库发布到crates.io上。

• 使用工作空间来组织更大的项目。

• 下载安装crates.io提供的二进制文件。

• 使用自定义命令来扩展Cargo。

## 使用发布配置来定制构建

```rust
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```
以上输出中的dev和release表明了编译器正在使用不同的配置。

```rust
Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

opt-level决定了Rust在编译时会对代码执行何种程度的优化，从0到3都是合法的配置值。
越高优化需要消耗越多的编译时间。

## 将包发布到crates.io上

### 编写有用的文档注释

自动打开文档网址
`cargo doc --open`

特殊的文档注释 （documentation comment）：使用三斜线（///）而不是双斜线来编写文档注释，
并且可以在文档注释中使用Markdown语法来格式化内容。
文档注释被放置在它所说明的条目之前。

```rust
src/lib.rs
/// 将传入的数字加

1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```


#### 常用的文档注释区域

• Panics ，指出函数可能引发panic的场景。不想触发panic的调用者应当确保自己的代码不会在这些场景下调用该函数。

• Errors ，当函数返回Result作为结果时，这个区域会指出可能出现的错误，以及造成这些错误的具体原因，它可以帮助调用者在编写代码时为不同的错误采取不同的措施。

• Safety ，当函数使用了unsafe关键字（在第19章讨论）时，这个区域会指出当前函数不安全的原因，以及调用者应当确保的使用前提。

#### 将文档注释用作测试

#### 在条目内部编写注释

//!，它可以为包裹当前注释的外层条目（而不是紧随注释之后的条目）添加文档。

### 使用pub use来导出合适的公共API

```rust
src/lib.rs
//! # Art
//!
//! 一个用来建模艺术概念的代码库



pub mod kinds {
    /// RYB颜色模型的三原色


    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB模型的调和色


    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 将两种等量的原色混合生成调和色


    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --略

--
    }
}
```

修改为

使用pub use语句将需要公开的条目重新导出到顶层结构中，

```rust
src/lib.rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --略

--
}

pub mod utils {
    // --略

--
}
```

调用

```rust
src/main.rs
use art::PrimaryColor;
use art::mix;

fn main() {
    // --略

--
}
```

### 创建crates.io账户

访问crates.io主页并使用GitHub账户登录来完成注册。

`$ cargo login abcdefghijklmnopqrstuvwxyz012345`

将API令牌存入～/.cargo/credentials 文件

### 为包添加元数据

`cargo publish`

已经上传的版本将无法被覆盖，对应的代码也不能被删除。这种行为正是crates.io的一个主要设计目标，它希望能够成为一个永久的代码文档服务器，并保证所有依赖于crates.io的包都能一直被正常构建。

### 发布已有包的新版本

为了在修改代码后发布新的版本，我们需要修改Cargo.toml 文件中的version字段并重新发布。你应当根据语义化版本规则来基于修改的内容决定下一个合理的版本号，然后执行cargo publish上传新的版本。

### 使用cargo yank命令从cargo.io上移除版本

所有已经产生Cargo.lock 的项目将不会受到撤回操作的影响，而未来所有产生的新Cargo.lock 文件将不会再使用已经撤回的版本。

运行cargo yank时，指定对应版本号即可撤回指定版本：

`$ cargo yank --vers 1.0.1`

添加--undo参数，也可以取消撤回操作
`$ cargo yank --vers 1.0.1 --undo`

## Cargo工作空间

Cargo提供了一个叫作工作空间 （workspace）的功能，它可以帮助开发者管理多个相互关联且需要协同开发的包。

### 创建工作空间

```
$ mkdir add
$ cd add

Cargo.toml
[workspace]

members = [
    "adder",
]
```

```
$ cargo new adder
```

使用cargo build来构建整个工作空间

### 在工作空间中创建第二个包

```rust
Cargo.toml
[workspace]

members = [
    "adder",
    "add-one",
]

$ cargo new add-one --lib
```

如下所示的目录和文件：
```rust
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

#### 指定adder依赖
```rust
adder/Cargo.toml
[dependencies]

add-one = { path = "../add-one" }

adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}

$ cargo build
```

```shell
调用cargo run时通过-p参数来指定需要运行的包名
$ cargo run -p adder
```

#### 在工作空间中依赖外部包

```rust
add-one/Cargo.toml
[dependencies]

// 引入依赖
$ cargo build
```

在adder包的Cargo.toml 文件中也需要添加rand依赖

#### 为工作空间增加测试

```rust
add-one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
            x + 1
}

#[cfg(test)]
mod tests {
    use super::*

;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

使用参数-p及指定的包名称来运行某一个特定包的测试：

`$ cargo test -p add-one`

你可以在项目规模逐渐增长时考虑使用工作空间：独立短小的组件要比繁复冗长的代码更容易理解一些。另外，当多个包经常需要同时修改时，将它们放于同一工作空间下也有助于协调同步。

## 使用cargo install从crates.io上安装可执行程序

cargo install命令使我们可以在自己的计算机设备中安装和使用二进制包。

所有通过cargo install命令安装的二进制文件都会被存储在Rust安装根目录下的bin 文件夹中。

`$ cargo install ripgrep`

## 使用自定义命令扩展Cargo的功能

Cargo允许我们添加子命令来扩展它的功能而无须修改Cargo本身。只要你的$PATH路径中存在二进制文件cargo-something，就可以通过运行cargo something来运行该二进制文件，就好像它是Cargo的子命令一样。运行cargo --list可以列出所有与此类似的自定义命令。借助于这一设计，我们可以使用cargo install来安装扩展，并把这些扩展视作内建的Cargo命令来运行。

## 总结

Cargo和crates.io共同构建出的代码分享机制，Rust的生态系统才能够应对许多不同类型的任务。

虽然Rust的标准库小巧且稳定，但是我们依然可以借助包机制来轻松地分享与使用代码，并随着时间不断地演化进步而不必拘泥于语言本身的更新频率。