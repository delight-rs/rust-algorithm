# Rust 常用 API 笔记：字符串、迭代器与集合

> 主题：`String` / `&str`、字节缓冲与 ASCII 处理、文本分割与格式化、迭代器适配器、`Option` / `Result`、`Vec` / slice、排序与查找、`HashMap` / `HashSet` 与 `BTreeMap` / `BTreeSet`、数值位运算与类型转换、`Rc` / `RefCell`、`mem` 的常用方法与易混点。
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

`as_ref()`、`as_str()` 与 `Deref` 都只借出指针，O(1) 时间与 O(1) 额外空间。

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

范围写法与 `Vec` 一样，`..` 表示省略一端，但两端仍必须是字符边界：

```rust
let s = "héllo";
assert_eq!(&s[..1], "h");
assert_eq!(&s[1..], "éllo");
assert_eq!(s.get(..1), Some("h"));
assert_eq!(s.get(1..), Some("éllo"));
assert_eq!(s.get(..2), None); // 2 不是字符边界
```

`&s[a..b]` 在越界、`a > b` 或端点不是字符边界时 panic；不确定时用 `get`，它返回 `Option<&str>`。切片的 `Index` / `get` 都是 O(1) 时间与空间，不复制字符。

### 1.4 前缀与后缀

| API                         | 作用                          | 时间 | 空间 |
| --------------------------- | ----------------------------- | ---- | ---- |
| `s.starts_with(pat)`        | 判断是否以 `pat` 开头         | O(m) | O(1) |
| `s.ends_with(pat)`          | 判断是否以 `pat` 结尾         | O(m) | O(1) |
| `s.strip_prefix(pat)`       | 去掉前缀，返回 `Option<&str>` | O(m) | O(1) |
| `s.strip_suffix(pat)`       | 去掉后缀，返回 `Option<&str>` | O(m) | O(1) |
| `s.trim_start_matches(pat)` | 反复去掉匹配的前缀            | O(n) | O(1) |
| `s.trim_end_matches(pat)`   | 反复去掉匹配的后缀            | O(n) | O(1) |

`m` 是模式长度，`n` 是字符串字节数。模式可以是 `char`、`&str` 或闭包；`strip_*` 返回的是原串的借用片段，不复制内容：

```rust
let s = "hello.rs";
assert!(s.starts_with("hell"));
assert!(s.ends_with(".rs"));
assert_eq!(s.strip_prefix("hell"), Some("o.rs"));
assert_eq!(s.strip_suffix(".rs"), Some("hello"));
assert_eq!(s.strip_prefix("hi"), None);

let padded = "xxxabcxxx";
assert_eq!(padded.trim_start_matches('x'), "abcxxx");
assert_eq!(padded.trim_end_matches('x'), "xxxabc");
```

`trim_start_matches` / `trim_end_matches` 会反复剥离，直到不再匹配为止；只去掉固定前缀时 `strip_prefix` 更明确。

### 1.5 比较字符串

| API                         | 作用                             | 时间 | 空间 |
| --------------------------- | -------------------------------- | ---- | ---- |
| `a == b` / `a != b`         | `PartialEq` 比较，按字节逐位比较 | O(n) | O(1) |
| `a.eq(b)` / `a.ne(b)`       | `PartialEq` 的方法形式           | O(n) | O(1) |
| `a.eq_ignore_ascii_case(b)` | ASCII 范围忽略大小写比较         | O(n) | O(1) |

`String`、`&str` 之间实现了跨类型 `PartialEq`，所以 `String == &str`、`&str == String` 都能直接写；需要方法形式时用 `as_str()` 明确接收者。字符串也实现 `Ord`，字典序 `cmp` 见 9.3，`sort` 见 9.1：

```rust
let a = String::from("abc");
assert!(a == "abc");
assert!("abc".eq(a.as_str()));
assert!(!a.as_str().eq("ab"));
assert!("Hello".eq_ignore_ascii_case("hello"));
assert!("apple".cmp("banana").is_lt());
```

比较按 UTF-8 字节进行，时间与字节长度成正比；`eq_ignore_ascii_case` 只处理 ASCII 大小写，非 ASCII 字符按原样比较。

## 2. 字符串的创建与容量

| API                               | 说明                                  | 时间 / 空间                          |
| --------------------------------- | ------------------------------------- | ------------------------------------ |
| `String::new()`                   | 空串，不分配                          | O(1) / O(1)                          |
| `String::from("...")`             | 从 `&str` 复制                        | O(n) / O(n)                          |
| `"".to_string()` / `to_owned()`   | 同上，写法不同                        | O(n) / O(n)                          |
| `s.repeat(k)`                     | 把 `s` 重复 k 次，生成新 `String`     | O(输出字节) / O(输出字节)            |
| `String::with_capacity(n)`        | 预分配至少 n **字节**                 | O(1) / O(n)                          |
| `reserve(n)` / `reserve_exact(n)` | 额外预留 n 字节（精确版本不超额分配） | 需扩容时 O(n) 复制，否则 O(1) / O(n) |
| `shrink_to_fit()`                 | 把容量收缩到当前长度                  | 需重分配时 O(n) / O(n)               |
| `String::from_utf8(vec)`          | 校验并接管 `Vec<u8>`                  | O(n) 校验 / 成功时无额外分配         |
| `String::from_utf8_lossy(&[u8])`  | 非法字节替换为 U+FFFD，不报错         | O(n) / 可能 O(n)                     |
| `into_bytes()`                    | 反向拆成 `Vec<u8>`，不复制            | O(1) / O(1)                          |
| `String::from_iter(iter)`         | 从字符或字符串迭代器构造              | O(输出字节) / O(输出字节)            |

`n` 表示结果或接收者的字节数；`repeat(k)` 的 k 是重复次数。`format!` 与 `to_string` 每次调用都会分配新 `String`，热路径里优先复用 `push_str`。

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

### 2.1 与字节缓冲互转

`String` 与 `Vec<u8>` 可以**零拷贝**地互相转换，只是方向不同：

| API                                | 说明                                              | 复杂度                  |
| ---------------------------------- | ------------------------------------------------- | ----------------------- |
| `s.into_bytes()`                   | 消费 `String`，返回底层 `Vec<u8>`（容量一并带走） | O(1)，不复制            |
| `String::from_utf8(vec)`           | 接管 `Vec<u8>`，并校验 UTF-8                      | O(n) 校验，成功时不复制 |
| `String::from_utf8_unchecked(vec)` | 跳过校验，由调用方保证合法（`unsafe`）            | O(1)                    |
| `String::from_utf8_lossy(&[u8])`   | 非法序列替换为 U+FFFD，返回 `Cow<str>`            | O(n)，可能分配          |
| `err.into_bytes()`                 | 从 `FromUtf8Error` 取回原始 `Vec<u8>`             | O(1)                    |

```rust
let mut bytes = String::from("hello").into_bytes(); // 复用原缓冲区
bytes[0] = b'H';
assert_eq!(String::from_utf8(bytes).unwrap(), "Hello");

let err = String::from_utf8(vec![0xC3]).unwrap_err();
assert_eq!(err.into_bytes(), vec![0xC3]); // 失败也能拿回原数据
```

`from_utf8` 的 O(n) 只是校验扫描；数据确定是 ASCII 时开销很小。不要为了省这一次扫描而默认使用 `unsafe`；如果确实需要，先 `debug_assert!(vec.is_ascii())` 并在注释中说明不变量。

## 3. 字符串的修改

| API                                       | 说明                                    | 复杂度                   |
| ----------------------------------------- | --------------------------------------- | ------------------------ |
| `push(char)`                              | 追加一个字符                            | 均摊 O(1)                |
| `push_str(&str)`                          | 追加字符串切片                          | 摊销 O(m)，最坏 O(n + m) |
| `extend(iter)`                            | 追加字符/字符串迭代器                   | 摊销 O(m)，最坏 O(n + m) |
| `s += "..."`                              | 依赖 `AddAssign<&str>`                  | 摊销 O(m)，最坏 O(n + m) |
| `insert(i, char)` / `insert_str(i, &str)` | 在字节位置插入，要求字符边界            | O(n)                     |
| `replace_range(range, &str)`              | 替换字节范围                            | O(n)                     |
| `truncate(len)`                           | 截断到指定字节长度，要求字符边界        | O(1)                     |
| `pop()`                                   | 删除并返回最后一个字符，`Option<char>`  | O(1)                     |
| `remove(i)`                               | 删除并返回位置 i 处的字符，后续字节左移 | O(n)                     |
| `drain(range)`                            | 删除范围并返回迭代器                    | O(n)                     |
| `retain(f)`                               | 按字符过滤                              | O(n)                     |
| `clear()`                                 | 清空内容，保留容量                      | O(1)                     |
| `split_off(i)`                            | 从 i 处拆成两个 `String`                | O(n)                     |

`m` 表示追加的字节数；容量不足时 `push_str` / `extend` / `+=` 需要重新分配并复制已有内容，因此最坏为 O(n + m)。

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

这些迭代器构造都是 O(1)，完整遍历为 O(n) 时间与 O(1) 额外空间；`as_bytes()` 只是借出切片，O(1)。

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

### 4.1 ASCII 判定与大小写

以下方法只处理 **ASCII 子集**，复杂度均为 O(1)；对非 ASCII 字符返回 `false` 或原样返回：

| API                                               | 说明                           |
| ------------------------------------------------- | ------------------------------ |
| `c.is_ascii()`                                    | 是否落在 `0x00..=0x7F`         |
| `c.is_ascii_alphabetic()` / `is_ascii_digit()`    | 字母 / 数字                    |
| `c.is_ascii_alphanumeric()`                       | 字母或数字                     |
| `c.is_ascii_lowercase()` / `is_ascii_uppercase()` | 小写 / 大写                    |
| `c.is_ascii_whitespace()`                         | 空格、`\t`、`\n`、`\r`、`\x0c` |
| `c.to_ascii_lowercase()` / `to_ascii_uppercase()` | 只转换 ASCII，其余原样返回     |
| `c.eq_ignore_ascii_case(&other)`                  | ASCII 范围忽略大小写比较       |

