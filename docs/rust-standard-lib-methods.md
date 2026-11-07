# Rust 常用 API 笔记：字符串与迭代器

> 主题：`String` / `&str`、迭代器适配器、`Option` / `Result`、`Vec` / slice、`mem` 的常用方法与易混点。
> 示例都是最小片段，可以直接粘进 `#[test]` 里运行。

## 1. `String` 与 `&str`

### 1.1 两个类型的关系

| 类型     | 所有权 | 说明                                     |
| -------- | ------ | ---------------------------------------- |
| `String` | 拥有   | 堆上的 UTF-8 缓冲区，可增长              |
| `&str`   | 借用   | 指向某段 UTF-8 数据的胖指针（ptr + len） |

`String` 实现了 `Deref<Target = str>`，所以 `&String` 能自动转成 `&str`：

```rust
fn takes_str(s: &str) -> usize {
    s.len()
}

let owned = String::from("hi");
takes_str(&owned);                       // &String -> &str
let slice: &str = owned.as_str();        // 显式取切片
assert_eq!(slice, "hi");
```

相关 trait：`AsRef<str>`、`Borrow<str>`，写泛型参数时常用：

```rust
fn takes_any(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

assert_eq!(takes_any("hi"), 2);
assert_eq!(takes_any(String::from("hi")), 2);
```

### 1.2 字节长度 vs 字符数量

`len()` 返回**字节数**，`chars().count()` 返回**字符数**：

```rust
let s = "héllo";
assert_eq!(s.len(), 6);           // h + é(2 字节) + l + l + o
assert_eq!(s.chars().count(), 5); // 5 个 char
```

| 表达式              | 含义               | 复杂度 |
| ------------------- | ------------------ | ------ |
| `s.len()`           | 字节数             | O(1)   |
| `s.is_empty()`      | 是否为空           | O(1)   |
| `s.chars().count()` | 字符数             | O(n)   |
| `s.capacity()`      | 已分配容量（字节） | O(1)   |

### 1.3 UTF-8 与下标

`String` **没有**实现 `Index<usize>`，不能写 `s[0]`：

```rust
let s = String::from("héllo");
// let c = s[0];              // 编译错误
let first = s.chars().next(); // Some('h')
let first_byte = s.as_bytes()[0]; // b'h'
```

切片语法 `&s[a..b]` 用的是**字节**下标，且必须落在字符边界上，否则 panic。安全版本是 `get`：

```rust
let s = "héllo";
assert_eq!(s.get(0..1), Some("h"));
assert_eq!(s.get(0..2), None);     // 2 不是字符边界
assert!(!s.is_char_boundary(2));
```

## 2. 字符串的创建与容量

| API                                | 说明                                  |
| ---------------------------------- | ------------------------------------- |
| `String::new()`                    | 空串，不分配                          |
| `String::from("...")`              | 从 `&str` 复制                        |
| `"".to_string()` / `"".to_owned()` | 同上，写法不同                        |
| `s.repeat(n)`                      | 把 `s` 重复 n 次，生成新 `String`     |
| `String::with_capacity(n)`         | 预分配至少 n **字节**                 |
| `reserve(n)` / `reserve_exact(n)`  | 额外预留 n 字节（精确版本不超额分配） |
| `shrink_to_fit()`                  | 把容量收缩到当前长度                  |
| `String::from_utf8(vec)`           | 校验并接管 `Vec<u8>`                  |
| `String::from_utf8_lossy(&[u8])`   | 非法字节替换为 U+FFFD，不报错         |
| `into_bytes()`                     | 反向拆成 `Vec<u8>`，不复制            |

```rust
let mut s = String::with_capacity(16);
assert!(s.capacity() >= 16);

s.push_str("hello");
s.reserve(32);
assert!(s.capacity() >= 37); // 5 + 32
assert_eq!(s.len(), 5);

let bytes = vec![b'h', b'i'];
let from_bytes = String::from_utf8(bytes).unwrap();
assert_eq!(from_bytes, "hi");

assert!(String::from_utf8(vec![0xC3]).is_err());        // 截断的 UTF-8
assert_eq!(String::from_utf8_lossy(&[0xC3]), "\u{FFFD}");

assert_eq!("ab".repeat(3), "ababab");
assert_eq!("".repeat(10), "");
assert_eq!("x".repeat(0), "");
```

## 3. 字符串的修改

