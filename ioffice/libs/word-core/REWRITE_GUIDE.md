# Java → Rust MCP Pipeline 重写指引

## 概述

本项目使用 MCP (Model Context Protocol) 流水线驱动 Java 到 Rust 的渐进式重写。
AI Agent 通过两个 MCP 工具 (`pipeline_start`, `pipeline_continue`) 与流水线交互，
逐文件完成代码转换。每个 Rust crate 拥有独立的 `rewrite_config.json` 配置文件。

---

## 文件说明

每个 Rust crate 目录下包含以下文件：

| 文件 | 作用 |
|------|------|
| `rewrite_config.json` | **crate 配置**：Java/Rust 路径映射、CSV 文件名等 |
| `rewrite_modules.csv` | 模块级计划：按依赖顺序排列的代码目录列表 |
| `rewriting_files.csv` | 文件级队列：当前正在重写的文件 |
| `rewriter_history_steady.csv` | 已确认完成的文件历史记录 |
| `current_progress.json` | 运行时状态：当前 plan 的进度快照 |

---

## 核心流程

```
┌─────────────────────────────────────────────────────┐
│  pipeline_start(config)                             │
│  ├─ 恢复已有 plan (current_progress.json)           │
│  ├─ 或从 rewriting_files.csv 取下一个文件           │
│  ├─ 或从 rewrite_modules.csv 取下一个模块的文件     │
│  └─ 执行 gen_template → 输出模板 + 改写指令         │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────┐
│  AI Agent 根据模板改写 Rust 文件                     │
│  遵循重写规则（见下方"重写规则"章节）                │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────┐
│  pipeline_continue(config)                          │
│  ├─ 运行 check (一致性检查 + cargo check)           │
│  │   ├─ 失败 → 返回错误信息，AI修复后重新调用       │
│  │   └─ 通过 → 标记文件完成                         │
│  └─ 自动运行 pipeline_start 获取下一个文件           │
└─────────────────────────────────────────────────────┘
```

---

## MCP 工具详解

### `pipeline_start`

启动流水线，按以下优先级查找要处理的文件：

1. **恢复已有 plan** — 如果 `current_progress.json` 存在且有未完成文件
2. **rewriting_files.csv** — 从上到下找第一个非 steady/review 的文件
3. **rewrite_modules.csv** — 找第一个 in_progress/pending 的模块，挑选未转换的文件

参数：
- `config` (可选) — `rewrite_config.json` 的绝对路径（未提供则自动从 CWD 检测）
- `javaFile` (可选) — 指定要处理的 Java 文件
- `force` (可选) — 强制重新处理已完成的文件
- `restart` (可选) — 忽略进度，从头开始

输出：
- 生成 Rust 模板文件
- 输出模板路径和改写指令

### `pipeline_continue`

改写完成后调用，执行验证并推进流水线：

参数：
- `config` (可选) — `rewrite_config.json` 的绝对路径（未提供则自动从 CWD 检测）
- `rustPath` (可选) — 刚改写完的 Rust 文件路径

执行步骤：
1. **一致性检查** — 验证 `// Java:` 注释完整保留
2. **cargo check** — 编译检查
3. 若通过 → 标记完成，自动启动下一个文件
4. 若失败 → 返回错误信息和修复指令

## 重要规则

1. **必须遵循 MCP Pipeline 流程** — 不能跳过步骤或并行处理
2. **每次改写后必须调用 `pipeline_continue`** — 这是唯一推进流水线的方式
3. **仔细阅读工具返回信息** — 包含错误详情和下一步指令
4. **遵守重写规则** — 模板保留、类型映射、命名规则等（见下方"重写规则"章节）
5. **不忽略任何报错** — 所有 error 和 warning 必须解决后文件才算完成
6. **简单文件自动标记 steady** — 通过 check 后如果是纯接口/枚举/常量等简单文件，自动为 steady
7. **复杂文件标记 review** — 包含逻辑的文件标记为 review，等待人工确认

---

## 文件状态流转

```
pending → (pipeline处理) → steady   (简单文件，自动完成)
pending → (pipeline处理) → review   (复杂文件，等待人工确认)
review  → (人工确认通过) → steady
fail    → (修复后重跑)   → steady/review
```

---

## 模块处理顺序

`rewrite_modules.csv` 按依赖关系排列，每个 crate 有自己的模块定义。
每个模块内的文件按字母序处理。当前模块所有文件完成后，自动进入下一个模块。

---

## 重写规则

### 1. 模板保留原则

- **100% 保留模板结构**：不删除 `// Java:` 注释、struct/field/method 声明
- **只填写 `// TODO_RUST_CODE`**：在标记处补充等价 Rust 代码
- **不修改命名**：类、方法、变量名保持与 Java 一致，便于溯源对比
- **方法重载**：Rust 无重载，使用后缀命名 `OriginName_Suffix`（如 `obtain_from`, `DefTable_from_with`）

