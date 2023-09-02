#set heading(numbering: "1.")
#set text(size: 15pt)

= Rust 语言特点
+ 性能\
    Rust 速度极快且内存高效：无需运行时或垃圾收集器，它可以为性能关键型服务提供支持，在嵌入式设备上运行，并轻松与其他语言集成。
+ 可靠性\
    Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全——使您能够在编译时消除许多类别的错误。
+ 生产效率\
    Rust 拥有出色的文档、带有有用错误消息的友好编译器以及一流的工具——集成的包管理器和构建工具、具有自动完成和类型检查功能的智能多编辑器支持、自动格式化程序等等。

= Rust 与其它语言的比较
+ Go\
    Rust 语言表达能力更强，性能更高。同时线程安全方面 Rust 也更强，不容易写出错误的代码。包管理 Rust 也更好，Go 虽然在 1.10 版本后提供了包管理，但是目前还比不上 Rust 。
+ C++\
    Rust 与 C++ 的性能旗鼓相当，但是在安全性方面 Rust 会更优，特别是使用第三方库时，Rust 的严格要求会让三方库的质量明显高很多。语言本身的学习，Rust 的前中期学习曲线会更陡峭，但是在实际的项目开发过程中，C++ 会更难，代码也更难以维护。
+ Java\
    除了极少数纯粹的数字计算性能，Rust 的性能全面领先于 Java 。同时 Rust 占用内存小的多，因此实现同等规模的服务，Rust 所需的硬件成本会显著降低。

= Rust 语言的现状
+ Rust for Linux 从 Linux 6.1 开始进入 Linux 内核，成为开发内核的第二门语言。
+ 截止到2022年12月已经有39家不同领域的头部公司成为 Rust 基金会的成员（包括 Mozilla、Amazon、华为、Google、Microsoft 等），推动 Rust 在各自领域的落地。
+ AWS 从 2017 年开始就用 Rust 实现了无服务器计算平台： AWS Lambda 和 AWS Fargate。
+ Dropbox 的底层存储服务完全由 Rust 重写。
+ Google 在 Android 13 中，大约有 21% 的新原生代码是Rust，还在它最新的操作系统 Fuchsia 中重度使用 Rust。
+ Microsoft 使用 Rust 为 Azure 平台提供一些组件，其中包括 IoT 的核心服务，用 Rust 重写 Windows 部分底层组件。
+ Rust 目前已经成为全世界区块链平台的首选开发语言。

= 教程的配套书籍
#link("https://course.rs", [Rust 语言圣经]): https://course.rs
#image("images/Rust语言介绍/rust_course.png")
