# S01 — `.docx` 字节 → OPC 包

## 1. 职责

把 `.docx` 字节流解析成 **OPC（Open Packaging Conventions）包**：解压 zip、读 `[Content_Types].xml` 与 `_rels/.rels`，建立 part 表、类型表、关系图，使每个 Part 可 `getInputStream()`。所有后续环节都基于 OPC 包取流。

管线位置：**入口**（无上游；下游 S02）。

## 2. 核心 Java 代码

主类（wave0）：

| 类 | 路径 | 职责 |
|----|------|------|
| `ZipEntrySource` | `cn.wps.io.xwpf.util.ZipEntrySource` | zip 入口 → 字节源的抽象（文件/RAF/流），供 `OPCPackage` 读取 |
| `OPCPackage` | `cn.wps.io.opc.OPCPackage` | 包容器：part 表、`open(pkg)`、按 URI 取 part |
| `ContentTypeManager` | `cn.wps.io.opc.internal.ContentTypeManager` | `[Content_Types].xml` → 扩展名/默认类型/覆盖类型 → content type 解析 |
| `PackageRelationshipCollection` | `cn.wps.io.opc.PackageRelationshipCollection` | `_rels/.rels` 与 part 级 `.rels` → 关系图 |

关键入口逻辑（`OPCPackage.open`）：

```java
// 伪结构（wave0 07 opc_package 语义）
PackageRelationshipCollection rels = new PackageRelationshipCollection(pkg, ROOT_REL, null);
Iterator<PackageRelationship> it = rels.iterator();
while (it.hasNext()) {
    PackageRelationship pr = it.next();
    if (pr.getRelationshipType().equals(XML_REL + "/officeDocument")) {
        // 找到主文档 part，后续 S03 从这里挂 styles/document 等 handler
    }
}
```

数据流：`ZipEntrySource`（字节）→ `ContentTypeManager`（识别 part 类型）→ 关系遍历（找 officeDocument 根）→ part 可读流。

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `OPCPackage` | part 表：`URI → PackagePart`；类型解析器；关系集合 |
| `PackagePart` | 一个包内条目：URI + content type + `getInputStream()` |
| `PackageRelationship` | 源 part（或包根）→ 目标 part 的有向边 + relationship type + id |
| `ContentTypeManager` | 默认类型（按扩展名）+ 覆盖类型（按 part 名） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | `.docx` 字节（或路径 / RAF / 流）、PK 魔数、`[Content_Types].xml`、`_rels/.rels` |
| **O** | `OPCPackage`：part 表（URI→Part）、类型表、关系图；每 Part 可 `getInputStream()` |

## 5. 上下游

- **上游**：无（文件输入）。
- **下游**：S02 `XmlReader` 从任意 Part `getInputStream()` 读 XML。

## 6. 边界 / 失败语义 / 不变量

- 坏 zip / 缺 `[Content_Types].xml` / 类型缺失 → **在本环节红**，不得泄漏成下游「importer 没回调」。
- PK 魔数错误 → 立即拒绝。
- 关系解析必须处理包根 `_rels/.rels` 与 part 级 `.rels` 两种位置。

## 7. Rust 承接与现状

wave0 `02_function_unzip` `03_function_opc_content_types` `04_function_opc_relationships` `05_function_opc_parts` `07_function_opc_package` —— **✅ pass**（wps-io crate）。