| API                                       | 说明                                    | 复杂度    |
| ----------------------------------------- | --------------------------------------- | --------- |
| `push(char)`                              | 追加一个字符                            | 均摊 O(1) |
| `push_str(&str)`                          | 追加字符串切片                          | 均摊 O(1) |
| `extend(iter)`                            | 追加字符/字符串迭代器                   | 均摊 O(1) |
| `s += "..."`                              | 依赖 `AddAssign<&str>`                  | 均摊 O(1) |
| `insert(i, char)` / `insert_str(i, &str)` | 在字节位置插入，要求字符边界            | O(n)      |
| `replace_range(range, &str)`              | 替换字节范围                            | O(n)      |
| `truncate(len)`                           | 截断到指定字节长度，要求字符边界        | O(1)      |
| `pop()`                                   | 删除并返回最后一个字符，`Option<char>`  | O(1)      |
| `remove(i)`                               | 删除并返回位置 i 处的字符，后续字节左移 | O(n)      |
| `drain(range)`                            | 删除范围并返回迭代器                    | O(n)      |
| `retain(f)`                               | 按字符过滤                              | O(n)      |
| `clear()`                                 | 清空内容，保留容量                      | O(1)      |
| `split_off(i)`                            | 从 i 处拆成两个 `String`                | O(n)      |

```rust
let mut s = String::new();
s.push('a');
s.push_str("bc");
s.extend(['d', 'e']);
s += "fg";
assert_eq!(s, "abcdefg");

let mut s = String::from("hello world");
s.truncate(5);
assert_eq!(s, "hello");

let mut s = String::from("hello");
assert_eq!(s.pop(), Some('o'));
assert_eq!(s, "hell");
assert_eq!(s.remove(0), 'h');
assert_eq!(s, "ell");

let mut s = String::from("hello");
s.retain(|c| c != 'l');
assert_eq!(s, "heo");

let mut s = String::from("hello");
let tail = s.split_off(2);
assert_eq!(s, "he");
assert_eq!(tail, "llo");

let mut s = String::from("abcdef");
let removed: String = s.drain(1..3).collect();
assert_eq!(removed, "bc");
assert_eq!(s, "adef");
```

注意 `remove` / `insert` / `truncate` 的下标都是**字节位置**，落在多字节字符中间会 panic：

```rust
let mut s = String::from("é");
// s.remove(1);   // panic: byte index 1 is not a char boundary
```

## 4. 字符与字节遍历

| API               | 产出            | 说明                                   |
| ----------------- | --------------- | -------------------------------------- |
| `chars()`         | `char`          | 按 UTF-8 解码逐字符                    |
| `char_indices()`  | `(usize, char)` | 字符及其起始字节偏移                   |
| `bytes()`         | `u8`            | 原始字节，不做解码                     |
| `as_bytes()`      | `&[u8]`         | 零拷贝借出字节切片，可用 slice 方法    |
| `chars().rev()`   | `char`          | 反向，`Chars` 是 `DoubleEndedIterator` |
| `Chars::as_str()` | `&str`          | 返回**尚未消费**的剩余片段             |

```rust
let mut chars = "hello".chars();
assert_eq!(chars.next(), Some('h'));
assert_eq!(chars.as_str(), "ello"); // 剩余部分

let s = "hello";
assert_eq!(s.chars().rev().collect::<String>(), "olleh");
assert_eq!(s.bytes().count(), 5);

let offsets: Vec<(usize, char)> = "aé中".char_indices().collect();
assert_eq!(offsets, vec![(0, 'a'), (1, 'é'), (3, '中')]);
```

`char` 在内存里固定占 4 字节，但写入 UTF-8 字符串时只占 1~4 字节：

```rust
assert_eq!('a'.len_utf8(), 1);
assert_eq!('é'.len_utf8(), 2);
assert_eq!('中'.len_utf8(), 3);
assert_eq!(std::mem::size_of::<char>(), 4);
```

## 5. 迭代器

### 5.1 惰性

适配器只是"描述"，不消费就不执行：

```rust
let v = vec![1, 2, 3];
let iter = v.iter().map(|x| {
    println!("计算 {x}");
    x * 2
});
// 这里不会打印任何内容
let doubled: Vec<i32> = iter.collect(); // 此刻才逐个求值
assert_eq!(doubled, vec![2, 4, 6]);
```

### 5.2 常用适配器