`u8` 也有同名方法，扫描字节序列时直接对 `u8` 判断即可，不必先转成 `char`。

需要完整 Unicode 大小写时用 `to_lowercase()` / `to_uppercase()`：它们返回迭代器，一个字符可能展开成多个（如 `ß`），时间 O(字符数)，收集时通常需要分配。

多分支判定可以用 `matches!` 宏，比一串 `if` 更紧凑：

```rust
fn is_hex(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
}

assert!(is_hex('f'));
assert!(!is_hex('g'));
```

### 4.2 按字符还是按字节

| 方式                       | 前提                              | 时间 | 额外空间         | 说明                                |
| -------------------------- | --------------------------------- | ---- | ---------------- | ----------------------------------- |
| `Vec<char>` + `collect()`  | 任意 UTF-8                        | O(n) | O(4n)            | 每个 `char` 固定 4 字节，逻辑最直观 |
| `into_bytes()` / `Vec<u8>` | 只操作 ASCII，或只交换 ASCII 字节 | O(n) | O(1)，复用缓冲区 | 省去解码/编码，速度和内存更优       |
| `as_bytes()`               | 只读分析                          | O(n) | O(1)             | 借出字节切片，不能直接修改          |

只交换 ASCII 字节是安全的：多字节序列的每个字节都 ≥ 0x80，不会被 ASCII 判定命中，也不会被移动，因此结果仍是合法 UTF-8。若输入不保证 ASCII、又需要按字符语义处理，优先选 `Vec<char>`。

## 5. 分割、拼接与格式化

### 5.1 `split` 系列

| API                       | 产出                   | 说明                                    | 时间                 | 空间 |
| ------------------------- | ---------------------- | --------------------------------------- | -------------------- | ---- |
| `s.split(pat)`            | `&str`                 | 按 `char` / `&str` / 闭包切分，保留空串 | 构造 O(1)，消费 O(n) | O(1) |
| `s.split_whitespace()`    | `&str`                 | 按 Unicode 空白切分，跳过并合并连续空白 | 构造 O(1)，消费 O(n) | O(1) |
| `s.splitn(n, pat)`        | `&str`                 | 最多 n 段，最后一段包含剩余内容         | 构造 O(1)，消费 O(n) | O(1) |
| `s.rsplit(pat)`           | `&str`                 | 从右向左切分                            | 构造 O(1)，消费 O(n) | O(1) |
| `s.split_once(pat)`       | `Option<(&str, &str)>` | 在第一个匹配处切成两半                  | O(n)                 | O(1) |
| `s.rsplit_once(pat)`      | `Option<(&str, &str)>` | 在最后一个匹配处切成两半                | O(n)                 | O(1) |
| `s.lines()`               | `&str`                 | 按行切分，去掉 `\n` / `\r\n`            | 构造 O(1)，消费 O(n) | O(1) |
| `s.split_terminator(pat)` | `&str`                 | 与 `split` 相同，但不产出末尾空串       | 构造 O(1)，消费 O(n) | O(1) |
| `s.matches(pat)`          | `&str`                 | 产出所有不重叠的匹配                    | 构造 O(1)，消费 O(n) | O(1) |

`split` 的返回值是**借用**原字符串的切片，不复制内容；收集成 `Vec<&str>` 时只分配指针与长度，不复制字符：

```rust
let s = "  a  bb   ccc ";
let words: Vec<&str> = s.split_whitespace().collect();
assert_eq!(words, vec!["a", "bb", "ccc"]);

let parts: Vec<&str> = "a,b,c".split(',').collect();
assert_eq!(parts, vec!["a", "b", "c"]);

assert_eq!("a=b".split_once('='), Some(("a", "b")));
assert_eq!("a\nb\r\nc".lines().collect::<Vec<_>>(), vec!["a", "b", "c"]);
```

`split` 会保留空串，`split_terminator` 不会；`splitn` 最多产出 n 段：

```rust
assert_eq!("a,".split(',').collect::<Vec<_>>(), vec!["a", ""]);
assert_eq!("a,".split_terminator(',').collect::<Vec<_>>(), vec!["a"]);
assert_eq!("a,b,c".splitn(2, ',').collect::<Vec<_>>(), vec!["a", "b,c"]);
assert_eq!("".split(',').collect::<Vec<_>>(), vec![""]);
```

按单个空格切分与 `split_whitespace` 不同：`"a  b".split(' ')` 会产出空串，而 `split_whitespace` 会跳过连续空白。需要处理任意空白时优先用后者。

### 5.2 拼接：`join` 与 `concat`

| API                 | 作用                         | 时间                     | 空间                       |
| ------------------- | ---------------------------- | ------------------------ | -------------------------- |
| `slice.join(sep)`   | 用分隔符连接字符串切片       | O(总字节 + 分隔符)       | O(总字节)，返回新 `String` |
| `slice.concat()`    | 直接连接，无分隔符           | O(输出长度)              | O(输出长度)，返回新集合    |
| `s.push_str(other)` | 原地追加，见第 3 节          | 摊销 O(m)，最坏 O(n + m) | O(1) 额外，可能扩容        |
| `s += other`        | `AddAssign<&str>`，见第 3 节 | 摊销 O(m)，最坏 O(n + m) | O(1) 额外，可能扩容        |

```rust
let words = ["a", "b", "c"];
assert_eq!(words.join("-"), "a-b-c");
assert_eq!(words.concat(), "abc");

let parts = vec![String::from("x"), String::from("y")];
assert_eq!(parts.join(" "), "x y");

let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3]];
assert_eq!(nested.concat(), vec![1, 2, 3]);
```

`join` 要求元素能借出 `str`（`&str`、`String`、`Cow<str>` 等），不能直接拼接整数；拼接非字符串内容用 `format!` 或先 `map` 成字符串。空切片 `join` / `concat` 返回空 `String`。

### 5.3 格式化与调试输出

| API            | 作用                                                     | 时间    | 空间               |
| -------------- | -------------------------------------------------------- | ------- | ------------------ |
| `format!("…")` | 返回格式化后的 `String`                                  | O(输出) | O(输出)            |
| `println!`     | 写入标准输出并换行                                       | O(输出) | O(输出) 的内部缓冲 |
| `eprintln!`    | 写入标准错误并换行                                       | O(输出) | O(输出) 的内部缓冲 |
| `{:?}`         | `Debug` 格式；`{:#?}` 为多行美化                         | O(输出) | O(输出)            |
| `dbg!(expr)`   | 把文件、行号、表达式和值打印到 stderr，并原样返回 `expr` | O(输出) | O(输出)            |

```rust
let n = 42;
assert_eq!(format!("n={n:04}"), "n=0042");
assert_eq!(format!("{:?}", vec![1, 2]), "[1, 2]");

let len = dbg!(n.to_string().len()); // stderr 打印，同时返回 2
assert_eq!(len, 2);
assert_eq!(3.5.to_string(), "3.5");
```

`{}` 使用 `Display`，`{:?}` 使用 `Debug`：自定义类型想被 `assert_eq!` 打印或 `dbg!` 输出，需要 `#[derive(Debug)]`；想直接 `to_string()`，需要实现 `Display` 或改用 `format!("{:?}", x)`。`dbg!` 返回传入值的所有权，可以写在表达式中间，但会向 stderr 输出，提交前记得删除。

## 6. 迭代器

### 6.1 惰性

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

### 6.2 常用适配器

| 分类 | 方法                                          | 说明                                                         |
| ---- | --------------------------------------------- | ------------------------------------------------------------ |
| 变换 | `map`                                         | 一对一                                                       |
| 变换 | `flat_map`                                    | 一对一后展平（`map(f).flatten()` 的简写）                    |
| 变换 | `flatten`                                     | 元素本身可迭代时展平                                         |
| 变换 | `filter` / `filter_map`                       | 过滤 / 过滤并变换（返回 `Option`）                           |
| 变换 | `inspect`                                     | 只观察，不改变元素，调试用                                   |
| 截取 | `take(n)` / `take_while(f)`                   | 取前 n 个 / 取到条件不满足为止                               |
| 截取 | `skip(n)` / `skip_while(f)`                   | 跳过前 n 个 / 跳过到条件不满足为止                           |
| 截取 | `step_by(n)`                                  | 隔 n 个取一个                                                |
| 组合 | `zip(other)`                                  | 并行配对，长度取较短者                                       |
| 组合 | `chain(other)`                                | 串行拼接，产出全部                                           |
| 组合 | `rev()`                                       | 反向（需 `DoubleEndedIterator`）                             |
| 组合 | `enumerate()`                                 | 附带下标                                                     |
| 组合 | `peekable()` / `peek()`                       | 预读                                                         |
| 借用 | `by_ref()`                                    | 返回 `&mut Iterator`，用于部分消费后继续用                   |
| 基础 | `next()` / `next_back()`                      | 取下一个 / 从尾部取一个，返回 `Option<Item>`，O(1)           |
| 消费 | `collect()` / `count()` / `last()` / `nth(n)` | 收集 / 计数 / 最后一个 / 第 n 个                             |
| 消费 | `max()` / `min()`                             | 极值，需 `Ord`，详见 6.8                                     |
| 消费 | `fold(init, f)` / `reduce(f)` / `for_each(f)` | 累积 / 归约 / 逐项执行                                       |
| 消费 | `sum()` / `product()`                         | 求和 / 求积，需 `Sum` / `Product`                            |
| 查找 | `find(f)` / `find_map(f)` / `position(f)`     | 首个匹配元素 / 映射取首个 `Some` / 首个下标                  |
| 查找 | `rposition(f)`                                | 从右往左找下标，需 `DoubleEndedIterator + ExactSizeIterator` |
| 判定 | `all(f)` / `any(f)`                           | 全部满足 / 任一满足，可提前短路                              |
| 比较 | `eq(other)` / `ne(other)`                     | 与另一个迭代器逐元素比较，消耗双方                           |