### 2. 类型映射

| Java | Rust |
|------|------|
| `int` | `i32` |
| `long` | `i64` |
| `float` | `f32` |
| `double` | `f64` |
| `boolean` | `bool` |
| `String` | `String` |
| `int[]` | `Vec<i32>` 或 `&[i32]` |
| `null` | `Option<T>` |
| `static final` | `pub const` 或 `pub(crate) const` |

### 3. 共享引用与对象池 (Object Pool)

Java 中的对象池模式（`synchronized` + `HashMap` + `Stack`）在 Rust 中的等价实现：

#### 3.1 池存储：使用 `Arc<T>` 共享不可变数据

```rust
use std::sync::{Arc, Mutex, OnceLock};

// Java: private static HashMap<K, V> sPool
static S_POOL: OnceLock<Mutex<HashMap<i64, Arc<FrozenType>>>> = OnceLock::new();
```

- `obtain()`/`freeze()` 返回 `Arc<T>`，调用 `Arc::clone()` 只增加引用计数（极快），不复制数据
- 等价于 Java 中多个变量指向同一个 Object

#### 3.1.1 Java 引用返回 vs Rust `Arc` 返回

Java 对象池中 `return cached` 返回的是池中对象的**引用**（多个调用者共享同一实例）。
Rust 中 **禁止** 使用 `cached.clone()` 返回（这会完整复制数据，违背池化初衷）。

| 场景 | Java | Rust（正确） | Rust（错误） |
|------|------|-------------|-------------|
| 池命中 | `return cached;` | `return Arc::clone(cached);` | `return cached.clone();` |
| 池未命中 | `sPool.put(obj, obj); return obj;` | `let arc = Arc::new(obj); pool.insert(k, Arc::clone(&arc)); return arc;` | `pool.insert(k, obj.clone()); return obj;` |

```rust
// 正确: obtain() 返回 Arc<T>
pub fn obtain(...) -> Arc<BRC> {
    let mut pool = get_sPool().lock().unwrap();
    if let Some(cached) = pool.get(&key) {
        return Arc::clone(cached);  // 引用计数 +1, O(1)
    }
    let obj = Arc::new(BRC::new(...));
    pool.insert(key, Arc::clone(&obj));
    obj
}
```

#### 3.2 复用栈：使用 `std::mem::replace` 回收实例

```rust
// Java: this.initDefault(); sReuseStack.push(this);
// Rust: 用 replace 交换 self，将旧实例回收，避免 clone
let mut old_self = std::mem::replace(self, Builder::new());
old_self.initDefault();
reuse_stack.push(old_self);
```

- **禁止** `self.clone()` 后 push 到复用栈（违背池设计初衷，等于分配新内存）
- 使用 `std::mem::replace` 将 self 替换为空默认值，旧值移入复用栈

#### 3.3 `synchronized` → `Mutex`

```rust
// Java: synchronized(sPool) { ... }
let mut pool = get_sPool().lock().unwrap();
// ... 操作 pool ...
// lock 在作用域结束时自动释放
```

## CLI 命令参考

通过全局命令 `java2rust` 操作：

```bash
# 启动/继续流水线（自动检测当前目录下的 rewrite_config.json）
java2rust run start

# 指定配置文件
java2rust run start --config <path/to/rewrite_config.json>

# 指定文件
java2rust run start --file <path.java>

# 运行 check (一致性 + cargo check)
java2rust run check --file <path>

# 不指定文件则检查当前计划中第一个未通过的
java2rust run check
```

---

## 操作步骤

### 1. 启动流水线

调用 `pipeline_start`，传入目标 crate 的 `rewrite_config.json` 绝对路径，获取模板文件路径和改写指令。

### 2. 改写代码

根据控制台输出的模板路径，打开对应 `.rs` 文件，遵循以下规则改写：

- **保留所有 `// Java:` 注释** — 这些是一致性检查的基准
- **填写 `// TODO_RUST_CODE` 标记处** — 补充等价 Rust 实现
- **遵循重写规则** — 类型映射、命名规范、Arc 共享等（见下方"重写规则"章节）

### 3. 提交验证

改写完成后调用 `pipeline_continue`（传入相同的 `config` 路径）：
- **通过** → 自动进入下一个文件，重复步骤 2
- **失败** → 根据错误信息修复代码，再次调用 `pipeline_continue`

### 4. 循环直到完成

重复步骤 2-3，直到所有模块完成。

---

请使用MCP 工具 pipeline_start，开始重写流程, 直到 rewrite_modules.csv 中的模块全部完成