| 分类 | 方法                                          | 说明                                       |
| ---- | --------------------------------------------- | ------------------------------------------ |
| 变换 | `map`                                         | 一对一                                     |
| 变换 | `flat_map`                                    | 一对一后展平（`map(f).flatten()` 的简写）  |
| 变换 | `flatten`                                     | 元素本身可迭代时展平                       |
| 变换 | `filter` / `filter_map`                       | 过滤 / 过滤并变换（返回 `Option`）         |
| 变换 | `inspect`                                     | 只观察，不改变元素，调试用                 |
| 截取 | `take(n)` / `take_while(f)`                   | 取前 n 个 / 取到条件不满足为止             |
| 截取 | `skip(n)` / `skip_while(f)`                   | 跳过前 n 个 / 跳过到条件不满足为止         |
| 截取 | `step_by(n)`                                  | 隔 n 个取一个                              |
| 组合 | `zip(other)`                                  | 并行配对，长度取较短者                     |
| 组合 | `chain(other)`                                | 串行拼接，产出全部                         |
| 组合 | `rev()`                                       | 反向（需 `DoubleEndedIterator`）           |
| 组合 | `enumerate()`                                 | 附带下标                                   |
| 组合 | `peekable()` / `peek()`                       | 预读                                       |
| 借用 | `by_ref()`                                    | 返回 `&mut Iterator`，用于部分消费后继续用 |
| 消费 | `collect()` / `count()` / `last()` / `nth(n)` | 收集 / 计数 / 最后一个 / 第 n 个           |
| 消费 | `max()` / `min()`                             | 极值，需 `Ord`，详见 5.8                   |
| 消费 | `fold(init, f)` / `reduce(f)` / `for_each(f)` | 累积 / 归约 / 逐项执行                     |
| 判定 | `all(f)` / `any(f)`                           | 全部满足 / 任一满足，可提前短路            |
| 比较 | `eq(other)` / `ne(other)`                     | 与另一个迭代器逐元素比较，消耗双方         |

```rust
let doubled: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
assert_eq!(doubled, vec![2, 4, 6]);

let words = ["ab", "cd"];
let letters: String = words.iter().flat_map(|w| w.chars()).collect();
assert_eq!(letters, "abcd");

let flat: Vec<i32> = vec![vec![1, 2], vec![3]].into_iter().flatten().collect();
assert_eq!(flat, vec![1, 2, 3]);

let evens: Vec<i32> = (1..10).filter(|x| x % 2 == 0).collect();
assert_eq!(evens, vec![2, 4, 6, 8]);

let parsed: Vec<i32> = ["1", "x", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
assert_eq!(parsed, vec![1, 3]);

let sum = (1..=5).fold(0, |acc, x| acc + x);
assert_eq!(sum, 15);

let desc: Vec<i32> = (1..=3).rev().collect();
assert_eq!(desc, vec![3, 2, 1]);
```

### 5.3 `zip` 与 `chain`

```rust
let pairs: Vec<(i32, &str)> = [1, 2, 3].iter().copied().zip(["a", "b"]).collect();
assert_eq!(pairs, vec![(1, "a"), (2, "b")]); // 长度取较短者

let all: Vec<i32> = [1, 2].iter().copied().chain([3, 4]).collect();
assert_eq!(all, vec![1, 2, 3, 4]); // 全部拼接
```

标准库没有 `zip_longest`。需要"取较长者"时可以用 `std::iter::from_fn` 自己写：

```rust
use std::iter::from_fn;

fn zip_longest<A, B>(a: A, b: B) -> impl Iterator<Item = (Option<A::Item>, Option<B::Item>)>
where
    A: Iterator,
    B: Iterator,
{
    let (mut a, mut b) = (a, b);
    from_fn(move || {
        let pair = (a.next(), b.next());
        (pair.0.is_some() || pair.1.is_some()).then_some(pair)
    })
}

let merged: Vec<i32> = zip_longest([1, 2].into_iter(), [10, 20, 30].into_iter())
    .flat_map(|(x, y)| x.into_iter().chain(y))
    .collect();
assert_eq!(merged, vec![1, 10, 2, 20, 30]);
```

这里用到了 `Option<T>: IntoIterator`：`Some(x)` 产出 1 个元素，`None` 产出 0 个。所以 `Option` 也能直接传给 `extend` / `chain` / `flatten`。

### 5.4 `skip` / `take` / `by_ref`

```rust
let picked: Vec<i32> = (1..=10).skip(3).take(2).collect();
assert_eq!(picked, vec![4, 5]);

let third = (1..=10).nth(2);
assert_eq!(third, Some(3));

let mut it = 1..=5;
let head: Vec<i32> = it.by_ref().take(2).collect();
let rest: Vec<i32> = it.collect();
assert_eq!(head, vec![1, 2]);
assert_eq!(rest, vec![3, 4, 5]);
```

### 5.5 `collect` 的类型由目标决定

