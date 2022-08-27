# Cargo工具的常用命令

## 创建一个新项目
```rust
cargo new hello_cargo
```

## 构建
```rust
cargo build
// 可执行文件目录
target/debug
```

## 编译与运行
```rust
cargo run
```

## 快速检查代码
```rust
cargo check
```

## 以release模式进行构建
```rust
cargo build --release
// 可执行文件目录
target/release
```
1. 这种模式以编译时间为代价来优化代码
2. 用于构建交付给用户的最终程序
3. 上面的debug模式则是用于开发
  
**使用target/release目录下的可执行文件进行基准测试**