适配器构造本身是 O(1) 且惰性；真正的时间花在消费时。`find` / `find_map` / `position` / `any` 命中即停，最坏 O(n)；`count` / `last` / `fold` / `sum` / `collect` 需要遍历完，O(n)。`nth(n)` 会消费并丢弃前 n 个元素，O(n)。`max` / `min` 见 6.8，查找与聚合的细节见 6.9、6.10。

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

`rev()` 只构造适配器，本身 O(1)；消费时反向逐个产出，总计 O(n)。它要求底层迭代器实现 `DoubleEndedIterator`（`Range`、slice 迭代器、`Chars` 都满足）。配套的消费方法是 `next_back()`（从尾部取一个，O(1)）和 `rfold()`（从右向左折叠，O(n)）：

```rust
let mut it = 1..=3;
assert_eq!(it.next_back(), Some(3));

let folded = (1..=4).rfold(0, |acc, x| acc * 10 + x);
assert_eq!(folded, 4321); // 4 -> 43 -> 432 -> 4321
```

### 6.3 `zip` 与 `chain`

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

### 6.4 `skip` / `take` / `by_ref`

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

### 6.5 `collect` 的类型由目标决定

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

`collect()` 与关联函数 `FromIterator::from_iter` 等价（2018 及更早 edition 需 `use std::iter::FromIterator;`）：

```rust
let s1: String = ['h', 'i'].into_iter().collect();
let s2 = String::from_iter(['h', 'i']);
assert_eq!(s1, s2);
```

复杂度：`collect::<String>()` / `String::from_iter` 需要遍历并逐个编码，时间 O(输出字节数)，空间 O(输出字节数)；若源是 `Vec<char>`，中间还会额外占用 4 字节/字符。`collect::<Vec<T>>()` 时间 O(n)，空间 O(n)。

### 6.6 一个容易踩的坑

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

### 6.7 `all` / `any` / `eq`：判定与比较

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

### 6.8 极值：`max` / `min` 与 `copied`

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

### 6.9 `find` / `position`：查找与定位

| API            | 返回            | 说明                                                         | 时间            | 空间 |
| -------------- | --------------- | ------------------------------------------------------------ | --------------- | ---- |
| `find(f)`      | `Option<Item>`  | 返回首个满足条件的元素                                       | O(k)，最坏 O(n) | O(1) |
| `find_map(f)`  | `Option<B>`     | `f` 返回 `Option<B>`，取首个 `Some`                          | O(k)，最坏 O(n) | O(1) |
| `position(f)`  | `Option<usize>` | 返回首个满足条件的下标                                       | O(k)，最坏 O(n) | O(1) |
| `rposition(f)` | `Option<usize>` | 从右往左找下标，需 `DoubleEndedIterator + ExactSizeIterator` | O(k)，最坏 O(n) | O(1) |
| `any(f)`       | `bool`          | 是否存在满足条件的元素                                       | O(k)，最坏 O(n) | O(1) |

`k` 是命中前检查过的元素个数，命中后立即停止。`find` 的闭包收到 `&Item`，而 `position` / `rposition` / `find_map` 收到 `Item` 本身；对 `iter()` 而言 `Item` 已经是 `&T`，所以 `find` 需要多解一层引用（`|&&x|`），`position` 写 `|&x|` 即可：

```rust
let v = [1, 3, 4, 6];
assert_eq!(v.iter().find(|&&x| x % 2 == 0), Some(&4));
assert_eq!(v.iter().position(|&x| x % 2 == 0), Some(2));
assert_eq!(v.iter().find_map(|&x| (x > 3).then_some(x * 10)), Some(40));
assert_eq!(v.iter().rposition(|&x| x % 2 == 0), Some(3));
```

`find` 返回元素的引用或所有权，取决于迭代器：`v.iter()` 得到 `Option<&T>`，`v.into_iter()` 得到 `Option<T>`。`position` 只关心位置，不做额外拷贝。

```rust
use std::collections::HashMap;

let map = HashMap::from([("a", 1), ("b", 5)]);
let hit = map.iter().find(|(_, v)| **v > 3);
assert!(hit.is_some());

let words = ["x", "long", "mid"];
assert_eq!(words.iter().position(|w| w.len() > 2), Some(1));
```

### 6.10 `fold` / `reduce` / `for_each`：聚合与副作用

| API                   | 说明                                    | 时间 | 空间      |
| --------------------- | --------------------------------------- | ---- | --------- |
| `fold(init, f)`       | 用初值累积，返回最终累加器              | O(n) | O(1) 额外 |
| `reduce(f)`           | 用首元素作初值累积，空迭代器返回 `None` | O(n) | O(1) 额外 |
| `for_each(f)`         | 逐项执行闭包，不返回值                  | O(n) | O(1) 额外 |
| `sum()` / `product()` | 求和 / 求积，需 `Sum` / `Product`       | O(n) | O(1) 额外 |
| `try_for_each(f)`     | 闭包返回 `ControlFlow`，可提前中断      | O(k) | O(1) 额外 |

```rust
let v = [1, 2, 3];
assert_eq!(v.iter().fold(0, |acc, &x| acc + x), 6);
assert_eq!(v.iter().copied().reduce(|a, b| a + b), Some(6));
let total: i32 = v.iter().sum();
assert_eq!(total, 6);
```

`reduce` 要求元素类型与累加器类型一致；`iter_mut()` 上使用 `reduce` 时元素是 `&mut T`，返回 `Option<&mut T>`，可以在原位置上累积：

```rust
let mut v = [1, 2, 3];
let acc = v.iter_mut().reduce(|a, b| {
    *a += *b;
    a
});
assert_eq!(acc.map(|x| *x), Some(6));
assert_eq!(v, [6, 2, 3]);
```

`for_each` 里的 `return` 只会结束当前这一次闭包调用，不会从外层函数返回；需要在第一个命中处退出时用 `try_for_each` 配合 `ControlFlow`：

```rust
let mut sum = 0;
(1..=5).for_each(|x| {
    if x > 3 {
        return; // 只跳过本次调用
    }
    sum += x;
});
assert_eq!(sum, 6);

use std::ops::ControlFlow;
let result = (1..=5).try_for_each(|x| {
    if x == 4 {
        ControlFlow::Break(x)
    } else {
        ControlFlow::Continue(())
    }
});
assert_eq!(result, ControlFlow::Break(4));
```

### 6.11 迭代器副本与重复扫描

大多数标准库迭代器在底层可克隆时都实现 `Clone`，克隆只复制遍历位置，通常 O(1) 时间与 O(1) 额外空间，不复制元素。`Chars` 克隆后各自独立前进：

```rust
let mut chars = "hello".chars();
let snapshot = chars.clone();

chars.next();
assert_eq!(chars.as_str(), "ello");
assert_eq!(snapshot.count(), 5);
```

克隆本身便宜，但每次从副本重新 `nth(i)` 都会从头扫描，嵌套循环里会退化成 O(n²)：

```rust
let words = ["flower", "flow", "flight"];
let first: Vec<char> = words[0].chars().collect();

for i in 0..first.len() {
    let hit = words.iter().all(|w| w.chars().nth(i) == Some(first[i]));
    if !hit {
        break;
    }
}
```

需要多次按下标访问字符时，先 `collect::<Vec<char>>()`（O(n) 时间与空间）或改用 `char_indices()`，避免反复重建迭代器。

## 7. `Option` 与 `Result`

### 7.1 `Option`

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

以上方法的结构开销都是 O(1) 时间与 O(1) 额外空间；`map` / `and_then` / `filter` / `unwrap_or_else` 的总时间取决于传入闭包，`take` 只移动值，不复制内部数据。

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

### 7.2 `Result`

| API                                  | 说明                                              |
| ------------------------------------ | ------------------------------------------------- |
| `is_ok()` / `is_err()`               | 判断                                              |
| `map(f)` / `map_err(f)`              | 变换成功值 / 错误值                               |
| `and_then(f)`                        | 链式处理，失败短路                                |
| `unwrap()` / `expect(msg)`           | 取值，失败 panic                                  |
| `unwrap_or(v)` / `unwrap_or_else(f)` | 提供默认值                                        |
| `?` 运算符                           | 失败时提前返回（函数需返回 `Result` 或 `Option`） |

`Result` 的结构操作同样都是 O(1)；`map` / `map_err` / `and_then` 的耗时取决于闭包，`?` 只做匹配与提前返回，不分配。

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

## 8. `Vec` 与 slice

| API                                          | 说明                                            |
| -------------------------------------------- | ----------------------------------------------- |
| `Vec::new()` / `Vec::with_capacity(n)`       | 创建，后者预分配                                |
| `vec![...]` / `vec![v; n]`                   | 由元素创建 / 重复 n 个（需 `Clone`）            |
| `reserve(n)` / `shrink_to_fit()`             | 额外预留容量 / 收缩到当前长度                   |
| `push(v)` / `pop()`                          | 尾部入栈 / 出栈，摊销 O(1)                      |
| `extend(iter)` / `extend_from_slice(&[T])`   | 追加迭代器 / 切片，O(m)                         |
| `append(&mut other)`                         | 把另一个 `Vec` 的元素全部移入，O(m)，other 清空 |
| `insert(i, x)` / `remove(i)`                 | 在 i 处插入 / 删除，O(n)，越界 panic            |
| `resize(n, value)`                           | 调整长度，O(n)，需 `T: Clone`                   |
| `dedup()`                                    | 删除**相邻**重复元素，O(n)                      |
| `split_at(i)` / `split_at_mut(i)`            | 在 i 处切成两个切片，O(1)                       |
| `v[i]`（`Index` / `IndexMut`）               | 直接下标，越界 panic                            |
| `get(i)` / `get_mut(i)`                      | 安全下标，越界返回 `None`                       |
| `first()` / `last()`（及 `_mut` 版本）       | 首 / 尾元素，`Option<&T>`                       |
| `as_slice()`                                 | 借出 `&[T]`                                     |
| `clone()` / `to_vec()`                       | 深拷贝成新的 `Vec<T>`，O(n)                     |
| `contains(&x)`                               | 线性查找，O(n)                                  |
| `chunks(n)` / `chunks_exact(n)`              | 连续分块；exact 丢弃不足 n 的尾块               |
| `rchunks(n)` / `windows(n)`                  | 反向分块 / 定长滑动窗口                         |
| `swap(i, j)`                                 | 交换两个元素，O(1)                              |
| `len()` / `is_empty()` / `capacity()`        | 长度 / 判空 / 容量，O(1)                        |
| `truncate(n)` / `retain(f)` / `drain(range)` | 截断 / 过滤 / 删除范围，O(n)                    |
| `into_iter()` / `iter()` / `iter_mut()`      | 三种遍历方式                                    |

