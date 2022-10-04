# Trait

Trait 告诉 Rust 编译器：

某种类型具有哪些可以与其它类型共享的功能。

Trait：抽象的定义共享行为

Trait bounds（约束）：泛型类型参数指定为实现了特定行为的类型

Trait 与其它语言的接口（interface）类似，但有些区别

## 定义一个 Trait

Trait 的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为

关键字：trait

只有方法签名，没有具体实现

trait 可以有多个方法：每个方法签名占一行，以 `;` 结尾

实现该 trait 的类型必须提供具体的方法实现

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
}

fn main() {
    println!("Hello, world!");
}
```

## 在类型上实现 trait

与为类型实现方法类似

不同之处：

- `impl Xxxx for Tweet { ...}`
- 在 impl 的块里，需要对 Trait 里的方法签名进行具体的实现

```rust
// lib.rs
pub trait Summary {
    fn summarize(&self) -> String; // 可以添加默认实现
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

```rust
// main.rs
// use Trait::Summary;
use Trait::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}
```

## 实现 trait 的约束

可以在某个类型上实现某个 trait 的前提条件是：

这个类型或这个 trait 是在本地 crate 里定义的

无法为外部类型来实现外部的 trait：

- 这个限制是程序属性的一部分（也就是一致性）

- 更具体地说是孤儿规则：之所以这样命名是因为父类型不存在

- 此规则确保其他人的代码不能破坏您的代码，反之亦然

- 如果没有这个规则，两个 crate 可以为同一类型实现同一个 trait，Rust 就不知道应该使用哪个实现了

## 默认实现

可以在定义 trait 时给方法赋予默认实现

默认实现的方法可以调用 trait 中其它的方法，即使这些方法没有默认实现

注意：无法从方法的重写实现里面调用默认的实现

## Trait 作为参数

`impl Trait` 语法：适用于简单情况

```rust
pub fn notify(item: impl Summary) { // 参数item必须实现了Summary方法
    println!("Breaking news! {}", item.summarize());
}
```

`Trait bound` 语法：可用于复杂情况

```rust
pub fn notify<T: Summary>(item: T) { // 参数item实现了Summary方法
    println!("Breaking news! {}", item.summarize());
}
```

`impl Trait` 语法是 `Trait bound` 的语法糖

使用 ＋ 指定多个 Trait bound

```rust
pub fn f1(item1: impl Summary + Display) {
	// item1 必须实现Summary和Display两个方法
}

pub fn f2<T: Summary + Display>(item1: T) {
	// 
}
```

`Trait bound` 使用 `where` 子句

```rust
pub fn f3<T, U>(item1: T, item2: U) 
    where T: Summary + Display, U: Clone + Debug {
    // T实现Summary和Display
    // U实现Clone和Debug
}
```

#### 实现 Trait 作为返回类型

`impl Trait` 语法

注意：impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错

```rust
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    
    largest // 这里是在返回值，也可以返回引用
}
```

#### 使用 Trait Bound 有条件的实现方法

在使用泛型类型参数的 `impl` 块上使用 Trait bound，我们可以有条件的为实现了特定 Trait 的类型来实现方法

也可以为实现了其它 Trait 的任意类型有条件的实现某个 Trait

为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> { // new函数都会实现
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> { // 满足条件才行
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}
```

