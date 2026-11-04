/* DEFERRED_NON_FUNCTION —— `data/table` 子树整体延后：其 `TableTool` 结构体持有
 *   `PropertyTool` 字段 ⇒ 经 `core::PropertyTool` 拉入 `core::shape` 闭包；
 *   其余文件多为**自动生成桩**（`// Rust stub for X (auto-generated)`）。
 *   与 docx 文本解析→排版→渲染必径无关 ⇒ 只保留原 `grid::GridResizeRequest`。 */
pub mod table {
    pub mod grid {
        pub mod GridResizeRequest;
        pub mod GridCellImpl;
        pub mod GridColImpl;
        pub mod GridColsImpl;
        pub mod GridCols;
        pub mod GridCol;
        pub mod GridCell;
        pub mod GridAutoSize;
        pub mod GridSizeCommit;
    }
    pub mod Data;
    pub mod CellInfo;
    pub mod RowInfo;
    pub mod TableInfo;
    pub mod TableInfos;
    pub mod TableInfosImpl;
    pub mod InfoFactory;
    pub mod KTableManager;
    pub mod TableManagerUtil;
    pub mod TableTool;
    pub mod TableJCData;
    pub mod TablePos;
    pub mod tree {
        pub mod IOCache;
        pub mod CellInfoNode;
        pub mod RowInfoNode;
        pub mod TableInfoNode;
        pub mod TreeTableCache;
    }
}
pub mod PLCTextboxText;
pub mod DefaultSectionProperties;
pub mod DefaultValue;
pub mod PLC_Headings;
pub mod PLCBookmarkStart;
pub mod PLCFootnoteEndnoteTxt;
pub mod PLCFootnoteEndnoteRef;
pub mod PLCBookmarkEnd;
pub mod BookmarkNode;
pub mod PLCCommentTxt;
pub mod PLCCommentRef;
pub mod PLC_Char;
pub mod PLCDocumentFields;
pub mod PropertyId;
pub mod PropertyUtil;
pub mod DefaultParagraphProperties;
pub mod DefaultRunProperties;
pub mod theme {
    pub mod Theme;
    pub mod color {
        pub mod ThemeColorScheme;
    }
    pub mod format {
        pub mod ThemeFormatScheme;
        pub mod EffectLst;
        pub mod EffectStyle;
    }
    pub mod font {
        pub mod SupplementalFont;
        pub mod ThemeFontScheme;
        pub mod FontCollection;
    }
}
// rust: ★T7 —— 原为**内联块**，只声明 BRC/CellSpacing/DateTime 等少数子模块，
//   使 `data/value/` 下其余文件（含 `TableTool` 需要的 `TableDefSHD`/
//   `TableStyleProperties`）成为死代码。`value/mod.rs` 是完整超集。
pub mod value;
pub mod ITable;
pub mod IPLC_Words;
pub mod IChpxTable;
pub mod IPapxTable;
pub mod UnknownDataStructure;
pub mod UnknownDataMap;
pub mod RANGE;
pub mod RANGEFactory;
pub mod PLCMathEnd;
pub mod PLCMathStart;
pub mod PLCMathFunction;
pub mod MathTool;
pub mod math;
pub mod PLC;
pub mod PLCSection;
pub mod PLCHeader;
pub mod CPieceTable;
pub mod DrawingContainer;
pub mod autonum {
    pub mod AutoNumCount;
    pub mod AutoNumTable;
    pub mod LstDataTable;
    pub mod LfoDataTable;
    pub mod LstLfoTool;
    pub mod LFO;
    pub mod LfoLvlData;
    pub mod LSTF;
    pub mod LVLF;
    pub mod LFOLVL;
    pub mod LvlfData;
    pub mod LfoData;
    pub mod LstData;
}
pub mod internal {
    pub mod KPropertySetRegistry;
}
pub mod internal_v2 {
    pub mod ListNode;
    pub mod ListTreeExtension;
    pub mod ListPositionTree;
    pub mod ListIndexNode;
    pub mod ListIndexPositionTree;
    pub mod TextReference;
    pub mod ReferenceTable;
    pub mod CPIterator;
    pub mod CPTable;
    pub mod ChpxTable;
    pub mod ChpxTableFactory;
    pub mod PapxTable;
    pub mod PapxTableFactory;
}
pub mod BookmarkNodeType;
pub mod PLC_CP;
pub mod PLC_Range;

pub mod PropertyOperation;
pub mod PropertyAction;
pub mod PropertyOperations;
pub mod ITextStreamFilter;
pub mod AbstractTextStreamFilter;
pub mod TextStream_NormalFilter;
pub mod ITextStream;
pub mod TextData;
pub mod KTextStream;
pub mod PLCShapeAnchor;
pub mod autoWordStat {
    pub mod AutoWordStat;
    pub mod IRevisionModifyListener;
}

// probe: PLCField + field closure
pub mod PLCField;
pub mod field;

// rust: ★T7 —— `RangeTool` 依赖 `data::TableTool`（`data/TableTool.rs`，806 行；
//   注意与**已延后**的 `data/table/TableTool.rs` 同名不同文件）。
// Java: import cn.wps.moffice.writer.data.TableTool
pub mod TableTool;
