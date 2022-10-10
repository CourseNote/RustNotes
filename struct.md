# `struct`

`struct`，结构体

- 自定义的数据类型
- 为相关联的值命名，打包成有意义的组合

## 定义 `struct`

使用 `struct` 关键字，并为整个 `struct` 命名

在花括号内，为所有字段（Field）定义名称和类型

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## 实例化 `struct`

想要使用 `struct`，需要创建 `struct` 的实例:

- 为每个字段指定具体值

- 无需按声明的顺序进行指定

```rust
let user1 = User { // 不可以省略字段
    email: String::from("acb@163.com"),
    username: String::from("wenhao"),
    active: true,
};
```

## 取得 `struct` 里面的某个值

使用点标记法

```rust
let mut user1 = User { // 必须是 mut 才可以更改值
	email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
	sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
```

**注意**

- 一旦 `struct` 的实例是可变的，那么实例中所有的字段都是可变的

- 并且**不允许部分可变，部分不可变**

## `struct` 作为函数的返回值

```rust
fn build_user(email: String, username: String) -> User {
    User {
		email: email,
		username: username,
        active: true,
		sign_in_count: 1,
    }
}
```

## 字段初始化简写

当字段名与字段值对应变量名相同时，就可以使用字段初始化简写的方式

```rust
fn build_user(email: String, username: String) -> User {
    User {
		email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## `struct` 更新语法

当你想基于某个 `struct` 实例来创建一个新实例的时候，可以使用 `struct` 更新语法

```rust
let user2 = User {
	email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
	sign_in_count: user1.sign_in_count,
};
// 使用实例更新
let user2 = User {
	email: String::from( "another@example.com"),
    username: String::from( "anotherusername567"),
    ..user1
};
```

The code in Listing 5-7 also creates an instance in `user2` that has a different value for `email` but has the same values for the `username`, `active`, and `sign_in_count` fields from `user1`. The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the `struct`’s definition.

Note that the `struct` update syntax uses `=` like an assignment; this is because it moves the data, just as we saw in the [“Ways Variables and Data Interact: Move”](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move) section. In this example, we can no longer use `user1` after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`. If we had given `user2` new `String` values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`. The types of `active` and `sign_in_count` are types that implement the `Copy` trait, so the behavior we discussed in the [“Stack-Only Data: Copy”](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) section would apply.

## Tuple `struct`

可定义类似 tuple 的 `struct`，叫做 tuple `struct`

- Tuple `struct` 整体有个名，但里面的元素没有名

- 适用：想给整个 tuple 起名，并让它不同于其它 tuple，而且又不需要给每个元素起名

定义 tuple `struct`：使用 `struct` 关键字，后边是名字，以及里面元素的类型

```rust
struct Color(i32, i32, i32);
struct Point(i32,i32,i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
// black 和 origin 是不同的类型，是不同 tuple struct 的实例
```

Note that the `black` and `origin` values are different types, because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type `Color` cannot take a `Point` as an argument, even though both types are made up of three `i32` values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a `.` followed by the index to access an individual value.

## Unit-Like `Struct`（没有任何字段）

- 可以定义没有任何字段的 `struct`，叫做 Unit-Like struct（因为与()，单元类型类似）

- 适用于需要在某个类型上实现某个 trait，但是在里面又没有想要存储的数据

## struct 数据的所有权

```rust
struct User {
	username: String,
    email: String,
	sign_in_count: u64,
    active: bool,
}
```

这里的字段使用了 `string` 而不是 `&str` 

- 该 struct 实例拥有其所有的数据
- 只要 struct 实例是有效的，那么里面的字段数据也是有效的

struct 里也可以存放引用，但这需要使用生命周期（以后讲）

- 生命周期保证只要 struct 实例是有效的，那么里面的引用也是有效的

## 实例

```rust
#[derive(Debug)]

struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let rect = Rectangle {
        w: 30,
        h: 50,
    };

    println!("{}", area(&rect));
    println!("{:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.w * rect.h
}
```

## struct 的方法

方法和函数类似：`fn` 关键字、名称、参数、返回值

方法与函数不同之处：

- 方法是在 `struct`（或 `enum`、`trait` 对象）的上下文中定义
- 第一个参数是 self，表示方法被调用的 `struct` 实例

### 定义方法

```rust
#[derive(Debug)]

struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // The &self is actually short for self: &Self.
        self.w * self.h
    }
}

fn main() {
    let rect = Rectangle {
        w: 30,
        h: 50,
    };

    println!("{}", rect.area());
    println!("{:#?}", rect);
}
```

在 `impl` 块里定义方法

方法的第一个参数可以是 `&self`，也**可以获得其所有权**或**可变借用**。和其他参数一样。

> In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`. The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for. Methods must have a parameter named `self` of type `Self` for their first parameter, so Rust lets you abbreviate this with only the name `self` in the first parameter spot. Note that we still need to use the `&` in front of the `self` shorthand to indicate this method borrows the `Self` instance, just as we did in `rectangle: &Rectangle`. Methods can take ownership of `self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably, just as they can any other parameter.

### 方法调用的运算符

- C/C++: object->something() 和 (*object).something() 一样
- Rust没有->运算符

- Rust 会自动引用或解引用
    在调用方法时就会发生这种行为

- 在调用方法时，Rust 根据情况自动添加 &、&mut 或 *，以便 object 可以匹配方法的签名

下面两行代码效果相同：

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

### 关联函数

- 可以在 `impl` 块里定义不把 `self` 作为第一个参数的函数，它们叫关联函数（不是方法)

    例如：`String::from()`

- 关联函数通常用于构造器（例子）

- `::` 符号
    - 关联函数
    - 模块创建的命名空间

```rust
#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.h > another.h && self.w > another.w
    }
    fn square(size: u32) -> Self { // 返回类型可以使用 Self
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let s = Rectangle::square(20); // 调用的时候使用::

    let rect = Rectangle {
        w: 30,
        h: 50,
    };
    let rect1 = Rectangle {
        w: 100,
        h: 200,
    };
    let rect2 = Rectangle {
        w: 90,
        h: 150,
    };

    println!("{}", rect1.can_hold(&rect2));
}
```

### 多个 impl 块

- 每个 `struct` 允许拥有多个 `impl` 块

- 看情况去做