未单独标注的方法均为 O(1) 结构操作；`extend` / `extend_from_slice` / `append` 为 O(m)（m 为追加元素数）；`insert` / `remove` / `truncate` / `retain` / `drain` / `resize` 为 O(n)；`clone` / `to_vec` 为 O(n) 时间与空间；`chunks` / `windows` 每产出一个块是 O(1)。排序与二分查找见第 9 节。

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

let mut v = vec![0; 4]; // 重复 4 个 0
assert_eq!(v, vec![0, 0, 0, 0]);
v.reserve(8); // 保证还能再放 8 个而不重新分配
assert!(v.capacity() >= 12);
v.shrink_to_fit(); // 尽可能收缩到当前长度
assert!(v.capacity() >= v.len());
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

### 8.1 遍历方式

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

空 `Vec` 的 `0..v.len()` 是空范围，循环体不会执行；但直接写 `v[0]` 仍会 panic，需先判空或用 `first()`。含右端点的区间写作 `0..=n`；倒序遍历用 `(0..v.len()).rev()`（只产出下标）或 `v.iter().rev()`（产出 `&T`），两者都是 O(n)。

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

### 8.2 克隆与复制：`clone` / `to_vec` / `Copy`

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

### 8.3 追加、插入与删除

| API                | 作用                          | 时间 | 空间      |
| ------------------ | ----------------------------- | ---- | --------- |
| `a.append(&mut b)` | 把 b 的所有元素移动到 a 末尾  | O(m) | O(1) 额外 |
| `v.insert(i, x)`   | 在 i 前插入，后续元素右移     | O(n) | O(1) 额外 |
| `v.remove(i)`      | 删除并返回 i 处元素，后续左移 | O(n) | O(1) 额外 |
| `v.swap(i, j)`     | 交换两个位置                  | O(1) | O(1)      |

```rust
let mut a = vec![1, 2];
let mut b = vec![3, 4];
a.append(&mut b);
assert_eq!(a, vec![1, 2, 3, 4]);
assert!(b.is_empty());

let mut v = vec![1, 3];
v.insert(1, 2);
assert_eq!(v, vec![1, 2, 3]);
assert_eq!(v.remove(0), 1);
assert_eq!(v, vec![2, 3]);
```

`insert(i)` 允许 `i == len`（等价于尾部追加），`i > len` 会 panic；`remove(i)` 在 `i >= len` 时 panic。`append` 会搬移元素并清空源 `Vec`，容量不够时还会重新分配。

### 8.4 `resize` 与 `dedup`

| API                   | 作用                       | 时间 | 空间      |
| --------------------- | -------------------------- | ---- | --------- |
| `v.resize(n, value)`  | 截断或用克隆值补齐到长度 n | O(n) | O(1) 额外 |
| `v.resize_with(n, f)` | 用闭包生成新元素           | O(n) | O(1) 额外 |
| `v.dedup()`           | 删除相邻重复元素           | O(n) | O(1) 额外 |
| `v.dedup_by_key(f)`   | 按投影键删除相邻重复       | O(n) | O(1) 额外 |

```rust
let mut v = vec![1, 2];
v.resize(4, 0);
assert_eq!(v, vec![1, 2, 0, 0]);
v.resize(1, 0);
assert_eq!(v, vec![1]);

let mut v = vec![1, 1, 2, 2, 2, 3];
v.dedup();
assert_eq!(v, vec![1, 2, 3]);

let mut v = vec![1, 1, 3, 3, 2];
v.dedup(); // 2 与前面的 3 不相邻，不会被删
assert_eq!(v, vec![1, 3, 2]);
```

`resize` 要求 `T: Clone`，补值时会克隆同一个 `value`；`resize_with` 每次调用闭包生成新值，适合非 `Clone` 类型。`dedup` 只比较**相邻**元素，要去掉全部重复，先 `sort()` 再 `dedup()`，总代价 O(n log n)。

### 8.5 数组的 `map` 与 `to_vec`

| API                  | 作用                         | 时间 | 空间      |
| -------------------- | ---------------------------- | ---- | --------- |
| `[T; N]::map(f)`     | 逐元素变换，返回同长度数组   | O(n) | O(n) 结果 |
| `[T; N]::to_vec()`   | 复制成 `Vec<T>`              | O(n) | O(n)      |
| `[T; N]::as_slice()` | 借出 `&[T]`                  | O(1) | O(1)      |
| `array.into_iter()`  | 按值产出元素（2021 edition） | O(n) | O(1)      |

```rust
let nums = [1, 2, 3].map(|x| x * 2);
assert_eq!(nums, [2, 4, 6]);

let owned: Vec<String> = ["a", "b"].map(|s| s.to_string()).to_vec();
assert_eq!(owned, vec!["a".to_string(), "b".to_string()]);

let arr = [10, 20];
let borrowed: &[i32] = arr.as_slice();
assert_eq!(borrowed, &[10, 20][..]);
```

数组的 `map` 不是惰性迭代器：它立即对每个元素求值，返回长度由编译期固定的数组。输出长度可能变化时，用 `iter().map(...).collect::<Vec<_>>()`。

### 8.6 区间索引与 `split_at`

| API                 | 作用                                 | 时间 | 空间 |
| ------------------- | ------------------------------------ | ---- | ---- |
| `&v[a..b]`          | 左闭右开切片                         | O(1) | O(1) |
| `&v[a..=b]`         | 闭区间切片                           | O(1) | O(1) |
| `v.get(a..b)`       | 安全切片，越界或 `a > b` 返回 `None` | O(1) | O(1) |
| `v.split_at(i)`     | 切成两个切片                         | O(1) | O(1) |
| `v.split_at_mut(i)` | 可变版本，得到两个 `&mut [T]`        | O(1) | O(1) |

```rust
let v = [0, 1, 2, 3, 4];
assert_eq!(&v[1..3], &[1, 2][..]);
assert_eq!(&v[1..=2], &[1, 2][..]);
assert_eq!(v.get(3..), Some(&[3, 4][..]));
assert_eq!(v.get(3..9), None);

let (left, right) = v.split_at(2);
assert_eq!((left, right), (&[0, 1][..], &[2, 3, 4][..]));
```

`&v[a..b]` 在 `a > b` 或 `b > len` 时 panic；`split_at(i)` 在 `i > len` 时 panic。`split_at_mut` 是用来同时可变借用两个不相交半段的惯用方法，避免借用冲突。

## 9. 排序、查找与比较

本章 `n` 表示元素个数，`m` 表示参数量；O(1) < O(log n) < O(n) < O(n log n) < O(n²)。字符串比较按字节数计。

### 9.1 排序

| API                       | 作用                 | 时间                              | 空间        |
| ------------------------- | -------------------- | --------------------------------- | ----------- |
| `v.sort()`                | 稳定升序，需 `Ord`   | O(n log n)                        | O(n)        |
| `v.sort_unstable()`       | 不稳定升序，通常更快 | O(n log n)                        | O(log n) 栈 |
| `v.sort_by(f)`            | 稳定，自定义比较器   | O(n log n)                        | O(n)        |
| `v.sort_unstable_by(f)`   | 不稳定，自定义比较器 | O(n log n)                        | O(log n) 栈 |
| `v.sort_by_key(f)`        | 稳定，按投影键排序   | O(n log n) 次比较，键可能多次计算 | O(n)        |
| `v.sort_by_cached_key(f)` | 稳定，键只计算一次   | O(n log n)                        | O(n)        |
| `v.reverse()`             | 原地反转             | O(n)                              | O(1)        |
| `v.dedup()`               | 删除相邻重复         | O(n)                              | O(1)        |

```rust
let mut v = vec![3, 1, 2];
v.sort();
assert_eq!(v, vec![1, 2, 3]);

let mut v = vec![3, 1, 2];
v.sort_by(|a, b| b.cmp(a)); // 降序
assert_eq!(v, vec![3, 2, 1]);

let mut words = vec![String::from("bb"), String::from("a")];
words.sort(); // 字典序
assert_eq!(words, vec!["a", "bb"]);
```

`sort` 是稳定排序：相等元素保持原顺序，代价是 O(n) 额外空间。`sort_unstable` 不保证相等元素的顺序，但通常更快、额外空间更小。比较器必须是自洽的全序，否则排序结果未定义，较新的标准库还可能 panic。

要对全部元素去重，先排序再 `dedup`：

```rust
let mut v = vec![3, 1, 3, 2, 1];
v.sort();
v.dedup();
assert_eq!(v, vec![1, 2, 3]);
```

### 9.2 二分查找

