# Java ↔ Rust 核心差异对照表

## 背景与目的

Java 语言和 Rust 语言存在差异，在一些语法上不能做到一一对应，因此需要进行转换。以下是完整的差异对照表，`verify_rewrite.py` 的 `_compare_method_logic` 会对这些情况进行重点检查与校验。

因为Java 语法 和 Rust 语法不一样，一行Java 代码对应一行Rust 代码可能行不通，但是可以实现一个Java 语句 对应 一个Rust 语句。

### 对于Java 和 Rust 转写，需要遵循以下类型安全原则：

> `**verify_rewrite.py` 强制校验**: 以下禁止类型会在文件级和方法体级两层扫描，
> 检测到即报 ❌ 错误，不通过验证。

### ✅ 可以放心使用的 3 种类型（业务代码推荐）

**1. `<T>` 编译期泛型 — 最推荐、首选、必须优先用**

对应 Java：

```java
<T> T method(T t) { ... }
```

Rust：

```rust
fn method<T>(t: T) -> T { t }
```

完全安全、零成本、类型安全、无任何风险。

**2. `enum` 枚举（固定多类型） — Rust 特色，非常推荐**

对应 Java：

```java
Object value = 某个固定类型;
```

Rust：

```rust
enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
}
```

类型安全、无向下转型、不会崩溃。

**3. `dyn Trait` trait 对象 — 正统替代 Java Object/Interface**

对应 Java：

```java
List<MyInterface> list = new ArrayList<>();
```

Rust：

```rust
trait MyInterface { ... }
let list: Vec<Box<dyn MyInterface>> = vec![];
```

Rust 官方推荐的动态类型。业务代码最常用、最安全、最接近 Object 的正统写法。

### ❌ 禁止使用的 2 种类型（verify_rewrite.py 强制报错）

**4. `dyn Any` / `Box<dyn Any>` ❌ 禁止业务代码使用**

- 类型不安全，强制向下转型（`downcast_ref`/`downcast_mut`）
- 运行时类型匹配失败会 panic，崩溃
- 代码难维护，隐藏了真实类型信息
- 只有底层框架、反射、插件系统才用
- Java 转 Rust 99% 场景都不应该用
- `**verify_rewrite.py` 校验**: 扫描 Rust 文件中的 `dyn Any`、`Box<dyn Any>`、
`&dyn Any`、`downcast_ref`、`downcast_mut` 关键字，检测到即报错

**正确做法**: 用 `enum` 或 `dyn Trait` 替代。例如 Java `Object value` 可存储
`String` / `Double` / `Boolean`，Rust 应用：

```rust
enum CellValue {
    Str(String),
    Dbl(f64),
    Bool(bool),
}
```

**5. `*const ()` / `*mut ()` ❌ 极底层、unsafe、禁止业务使用**

- 对应 C 的 `void`*
- 只有 FFI、操作系统、底层库才会用
- Java 转 Rust 业务代码永远不要碰
- `**verify_rewrite.py` 校验**: 扫描 `*const ()`、`*mut ()`、`*const T`（非 FFI 模块）、
`*mut T`（非 FFI 模块）关键字，检测到即报错

### 类型安全总结


| 类别                              | 类型       | 是否允许            | verify_rewrite.py |
| ------------------------------- | -------- | --------------- | ----------------- |
| `<T>` 泛型                        | 编译期多态    | ✅ 首选            | —                 |
| `enum` 枚举                       | 固定多类型    | ✅ 推荐            | —                 |
| `dyn Trait`                     | 动态多态     | ✅ 正统替代 Object   | —                 |
| `dyn Any` / `Box<dyn Any>`      | 运行时反射    | ❌ 禁止            | 文件级+方法体级扫描，报错     |
| `*const ()` / `*mut ()`         | 裸指针      | ❌ 禁止            | 文件级+方法体级扫描，报错     |
| `downcast_ref` / `downcast_mut` | Any 向下转型 | ❌ 禁止（暗示 Any 使用） | 文件级扫描，报错          |


### 类型对应校验（verify_rewrite.py 强制报错）

> Rust struct 字段类型必须与 Java 类中对应成员变量的类型**语义一致**，
> 不允许将对象类型替换为无关的基础类型（如 `i32`、`String`）。
> `verify_rewrite.py` 会在字段对照阶段检查类型对应关系。

