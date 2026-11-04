# 03_function_paragraph_end — 拆分理由 / 当前代码现状 / 实现级分析

## 0. 结论（先读这个）

- **本 function 自身 src 未实现**：`SubDocumentImporter.rs` 不存在。
- **已实现的上游依赖**：`CoreHelper.appendParagraph__char_int_ps_ps`、`ParaPropsConverterSimple.correctProp`、`KParagraphsID.addParaphId/putId`、`PLCSection.setLastSectionEndCP`、`ParagraphContext`（01，待 loop）。
- **硬缺口**：`ImporterHelper.amendTableProps`（空 todo 签名）；`ParagraphSimple.getParaProp/getRunProp` 返回 `Option<&dyn Any>`（需 downcast）；numbering 懒解析 defer。
- **下一步**：loop L5 实现 `importParagraphStart/importParagraphEnd/isContainNumbering/resetParagraph`，并在 loop 内修 `amendTableProps` 签名。

## 1. 为什么拆成这个 function（I/O 边界）

`importParagraphStart` 的唯一可观察输出（段首 CP）只有在 `importParagraphEnd` 用它做 PAPX insert 时才落地——Start 是「私有胶水」，End 是公开入口。所以两者必须同一个 function，才能测「PAPX@cp0」这一 feature 最关键不变量。Run 文本是独立的中间事件，已拆到 02。

## 2. 当前代码现状

### 2.1 Java 现状（8 步分解）

```java
// SubDocumentImporter.java:92–98
public void importParagraphStart(String paraId) {
   this.mCurrentParagraph = new ParagraphContext(this.mDocument.getLength());
   if (null != paraId) { this.mCurrentParagraph.mParaId = paraId; }
}

// :370–417（节选）
public IPapxTable.Entry importParagraphEnd(ParagraphSimple paragraph, int layerTable) {
   char endMark = paragraph.getEndMark();                       // 1
   MutablePropertySet dstParaProps = paragraph.getParaProp();   // 2（null→new）
   MutablePropertySet dstRunProps  = paragraph.getRunProp();
   if (null == dstParaProps) dstParaProps = new MutablePropertySet();
   if (null != this.mCurrentParagraph.mParaId) {                // 3
      dstParaProps.setValue(282, this.mCurrentParagraph.mParaId);
      this.mDocument.getTextDocument().getKParagraphsID().addParaphId(this.mCurrentParagraph.mParaId);
   }
   ParaPropsConverterSimple.correctProp(dstParaProps);          // 4
   if (layerTable >= 1) {                                       // 5
      ImporterHelper.amendTableProps(dstParaProps, layerTable);
   }
   this.mCurrentParagraph.mEntry = CoreHelper.appendParagraph(  // 6
      this.mDocument, endMark, this.mCurrentParagraph.mPositionInserted,
      null == dstRunProps ? PropertySet.empty : dstRunProps.freeze(),
      null == dstParaProps ? PropertySet.empty : dstParaProps.freeze());
   if (null != this.mCurrentParagraph.mParaId) {                // 7
      this.mDocument.getTextDocument().getKParagraphsID().putId(
         this.mCurrentParagraph.mParaId, this.mDocument.getType(), this.mCurrentParagraph.mEntry);
   }
   if ('\f' == endMark) {                                       // 8
      this.mDocument.getPLCSection().setLastSectionEndCP(this.mDocument.getLength());
   }
   if (this.isContainNumbering(this.mCurrentParagraph.mEntry)) { /* numbering 懒解析 */ }
   return this.mCurrentParagraph.mEntry;
}
```

### 2.2 Rust 本 function src 现状

`SubDocumentImporter.rs` 不存在；loop L5 需连同 02 的方法一起按「函数改写列表」停车非本 function 方法。

### 2.3 上游依赖 Rust 现状（逐个）

`CoreHelper.appendParagraph__char_int_ps_ps`（已实现，wc-01）：