| API                             | 返回                   | 说明                         | 时间     | 空间 |
| ------------------------------- | ---------------------- | ---------------------------- | -------- | ---- |
| `v.binary_search(&x)`           | `Result<usize, usize>` | `Ok` 命中下标，`Err` 插入点  | O(log n) | O(1) |
| `v.binary_search_by(f)`         | `Result<usize, usize>` | 用比较器探测                 | O(log n) | O(1) |
| `v.binary_search_by_key(&k, f)` | `Result<usize, usize>` | 按投影键查找                 | O(log n) | O(1) |
| `v.partition_point(f)`          | `usize`                | 首个使 `f` 为 `false` 的位置 | O(log n) | O(1) |

```rust
let v = vec![1, 3, 5, 7];
assert_eq!(v.binary_search(&5), Ok(2));
assert_eq!(v.binary_search(&4), Err(2)); // 4 应插入到下标 2
assert_eq!(v.partition_point(|&x| x < 5), 2);
assert!(v.contains(&7)); // 线性查找 O(n)，不要求有序
```

二分查找要求元素已按 `Ord` 升序排列；在未排序数据上结果是未定义的。存在重复元素时，`Ok` 返回的下标不保证是第一个或最后一个匹配项；需要边界时用 `partition_point`。

### 9.3 比较与极值

| API                                 | 作用                      | 时间                     | 空间 |
| ----------------------------------- | ------------------------- | ------------------------ | ---- |
| `a.cmp(&b)`                         | 返回 `Ordering`           | 标量 O(1)，字符串 O(len) | O(1) |
| `a.min(b)` / `a.max(b)`             | 两个值取小 / 大，需 `Ord` | 同上                     | O(1) |
| `std::cmp::min(a, b)` / `max(a, b)` | 自由函数版本              | 同上                     | O(1) |
| `Ordering::reverse()`               | 反转 `Less` / `Greater`   | O(1)                     | O(1) |
| `Reverse(x)`                        | 包装类型，用于反转排序键  | O(1)                     | O(1) |

```rust
use std::cmp::{max, min, Ordering, Reverse};

assert_eq!(3.cmp(&5), Ordering::Less);
assert_eq!(min(3, 5), 3);
assert_eq!(3.max(5), 5);
assert_eq!(Ordering::Less.reverse(), Ordering::Greater);

let mut v = vec![1, 3, 2];
v.sort_by_key(|&x| Reverse(x));
assert_eq!(v, vec![3, 2, 1]);
```

`min` / `max` 的自由函数需要 `use std::cmp::{min, max}`；方法形式 `a.min(b)` 来自 `Ord`，按值接收。`f64` 没有实现 `Ord`，因此不能用于 `sort`、`binary_search` 或 `Ord::min/max`；浮点需要排序时可用 `sort_by(|a, b| a.partial_cmp(b).unwrap())` 并先决定 NaN 的处理，或用 `total_cmp` 得到包含 NaN 的确定性全序。

## 10. `HashMap` / `HashSet` 与 `BTreeMap` / `BTreeSet`

`n` 是集合元素个数。哈希表操作一律写「平均 O(1)，最坏 O(n)」：默认哈希器带随机种子，正常情况下接近 O(1)，极端碰撞时退化。有序树操作是 O(log n)。

### 10.1 如何选择

| 集合                             | 顺序     | 查找 / 插入 / 删除           | 要求        | 适用场景                   |
| -------------------------------- | -------- | ---------------------------- | ----------- | -------------------------- |
| `HashSet<T>` / `HashMap<K, V>`   | 无保证   | 平均 O(1)，最坏 O(n)         | `Eq + Hash` | 去重、计数、快速成员判断   |
| `BTreeSet<T>` / `BTreeMap<K, V>` | 按键升序 | O(log n)                     | `Ord`       | 有序遍历、范围查询、取最值 |
| `Vec<T>`                         | 插入顺序 | 查找 O(n)，尾部插入摊销 O(1) | -           | 元素少或主要按顺序访问     |

### 10.2 `HashSet`

| API                                  | 作用                   | 时间                   | 空间      |
| ------------------------------------ | ---------------------- | ---------------------- | --------- |
| `HashSet::new()`                     | 创建空集合，不分配     | O(1)                   | O(1)      |
| `HashSet::with_capacity(n)`          | 预分配可容纳 n 个元素  | O(1)                   | O(n)      |
| `set.insert(x)`                      | 插入，返回是否为新元素 | 平均 O(1)，最坏 O(n)   | O(1) 额外 |
| `set.contains(&x)`                   | 成员判断               | 平均 O(1)，最坏 O(n)   | O(1)      |
| `set.remove(&x)`                     | 删除，返回是否存在     | 平均 O(1)，最坏 O(n)   | O(1)      |
| `set.len()` / `is_empty()`           | 元素个数 / 判空        | O(1)                   | O(1)      |
| `set.iter()`                         | 任意顺序遍历           | 构造 O(1)，消费 O(n)   | O(1)      |
| `set.extend(iter)`                   | 批量插入               | 平均 O(m)，最坏 O(m·n) | O(1) 额外 |
| `HashSet::from([...])` / `collect()` | 从迭代器构造           | 平均 O(n)，最坏 O(n²)  | O(n)      |

```rust
use std::collections::HashSet;

let mut set = HashSet::with_capacity(4);
assert!(set.insert(1));
assert!(!set.insert(1)); // 已存在
assert!(set.contains(&1));
assert!(set.remove(&1));
assert!(set.is_empty());

let unique: HashSet<i32> = [1, 2, 2, 3].into_iter().collect();
assert_eq!(unique.len(), 3);
```

`insert` 返回 `bool`，把「判断是否存在」和「插入」合并成一次查找，常用于保留首次出现的元素：

```rust
use std::collections::HashSet;

let nums = vec![1, 2, 1, 3, 2];
let mut seen = HashSet::new();
let first_occurrences: Vec<i32> = nums.into_iter().filter(|n| seen.insert(*n)).collect();
assert_eq!(first_occurrences, vec![1, 2, 3]);
```

`HashSet` 的遍历顺序是任意的，不能依赖 `iter()` 的顺序；需要有序结果时排序，或直接用 `BTreeSet`。`HashSet<&T>` 存的是借用，被引用的数据必须比集合活得更久。

### 10.3 `HashMap` 与 `entry`

| API                                   | 作用                                 | 时间                  | 空间      |
| ------------------------------------- | ------------------------------------ | --------------------- | --------- |
| `HashMap::new()` / `with_capacity(n)` | 创建，后者预分配                     | O(1)                  | O(n)      |
| `map.insert(k, v)`                    | 插入，返回旧值 `Option<V>`           | 平均 O(1)，最坏 O(n)  | O(1) 额外 |
| `map.get(&k)` / `get_mut(&k)`         | 取 `Option<&V>` / `Option<&mut V>`   | 平均 O(1)，最坏 O(n)  | O(1)      |
| `map.contains_key(&k)`                | 是否存在                             | 平均 O(1)，最坏 O(n)  | O(1)      |
| `map.remove(&k)`                      | 删除，返回 `Option<V>`               | 平均 O(1)，最坏 O(n)  | O(1)      |
| `map.entry(k)`                        | 返回 `Entry`，一次查找完成插入或修改 | 平均 O(1)，最坏 O(n)  | O(1)      |
| `map.keys()` / `values()` / `iter()`  | 任意顺序遍历                         | 构造 O(1)，消费 O(n)  | O(1)      |
| `map.len()` / `is_empty()`            | 数量 / 判空                          | O(1)                  | O(1)      |
| `HashMap::from([...])` / `collect()`  | 从键值对构造                         | 平均 O(n)，最坏 O(n²) | O(n)      |

`Entry` 上的常用方法：

| API                 | 作用                                  | 时间          | 空间 |
| ------------------- | ------------------------------------- | ------------- | ---- |
| `or_insert(v)`      | 空则插入 v，返回 `&mut V`             | 平均 O(1)     | O(1) |
| `or_insert_with(f)` | 空则调用 f 生成值，返回 `&mut V`      | 平均 O(1) + f | O(1) |
| `and_modify(f)`     | 有值时执行 f，返回 `Entry` 供链式调用 | 平均 O(1) + f | O(1) |
| `or_default()`      | 空则插入 `V::default()`               | 平均 O(1)     | O(1) |
| `key()`             | 返回 `&K`                             | O(1)          | O(1) |

```rust
use std::collections::HashMap;

let mut counts = HashMap::new();
for word in ["a", "b", "a"] {
    *counts.entry(word).or_insert(0) += 1;
}
assert_eq!(counts.get("a"), Some(&2));
assert_eq!(counts["b"], 1);

let old = counts.insert("a", 10);
assert_eq!(old, Some(2));
```

`and_modify` 只在键存在时执行，`or_insert` / `or_insert_with` 只在键不存在时执行；链式写法一次查找就能完成「有则修改、无则插入」：

```rust
use std::collections::HashMap;

let mut index: HashMap<i32, Vec<usize>> = HashMap::new();
for (i, n) in [5, 7, 5].iter().enumerate() {
    index
        .entry(*n)
        .and_modify(|v| v.push(i))
        .or_insert_with(|| vec![i]);
}
assert_eq!(index[&5], vec![0, 2]);
```

`or_insert(v)` 会先求值 `v` 再调用，即使键已存在也会构造；值需要分配或计算时用 `or_insert_with`，闭包只在空位时执行。`contains_key` + `insert` 会做两次查找，能用 `entry` 时优先用 `entry`。

### 10.4 `BTreeSet` 与 `BTreeMap`