**6. Rust 字段类型必须与 Java 字段类型对应 ❌ 禁止类型篡改**

Java 类型到 Rust 类型的标准映射：


| Java 类型                         | Rust 正确类型                         | 说明                       |
| ------------------------------- | --------------------------------- | ------------------------ |
| `int`                           | `i32`                             | 基础类型直接对应                 |
| `long`                          | `i64`                             | 基础类型直接对应                 |
| `short`                         | `i16`                             | 基础类型直接对应                 |
| `byte`                          | `i8` / `u8`                       | 基础类型直接对应                 |
| `float`                         | `f32`                             | 基础类型直接对应                 |
| `double`                        | `f64`                             | 基础类型直接对应                 |
| `boolean`                       | `bool`                            | 基础类型直接对应                 |
| `char`                          | `char`                            | 基础类型直接对应                 |
| `String`                        | `String` / `Option<String>`       | 字符串类型                    |
| `StringBuilder`                 | `String`                          | Rust 用 String 替代         |
| `Integer` / `Long` / `Double` 等 | `Option<i32>` / `Option<i64>` 等   | 装箱类型→Option 包装           |
| `List<T>` / `ArrayList<T>`      | `Vec<T>`                          | 集合类型                     |
| `Map<K,V>` / `HashMap<K,V>`     | `HashMap<K,V>`                    | 映射类型                     |
| `Set<T>` / `HashSet<T>`         | `HashSet<T>`                      | 集合类型                     |
| `Object`                        | `enum` / `dyn Trait` / `T` 泛型     | 禁止 `dyn Any`             |
| `SomeClass`（自定义类）               | `SomeClass` / `Option<SomeClass>` | **必须保留原类型名，禁止替换为 i32 等** |


**错误示例**：

```rust
// ❌ 错误：Java 类型是 DocumentEnv / ViewEnv / StringBuilder，
//    但 Rust 类型被替换为 i32 / String
pub struct AutoNumLayouter {
    pub mDocEnv: i32,       // ❌ Java: DocumentEnv → 应为 Option<DocumentEnv>
    pub mViewEnv: i32,      // ❌ Java: ViewEnv → 应为 Option<ViewEnv>
    pub mResult: String,    // ⚠️ Java: StringBuilder → String 可接受（标准映射）
}
```

**正确示例**：

```rust
// ✅ 正确：类型与 Java 一致
pub struct AutoNumLayouter {
    pub mDocEnv: Option<DocumentEnv>,   // Java: DocumentEnv
    pub mViewEnv: Option<ViewEnv>,      // Java: ViewEnv
    pub mResult: String,                // Java: StringBuilder → String (标准映射)
}
```

- **verify_rewrite.py 校验**: 对比 Java 和 Rust 的字段类型，若 Java 字段是自定义类类型
（非基础类型 int/long/boolean/String 等）而 Rust 字段是基础类型（i32/i64/f64/String/bool），
则报错：类型不对应，Java 类类型不能降级为基础类型。

### Java 语句与和Rust 语句对应表：