```rust
let chars: Vec<char> = "hello".chars().collect();
assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);

let s: String = ['h', 'i'].into_iter().collect();
assert_eq!(s, "hi");

// 元素是 Result 时可以整体收集
let parsed: Result<Vec<i32>, _> = ["1", "2"].iter().map(|x| x.parse::<i32>()).collect();
assert_eq!(parsed, Ok(vec![1, 2]));
```

`String` 实现的是 `FromIterator<char>` / `FromIterator<&str>` / `FromIterator<String>`；`Vec<T>` 实现的是 `FromIterator<T>`。

### 5.6 一个容易踩的坑

两个迭代器放进同一个元组里 `next()` 时，**两边都会被调用**：

```rust
let (mut a, mut b) = ([1, 2].into_iter(), [10, 20, 30].into_iter());

// 错误：第三轮 a 返回 None、b 返回 Some(30)，30 已被消费并丢弃
while let (Some(x), Some(y)) = (a.next(), b.next()) {
    println!("{x} {y}");
}
```

正确做法是让 `None` 单独成支：

```rust
let (mut a, mut b) = ([1, 2].into_iter(), [10, 20, 30].into_iter());

loop {
    match (a.next(), b.next()) {
        (None, None) => break,
        (x, y) => {
            if let Some(x) = x {
                print!("{x} ");
            }
            if let Some(y) = y {
                print!("{y} ");
            }
        }
    }
}
```

### 5.7 `all` / `any` / `eq`：判定与比较

```rust
let bytes: Vec<u8> = "12345".bytes().collect();
assert!(bytes.iter().all(u8::is_ascii_digit));
assert!(bytes.iter().any(|b| *b == b'3'));

// 与另一个迭代器逐元素比较，双方都会被消费
assert!("hello".bytes().eq("hello".bytes()));
assert!("abc".bytes().ne("abd".bytes()));

// 空迭代器：all 恒为 true，any 恒为 false
assert!([0u8; 0].iter().all(|_| false));
assert!(![0u8; 0].iter().any(|_| true));
```

`eq` / `ne` 要求元素实现 `PartialEq`；只要 `Item` 类型相同，不同来源的迭代器可以互相比较。

### 5.8 极值：`max` / `min` 与 `copied`

`max()` / `min()` 消费整个迭代器，返回 `Option<Self::Item>`，要求元素实现 `Ord`；空迭代器返回 `None`。因为 `iter()` 的元素是 `&T`，所以得到的是 `Option<&T>`：

```rust
let v = vec![2, 3, 5, 1, 3];
assert_eq!(v.iter().max(), Some(&5));
assert_eq!(v.iter().min(), Some(&1));

let m = match v.iter().max() {
    Some(&m) => m,      // 把 i32 按值拷贝出来
    None => return,     // 在函数内处理空迭代器
};
assert_eq!(m, 5);

assert_eq!(v.iter().copied().max(), Some(5)); // 先拷贝，得到 Option<i32>
assert_eq!(v.into_iter().max(), Some(5));     // 所有权版本，v 被消费
assert_eq!(Vec::<i32>::new().iter().max(), None); // 空迭代器
```

取出 `Option<&T>` 里的值有两种惯用方式：`&` 模式解构（要求 `T: Copy`），或先用 `copied()` / `cloned()` 转成 `Option<T>`。非 `Copy` 类型直接绑定引用即可：

```rust
let s = vec![String::from("a"), String::from("bb")];
let longest = s.iter().max_by_key(|s| s.len()); // Option<&String>
assert_eq!(longest, Some(&String::from("bb")));
```

自定义比较：

| 方法                              | 说明                                   |
| --------------------------------- | -------------------------------------- |
| `max_by(f)` / `min_by(f)`         | 用比较函数，返回 `Ordering`            |
| `max_by_key(f)` / `min_by_key(f)` | 按投影出的键比较，键对每个元素只算一次 |

```rust
let v: Vec<i32> = vec![1, -2, 3];
assert_eq!(v.iter().max_by(|a, b| a.cmp(b)), Some(&3));
assert_eq!(v.iter().copied().min_by_key(|&x| x.abs()), Some(1));
```

复杂度：O(n) 时间，O(1) 额外空间（只保留当前最优元素与键）；`copied()` / `cloned()` 每个元素 O(1)。并列时 `max` 取最后一个最大值，`min` 取第一个最小值。

## 6. `Option` 与 `Result`

### 6.1 `Option`