| API                                          | 作用                      | 时间                 | 空间 |
| -------------------------------------------- | ------------------------- | -------------------- | ---- |
| `BTreeSet::new()` / `BTreeMap::new()`        | 创建空集合                | O(1)                 | O(1) |
| `set.insert(x)`                              | 插入，返回是否新增        | O(log n)             | O(1) |
| `set.contains(&x)` / `remove(&x)`            | 成员判断 / 删除           | O(log n)             | O(1) |
| `set.len()` / `is_empty()`                   | 元素个数 / 判空           | O(1)                 | O(1) |
| `set.first()` / `last()`                     | 最小 / 最大元素的引用     | O(log n)             | O(1) |
| `set.pop_first()` / `pop_last()`             | 删除并返回最小 / 最大元素 | O(log n)             | O(1) |
| `set.range(a..=b)`                           | 范围遍历                  | O(log n + k)         | O(1) |
| `set.iter()`                                 | 升序遍历                  | 构造 O(1)，消费 O(n) | O(1) |
| `map.insert(k, v)` / `get(&k)` / `entry(k)`  | 与 `HashMap` 同名         | O(log n)             | O(1) |
| `map.keys()` / `values()` / `iter()`         | 按键升序遍历              | 构造 O(1)，消费 O(n) | O(1) |
| `map.first_key_value()` / `last_key_value()` | 最小 / 最大的键值对       | O(log n)             | O(1) |

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
for x in [5, 1, 3, 3] {
    set.insert(x);
}
assert_eq!(set.len(), 3);
assert_eq!(set.first(), Some(&1));
assert_eq!(set.last(), Some(&5));
assert_eq!(set.pop_first(), Some(1));
assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![3, 5]);

let in_range: Vec<i32> = set.range(3..=5).copied().collect();
assert_eq!(in_range, vec![3, 5]);
```

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(2, "b");
map.insert(1, "a");
assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1, 2]);
assert_eq!(map.first_key_value().map(|(k, v)| (*k, *v)), Some((1, "a")));
```

`BTreeSet` / `BTreeMap` 按 `Ord` 保持有序，`first` / `last` / `pop_first` / `pop_last` / `first_key_value` / `last_key_value` 等接口需要较新的标准库（1.66+）。`first` / `last` 返回借用，`pop_first` / `pop_last` 返回所有权。

### 10.5 常见模式与陷阱

```rust
use std::collections::HashMap;

let nums = [2, 7, 11, 15];
let target = 9;
let mut seen = HashMap::new();
let mut pair = None;

for (i, &n) in nums.iter().enumerate() {
    if let Some(&j) = seen.get(&(target - n)) {
        pair = Some((j, i));
        break;
    }
    seen.insert(n, i);
}
assert_eq!(pair, Some((0, 1)));
```

- 哈希集合 / 映射的遍历顺序不做保证；需要确定性输出时先收集键并排序，或改用 `BTreeMap` / `BTreeSet`。
- `map[key]` 在键不存在时 panic；不确定时用 `get`。
- 迭代 `map.iter()` 时不能同时 `insert` / `remove`，借用检查器会阻止；先收集键再修改。
- 键类型必须实现 `Eq + Hash`（哈希表）或 `Ord`（有序树）；`f64` 两者都不满足，不能直接作为键。
- `HashMap<&str, V>` 的键是借用，字符串数据必须活得比映射久；需要长期持有时用 `String` 键。

## 11. 数值、位运算与类型转换

本章的算术、位运算与 `as` 转换都是 O(1) 时间与 O(1) 额外空间；涉及字符串的转换按字节数计。

### 11.1 复合赋值与算术

复合赋值运算符背后是 `std::ops` 中的 trait，基础数值类型均已实现，单次运算 O(1)：

| 表达式   | trait       | 说明                     |
| -------- | ----------- | ------------------------ |
| `a += b` | `AddAssign` | 加法赋值                 |
| `a -= b` | `SubAssign` | 减法赋值                 |
| `a *= b` | `MulAssign` | 乘法赋值                 |
| `a /= b` | `DivAssign` | 除法赋值；整数除零 panic |
| `a %= b` | `RemAssign` | 取余赋值；整数除零 panic |

`v[i] *= k` 同时用到 `IndexMut`（见第 8 节）与 `MulAssign`，直接在原位置修改：

```rust
let mut v = vec![1, 2, 3];
for i in 0..v.len() {
    v[i] *= 2;
}
assert_eq!(v, vec![2, 4, 6]);
```

### 11.2 位运算

| 表达式              | trait          | 说明                                     | 时间 |
| ------------------- | -------------- | ---------------------------------------- | ---- |
| `a & b`             | `BitAnd`       | 按位与                                   | O(1) |
| `a \| b`            | `BitOr`        | 按位或                                   | O(1) |
| `a ^ b`             | `BitXor`       | 按位异或                                 | O(1) |
| `!a`                | `Not`          | 按位取反                                 | O(1) |
| `a << k` / `a >> k` | `Shl` / `Shr`  | 左移 / 右移                              | O(1) |
| `a ^= b`            | `BitXorAssign` | 异或赋值；`&=`、`\|=`、`<<=`、`>>=` 同理 | O(1) |

按位异或有三条常用性质：`x ^ x == 0`、`x ^ 0 == x`，且满足交换律与结合律，因此可以在一组数中找出唯一出现奇数次的元素。无符号数的右移是逻辑右移（高位补 0），有符号数是算术右移（高位补符号位），所以负数的 `>> 1` 与 `/ 2` 结果不同。

```rust
assert_eq!(0b1010 & 0b1100, 0b1000);
assert_eq!(0b1010 | 0b1100, 0b1110);
assert_eq!(0b1010 ^ 0b1100, 0b0110);
assert_eq!(!0u8, 0xFF);

let mut x = 0b1010u8;
x ^= 0b1111;
assert_eq!(x, 0b0101);

assert_eq!((-3i32) >> 1, -2); // 算术右移，符号位补 1
assert_eq!((-3i32) / 2, -1);  // 除法向零截断
```

移位量大于等于位宽属于算术溢出：debug 构建 panic，release 构建会把移位量掩码后再移位。不要依赖 release 下的掩码结果。

### 11.3 常量与边界

| 常量 / 方法             | 说明                          | 时间 | 空间 |
| ----------------------- | ----------------------------- | ---- | ---- |
| `i32::MAX` / `i32::MIN` | 类型上下界                    | O(1) | O(1) |
| `usize::MAX`            | 平台相关的最大下标            | O(1) | O(1) |
| `u8::MAX` / `u8::BITS`  | 最大值 / 位数                 | O(1) | O(1) |
| `a.unsigned_abs()`      | 无符号绝对值，避免 `MIN` 溢出 | O(1) | O(1) |
| `a.abs()`               | 绝对值；`MIN.abs()` 会溢出    | O(1) | O(1) |

```rust
assert_eq!(i32::MAX, 2_147_483_647);
assert_eq!(i32::MIN, -2_147_483_648);
assert_eq!(i32::MIN.unsigned_abs(), 2_147_483_648);
assert_eq!(u8::MAX, 255);
assert_eq!(u8::BITS, 8);
```

`usize` 的宽度随平台变化（32 位或 64 位），涉及下标上界时用 `usize::MAX` 表达「比任何真实下标都大」是常见写法，例如求最小值时作初值。注意 `usize::MAX + 1` 会溢出，不能拿它做累加。

### 11.4 `as` 转换与 `TryFrom`

| 写法           | 行为                                            | 时间 |
| -------------- | ----------------------------------------------- | ---- |
| `u8 as u32`    | 无符号扩展，无损                                | O(1) |
| `u16 as u8`    | 截断高位，只保留低 8 位                         | O(1) |
| `i32 as u32`   | 按补码回绕，`-1` 变成 `u32::MAX`                | O(1) |
| `i32 as usize` | 负数变成极大的 `usize`，不是 0                  | O(1) |
| `u8 as char`   | `0..=255` 映射到对应 Unicode 标量               | O(1) |
| `char as u32`  | 取 Unicode 标量值；反向用 `char::from_u32` 检查 | O(1) |
| `f64 as i32`   | 向零截断并饱和，`NaN` 变成 0                    | O(1) |
| `bool as u8`   | `true` 为 1，`false` 为 0                       | O(1) |

`as` 转换不会 panic，但可能静默截断或回绕。需要可失败转换时用 `TryFrom` / `try_into`，需要无损扩展时用 `From` / `into`：

| API                | 说明                       | 时间 | 空间 |
| ------------------ | -------------------------- | ---- | ---- |
| `u32::from(255u8)` | 无损扩展，返回 `u32`       | O(1) | O(1) |
| `i32::try_from(x)` | 失败返回 `Err`，成功返回值 | O(1) | O(1) |
| `x.try_into()?`    | 目标类型由上下文推断       | O(1) | O(1) |

```rust
assert_eq!(300u16 as u8, 44);             // 截断高位
assert_eq!((-1i32) as u32, u32::MAX);     // 补码回绕
assert_eq!((-1i32) as usize, usize::MAX); // 变成极大值
assert_eq!(65u8 as char, 'A');            // 0..=255 映射到 Unicode 标量
assert_eq!('A' as u32, 65);
assert_eq!(3.9_f64 as i32, 3);
assert_eq!(f64::NAN as i32, 0);

use std::convert::TryFrom;
assert!(i32::try_from(3_000_000_000u32).is_err());
assert_eq!(u32::from(255u8), 255);
```

负数转无符号时尤其容易出错：`(-1i32) as usize` 得到一个极大的值，后续参与下标或长度计算会直接越界。需要确认范围时用 `usize::try_from(i)` 并处理 `Err`。

### 11.5 整数溢出

`i32` 等整数的 `+`、`-`、`*` 溢出行为取决于构建配置：debug 构建 panic，release 构建默认按补码回绕（开启 `overflow-checks` 后同样会 panic）。需要与构建配置无关的行为时，改用显式方法：

| 方法                   | 溢出时返回值        | 时间 |
| ---------------------- | ------------------- | ---- |
| `a.checked_mul(b)`     | `None`              | O(1) |
| `a.saturating_mul(b)`  | `T::MAX` / `T::MIN` | O(1) |
| `a.wrapping_mul(b)`    | 补码回绕后的值      | O(1) |
| `a.overflowing_mul(b)` | `(值, 是否溢出)`    | O(1) |

`checked_*`、`wrapping_*`、`overflowing_*` 对四则运算都有对应版本（如 `checked_add`、`wrapping_sub`）；`saturating_*` 覆盖 `+`、`-`、`*`、`/` 和 `pow`，没有 `saturating_rem`。