| 序号  | Java 语句类型        | Java 代码示例                                | Rust 对应语句                                   | 校验规则（verify_rewrite.py 可用）                                                                           |
| --- | ---------------- | ---------------------------------------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| 1   | 成员变量赋值           | `this.field = new X();`                  | `self.field = Some(X::new());`              | 必须赋值给 self                                                                                           |
| 2   | 成员变量赋值（无包装）      | `this.value = 100;`                      | `self.value = 100;`                         | 直接赋值                                                                                                 |
| 3   | 空值判断             | `if (obj != null)`                       | `if obj.is_some()`                          | 非空判断                                                                                                 |
| 4   | 空值判断             | `if (obj == null)`                       | `if obj.is_none()`                          | 空判断                                                                                                  |
| 5   | 安全调用成员           | `if (obj != null) obj.method()`          | `if let Some(v) = obj { v.method() }`       | 安全解包调用                                                                                               |
| 6   | 方法调用             | `this.method();`                         | `self.method();`                            | 成员方法                                                                                                 |
| 7   | 方法调用（带参）         | `this.parse(a, b);`                      | `self.parse(a, b);`                         | 一一对应                                                                                                 |
| 8   | 构造对象             | `X x = new X();`                         | `let x = X::new();`                         | 构造函数                                                                                                 |
| 9   | 构造并赋值            | `this.f = new X(a, b);`                  | `self.f = Some(X::new(a, b));`              | 构造 + 成员赋值                                                                                            |
| 10  | if 语句            | `if (a > 0) { ... }`                     | `if a > 0 { ... }`                          | 条件完全一致                                                                                               |
| 11  | if else          | `if (a) b; else c;`                      | `if a { b } else { c }`                     | 结构一致                                                                                                 |
| 12  | return 返回        | `return value;`                          | `return value;`                             | 相同                                                                                                   |
| 13  | return 错误码       | `return this.nRet = 2;`                  | `self.n_ret = 2; return 2;`                 | 两步对应                                                                                                 |
| 14  | 抛出异常             | `throw new XException();`                | `return Err(Error::X);`                     | 异常→Result 错误                                                                                         |
| 15  | try 代码块          | `try { ... }`                            | `match ... { Ok(_) => ..., Err(e) => ... }` | 错误处理                                                                                                 |
| 16  | catch 异常         | `catch (IOException e) { return 2; }`    | `Err(e) => { self.n_ret = 2; return 2; }`   | 异常类型→错误分支                                                                                            |
| 17  | catch 异常         | `catch (XException e) { ... }`           | `Err(Error::X(e)) => { ... }`               | 按错误类型匹配                                                                                              |
| 18  | 局部变量             | `String s = "abc";`                      | `let s = "abc";`                            | 变量定义                                                                                                 |
| 19  | 可变局部变量           | `int i = 0; i = 1;`                      | `let mut i = 0; i = 1;`                     | 必须带 mut                                                                                              |
| 20  | 数组 / 列表创建        | `List<String> list = new ArrayList<>();` | `let mut list = Vec::new();`                | Vec 对应 ArrayList                                                                                     |
| 21  | 添加元素             | `list.add("a");`                         | `list.push("a".to_string());`               | push 对应 add                                                                                          |
| 22  | 字符串拼接            | `sb.append("a");`                        | `s.push_str("a");`                          | StringBuilder → String                                                                               |
| 23  | 获取长度             | `str.length()`                           | `str.len()`                                 | 方法对应                                                                                                 |
| 24  | synchronized 锁   | `synchronized(obj) { ... }`              | `let _g = obj.lock().unwrap();`             | Mutex 锁                                                                                              |
| 25  | volatile 读       | `int v = value;`                         | `let v = value.load(SeqCst);`               | 原子读                                                                                                  |
| 26  | volatile 写       | `value = 10;`                            | `value.store(10, SeqCst);`                  | 原子写                                                                                                  |
| 27  | 父类调用             | `super.method(a, b);`                    | `self.parent.method(a, b);`                 | **必须**用组合替代继承，struct 中定义 `parent: ParentType` 字段，通过 `self.parent.method()` 调用；verify_rewrite.py 强制校验 |
| 28  | 类型强转             | `(String) obj`                           | 使用 enum match 或泛型                           | 禁止使用 `downcast_ref`（暗示 dyn Any），应用 enum/trait 替代                                                     |
| 29  | 实例类型判断           | `obj instanceof X`                       | `obj.is::<X>()`                             | 类型判断                                                                                                 |
| 30  | for 循环           | `for (int i=0; i<5; i++)`                | `for i in 0..5`                             | 区间循环                                                                                                 |
| 31  | 遍历集合             | `for (String s : list)`                  | `for s in &list`                            | 迭代遍历                                                                                                 |
| 32  | 三元运算符            | `int x = a > 0 ? a : b;`                 | `let x = if a > 0 { a } else { b };`        | if 表达式替代三元                                                                                           |
| 33  | 静态方法调用           | `ClassName.staticMethod(a);`             | `ClassName::static_method(a);`              | 静态→关联函数                                                                                              |
| 34  | 静态字段访问           | `ClassName.CONSTANT`                     | `ClassName::CONSTANT`                       | 静态字段→关联常量                                                                                            |
| 35  | 类型转换(基础)         | `(int)x`                                 | `x as i32`                                  | as 转换                                                                                                |
| 36  | 数组访问             | `arr[i]`                                 | `arr[i]`                                    | 索引访问                                                                                                 |
| 37  | 数组长度             | `arr.length`                             | `arr.len()`                                 | 属性→方法                                                                                                |
| 38  | Map.get          | `map.get(key)`                           | `map.get(&key)`                             | get 对应                                                                                               |
| 39  | Map.put          | `map.put(key, value)`                    | `map.insert(key, value)`                    | put → insert                                                                                         |
| 40  | Map.containsKey  | `map.containsKey(key)`                   | `map.contains_key(&key)`                    | 方法对应                                                                                                 |
| 41  | Map.remove       | `map.remove(key)`                        | `map.remove(&key)`                          | 方法对应                                                                                                 |
| 42  | List.get         | `list.get(i)`                            | `list[i]` 或 `list.get(i)`                   | 索引访问                                                                                                 |
| 43  | List.size        | `list.size()`                            | `list.len()`                                | 方法对应                                                                                                 |
| 44  | List.remove      | `list.remove(i)`                         | `list.remove(i)`                            | 方法对应                                                                                                 |
| 45  | String.charAt    | `s.charAt(i)`                            | `s.chars().nth(i)` 或 `s.as_bytes()[i]`      | 字符访问                                                                                                 |
| 46  | String.substring | `s.substring(a, b)`                      | `&s[a..b]` 或 `s.get(a..b)`                  | 子串截取                                                                                                 |
| 47  | String.indexOf   | `s.indexOf("x")`                         | `s.find("x")`                               | 查找方法                                                                                                 |
| 48  | String.equals    | `s1.equals(s2)`                          | `s1 == s2`                                  | 运算符替代                                                                                                |
| 49  | switch 语句        | `switch(x) { case 1: ... }`              | `match x { 1 => { ... } }`                  | switch→match                                                                                         |
| 50  | do-while 循环      | `do { ... } while (cond);`               | `loop { ... if !cond { break; } }`          | loop + break                                                                                         |
| 51  | null 赋值          | `this.field = null;`                     | `self.field = None;`                        | null → None                                                                                          |
| 52  | 类型判断后强转          | `if (x instanceof T) { T t = (T)x; }`    | `if let Some(t) = x.as_T() { ... }`         | enum match / if let                                                                                  |


