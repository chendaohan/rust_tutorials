#set heading(numbering: "1.")
#set text(size: 15pt)

= 字符类型（演示在`main()`中）
Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，所以在 Rust 中字符类型的大小是 32 bit ，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。

= 布尔（演示在`bool_type()`中）
Rust 中的布尔类型有两个可能的值：true 和 false 。

= 单元类型（演示在`unit_type()`中）
单元类型是 `()` ，唯一的值也是 `()`

= Never 类型（演示在`never_type()`中）
Never 类型是 `!`，其值不能直接创造。返回 `!` 类型的函数被称为发散函数（diverge function）。