| API                                                          | 说明                                |
| ------------------------------------------------------------ | ----------------------------------- |
| `is_some()` / `is_none()`                                    | 判断                                |
| `is_some_and(f)`                                             | 有值且满足条件                      |
| `map(f)`                                                     | 对 `Some` 做变换，`None` 原样传递   |
| `and_then(f)`                                                | 变换后可能仍为 `Option`（避免嵌套） |
| `filter(f)`                                                  | 值不满足条件时变 `None`             |
| `or(other)` / `or_else(f)`                                   | 为 `None` 时提供后备                |
| `take()`                                                     | 取出值，原位置留下 `None`           |
| `unwrap()` / `expect(msg)`                                   | 取值，`None` 时 panic               |
| `unwrap_or(v)` / `unwrap_or_else(f)` / `unwrap_or_default()` | 提供默认值                          |
| `ok_or(e)` / `ok_or_else(f)`                                 | 转成 `Result`，便于用 `?`           |

```rust
let x = Some(3);
assert_eq!(x.map(|v| v * 2), Some(6));
assert_eq!(x.filter(|v| *v > 5), None);
assert_eq!(x.and_then(|v| (v > 0).then_some(v)), Some(3));
assert_eq!(x.unwrap_or(0), 3);
assert_eq!(None::<i32>.unwrap_or_default(), 0);

let mut slot = Some(String::from("hi"));
let taken = slot.take();
assert_eq!(taken, Some(String::from("hi")));
assert_eq!(slot, None);

let missing: Option<i32> = None;
let result: Result<i32, &str> = missing.ok_or("missing");
assert_eq!(result, Err("missing"));
```

### 6.2 `Result`

| API                                  | 说明                                              |
| ------------------------------------ | ------------------------------------------------- |
| `is_ok()` / `is_err()`               | 判断                                              |
| `map(f)` / `map_err(f)`              | 变换成功值 / 错误值                               |
| `and_then(f)`                        | 链式处理，失败短路                                |
| `unwrap()` / `expect(msg)`           | 取值，失败 panic                                  |
| `unwrap_or(v)` / `unwrap_or_else(f)` | 提供默认值                                        |
| `?` 运算符                           | 失败时提前返回（函数需返回 `Result` 或 `Option`） |

```rust
let n: i32 = "42".parse().unwrap();
assert_eq!(n, 42);
assert!("abc".parse::<i32>().is_err());

fn double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

assert_eq!(double("21"), Ok(42));
```

## 7. `Vec` 与 slice

| API                                          | 说明                                 |
| -------------------------------------------- | ------------------------------------ |
| `Vec::new()` / `Vec::with_capacity(n)`       | 创建，后者预分配                     |
| `vec![...]` / `vec![v; n]`                   | 由元素创建 / 重复 n 个（需 `Clone`） |
| `push(v)` / `pop()`                          | 尾部入栈 / 出栈                      |
| `extend(iter)` / `extend_from_slice(&[T])`   | 追加迭代器 / 切片                    |
| `v[i]`（`Index` / `IndexMut`）               | 直接下标，越界 panic                 |
| `get(i)` / `get_mut(i)`                      | 安全下标，越界返回 `None`            |
| `first()` / `last()`（及 `_mut` 版本）       | 首 / 尾元素，`Option<&T>`            |
| `as_slice()`                                 | 借出 `&[T]`                          |
| `clone()` / `to_vec()`                       | 深拷贝成新的 `Vec<T>`，O(n)          |
| `chunks(n)` / `chunks_exact(n)`              | 连续分块；exact 丢弃不足 n 的尾块    |
| `rchunks(n)` / `windows(n)`                  | 反向分块 / 定长滑动窗口              |
| `swap(i, j)`                                 | 交换两个元素，O(1)                   |
| `len()` / `is_empty()` / `capacity()`        | 长度 / 判空 / 容量                   |
| `truncate(n)` / `retain(f)` / `drain(range)` | 截断 / 过滤 / 删除范围               |
| `into_iter()` / `iter()` / `iter_mut()`      | 三种遍历方式                         |

```rust
let mut v: Vec<i32> = Vec::with_capacity(4);
assert!(v.capacity() >= 4);

v.push(1);
v.extend([2, 3]);
v.extend_from_slice(&[4, 5]);
assert_eq!(v.len(), 5);
assert_eq!(v.get(10), None);
assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);

if let Some(first) = v.get_mut(0) {
    *first = 100;
}
assert_eq!(v[0], 100);

let data = [1, 2, 3, 4, 5];
let blocks: Vec<&[i32]> = data.chunks(2).collect();
assert_eq!(blocks, vec![&data[0..2], &data[2..4], &data[4..5]]);

let exact: Vec<&[i32]> = data.chunks_exact(2).collect();
assert_eq!(exact, vec![&data[0..2], &data[2..4]]); // 尾块 [5] 被丢弃

let mut v = vec![1, 2, 3];
v.swap(0, 2);
assert_eq!(v, vec![3, 2, 1]);
```