---

## 1. 基础语法


| #   | 场景         | Java 代码示例                     | Rust 改写规则                          | Rust 正确代码                                | 验证要点                                                    |
| --- | ---------- | ----------------------------- | ---------------------------------- | ---------------------------------------- | ------------------------------------------------------- |
| 1.1 | 字段赋值       | `this.mField = new X();`      | `self.字段 = X::new()`, 可空用 `Option` | `self.mField = Some(X::new());`          | `this.xxx =` 必须对应 `self.xxx =`，禁止赋给局部变量                 |
| 1.2 | null 值     | `String s = null;`            | 禁止 null，必须用 `Option` 包装            | `let s: Option<String> = None;`          | Java `null` → Rust `None`；Java `!= null` → `.is_some()` |
| 1.3 | 可变性        | `final int x = 1; int y = 2;` | 默认不可变（对应 final），可变需加 `mut`         | `let x = 1; let mut y = 2;`              | —                                                       |
| 1.4 | 无返回值       | `void method() {}`            | 无返回值省略 `-> ()`                     | `fn method(&self) {}`                    | —                                                       |
| 1.5 | 类型转换       | `(int)x` / `(String)obj`      | `as i32` / 具体类型 downcast           | `x as i32`                               | Java 强制转换 → Rust `as` 或类型匹配                             |
| 1.6 | instanceof | `obj instanceof String`       | 无直接对应，用 trait/enum 匹配              | `obj.downcast_ref::<String>().is_some()` | —                                                       |


## 2. 面向对象


| #   | 场景       | Java 代码示例                            | Rust 改写规则              | Rust 正确代码                                              | 验证要点                                                                                     |
| --- | -------- | ------------------------------------ | ---------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
| 2.1 | 继承       | `class A extends B {}`               | 无继承，用组合（Composition）替代 | `struct A { parent: B }`                               | `extends` → struct 中必须定义 `parent: B` 组合字段                                                |
| 2.2 | super 调用 | `super.method();`                    | 调用组合的 parent 字段方法      | `self.parent.method();`                                | **verify_rewrite.py 强制校验**: Java `super.xxx()` 必须映射为 `self.parent.xxx()`，不得忽略、跳过或用其他方式替代 |
| 2.3 | 接口       | `interface I { void f(); }`          | 用 `trait` 替代           | `trait I { fn f(&self); }`                             | —                                                                                        |
| 2.4 | 方法重载     | `void f(int a)` / `void f(String b)` | Rust 不支持重载，用不同方法名      | `fn f_int(&self, a: i32)` / `fn f_str(&self, b: &str)` | Java 同名方法 → Rust 不同名（后缀区分）                                                               |
| 2.5 | 静态方法     | `static void foo()`                  | 无 `self` 参数的关联函数       | `fn foo() {}` （无 `&self`）                              | —                                                                                        |


