#set heading(numbering: "1.")
#set text(size: 15pt)

#let path = "images/安装Rust/"

= 打开 Rust 官网
#link("https://www.rust-lang.org", [Rust Programming Language]): https://www.rust-lang.org

#image(path + "home_page.png")

= 点击 Install 或 Get Started
#image(path + "install_page_link.png")

= Linux 安装
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

= Windows 安装
== 下载安装包
#image(path + "install_package.png")

== 安装 Visual Studio（需重启）
#image(path + "install_vc_option.png")
#image(path + "visual_studio_installer.png")

= 安装 Rust 工具链
#image(path + "install_rust_option.png")
#image(path + "finish.png")

= 检查是否安装成功
```sh
rustc --version
```
#image(path + "rustc.png")

= 更新 Rust 工具链
```sh
rustup update
```
#image(path + "rustup.png")

= 安装 VS Code 的 Rust 插件 rust-analyzer
#image(path + "rust_analyzer.png")

= 其它 VS Code 插件
#figure(
    image(path + "toml.png"),
    caption: [toml 语法高亮，代码提示]
)
#figure(
    image(path + "crates.png"),
    caption: [Rust 依赖管理]
)
#figure(
    image(path + "error_lens.png"),
    caption: [高亮错误、警告]
)
#figure(
    image(path + "codelldb.png"),
    caption: [调试代码插件]
)