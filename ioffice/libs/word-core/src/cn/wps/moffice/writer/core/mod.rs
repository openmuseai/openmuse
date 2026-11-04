pub mod WtStatistic;
pub mod list {
    pub mod ListParagraphs;
}
pub mod sectionProp {
    pub mod ISectionProperty;
}
pub mod io {
    pub mod IOListener;
    pub mod IOThreadSign;
    pub mod IDocumentCallback;
    pub mod ICoreDataAcceptor;
}
pub mod InnerIOListener;
pub mod KParagraphsID;
pub mod AsiaTypography;
pub mod DocumentListener;
pub mod ModifyFlag;
pub mod FileTime;
pub mod DocumentVariables;
pub mod KDocumentWriteProtection;
pub mod SaveAccelerator;
pub mod CharParse;
pub mod CharMap;
pub mod CharFontTool;
pub mod WdCompatibilityMode;
pub mod pool {
    pub mod Reusable;
    pub mod ShareObjIndexedMapImpl;
}
pub mod Document;
pub mod TextRopeListeners;
pub mod StyleListenerBase;
pub mod UUCommentId;
pub mod UUSti;
pub mod KStyle;
pub mod KStyles;
pub mod FontTable;
pub mod protection {
    pub mod ProtectionType;
    pub mod DocumentProtectionException;
    pub mod OfficePasswordUtil;
    pub mod IncorrectPasswordException;
    pub mod ProtectionPassword;
    pub mod DocumentProtectionIO;
    pub mod KDocumentProtection;
    pub mod ProtectionListener;
}
pub mod BuiltinFontTable;
pub mod DocumentEvent;
pub mod DocumentEventDispatcher;
pub mod ErrorCollector;
pub mod OleManager;
pub mod IDebugger;
pub mod DocumentLoadListener;
pub mod DefaultModelEvent;
pub mod Collection;
pub mod SelectionType;
pub mod StyleListener4PapxTable;
pub mod StyleListener4ChpxTable;
pub mod WtReadingOrder;
pub mod ParagraphFormat;
pub mod KRange;
pub mod SubDocument;
pub mod RevisionListener;
pub mod TextDocument;
pub mod KRevision;
pub mod KParaRunRevision;
pub mod shape {
    pub mod KShapes;
    pub mod ShapeMetrics;
    pub mod DumpInfo;
    pub mod PictureDumpMap;
    pub mod PictureLoader;
    pub mod internal {
        pub mod FilePage;
        pub mod KShapePool;
    }
}
pub mod heading {
    pub mod KPLC_Headings;
}
pub mod words {
    pub mod KPLC_Words;
}
// rust: ★T7-8 —— `KExchange` / `KFootnoteEndnote` 探测记录（**本批未登记，原因实测**）。
//   Java: `cn.wps.moffice.writer.core.footnoteendnote.{KExchange,KFootnoteEndnote}`
//   —— 脚注/尾注交换与设置，**在 docx 只读管线上**，是本文件与 `TextDocument.rs` 中
//   两个就地影子的真实归属。两文件本身是完整移植（KExchange 800 行 / KFootnoteEndnote
//   521 行，均 0 `todo!`），但登记后暴露 **139** 个错误，根因分布（实测）：
//     · 38 × `Option<*mut ()>` —— `core/Document.rs` 中 28 个 PLC 访问器门面被擦成裸 `*mut ()`
//       （`getFootnoteRef/getFootnoteTxt/getEndnoteRef/getEndnoteTxt/getPLCSection/...`），
//       调用方无法使用 ⇒ **必须先把 `Document` trait 的门面改回真实 PLC 类型**；
//     · 10 × `Box<dyn Document>: Document` —— `getMainDocument()` 返回 `Box<dyn Document>`
//       而 Java 侧按接口用（`&dyn Document`）⇒ 所有权/引用建模待定；
//     ·  9 × `KRange::getSubDocument` / `KRange::start` 缺方法；
//     ·  4 × 同目录 `FootnoteSettingItem` / `KFootEndnoteSetting` / `ChangeRule` 未登记
//           （三者均为 8/8/2 个 `todo!()` 的自动生成桩）；
//     ·  1 × `data::table::CellInfo`（`data::table` 子树整体延后）；
//     ·  2 × `Assert::assertEquals__int_int`（stub 缺重载）。
//   ⇒ 前置是「`Document` trait 门面真实化」（与 §13 对 `getOMathStarts/Ends` 的处置同类），
//     属跨模块 trait 契约变更，须单独一批；本批按纪律不登记，保持绿态。
//     影子就地标注理由（见 `TextDocument.rs`）。
pub mod table {
    pub mod KTableRange;
    pub mod Tools;
}