## 3. 异常处理


| #   | 场景        | Java 代码示例                          | Rust 改写规则                            | Rust 正确代码                                     | 验证要点                                   |
| --- | --------- | ---------------------------------- | ------------------------------------ | --------------------------------------------- | -------------------------------------- |
| 3.1 | throw     | `throw new Exception();`           | `panic!()` 或 `return Err(...)`       | `panic!("exception");` 或 `return Err(e);`     | **禁止** `throw` → `return`（裸 return 错误） |
| 3.2 | try-catch | `try { f(); } catch (E e) { ... }` | `match f() { Ok/Err }` 或 `?` + catch | `match f() { Ok(v) => v, Err(e) => { ... } }` | try 块计数应体现为 match/if-let               |
| 3.3 | finally   | `try { ... } finally { ... }`      | Rust 无 finally，用 Drop 或显式代码          | 在函数末尾放 cleanup 代码                             | finally 内代码必须保留                        |


## 4. 集合 / 字符串


| #   | 场景            | Java 代码示例                           | Rust 改写规则                       | Rust 正确代码                                     | 验证要点                                            |
| --- | ------------- | ----------------------------------- | ------------------------------- | --------------------------------------------- | ----------------------------------------------- |
| 4.1 | ArrayList     | `new ArrayList<>()`                 | `Vec::new()`                    | `let mut list = Vec::new();`                  | `add()` → `push()`；`get(i)` → `[i]` 或 `.get(i)` |
| 4.2 | HashMap       | `new HashMap<>()`                   | `HashMap::new()`                | `let mut map = HashMap::new();`               | `put()` → `insert()`；`get()` → `.get()`         |
| 4.3 | StringBuilder | `new StringBuilder()` / `.append()` | `String::new()` / `.push_str()` | `let mut s = String::new(); s.push_str("a");` | `append()` → `push_str()`                       |
| 4.4 | String 比较     | `s1.equals(s2)`                     | `==` 运算符                        | `s1 == s2`                                    | —                                               |
| 4.5 | 数组            | `new int[10]`                       | `vec![0; 10]`                   | `let arr = vec![0i32; 10];`                   | —                                               |


## 5. 循环 / 遍历


| #   | 场景          | Java 代码示例                  | Rust 改写规则                                         | Rust 正确代码                                 | 验证要点                            |
| --- | ----------- | -------------------------- | ------------------------------------------------- | ----------------------------------------- | ------------------------------- |
| 5.1 | for-each    | `for (T x : list) { ... }` | `for x in &list` (只读) / `for x in &mut list` (可变) | `for x in &list { }`                      | for 计数应一致                       |
| 5.2 | C-style for | `for (int i=0; i<10; i++)` | `for i in 0..10` 或 `0..=9`                        | `for i in 0..10 { }`                      | —                               |
| 5.3 | while       | `while (cond) { ... }`     | `while cond { ... }`                              | `while cond { }`                          | while 计数一致                      |
| 5.4 | do-while    | `do { ... } while (cond);` | `loop { ... if !cond { break; } }`                | `loop { /* body */ if !cond { break; } }` | Java do-while → Rust loop+break |


## 6. 锁 / 并发