下标索引（`Index` / `IndexMut`），越界会 panic：

```rust
let mut v = vec![10, 20, 30];
assert_eq!(v[0], 10);
v[1] = 25;                        // IndexMut，原地修改
assert_eq!(v[1], 25);
// v[3];                          // panic: index out of bounds

assert_eq!(v.first(), Some(&10)); // 不确定是否为空时更安全
assert_eq!(v.last(), Some(&30));
assert_eq!(Vec::<i32>::new().first(), None);
```

### 7.1 遍历方式

| 写法                                 | 元素类型      | 说明                          |
| ------------------------------------ | ------------- | ----------------------------- |
| `for x in v.iter()`                  | `&T`          | 借用，`v` 仍可用              |
| `for x in &v`                        | `&T`          | `for` 自动调用 `iter()`       |
| `for &x in &v`                       | `T`           | 模式解构，要求 `T: Copy`      |
| `for x in v`                         | `T`           | 所有权移动，之后 `v` 不可再用 |
| `for i in 0..v.len()`                | `usize`       | 下标循环，每次 `v[i]` 是 O(1) |
| `for (i, x) in v.iter().enumerate()` | `(usize, &T)` | 带下标的惯用写法              |

```rust
let v = vec![1, 2, 3];

for x in v.iter() { /* x: &i32 */ }
for &x in &v { /* x: i32 */ }
// for x in v { /* 消费 v，之后不能再使用 */ }

for i in 0..v.len() {
    let x = v[i]; // 每次下标访问 O(1)
}

for (i, x) in v.iter().enumerate() { /* i: usize, x: &i32 */ }
```

空 `Vec` 的 `0..v.len()` 是空范围，循环体不会执行；但直接写 `v[0]` 仍会 panic，需先判空或用 `first()`。含右端点的区间写作 `0..=n`。

`&str` 本身没有 `chunks`，先 `as_bytes()` 借出 `&[u8]` 再分块：

```rust
assert!("abab".as_bytes().chunks_exact(2).all(|b| b == b"ab"));
```

交换任意两个变量（不限类型）用 `std::mem::swap`；同类还有 `mem::take`（取出并留下 `T::default()`）与 `mem::replace`（取出并留下指定值）：

```rust
let (mut a, mut b) = ("left", "right");
std::mem::swap(&mut a, &mut b);
assert_eq!((a, b), ("right", "left"));

let mut s = String::from("hello");
let taken = std::mem::take(&mut s); // 要求 T: Default
assert_eq!(taken, "hello");
assert_eq!(s, "");

let mut n = 5;
assert_eq!(std::mem::replace(&mut n, 0), 5);
assert_eq!(n, 0);
```

### 7.2 克隆与复制：`clone` / `to_vec` / `Copy`

| 写法                                | 说明                           | 复杂度               |
| ----------------------------------- | ------------------------------ | -------------------- |
| `v.clone()`                         | 克隆整个 `Vec`，元素逐个克隆   | O(n) 时间，O(n) 空间 |
| `slice.to_vec()`                    | 由 `&[T]` 生成新的 `Vec<T>`    | O(n) 时间，O(n) 空间 |
| `slice.to_owned()`                  | 同上，`ToOwned` 的惯用写法     | O(n) 时间，O(n) 空间 |
| `s.clone()` / `s.to_string()`       | 复制字符串内容（`String`）     | O(n) 时间，O(n) 空间 |
| `Rc::clone(&rc)` / `Arc::clone(&a)` | 只增加引用计数，不复制内部数据 | O(1)                 |

```rust
let a = vec![1, 2, 3];
let b = a.clone(); // 深拷贝，a 仍然可用
assert_eq!(a, b);

let s: &[i32] = &a[..];
let c = s.to_vec(); // 切片转成新的 Vec
assert_eq!(c, a);

let d = vec![String::from("hi")];
let mut e = d.clone(); // 元素本身也会被克隆，两块内存互不影响
e[0].push_str("!");
assert_eq!(d[0], "hi");
```

`clone()` 是显式的「制作副本」，要求 `T: Clone`。`Copy` 类型（`i32`、`bool`、`char`、`&T` 等）赋值时直接按位复制，不需要 `clone()`；非 `Copy` 类型（`Vec`、`String`、`Box` 等）赋值默认是**移动**，原变量随后不可用：

