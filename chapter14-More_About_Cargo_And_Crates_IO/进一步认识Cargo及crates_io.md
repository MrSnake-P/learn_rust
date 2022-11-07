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
