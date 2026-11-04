# S03 — 事件 → handler 栈分发

## 1. 职责

把 S02 的元素事件流按**栈式**模型分发给具体 handler：首事件按根 filterIndex 匹配，其后「栈顶 `getElementHandler(id, ns)` → push → `onStart`；`onEnd` 弹栈」。styles / document / numbering / settings / fonts 各自一段 handler 链。

管线位置：上游 S02；下游 S04（属性转换）与 S05（正文落库）。

## 2. 核心 Java 代码

| 类 | 路径 | 职责 |
|----|------|------|
| `DispatchHandler2` | `cn.wps.io.simple.DispatchHandler2` | 事件 → handler 栈分发（根匹配 + 栈顶下钻） |
| `ElementHandler` | `cn.wps.io.xwpf.ElementHandler` | 单元素回调基类：`onStart/onEnd/onCharacters`、`getElementHandler(id, ns)` |
| 具体链 | `styles/document/numbering/settings/fonts/…` | 各 OOXML 域的 handler 族 |

栈式分发伪逻辑：

```java
// DispatchHandler2（wave0 08 语义）
onStart(id, ns, local, q, attrs):
    if (当前在根) rootHandler.filterIndexes 匹配 id → 进入对应子 handler
    else curHandler = stackTop.getElementHandler(id, ns)   // 栈顶下钻
    stack.push(curHandler); curHandler.onStart(id, ns, local, q, attrs)
onEnd(id, ...):
    stackTop.onEnd(id, ...)
    stack.pop()
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| handler 栈 | 从根 handler 到当前元素的父链；每个元素对应一个 handler 实例/回调 |
| `getElementHandler(id, ns)` | 栈顶按元素身份返回子 handler（未识别返回 null → 忽略子树） |
| 回调序列 | 与 XML 嵌套一一对应：`onStart`（进入）… `onEnd`（离开） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | S02 事件流 + 根 handler（如 `DocumentHandler` / `StylesHandler`） |
| **O** | 栈式回调序列：各 Handler 的 `onStart/onEnd/onCharacters` 被依次调用 |

## 5. 上下游

- **上游**：S02 元素事件。
- **下游**：S04（`Prop/Ppr/Rpr/PropChildHandler` 转换属性）、S05（`SubDocumentImporter` 段落回调）、S06（styles 两阶段消费）。

## 6. 边界 / 失败语义 / 不变量

- 未识别元素：栈顶 `getElementHandler` 返回 null → **跳过子树**，不抛错。
- 栈不匹配（onEnd 缺 onStart）→ 解析红。
- 根分发按根 handler 的 filterIndexes 匹配首层元素。

## 7. Rust 承接与现状

wave0 `08_function_xml_dispatch` + styles 链 `15_function_styles_handler_chain` —— **✅ pass**。
numbering / settings / fonts 子链 TODO（无编号/设置的最小 docx 可跳过）。