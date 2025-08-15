

### 1. 创建 Rust 项目 & 配置依赖
```
cargo new md5-wasm --lib
```

### 2. Cargo.toml 配置
```
[package]
name = "md5-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"] # 编译为 Wasm 动态库

[dependencies]
md-5 = "0.10.6"       # MD5 算法库
wasm-bindgen = "0.2.92" # 生成 JS 胶水代码
hex = "0.4.3"         # 二进制转十六进制
```


### 3.  编译为 WebAssembly
```
wasm-pack build --target web --release

```

### 4. 前端集成到 React+TypeScript








