# java-lang
用于rust的java语法解析器。

根据[Java 23 规范](https://docs.oracle.com/javase/specs/jls/se23/html/index.html)实现。
工作正在积极进行中，API还不稳定，欢迎大家参与贡献。


## 开始使用
```shell
cargo add java-lang
```

## 示例
在[examples](examples)文件夹中有相关示例。
```shell
cargo run --example hello
```

## 测试
```shell
cargo test --all-features
```

## 已经实现的功能
- [x] 从源代码生成Tokens；
- [x] 解析包声明（package xxx;）；
- 解析导入声明（import xxx;）；
- [x] 解析文档注释（/** ... */）；