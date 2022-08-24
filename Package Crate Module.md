# Package, Crate, Module

## Package、Crate、定义Module

Rust 的代码组织。代码组织主要包括：

- 哪些细节可以暴露，哪些细节是私有的
- 作用域内哪些名称有效

- 等等

模块系统：
- Package（包）：Cargo 的特性，让你构建、测试、共享 crate
- Crate（单元包）：一个模块树，它可产生一个 library 或可执行文件
- Module（模块）、use：让你控制代码的组织、作用域、私有路径

- Path（路径）：为 struct、function 或 module 等项命名的方式

### Pacakge 和 Crate

**Crate** 的类型：

- binary

- library

**Crate Root**：

- 是源代码文件

- Rust 编译器从这里开始，组成你的 Crate 的根 Module

一个 **Package**：

- 包含1个 Cargo.toml，它描述了如何构建这些 Crates

- 只能包含0-1个 library crate

- 可以包含任意数量的 binary crate

- 但必须至少包含一个 crate（library 或 binary）

### Cargo的惯例

1. **src/main.rs**：

- binary crate 的 crate root
- crate 名与 package 名相同

2. **src/lib.rs**：

- package 包含一个 library crate
- library crate 的 crate root
- crate 名与 package 名相同

3. Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary

4. 一个 Package 可以同时包含 src/main.rs 和 src/lib.rs 

- 一个 binary crate，一个 library crate

- 名称与 package 名相同

5. 一个 Package 可以有多个 binary crate:

- 文件放在 src/bin

- 每个文件是单独的 binary crate

### Crate 的作用

将相关功能组合到一个作用域内，便于在项目间进行共享

- 防止冲突。例如 rand crate，访问它的功能需要通过它的名字：rand

#### 定义 module 来控制作用域和私有性

**Module**：

- 在一个 crate 内，将代码进行分组一增加可读性，易于复用

- 控制项目（item）的私有性。public、private

**建立 module**：

- mod 关键字
- 可嵌套
- 可包含其它项（struct、enum、常量、trait、函数等）的定义

```rust
mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

### Module

src/main.rs 和 src/lib.rs 叫做crate roots

这两个文件（任意一个）的内容形成了名为 crate 的模块，位于整个模块树的根部

整个模块树在隐式的 crate 模块下

## 路径 Path

为了在 Rust 的模块中找到某个条目，需要使用路径

### 路径的两种形式

- 绝对路径：从 crate root 开始，使用 crate 名或字面值 crate 

- 相对路径：从当前模块开始，使用 self，super 或当前模块的标识符

路径至少由一个标识符组成，标识符之间使用 `::`

### 私有边界（privacy boundary）

模块不仅可以组织代码，还可以定义私有边界

如果想把函数或 struct 等设为私有，可以将它放到某个模块中

Rust 中所有的条目（函数，方法，struct，enum，模块，常量）默认是私有的

父级模块无法访问子模块中的私有条目

子模块里可以使用所有祖先模块中的条目

### pub 关键字

使用 `pub` 关键字来将某些条目标记为公共的

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
}

// 如果两个条目都是根级，就可以相互调用
```

### super 关键字

```rust
fn serve_order() {0}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {}
}
```

### pub struct

pub 放在 struct 前

```rust
mod back_of_house {
    pub struct breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
}
```

- struct 是公共的
- struct 的字段默认是私有的

struct 的字段需要单独设置 pub 来变成共有

### pub enum

pub 放在 enum 前

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

- enum是公共的
- enum的变体**也都是公共的**

## use 关键字

可以使用 use 关键字将路径导入到作用域内

- 仍遵循私有性规则

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use::crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

使用 use 来指定相对路径

```rust
use::front_of_house::hosting;
```

### use 的习惯用法

- 函数：将函数的父级模块引入作用域（指定到父级）

- struct，enum，其它：指定完整路径（指定到本身）

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Hello, world!");
}
```

- 同名条目：指定到父级

```rust
use std::io;
use std::fmt;

fn f1() -> fmt::Result {}
fn f2() -> io::Result {}
```

**as 关键字**

- as 关键字可以为引入的路径指定本地的**别名**

```rust
use std::io::Result as IoResult;
use std::fmt;

fn f1() -> fmt::Result {}
fn f2() -> IoResult {}
```

使用 pub use 重新导出名称

- 使用 use 将路径（名称）导入到作用域内后，该名称在此作用域内是私有的
- pub use：重导出

将条目引入作用域

该条目可以被外部代码引入到它们的作用域

### 使用外部包（package）

1. Cargo.toml 添加依赖的包（package）

2. use 将特定条目引入作用域

3. 标准库（std）也被当做外部包

- 不需要修改 Cargo.toml 来包含std
- 需要使用 use 将 std 中的特定条目引入当前作用域

### 使用嵌套路径清理大量的 use 语句

如果使用同一个包或模块下的多个条目

可使用嵌套路径在同一行内将上述条目进行引入：

- 路径相同的部分::{路径差异的部分}

```rust
use std::io;
use std::cmp::Ordering;
use std::{cmp::Ordering, io};

// 特殊情况
use std::io;
use std::io::Write;
use std::{self, Write};
```

### 通配符 *

使用 * 可以把路径中所有的公共条目都引入到作用域

注意：谨慎使用

应用场景:

- 测试。将所有被测试代码引入到 tests 模块
- 有时被用于预导入（(prelude）模块

## 将模块拆分为不同文件

模块定义时，如果模块名后边是“;”，而不是代码块

- Rust 会从与模块同名的文件中加载内容

- 模块树的结构不会变化

随着模块逐渐变大，该技术让你可以把模块的内容移动到其它文件中
