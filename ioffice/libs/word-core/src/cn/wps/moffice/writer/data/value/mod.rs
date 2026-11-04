pub mod BRC;
pub mod PageBorderProp;
pub mod CellHideMark;
pub mod CellMargin;
pub mod CellSpacing;
pub mod CompatSetting;
pub mod DateTime;
pub mod DefTable;
pub mod DefTableBuilder;
pub mod DxaColumn;
pub mod FitText;
pub mod FontInfo;
pub mod FrameTextFlow;
pub mod LineSpacing;
pub mod PageSetup;
pub mod PositionCode;
pub mod ReadOnlyArray;
pub mod Revision;
pub mod TBD;
pub mod TableBorders;
pub mod TableCell;
pub mod TableCellBuilder;
pub mod TableDefSHD;
pub mod TableDefSHDBuilder;
pub mod TableLength;
pub mod TableLook;
pub mod TableLookBuilder;
pub mod TableSetBRC;
pub mod TableStyleProperties;
pub mod Toggle;
pub mod TxTabStops;
pub mod WHeightAbs;
pub mod DropCap;
pub mod EastAsianLayout;
pub mod LBCOperand;
pub mod LID;
pub mod ConditionalCharacterFormatting;
pub mod ConditionalParagraphFormatting;

// rust: ★T7 —— 以下 6 个文件此前**未被 any mod.rs 声明**（死代码），
//   而 `io::reader::docxReader::importer::settings` 下的导入器真实引用它们
//   ⇒ 补齐声明，使 docx 设置导入路径可达。
pub mod AutoCaptionType;
pub mod CaptionType;
pub mod DocVar;
pub mod MathProperty;
pub mod SmartTag;
pub mod TextFlow;
