# S02 — Part 流 → XML 元素事件

## 1. 职责

把 OPC 包中任意 Part 的 XML 字节流转成**带元素身份的事件流**：`onStart_with_id(elementId, ns, local, q, attrs)` / `onEnd_with_id` / `onCharacters`。`elementId` 是对「命名空间 + 本地名」的整数哈希，供后续 handler switch 分发。

管线位置：上游 S01；下游 S03。

## 2. 核心 Java 代码

| 类 | 路径 | 职责 |
|----|------|------|
| `XmlReader` | `cn.wps.io.opt.XmlReader` | SAX/StAX 风格的 XML 读取，产出元素事件 |
| `Name2IdMap` | `cn.wps.io.opt` | `(namespace, localName)` → int elementId 的哈希映射 |
| `XmlParse` | `cn.wps.io.opt` | 装配 reader + Name2IdMap + 根 handler 的解析入口 |

关键契约（handler 侧看到的事件签名）：

```java
// XmlReader / dispatch 层向 handler 抛出的元素事件
void onStart_with_id(int elementId, String ns, String local, String q, Attributes attrs);
void onEnd_with_id(int elementId, String ns, String local, String q);
void onCharacters(char[] chars, int start, int len);
```

`elementId` 生成：Java `String.hashCode()` 作用于本地名（wc-03 已钉死字面量：`w:b→98`、`w:i→105`、`w:jc→3385`、`w:pStyle→-1008274655` …）。

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `elementId` | `int`；命名空间+本地名的整数身份（switch 用） |
| `Attributes` | 元素原始属性表：`getValue(uri, localName)`、`getLength()` |
| 事件流 | 有序 `onStart / onCharacters / onEnd`，元素属性**不透明** |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | Part `InputStream`、根 handler、namespaces（`w:` 主命名空间） |
| **O** | 元素事件流（elementId / ns / local / q / attrs / 字符） |

## 5. 上下游

- **上游**：S01 的 `PackagePart.getInputStream()`。
- **下游**：S03 的 `DispatchHandler2`（逐事件分发到 handler 栈）。

## 6. 边界 / 失败语义 / 不变量

- 属性在此环节是**不透明** `&dyn Any`；**读值发生在 S04**。
- elementId 必须与 Java `String.hashCode()` 同算法，否则所有下游 switch 全错。
- 命名空间前缀（`w:` 等）在 `ns` 中展开，`local` 是本地名。

## 7. Rust 承接与现状

wave0 `06_function_xml_elementid` + `08_function_xml_dispatch` —— **✅ pass**。