```rust
let x: i32 = 2_000_000_000;
assert_eq!(x.checked_mul(2), None);
assert_eq!(x.saturating_mul(2), i32::MAX);
assert_eq!(x.wrapping_mul(2), -294_967_296);
assert_eq!(x.overflowing_mul(2), (-294_967_296, true));
```

累加、累乘时若中间结果可能越界，优先用 `checked_*` 并显式处理 `None`，不要依赖 release 构建下的回绕结果。

## 12. `Rc`、`RefCell` 与共享所有权

`Rc<T>` 提供多所有者共享，`RefCell<T>` 把借用检查从编译期推迟到运行期；二者组合常用于树、图等需要多个父节点或反向引用的结构。这些结构操作都是 O(1)，不复制内部数据。

### 12.1 `Rc<T>`

| API                          | 作用                           | 时间 | 空间                     |
| ---------------------------- | ------------------------------ | ---- | ------------------------ |
| `Rc::new(v)`                 | 堆上分配并返回共享指针，计数 1 | O(1) | O(size_of::<T>()) + 计数 |
| `Rc::clone(&rc)` / `clone()` | 增加强引用计数，不复制 T       | O(1) | O(1)                     |
| `Rc::strong_count(&rc)`      | 当前强引用数                   | O(1) | O(1)                     |
| `Rc::ptr_eq(&a, &b)`         | 是否指向同一份分配             | O(1) | O(1)                     |
| `Rc::try_unwrap(rc)`         | 计数为 1 时取出内部值          | O(1) | O(1)                     |
| `Rc::get_mut(&mut rc)`       | 独占时取得 `&mut T`            | O(1) | O(1)                     |

```rust
use std::rc::Rc;

let a = Rc::new(vec![1, 2]);
let b = Rc::clone(&a); // 只增加计数
assert_eq!(Rc::strong_count(&a), 2);
assert!(Rc::ptr_eq(&a, &b));

drop(b);
assert_eq!(Rc::strong_count(&a), 1);
assert_eq!(Rc::try_unwrap(a), Ok(vec![1, 2]));
```

`Rc::clone` 是浅拷贝，优先写 `Rc::clone(&rc)` 而不是 `rc.clone()`，让「只增加计数」的意图更明显。最后一个强引用被丢弃时才释放内部值；存在其他 `Rc` 或 `Weak` 时 `try_unwrap` 会失败并把指针原样返回。`Rc` 不是线程安全的，多线程共享用 `Arc`。

### 12.2 `RefCell<T>`

| API                                 | 作用                               | 时间 | 空间                     |
| ----------------------------------- | ---------------------------------- | ---- | ------------------------ |
| `RefCell::new(v)`                   | 创建内部可变容器，值内联存储       | O(1) | O(size_of::<T>()) + 标志 |
| `cell.borrow()`                     | 不可变借用 `Ref<T>`，运行时检查    | O(1) | O(1)                     |
| `cell.borrow_mut()`                 | 可变借用 `RefMut<T>`，冲突时 panic | O(1) | O(1)                     |
| `try_borrow()` / `try_borrow_mut()` | 返回 `Result`，冲突时不 panic      | O(1) | O(1)                     |
| `cell.into_inner()`                 | 取出内部值                         | O(1) | O(1)                     |

```rust
use std::cell::RefCell;

let cell = RefCell::new(1);
{
    let mut v = cell.borrow_mut();
    *v += 1;
} // 可变借用在这里释放
assert_eq!(*cell.borrow(), 2);

assert!(cell.try_borrow_mut().is_ok());
let _held = cell.borrow();
assert!(cell.try_borrow_mut().is_err()); // 已有不可变借用
```

`RefCell` 允许「多个不可变借用」或「一个可变借用」，但规则在运行时检查：违反时 `borrow_mut()` 直接 panic，`try_borrow_mut()` 返回 `Err`。借用守卫在所在语句或作用域结束时释放，跨语句持有守卫再借用会冲突。

### 12.3 `Rc<RefCell<T>>` 组合

`Rc<RefCell<T>>` 把「共享所有权」和「内部可变性」组合起来：`Rc` 负责多所有者，`RefCell` 负责在共享状态下修改内容。树节点通常写成 `Option<Rc<RefCell<Node>>>`。

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

let tail = Rc::new(RefCell::new(Node { val: 2, next: None }));
let head = Rc::new(RefCell::new(Node {
    val: 1,
    next: Some(Rc::clone(&tail)),
}));

head.borrow_mut().val += 10;
assert_eq!(head.borrow().val, 11);
assert_eq!(tail.borrow().val, 2);
assert_eq!(Rc::strong_count(&tail), 2);