```rust
// CoreHelper.rs:67–80
pub fn appendParagraph__char_int_ps_ps(document, ch, cp, chpx, papx) -> Option<Box<dyn IPapxEntry>> {
   let Some(document) = document else { return None; };
   Self::appendText__char_ps_doc(document, ch, chpx);
   Self::insertToPapxTable(Some(document), cp, papx)   // ★ 第二参 cp = 段首 CP
}
```

`ParaPropsConverterSimple.correctProp`（已实现，wc-03）：

```rust
// ParaPropsConverterSimple.rs:406–414
pub fn correctProp(dst: Option<&mut MutablePropertySet>) {
   if let Some(dst) = dst {
      Self::correntStyle(dst);   // 缺191 → setInt(191, 0)
      Self::correctTabs(dst);
   }
}
```

`KParagraphsID.addParaphId/putId`（已实现，wc-02）：

```rust
// KParagraphsID.rs:35–38
pub fn addParaphId(&mut self, paraId: &str) -> bool { self.mParagraphIds.add(paraId) }
// KParagraphsID.rs:53–59
pub fn putId(&mut self, id: Option<&str>, documentType: i32, entry: &dyn Entry) {
   if let Some(id) = id {
      let mut hashId: i32 = id.hashCode();
      self.mId2Entry.put(hashId, ParaEntry::new(documentType, entry));
   }
}
```

`PLCSection.setLastSectionEndCP`（已实现，wc-01）：

```rust
// PLCSection.rs:125–127
pub fn setLastSectionEndCP(&mut self, cp: i32) { self.mLastSecEndCP = cp; }
```

**缺口** `ImporterHelper.amendTableProps`：

```rust
// ImporterHelper.rs:69（当前是空签名 todo）
pub fn amendTableProps() { todo!("DEFERRED_NON_FUNCTION"); }
```

loop 内必须先改成 `pub fn amendTableProps(dst: &mut MutablePropertySet, layerTable: i32)` 并实现 `setBoolean(223,true)+setInt(224,layerTable)`，否则 PE6 无法测。

## 3. 实现级分析

### 3.1 数据流 / 时序图

```mermaid
sequenceDiagram
    autonumber
    participant X as XWPF
    participant SDI as SubDocumentImporter
    participant PC as ParagraphContext
    participant CH as CoreHelper
    participant D as SubDocument
    X->>SDI: importParagraphStart(paraId)
    SDI->>D: getLength() → cp0
    SDI->>PC: new ParagraphContext(cp0)
    X->>SDI: importRunTextSimple("Hi", chpx)
    SDI->>CH: appendText(doc, "Hi", chpx)
    CH->>D: rope[0,2); CHPX@0
    X->>SDI: importParagraphEnd(paragraph, 0)
    SDI->>SDI: correctProp(dstParaProps)
    SDI->>CH: appendParagraph(doc, '\r', cp0=0, runProps, paraProps)
    CH->>D: rope[2]='\r'; CHPX@2
    CH->>D: PAPX.insert(cp0=0)   ★ 不是 2
    SDI-->>X: return mEntry
```

### 3.2 关键不变量

1. **PAPX cp = `mPositionInserted`（Start 时 getLength()），不是 End 时 length**。
2. **correctProp 不可省略**：无样式段要写 191=0。
3. **paraId 全链路**：`setValue(282)` → `addParaphId` → End 后 `putId`。
4. **`\f` 只 setLastSectionEndCP(length)**，真正 `addSection` 在 07。
5. **`amendTableProps` 仅 `layerTable>=1` 时调用**。

### 3.3 失败语义

- PAPX 挂错 CP：PE1 红（这是 feature 的头号验收项）。
- `correctProp` 被跳过：PE5 红（191 缺失）。
- paraId 链路断：PE3 红（282/putId 缺失）。

> 测试方案与用例表见 [FUNCTION.md](./FUNCTION.md) §3。