| #   | 场景              | Java 代码示例                       | Rust 改写规则              | Rust 正确代码                                                    | 验证要点                        |
| --- | --------------- | ------------------------------- | ---------------------- | ------------------------------------------------------------ | --------------------------- |
| 6.1 | synchronized 方法 | `synchronized void f()`         | `Mutex` 包裹共享数据         | `let mut data = self.data.lock().unwrap();`                  | synchronized → Mutex.lock() |
| 6.2 | synchronized 块  | `synchronized(lockObj) { ... }` | 显式 Mutex lock 代码块      | `{ let guard = lock.lock().unwrap(); ... }`                  | —                           |
| 6.3 | ReadWriteLock   | `ReentrantReadWriteLock`        | `RwLock`               | `self.value.read().unwrap()` / `self.value.write().unwrap()` | —                           |
| 6.4 | volatile        | `volatile int x`                | `AtomicI32`            | `AtomicI32::new(0)`                                          | —                           |
| 6.5 | wait/notify     | `obj.wait()` / `obj.notify()`   | `Condvar` + `Mutex`    | `cvar.wait(guard).unwrap()` / `cvar.notify_one()`            | —                           |
| 6.6 | Thread          | `new Thread(() -> {}).start()`  | `std::thread::spawn`   | `thread::spawn(move                                          |                             |
| 6.7 | Arc 共享          | —                               | `Arc<Mutex<T>>` 线程安全共享 | `Arc::new(Mutex::new(data))`                                 | —                           |


## 7. 类型系统


| #   | 场景        | Java 代码示例         | Rust 改写规则                                   | Rust 正确代码                                          | 验证要点                                                                                                              |
| --- | --------- | ----------------- | ------------------------------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| 7.1 | Object 类型 | `Object obj`      | **禁止** `Box<dyn Any>`，应使用 enum/泛型/dyn Trait | 使用 `enum Value { Str(String), Dbl(f64) }` 或 `T` 泛型 | **verify_rewrite.py 强制报错**: 检测到 `dyn Any`/`Box<dyn Any>`/`downcast_ref`/`downcast_mut`/`*const ()`/`*mut ()` 立即报错 |
| 7.2 | 泛型        | `List<String>`    | `Vec<String>`                               | `Vec<String>`                                      | —                                                                                                                 |
| 7.3 | 枚举        | `enum E { A, B }` | `enum E { A, B }`                           | `enum E { A, B }`                                  | —                                                                                                                 |
| 7.4 | 自动装箱      | `Integer x = 1`   | 直接使用基础类型                                    | `let x: i32 = 1;`                                  | `Integer` → `i32`，`Boolean` → `bool` 等                                                                            |


---

## 常见错误示例（Wrong Examples）

### 错误 1：`this.field = X` 赋值给局部变量而非 self

```java
// Java:
this.mCodeLex = new FieldCodeLex();
this.mCodeLex.parse(codeBuffer, codeBuffer.length);
```

**错误 Rust（局部变量赋值）:**

```rust
let mut code_lex = FieldCodeLex::new();      // ❌ 未赋值给 self
let _ = code_lex.parse(code_buffer, code_buffer.len());
```

**正确 Rust:**

```rust
// Java: this.mCodeLex = new FieldCodeLex();
self.mCodeLex = Some(FieldCodeLex::new());
// Java: this.mCodeLex.parse(codeBuffer, codeBuffer.length);
self.mCodeLex.as_mut().unwrap().parse(code_buffer, code_buffer.len());
```

### 错误 2：Java throw → Rust return（禁止）

```java
// Java:
throw new UnExpectedEndFormatException();
```

**错误 Rust:**

```rust
return;  // ❌ 异常被静默吞掉
```

**正确 Rust:**

```rust
// Java: throw new UnExpectedEndFormatException();
panic!("UnExpectedEndFormatException");
```

### 错误 3：super 调用被忽略或错误转换

```java
// Java:
super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
```

**错误 Rust:**

```rust
self.mSubDocument = Some(subDoc.clone()); // ❌ 完全不等价
```

**正确 Rust:**

```rust
// Java: super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
self.parent.parseByCharBuffer(code_buffer, sub_doc, field_type);
```

### 错误 4：锁转换缺少 synchronized 对应结构

```java
// Java:
synchronized(this.mWaitLock) {
    if (condition) { this.mWaitLock.wait(this.mWaitMillis); }
}
```

**正确 Rust:**

```rust
// Java: synchronized(this.mWaitLock) {
{
    let guard = self.mWaitLock.lock().unwrap();
    // Java: if (condition) { this.mWaitLock.wait(this.mWaitMillis); }
    if condition {
        let _guard = self.mWaitCvar.wait_timeout(guard, Duration::from_millis(self.mWaitMillis as u64)).unwrap();
    }
// Java: }
}
```