```rust
let a = vec![1, 2, 3];
let b = a; // 移动：a 不再可用
// println!("{:?}", a); // 编译错误：use of moved value

let x = 1;
let y = x; // Copy：x 仍然可用
assert_eq!(x, y);
```

注意 `clone()` 不一定深拷贝，语义由类型实现决定：`Vec` / `String` 复制堆数据，`Rc` / `Arc` 只是增加引用计数。需要从切片构造拥有所有权的副本时，优先用 `to_vec()` / `to_owned()` 表达意图。

## 8. 复杂度速查

| 操作                                                       | 复杂度                   |
| ---------------------------------------------------------- | ------------------------ |
| `String::len` / `is_empty` / `capacity`                    | O(1)                     |
| `String::chars().count()`                                  | O(n)                     |
| `String::push` / `push_str` / `extend`                     | 均摊 O(1)                |
| `String::remove(i)` / `insert(i)` / `insert_str(i)`        | O(n)                     |
| `String::replace_range` / `drain` / `retain` / `split_off` | O(n)                     |
| `String::truncate` / `pop` / `clear`                       | O(1)                     |
| `String::from_utf8`                                        | O(n) 校验，不复制缓冲    |
| `String::into_bytes` / `as_bytes` / `as_str`               | O(1)                     |
| `Vec::push`                                                | 均摊 O(1)                |
| `Vec::len` / `is_empty` / `capacity`                       | O(1)                     |
| `Vec` 下标 `v[i]` / `first()` / `last()`                   | O(1)                     |
| `vec![...]`                                                | O(n)，一次分配           |
| `Vec::clone` / `slice::to_vec` / `to_owned`                | O(n) 时间，O(n) 空间     |
| `Rc::clone` / `Arc::clone`                                 | O(1)，仅增加引用计数     |
| `slice::get` / `Vec::get`                                  | O(1)                     |
| `Iterator::count` / `nth(n)` / `skip(n)`                   | O(n)                     |
| `zip` / `flat_map` / `chain` / `filter`                    | 每个元素 O(1)            |
| `collect()`                                                | O(输出长度) + 可能的扩容 |
| `str::repeat(n)`                                           | O(n·len)，一次分配       |
| `slice::chunks` / `chunks_exact` / `windows`               | 每产生一个块 O(1)        |
| `Iterator::all` / `any` / `eq`                             | O(n)，可提前短路         |
| `Iterator::max` / `min` / `max_by` / `max_by_key`          | O(n)，O(1) 额外空间      |
| `Iterator::copied` / `cloned`                              | 每个元素 O(1)            |
| `slice::swap` / `std::mem::swap`                           | O(1)                     |

## 9. 易混点与陷阱

| 容易混淆                                          | 区别                                              |
| ------------------------------------------------- | ------------------------------------------------- |
| `len()` vs `chars().count()`                      | 字节数 O(1) / 字符数 O(n)，非 ASCII 不相等        |
| `s[i]` vs `s.chars().nth(i)`                      | 前者不合法；后者按下标取字符，O(n)                |
| `get(range)` vs `&s[range]`                       | 前者返回 `Option<&str>`，越界或非边界返回 `None`  |
| `remove` vs `pop`                                 | 按下标删 O(n) / 删末尾 O(1)                       |
| `remove` vs `truncate`                            | 返回被删字符 / 直接截断无返回值                   |
| `push` vs `push_str` vs `extend` vs `+=`          | 单个 `char` / `&str` / 迭代器 / `AddAssign<&str>` |
| `with_capacity` vs `reserve`                      | 创建时分配 / 对已有容器追加容量                   |
| `map` vs `flat_map` vs `flatten`                  | 一对一 / 映射后展平 / 展平已有可迭代元素          |
| `zip` vs `chain`                                  | 并行配对取短 / 串行拼接取全部                     |
| `skip(n)` vs `take(n)` vs `nth(n)`                | 跳过 n / 取前 n / 取第 n 个（并消费前面所有元素） |
| `collect::<String>()` vs `collect::<Vec<char>>()` | 逐字符拼接 / 每个字符固定占 4 字节                |
| `unwrap` vs `expect`                              | 都 panic，后者可带自定义信息                      |
| `min(a, b)` vs `a.min(b)`                         | 前者需 `use std::cmp::min`；后者是 `Ord` 方法     |
| `&String` vs `&str`                               | `Deref` 让 `&String` 可直接当 `&str` 传参         |
| `chunks(n)` vs `chunks_exact(n)`                  | 保留不足 n 的尾块 / 丢弃尾块                      |
| `v.swap(i, j)` vs `std::mem::swap`                | 交换同一 slice 的两个元素 / 交换任意两个 `&mut`   |
| `v[i]` vs `v.get(i)`                              | 越界 panic / 返回 `Option<&T>`                    |
| `v[0]` vs `v.first()`                             | 空 Vec panic / 返回 `None`                        |
| `v.clone()` vs `let b = v`                        | 显式复制、`v` 仍可用 / 非 `Copy` 类型是移动       |
| `clone()` vs `Copy` 隐式复制                      | 需 `T: Clone`、可能分配 / 按位复制、不额外分配    |
| `v.iter().max()` vs `v.into_iter().max()`         | 返回 `Option<&T>` / 返回 `Option<T>`              |
| `Iterator::max()` vs `Ord::max()`                 | 迭代器极值，返回 `Option` / 两个值取较大者        |
| `max()` vs `max_by_key(f)`                        | 直接比较元素 / 按投影出的键比较                   |
| `for i in 0..v.len()` vs `enumerate()`            | 下标循环 / 惯用的带下标遍历                       |
| `all` vs `any`（空迭代器）                        | `all` 恒为 true，`any` 恒为 false                 |
| `repeat(n)` vs 循环 `push_str`                    | 一次算出长度并分配 / 可能多次扩容                 |