// rust: ★T7-8 —— `data::field::parser::BaseParser` 依赖
//   `core::metadata::Metadata`（`BaseParser.rs:6/8`）。该目录**真实存在**
//   （`core/metadata/`，1117 行，含 `mod.rs`），此前未登记 ⇒ `E0433`。
pub mod metadata;

// rust: ★T7 —— `RangeTool`（1576 行）在**排版路径**上被真实调用：
//   `layout/base/ParaLayouter.rs:865`、`core/KRevisionChange.rs:1044`。
pub mod RangeTool;

// rust: ★T7-9 —— `data::PLC_Headings` 在 Java 中是**接口**，真实实现类是
//   `cn.wps.moffice.writer.core.heading.KPLC_Headings`（`KPLC_Headings.java`，
//   737 行 / 2 `todo!`）。`SubDocument.mHeadings` 的 Java 声明即
//   `KPLC_Headings mHeadings;`（`SubDocument.java`）⇒ 登记真实实现类，
//   使 `Document::getHeadings()` 门面能返回**真实类型**而非 `*mut ()` 空壳。
//   `KPLC_Headings` 依赖 `RangeTool`（已登记并接线）与 `KRange`/`KStyle`。
// rust: ★T7-9 —— `getHeadings` 探测记录（**本批未登记，原因实测**）。
//   Java: `Document.getHeadings()` 返回**接口** `data.PLC_Headings`；真实实现类是
//   `cn.wps.moffice.writer.core.heading.KPLC_Headings`（`KPLC_Headings.java`，737 行），
//   `SubDocument.mHeadings` 的 Java 声明即 `KPLC_Headings mHeadings;`。
//   但 `data/PLC_Headings.rs` 目前只是**软 trait**（仅 `size()` 一个方法），
//   而 Java 接口声明了 `fastCheckHasHeadings/size/get/getAll/addListener/removeListener`
//   与嵌套接口 `Listener`/`Heading`。登记 `core::heading` 后实测 **45 错误**：
//     · 5 × `E0407` —— `KPLC_Headings` 实现的方法不在软 trait 上；
//     · 1 × `E0405` —— 缺嵌套接口 `Listener`；
//     · 15 × `E0404` —— `PLC.Node` 模板被生成成**结构体**，而 Java 侧按**接口**使用；
//     · 4 × `E0425` —— 模板残留 `this` / `e`（Java 异常变量）未映射；
//     · 其余为 `core::TextTool`（910 行）自身缺陷（`JavaCharacter::isISOControl` 缺失、
//       `RangeTool` 未使用导入）。
//   ⇒ 前置是「`data::PLC_Headings` 接口真实化 + `PLC.Node` 接口化」，属跨模块建模变更，
//     须单独一批（与 §4.2 的 `Document` 门面真实化同类）。本批按纪律不登记，保持绿态。
//   影子就地标注理由（见 `Document.rs` 的 `getHeadings`）。
// rust: ★T7 —— `RangeTool` 依赖的 `core/` 叶子模块。
// Java: import cn.wps.moffice.writer.core.KHeaderFooterIndex
pub mod KHeaderFooterIndex;
// Java: import cn.wps.moffice.writer.core.WriterModelException
pub mod WriterModelException;
// rust: `RangeTool.getRangeHeader` 真实调用（排版取页眉页脚范围）。
pub mod helper;

// rust: ★T7 —— `RangeTool.getTargetHightLightColor` 依赖 `IPropertyCalculate`。
//   Factory/Impl/StyleCache 仍有模板缺陷，本批只登记接口（门面与 Java 一致）。
pub mod property;
