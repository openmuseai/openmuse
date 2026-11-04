# pp-12-opc-package — PPTX OPC 包桥接

> depends_on: 无（OPC 基础在 core-libs `io_wps_io`，wave0 pass）  
> 状态：**01 pass；02 pass；03 pass**
> 目的：补上「真实 .pptx 文件 → part/流」的最后一环（主分析结论：zip→OPC 层已存在，缺的是 OxfrPackage 等价桥接与 getPresPart 语义）

## §0 管线位置

```text
真实 .pptx path / RAF
  ├─ wave0 (core-libs/io): OPCPackage::from_path/from_raf（zip→content types→rels→parts，已 pass）
  │       ↓ 本 feature（ppt 侧封装，桥接而非重写）
  ├─ [01] OxfrPackage ctor          OxfrPackage ctor 等价；暴露 officeDocument/app/core/custom rel 定位
  ├─ [02] getPresPart              PptxReader.getPresPart 语义：FileDamaged("Not OOXML file")/SuffixError("Not pptx file")
  ├─ [03] part → InputStream       OpcrPackagePart.getInputStream 等价
  │       ↓
  ├─ pp-07 01 UnifiedXmlReader（part 流 → XML 事件）
  └─ pp-09 04 PptxReader.open（package 输入 + getPresPart + close）
```

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_package_open` | path 或 RAF | `OxfrPackage`（wps-io OPCPackage 桥接句柄 + officeDocument/app/core/custom rel 定位 + close） | wave0 `07_function_opc_package`（OPCPackage） | 02;03；pp-09:04 |
| 2 | `02_function_pres_part_locate` | `&mut OxfrPackage` | `Result<&PackagePart, FileDamaged\|SuffixError>` | 01 | pp-09:04 |
| 3 | `03_function_part_stream` | part（或 partName）+ isOriginInput | 字节流（zip entry 解压） | 01;02 | pp-07:01；pp-09:01（readApp/readCore 的 app/core/custom part） |

## §2 数据流

```text
path/RAF ── OxfrPackage ctor ──> OPCPackage(from_path/from_stream)
  ├─ getOfficeDocument：getRelationshipsByType(officeDocument) → getPart_rel
  ├─ getPresPart：officeDocument==null → FileDamaged("Not OOXML file")
  │              partName 不含 "ppt" → SuffixError("Not pptx file")
  │              否则 → part
  ├─ getAppPart/getCorePart/getCustomPart：OxfRelations APP/CORE/CUSTOM relsType → part
  └─ part.getInputStream(false) → Reader（解压字节；isOriginInput=true 走未解压原始流）
```

## §3 测试面

- src：`libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs`
- 测试镜像：`libs/ppt-core/tests/cn/wps/kfc/ooxfr/`
- fixture：wave0 `FakeZipEntry` 风格的内存 zip（`io_wps_io` 已有 `ZipEntrySource`/`FakeZipEntry`）——不依赖真实 pptx 文件
- 单测：`cargo test --manifest-path libs/ppt-core/Cargo.toml --test <function_id>`

## §4 串联验收

1. 01：合法 zip 打开成功；损坏文件返回 Err（不 panic）。
2. 02：三类输入（无 officeDocument / partName 无 "ppt" / 合法）恰好映射 Java 两异常一成功。
3. 03：part 流字节 == 对应 zip entry 解压字节；isOriginInput 分支语义锁定。
4. 回归：wave0 `02..07_function_opc_*` 不回退（本 feature 不碰 wps-io）。

## §5 loop 顺序

`01 → 02 → 03`

## §6 Java/Rust 范围

| Java | Rust 目标 | 裁决 |
|---|---|---|
| `kfc/ooxfr/OxfrPackage` | `src/cn/wps/kfc/ooxfr/OxfrPackage.rs` | 01 桥接；保持 Java 类名与注释布局 |
| `kfc/opcr/OpcrPackage`/`OpcrPackagePart` | wps-io `OPCPackage`/`PackagePart` | 复用（wave0 pass，禁改） |
| `show/pptxr/PptxReader.getPresPart/close` | 02 校验 + close 语义 | 02 语义源；04 编排属 pp-09 |
| `kfc/ooxfr/OxfRelations`（APP/CORE/CUSTOM） | 常量表 | 01 附属 |
| `PptxrFileDecrypt` | 加密 feature | 移出 |

## 文档

- [functions.csv](./functions.csv)
- `functions/<function_id>/FUNCTION.md`