let next = head.borrow().next.clone(); // 克隆 Rc，不复制节点
assert_eq!(next.unwrap().borrow().val, 2);
```

递归或修改多个节点时，先把子节点的 `Rc` 克隆出来再结束借用，不要跨递归层级持有 `borrow()` / `borrow_mut()`，否则下一层再次借用会 panic。`Rc` 的循环引用不会释放：父指针用 `Weak` 保存，读取时用 `Weak::upgrade()` 得到 `Option<Rc<T>>`，并在 `drop` 时自然断开。

## 13. 复杂度速查

| 操作                                                           | 复杂度                                             |
| -------------------------------------------------------------- | -------------------------------------------------- |
| `String::len` / `is_empty` / `capacity`                        | O(1)                                               |
| `String::chars().count()`                                      | O(n)                                               |
| `String::push`                                                 | 均摊 O(1)                                          |
| `String::push_str` / `extend` / `+=`                           | 摊销 O(m)，最坏 O(n + m)                           |
| `String::remove(i)` / `insert(i)` / `insert_str(i)`            | O(n)                                               |
| `String::replace_range` / `drain` / `retain` / `split_off`     | O(n)                                               |
| `String::truncate` / `pop` / `clear`                           | O(1)                                               |
| `String::from_utf8`                                            | O(n) 校验，不复制缓冲                              |
| `String::from_utf8_unchecked`                                  | O(1)，`unsafe`，需保证合法                         |
| `String::from_utf8_lossy`                                      | O(n)，非法字节替换为 U+FFFD                        |
| `FromUtf8Error::into_bytes`                                    | O(1)                                               |
| `String::into_bytes` / `as_bytes` / `as_str`                   | O(1)                                               |
| `String::is_ascii` / `str::is_ascii`                           | O(n)                                               |
| ASCII 判定与 `to_ascii_*`（`char` / `u8`）                     | O(1)                                               |
| `char::to_lowercase` / `to_uppercase`                          | O(字符数)，可能分配                                |
| `String::from_iter` / `collect::<String>`                      | O(输出字节数) 时间/空间                            |
| `str::split` / `split_whitespace` / `lines` / `matches`        | 构造 O(1)，消费 O(n)                               |
| `str::split_once` / `strip_prefix` / `strip_suffix`            | O(m)                                               |
| `slice::join` / `concat`                                       | O(输出长度)                                        |
| `format!` / `to_string`                                        | O(输出字节)，均需分配                              |
| `Vec::push`                                                    | 均摊 O(1)                                          |
| `Vec::len` / `is_empty` / `capacity`                           | O(1)                                               |
| `Vec::with_capacity` / `reserve` / `shrink_to_fit`             | `with_capacity` 一次分配；后两者重分配时 O(n) 复制 |
| `Vec` 下标 `v[i]` / `first()` / `last()`                       | O(1)                                               |
| `vec![...]` / `vec![v; n]`                                     | O(n) 时间/空间，一次分配                           |
| `Vec::clone` / `slice::to_vec` / `to_owned`                    | O(n) 时间，O(n) 空间                               |
| `Vec::insert` / `remove`                                       | O(n)                                               |
| `Vec::append`                                                  | O(m)                                               |
| `Vec::resize`                                                  | O(n)                                               |
| `slice::split_at` / `split_at_mut`                             | O(1)                                               |
| `array::map` / `array::to_vec`                                 | O(n) 时间，O(n) 空间                               |
| `Rc::clone` / `Arc::clone`                                     | O(1)，仅增加引用计数                               |
| `slice::get` / `Vec::get`                                      | O(1)                                               |
| `Iterator::count` / `nth(n)` / `skip(n)`                       | O(n)                                               |
| `Iterator::rev`                                                | 构造 O(1)，消费 O(n)                               |
| `Iterator::next_back` / `rfold`                                | O(1) / O(n)                                        |
| `zip` / `flat_map` / `chain` / `filter`                        | 每个元素 O(1)                                      |
| `collect()`                                                    | O(输出长度) + 可能的扩容                           |
| `str::repeat(n)`                                               | O(输出字节)，一次分配                              |
| `slice::chunks` / `chunks_exact` / `windows`                   | 每产生一个块 O(1)                                  |
| `Iterator::all` / `any` / `eq`                                 | O(n)，可提前短路                                   |
| `Iterator::max` / `min` / `max_by` / `max_by_key`              | O(n)，O(1) 额外空间                                |
| `Iterator::copied` / `cloned`                                  | 每个元素 O(1)                                      |
| `Iterator::find` / `find_map` / `position` / `rposition`       | O(k) 到命中，最坏 O(n)                             |
| `Iterator::fold` / `reduce` / `for_each` / `sum`               | O(n)，O(1) 额外空间                                |
| `Iterator::try_for_each`                                       | O(k)，命中即停                                     |
| `slice::swap` / `std::mem::swap`                               | O(1)                                               |
| `slice::sort` / `sort_by` / `sort_by_key`                      | O(n log n)，稳定版额外 O(n) 空间                   |
| `slice::sort_unstable` / `sort_unstable_by`                    | O(n log n)，O(log n) 栈                            |
| `slice::sort_by_cached_key`                                    | O(n log n)，O(n) 额外空间                          |
| `slice::binary_search` / `partition_point`                     | O(log n)                                           |
| `slice::contains` / `Vec::contains`                            | O(n)                                               |
| `slice::dedup`                                                 | O(n)                                               |
| `Ord::cmp` / `min` / `max`                                     | 标量 O(1)，字符串 O(len)                           |
| `HashMap` / `HashSet` 查找、插入、删除                         | 平均 O(1)，最坏 O(n)                               |
| `BTreeMap` / `BTreeSet` 查找、插入、删除                       | O(log n)                                           |
| `BTreeSet::range` / `BTreeMap::range`                          | O(log n + k)                                       |
| 复合赋值 `+=` / `-=` / `*=` / `/=` / `%=`                      | O(1)                                               |
| `checked_*` / `saturating_*` / `wrapping_*` / `overflowing_*`  | O(1)                                               |
| 位运算 `&` / `\|` / `^` / `!` / `<<` / `>>`                    | O(1)                                               |
| `as` 转换 / `From` / `TryFrom`                                 | O(1)                                               |
| `Rc::new` / `Rc::clone` / `Rc::strong_count` / `Rc::ptr_eq`    | O(1)                                               |
| `RefCell::borrow` / `borrow_mut` / `try_borrow` / `into_inner` | O(1)                                               |

## 14. 易混点与陷阱

| 容易混淆                                          | 区别                                              |
| ------------------------------------------------- | ------------------------------------------------- |
| `len()` vs `chars().count()`                      | 字节数 O(1) / 字符数 O(n)，非 ASCII 不相等        |
| `s[i]` vs `s.chars().nth(i)`                      | 前者不合法；后者按下标取字符，O(n)                |
| `get(range)` vs `&s[range]`                       | 前者返回 `Option<&str>`，越界或非边界返回 `None`  |
| `remove` vs `pop`                                 | 按下标删 O(n) / 删末尾 O(1)                       |
| `remove` vs `truncate`                            | 返回被删字符 / 直接截断无返回值                   |
| `push` vs `push_str` vs `extend` vs `+=`          | 单个 `char` / `&str` / 迭代器 / `AddAssign<&str>` |
| `with_capacity` vs `reserve`                      | 创建时分配 / 对已有容器追加容量                   |
| `vec![x; n]` vs `(0..n).map(...)`                 | 表达式只求值一次并克隆 / 每个元素分别求值         |
| `checked_mul` vs `wrapping_mul`                   | 溢出返回 `None` / 显式按补码回绕                  |
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
| `sort()` vs `sort_unstable()`                     | 稳定、额外 O(n) 空间 / 不稳定、通常更快           |
| `binary_search` vs `contains`                     | 需有序、O(log n) / 无需有序、O(n)                 |
| `split` vs `split_whitespace`                     | 按模式并保留空串 / 按 Unicode 空白并合并连续空白  |
| `join` vs `concat`                                | 带分隔符 / 不带分隔符，均返回新 `String`          |
| `HashMap::iter()` vs `BTreeMap::iter()`           | 任意顺序 / 按键升序                               |
| `contains_key` + `insert` vs `entry`              | 两次查找 / 一次查找                               |
| `set.insert(x)` vs `set.contains(&x)`             | 插入并返回是否新增 / 只判断是否存在               |
| `BTreeSet::first()` vs `pop_first()`              | 借用 `Option<&T>` / 移除并返回 `Option<T>`        |
| `Rc::clone` vs 内部值的 `clone`                   | 只增加引用计数 O(1) / 复制内部数据                |
| `as` vs `TryFrom`                                 | 静默截断或回绕 / 可失败的受检转换                 |
| `a >> 1` vs `a / 2`                               | 有符号数算术右移 / 向零截断，负数结果不同         |

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
- `v.len() - 1` 在空 `Vec` 上会下溢 panic（debug 构建），边界计算先判空，或用 `checked_sub(1)` / `saturating_sub(1)`。
- `s.into_bytes()` 是 O(1) 的所有权移交，调用后原 `String` 不可再用；只想查看用 `as_bytes()`。
- `String::from_utf8` 失败时数据不会丢失，`FromUtf8Error::into_bytes()` 可以把原 `Vec<u8>` 取回。
- ASCII 数据用 `Vec<char>` 处理会多占约 4 倍内存；能按字节处理时优先按字节，且只交换 ASCII 字节不会破坏 UTF-8。
- `clone()` 是否为深拷贝取决于类型实现：`Vec` / `String` 会复制堆数据，`Rc` / `Arc` 只增加引用计数。
- `vec![expr; n]` 只对 `expr` 求值一次，然后把结果克隆 n 份；表达式带副作用时不要期待执行 n 次。
- 整数溢出行为取决于构建配置：debug 构建 panic，release 构建默认回绕；需要确定行为时用 `checked_*` / `saturating_*` / `wrapping_*`。
- `rev()` 只适用于 `DoubleEndedIterator`；只实现了 `Iterator` 的迭代器不能直接反向。
- 哈希集合 / 映射的遍历顺序不做保证；需要确定性输出时排序键，或改用 `BTreeMap` / `BTreeSet`。
- `sort` 稳定但需要 O(n) 额外空间；`sort_unstable` 可能打乱相等元素的相对顺序。
- `sort_by` 的比较器必须构成全序且自洽，否则结果未定义，较新的标准库可能 panic。
- `binary_search` 只适用于有序数据；在未排序切片上调用不会报错，但结果没有意义。
- `dedup` 只删除相邻重复，去全部重复要先 `sort`。
- `or_insert(v)` 会先求值 v，即使键已存在；需要构造开销时改用 `or_insert_with`。
- `contains_key` 后再 `insert` 会做两次哈希查找；能用 `entry` 时优先用 `entry`。
- `HashMap` / `HashSet` 的键必须实现 `Eq + Hash`，`f64` 不满足；有序树要求 `Ord`，`f64` 同样不满足。
- `HashMap<&K, _>` 中的键是借用（例如 `&str` 或 `&i32`），被引用数据必须先于映射存活；需要长期持有时改用拥有所有权的键。
- `map.iter()` 产出的键和值都是引用；闭包里比较数值时注意解引用层数，必要时用 `**v` 或 `&v` 模式。
- `for_each` / `map` 闭包里的 `return` 只退出当前闭包调用，不会从外层函数返回。
- `find` 的闭包收到 `&Item`，`position` / `rposition` 收到 `Item`；对 `iter()` 使用 `find` 时要多解一层引用（如 `|&&x|`）。
- `Vec::insert(i)` 允许 `i == len`，`Vec::remove(i)` 要求 `i < len`，否则 panic。
- `Rc` 不是 `Send` / `Sync`，跨线程共享用 `Arc`；`Rc` 循环引用会泄漏，父指针用 `Weak`。
- `RefCell` 的借用冲突在运行时 panic；不要跨语句或递归层级持有 `borrow()` / `borrow_mut()` 守卫。
- `as` 转换不会 panic：负的 `i32` 转 `usize` 会变成极大值，窄化转换会截断；需要检查时用 `TryFrom`。
- 有符号数 `>>` 是算术右移，负数 `>> 1` 与 `/ 2` 结果不同；移位量越界在 debug 下 panic。
- `split` 会保留末尾空串，`split_terminator` 不会；`split_whitespace` 会跳过并合并空白。
- `join` 只接受能借出 `str` 的元素，整数需要先 `map` 成字符串。
- `dbg!` 输出到 stderr 并返回原值，适合临时调试，不要留在生产路径。
- `array::map` 立即求值且长度固定，不是惰性迭代器。

## 15. 测试常用

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

子模块可以通过 `super::` 访问父模块的项（包括私有项），通常用 `use super::*;` 简化路径；只用到单个类型时，也可以写 `use super::MyType;`。比较两个 `Vec` 或数组切片时可以直接用 `assert_eq!`，`Vec<T>` 实现了 `PartialEq`，断言失败会通过 `Debug` 打印两边内容。

测试函数也可以返回 `Result`，直接对 `parse` 等操作使用 `?`：

```rust
#[test]
fn parse_ok() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

`#[derive(Debug, PartialEq, Eq)]` 常用于测试中的自定义结构体：`assert_eq!` 需要 `Debug` 与 `PartialEq`，`Eq` 表示相等关系满足自反、对称、传递。比较 `Rc<RefCell<T>>` 时会递归比较内部值；只关心是否为同一份数据时用 `Rc::ptr_eq`。

## 16. 参考

- [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [`std::string::FromUtf8Error`](https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html)
- [`str` primitive](https://doc.rust-lang.org/std/primitive.str.html)
- [`char` primitive](https://doc.rust-lang.org/std/primitive.char.html)
- [`u8` primitive](https://doc.rust-lang.org/std/primitive.u8.html)
- [`matches!` macro](https://doc.rust-lang.org/std/macro.matches.html)
- [`std::str::Chars`](https://doc.rust-lang.org/std/str/struct.Chars.html)
- [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [`std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`std::mem`](https://doc.rust-lang.org/std/mem/index.html)
- [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html)
- [`std::ops`](https://doc.rust-lang.org/std/ops/index.html)
- [`slice` primitive](https://doc.rust-lang.org/std/primitive.slice.html)
- [`array` primitive](https://doc.rust-lang.org/std/primitive.array.html)
- [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [`std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- [`std::collections::hash_map::Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
- [`std::cmp::Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
- [`std::cmp::Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)
- [`std::cmp::Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)
- [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [`std::cell::RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [`std::ops::ControlFlow`](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html)
- [`std::convert::TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
- [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- [`dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html)
- [`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`std::iter::FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)
- [`std::ops::Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)
- [`std::ops::MulAssign`](https://doc.rust-lang.org/std/ops/trait.MulAssign.html)
- [`std::ops::Range`](https://doc.rust-lang.org/std/ops/struct.Range.html)
- [`i32` primitive](https://doc.rust-lang.org/std/primitive.i32.html)
- [`itertools::Itertools`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)（非标准库）
