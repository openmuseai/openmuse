# pp-12-opc-package — PPTX OPC 包桥接（OxfrPackage 等价）

> 状态：**pass**
> 依赖：无（OPC 基础能力在 core-libs `io_wps_io`，word wave0 已 pass；本 feature 只做 **PPT 侧封装与定位**，不再拆 zip）
> 管线：PIPELINE_LAYOUT_RENDER_PLAN.md §1-S0 / §2.1；本 feature = 「真实 .pptx 文件 → part/流」的最后一环

## 目标

把 Java 侧 `OxfrPackage extends OpcrPackage`（WPS 私有 OPC 封装，直接持有 `ZipFile`）与
`PptxReader.getPresPart` 的语义，桥接到 **Rust 已存在的 POI 风格 OPC**（`io_wps_io` 的
`OPCPackage`/`PackagePart`，wave0 `02..07_function_opc_*` 已 pass）：

```text
path / RandomAccessFile
  → [01] PptxPackage::open（桥接 wps-io OPCPackage::from_path/from_raf）
  → [02] getPresPart（officeDocument rel 定位 + partName 含 "ppt" 校验 → FileDamaged/SuffixError）
  → [03] part → InputStream（zip entry 字节源）
       ↓
  pp-07 UnifiedXmlReader / pp-09 PptxReader.open 消费
```

**不再**在 kfc_alpha 里按 WPS API 抄第二份 OPC；POI 风格 `OPCPackage` 的
`getRelationshipsByType`/`getPart_rel` 足以表达 `getOfficeDocument/getAppPart/getCorePart/getCustomPart`。

## I/O（锁死）

| | 契约 |
|--|------|
| **I** | 文件路径或 RAF（加密已解密后的路径）；`[Content_Types].xml` + 根 `_rels/.rels`（wps-io 加载） |
| **O** | `PptxPackage`（持有 `OPCPackage` 的桥接句柄）；`getPresPart() -> Result<PackagePart, PptxOpenError>`；`getAppPart/getCorePart/getCustomPart`（doc props，pp-09:01 消费）；part → 字节流（pp-07:01/pp-09 消费） |

三条输出不可改口：

1. `open(path|raf) -> PptxPackage`（成败即返回；Java ctor Throwable→`FileDamaged(t)` 的上抛语义归 pp-09:04）
2. `getPresPart() -> Result<Part, FileDamaged("Not OOXML file") | SuffixError("Not pptx file")>`（Java `PptxReader.getPresPart` 134-141 行逐句对齐）
3. `part.getInputStream() -> Reader`（Java `OpcrPackagePart.getInputStream(isOriginInput)`；`isOriginInput=false` 走解压读）

## Java 对照

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `kfc/ooxfr/OxfrPackage.java`（extends `OpcrPackage`；ctor path/RAF、`getOfficeDocument`、`getAppPart/getCorePart/getCustomPart`、`getMediasSize`） | `src/cn/wps/show/pptxr/opc/PptxPackage.rs`（桥接 `io_wps_io` `OPCPackage`） | 01/02/03 |
| `kfc/opcr/OpcrPackage.java`（ZipFile + 根 rels `mRelses` + ContentTypes） | wps-io `OPCPackage::from_path/from_raf` + `getRelationshipsByType` + `getPart_rel` | 复用（wave0 pass，不重写） |
| `kfc/opcr/OpcrPackagePart.java`（`getInputStream(isOriginInput)`/`getPartName`） | wps-io `PackagePart` | 03 语义源 |
| `show/pptxr/PptxReader.java`（`getPresPart` 134-141；`close` 110-131；`OxfPptRelations.setPptRelation`） | `02_function_pres_part_locate`（校验）／`close` 语义 | 02；04 编排在 pp-09 |
| `kfc/ooxfr/OxfRelations.java`（APP/CORE/CUSTOM relsType 常量） | 常量表 | 01 附属 |

## 能力解锁

- **01**：真实 `.pptx` 文件（path/RAF）第一次有 Rust 句柄——产品闭环轨的**物源**输入就绪。
- **02**：`FileDamaged("Not OOXML file")` / `SuffixError("Not pptx file")` 失败语义钉死；pp-09:04 的 getPresPart 必测项有实现载体。
- **03**：part 字节流就绪 → pp-07 `UnifiedXmlReader.read(stream, handler)` 与 pp-09 doc props (`readApp/readCore`) 的输入落地。

## 必须 / 壳 / 延后

- **must：** `open(path|raf)`；`getOfficeDocument`（officeDocument relsType 常量
  `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument`，Java
  `OxfrPackage.java:27`）；`getPresPart` 两异常语义；`close`（Java: IOException 仅 printStackTrace，不重抛）。
- **stub：** `getMediasSize`（zip 枚举 `/media/` 压缩尺寸和——首屏不需要）；`setPptRelation`（`OxfPptRelations` 全局布置，post-MVP）。
- **defer：** 加密解密路径（`PptxrFileDecrypt`，属于加密 feature）；repair-file 重试打开（`OpcrPackage` ctor 的 repair 分支）。

## 范围内 / 范围外

- **in：** path/RAF → PptxPackage；officeDocument/app/core/custom 四类 rel 定位；pres part 校验；part→流。
- **out：** zip 解压本身（wave0 `02_function_unzip`）；`[Content_Types]`/rel 解析（wave0 `03/04`）；part 集合构建（wave0 `05/07`）；XML 事件（pp-07:01）；编排（pp-09）。

## 验收

1. `open` 对非法文件返回「打开失败」而不 panic（Java Throwable→FileDamaged 语义上抛由 pp-09 抛）。
2. `getPresPart`：officeDocument 缺失 → `FileDamaged("Not OOXML file")`；partName 不含 `ppt` → `SuffixError("Not pptx file")`；合法 → 返回该 part（Java 逐句对齐）。
3. `close` 幂等；关闭失败不重抛（eprintln 级）。
4. 同一 `PptxPackage` 下 `getPresPart` 与 `getPart(partName)` 指向同一 part。

## 依赖

- **depends：** `io_wps_io`（`core_libs_io`，wave0 `02..07` pass——`OPCPackage`/`PackagePart`/`ZipEntrySource`；ppt-core Cargo.toml 已声明）；`zip` crate。
- **does not depend：** pp-01..pp-11 全部；`kfc_alpha` 的 opcr（不复刻）。
- **被依赖：** pp-07（part 流输入）、pp-09（PptxReader.open 的 package 输入）。

## 不过扩展

禁止在本 feature 重抄 POI OPC（zip/rels/content-types 全部走 `io_wps_io`）。禁止把
`PptxrFileDecrypt`（加密）塞进来「顺便做」。禁止改 wave0 已 pass 的 OPC 语义。