常见陷阱：

- 用字节下标修改字符串前，先确认 `is_char_boundary`，否则 `remove` / `insert` / `truncate` 会 panic。
- `while let (Some(a), Some(b)) = (it1.next(), it2.next())` 会吞掉先耗尽那一轮中另一侧的字符。
- 迭代器是惰性的，没有消费动作（`collect`、`for`、`count` 等）就不会执行。
- `assert_eq!(String, &str)` 能编译，是因为标准库为 `String` 和 `&str` 之间实现了跨类型 `PartialEq`。
- `String::new()` 是 `const fn`，不分配；`String::with_capacity` 才会分配。
- `Vec::with_capacity` / `String::with_capacity` 只保证"至少"，实际容量可能更大。
- `chunks(0)` / `chunks_exact(0)` / `windows(0)` 会 panic，块大小必须 ≥ 1。
- `all` / `any` / `eq` / `count` 等都会消费迭代器，调用后不能再使用。
- `iter().max()` / `iter().min()` 在空迭代器上返回 `None`，直接 `unwrap()` 会 panic。
- `iter()` 的元素是 `&T`：`max()` 得到 `Option<&T>`，取出按值用 `&` 模式解构（要求 `Copy`）或 `copied()` / `cloned()`。
- `v[0]` / `v[i]` 越界会 panic；不确定是否为空时用 `first()` / `get()`。
- `v.len() - 1` 在空 `Vec` 上会下溢 panic（debug 构建），边界计算先 `is_empty()` 判空，或改写成 `i + 1 == v.len()` 这类不易越界的形式。
- `clone()` 是否为深拷贝取决于类型实现：`Vec` / `String` 会复制堆数据，`Rc` / `Arc` 只增加引用计数。

## 10. 测试常用

| API                                                     | 说明                                 |
| ------------------------------------------------------- | ------------------------------------ |
| `#[cfg(test)]`                                          | 仅在 `cargo test` 时编译该模块       |
| `#[test]`                                               | 标记测试函数                         |
| `#[should_panic]` / `#[should_panic(expected = "...")]` | 断言会 panic，可匹配信息             |
| `assert!(cond)`                                         | 布尔断言                             |
| `assert_eq!(a, b)`                                      | 相等断言，要求 `PartialEq` + `Debug` |
| `assert_ne!(a, b)`                                      | 不等断言                             |
| `#[ignore]`                                             | 默认跳过该测试                       |

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn len_counts_bytes() {
        assert_eq!("héllo".len(), 6);
    }

    #[test]
    #[should_panic]
    fn remove_out_of_bounds_panics() {
        let mut s = String::from("hi");
        s.remove(9);
    }
}
```

子模块可以通过 `super::` 访问父模块的项（包括私有项），通常用 `use super::*;` 简化路径。

## 11. 参考

- [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [`str` primitive](https://doc.rust-lang.org/std/primitive.str.html)
- [`std::str::Chars`](https://doc.rust-lang.org/std/str/struct.Chars.html)
- [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [`std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`std::iter::FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)
- [`std::ops::Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)
- [`std::ops::Range`](https://doc.rust-lang.org/std/ops/struct.Range.html)
- [`itertools::Itertools`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
