#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_assignments,
    unreachable_code,
    non_snake_case,
    non_upper_case_globals,
    unused_must_use,
    unused_doc_comments
)]

//! word-core — rewritten from Java via java2rust feature queues (feat_parser).
//!
//! Full `pub mod cn` (writer/layout/io) is still off. wc-01 functions 01–07
//! export collections (textrope) plus CHPX/PAPX/PLCSection/CPieceTable/CoreHelper
//! without pulling the rest of the writer tree.

pub mod cn {
    pub mod wps {
        pub mod collections {
            pub mod IntervalMap;
            pub mod IntervalMapNode;
            pub mod IntervalNode;
            pub mod IntervalTree;
            pub mod textrope {
                pub mod INotifyTextRope;
                pub mod ITextPiece;
                pub mod ITextPool;
                pub mod ITextRope;
                pub mod KNotifyTextRope;
                pub mod KTextPieceTable;
                pub mod KTextPool;
                pub mod KTextRope;
                pub mod KTransactionTextRope;
            }
        }

        pub mod util {
            pub mod concurrent {
                pub mod locks {
                    pub mod IRange;
                    pub mod RangeLock;
                    pub mod RangeReadLock;
                    pub mod RangeWriteLock;
                    pub mod ReadLockPool;
                    pub mod WriteLockPool;
                }
            }
        }

        pub mod moffice {
            pub mod util {
                pub mod SearchUtil;
                pub mod BoolBitSet;
                pub mod IntBitSet;
            }
            pub mod writer {
                pub mod function {
                    pub mod card {
                        pub mod online {
                            pub mod KeyWords;
                            pub mod PaperKeyWordParam;
                        }
                    }
                }
                pub mod base {
                    pub mod ViewMode;
                }
                // rust: ★M-STAGE2 P1-D1 / T1 —— `TextDocument.java:... ArrayList<ITextEditor> mViews;`
                //   的真实类型；`TextDocument.rs:508` 原为就地影子，真实文件 24 行、零依赖。
                pub mod view {
                    pub mod ITextEditor;
                }
                // rust: ★M-STAGE2 P1-D1 / T1 —— `addWriterOpenEventListener` 的参数类型；
                //   `TextDocument.rs:578` 原为就地影子，真实文件 6 行、零依赖。
                pub mod event {
                    pub mod KmoWriterOpenEventListener;
                    pub mod KmoWriterOpenEventListenerGroup;
                }
                    // rust: ★T7-5b —— 编译入口修复：`core` 由**内联块**改为**文件驱动**。
                    //   内联块会导致 `core/mod.rs` 永远不被 Rust 读取（全仓 `#[path]` = 0），
                    //   使 `core/mod.rs` 中的声明全是死代码 —— 这正是本里程碑各批次
                    //   「登记→数百错误→回退」的根因。改为文件驱动后，模块清单由
                    //   `core/mod.rs` 单一维护，新增模块只改一处。
                    pub mod core;
                pub mod helper {
                    pub mod soft_drawing;
                    pub mod CoreHelper;
                }
                    pub mod cache {
                    pub mod TypoColumns;
                    pub mod TypoObjBase;
                    pub mod TypoMemoryObj;
                    pub mod TypoSnapshot;
                    pub mod RevisionCache;
                    pub mod TypoRectMemoryImpl;
                    pub mod TypoDrawing;
                    pub mod TypoDrawingLayers;
                    pub mod TypoList;
                    pub mod ITypoRect;
                    pub mod TypoToolsManager;
                    pub mod TypoLayoutPage;
                    pub mod TypoPara;
                    pub mod TypoTextLine;
                    pub mod TypoText;
                    pub mod TypoTextsOpIterator;
                    pub mod TypoRangeBase;
                    pub mod TypoPage;
                    pub mod TypoTextBox;
                    pub mod TypoTableCell;
                    pub mod TypoTableRow;
                    pub mod TypoRectCoreImpl;
                    pub mod TypoPageLinesIterator;
                    pub mod TypoRun;
                    pub mod TypoDocument;
                    pub mod SnapshotBuild;
                    pub mod Stage2Rows;
                    pub mod RustStage2;
                    pub mod SnapshotCommit;
                    pub mod Stage2Json;
                    pub mod PageHit;
                    pub mod CpHit;
                    pub mod HitTest;
                    pub mod style {
                        pub mod TypoSHDs;
                        pub mod TypoSHDAndBRCs;
                        pub mod TypoBRCs;
                        pub mod TypoFontStyle;
                        pub mod TypoLines;
                        pub mod TypoHighlights;
                        pub mod TypoBorders;
                    }
                }
                pub mod drawing {
                    // rust: ★M-STAGE2 P1-D1 / T2 —— `TextDocument.rs:560` 的就地影子；
                    //   真实文件 `writer/drawing/ShapeFactoryDoc.rs`（45 行）。
                    pub mod ShapeFactoryDoc;
                    // Java: cn.wps.moffice.writer.drawing.DrawingContainer
                    pub mod DrawingContainer;
                }
                pub mod layout {
                    // rust: ★T7-7 —— `data::field::StrFormatUtil` 依赖
                    //   `layout::numformat::{CodePage,NFIntDefine,NFFunDefine,NFStrDefine}`
                    //   （域代码数字/文本格式化的码页与格式定义）。该目录**真实存在**
                    //   （`layout/numformat/`，含 `mod.rs`），但 `layout` 在 lib.rs 中
                    //   为内联块 ⇒ 从未编译。登记以解 `E0433`。
                    pub mod numformat {
                        pub mod CodePage;
                        pub mod Numformat;
                        pub mod NFIntDefine;
                        pub mod NFFunDefine;
                        pub mod NFStrDefine;
                    }
                    pub mod FlowLayout;
                    pub mod text {
                        pub mod FontStack;
                        pub mod FontHost;
                        pub mod BaseTextLayout;
                    }
                    pub mod base {
                        pub mod env {
                            pub mod ViewEnv;
                            pub mod DrawingCacheItem;
                            pub mod FontSize;
                            pub mod RunFlags;
                            pub mod FontNames;
                            pub mod StyleMerge;
                            pub mod TypoFontStyle;
                            pub mod FontStyleFactory;
                            pub mod buildinfont {
                                pub mod metrics {
                                    pub mod SimsunMetrics;
                                }
                            }
                            pub mod UnitsToTwip;
                            pub mod GlyphAdvance;
                            pub mod FaceSelect;
                            pub mod LineMetrics;
                            pub mod ContentWidth;
                            pub mod LineWrap;
                            pub mod LineAlign;
                            pub mod LineBox;
                            pub mod LineLayout;
                            pub mod PageFit;
                            pub mod WidowControl;
                            pub mod PageLayout;
                            pub mod ParaEnv;
                            pub mod PageEnv;
                            pub mod ParaPropEnv;
                            pub mod DocumentEnv;
                            pub mod FootEndNoteEnv;
                            pub mod MainState;
                            pub mod Pools;
                            pub mod TBD;
                            pub mod IPropertyTool;
                            pub mod PropertyTool;
                            pub mod FontStyleCache;
                            pub mod CachedPropertySet;
                            pub mod LayoutState;
                            pub mod SuperEnvoy;
                            pub mod TableCellLayoutState;
                            pub mod TableCellStateEnvoy;
                            pub mod PropertyToolFactory;
                            pub mod FontStyleCacheFactory;
                        }
                        pub mod drawing {
                            pub mod DrawingCoordTool;
                            pub mod DrawingAlignRectTool;
                            pub mod DrawingLineTool;
                            pub mod DrawingShapeRectTool;
                            pub mod DrawingResizeTool;
                            pub mod textbox {
                                pub mod TextboxLayoutState;
                                pub mod TextboxStateEnvoy;
                            }
                            pub mod DrawingLayouter;
                            pub mod ShapeCollision;
                            pub mod TextWrapLayouter;
                        }
                        pub mod tools {
                            pub mod TypefaceTool;
                            pub mod BRCBorderTool;
                        }
                        pub mod bidi {
                            pub mod BidiEmbeddingsCache;
                            pub mod BidiItem;
                            pub mod BidiLevelCache;
                            pub mod BidiTool;
                        }
                        pub mod line {
                            pub mod RunMetrics;
                            pub mod LineLayouter;
                            pub mod TextOpCollector;
                            pub mod style {
                                pub mod UnderlineCollector;
                                pub mod StrikeThroughCollector;
                                pub mod HighlightCollector;
                                pub mod SHDCollector;
                                pub mod BordersCollector;
                            }
                            pub mod AutoNumLayouter;
                            pub mod LineCompressEnv;
                            pub mod LineCompressTool;
                            pub mod SubLineLayouterBase;
                            pub mod MathParaSubLineTool;
                            pub mod LineRuleGraphsPos;
                            pub mod SubLineEnv;
                            pub mod SubLineLayouter;
                            pub mod ILineHeightCallback;
                            pub mod LineHeightLayouter;
                            pub mod TabHandler;
                            pub mod charSpace;
                            pub mod charborder;
                            pub mod align;
                        }
                        pub mod wordbreaker {
                            pub mod HyphenFlag;
                        }
                        pub mod languages {
                            pub mod ILangLayout;
                            pub mod LangLayout;
                            pub mod LangLayoutFactory;
                            pub mod WestLayout;
                            pub mod FarEastLayout;
                            pub mod SharedFarEastLayout;
                        }
                        pub mod text {
                            pub mod RunWidthCache;
                            pub mod Wrap;
                            pub mod Alignment;
                            pub mod LineHeight;
                            pub mod CpText;
                            pub mod ChpxRuns;
                            pub mod PapxParas;
                            pub mod RunStream;
                        }
                        pub mod PageBreak;
                        pub mod PageGrid;
                        pub mod RelayoutCache;
                        pub mod graph {
                            pub mod GraphReaderTool;
                            pub mod Graphs;
                            pub mod GraphAlignTool;
                            pub mod GraphReader;
                            pub mod GraphBuff;
                            pub mod GraphFactory;
                            pub mod GraphBuffFactory;
                            pub mod CharsNode;
                            pub mod SrcNode;
                            pub mod FilterArgs;
                            pub mod IFilter;
                            pub mod IRunPropFilter;
                            pub mod FunctionSepField;
                            pub mod FieldFilter;
                            pub mod MathFilter;
                            pub mod OMathFilter;
                            pub mod PhoneViewOMathFilter;
                            pub mod PhoneViewBalloonDrawingFilter;
                            pub mod FootEndNoteFilter;
                            pub mod FootEndRefCharFilter;
                            pub mod GraphArrayDeque;
                            pub mod DrawingNode;
                            pub mod OrgWidthCache;
                        }
                        pub mod LayouterBase;
                        pub mod ParaLayouter;
                        pub mod SectionTool;
                        pub mod footendnote {
                            pub mod IFootEndNoteNumLayouter;
                            pub mod FootEndNotePropCalc;
                        }
                        pub mod TextLayouter;
                        pub mod LayoutTraceCollector;
                        pub mod MainLayouter;
                        pub mod page {
                            pub mod PageLayouter;
                            pub mod MainPageLayouter;
                        }
                        pub mod table {
                            pub mod TableRowLayouter;
                            pub mod TableCellLayouter;
                            pub mod TableCellPageLayouter;
                            pub mod TableCellBrcTool;
                            pub mod TableLayouter;
                        }
                        pub mod balloons {
                            pub mod content {
                                pub mod BalloonContentLayouter;
                            }
                        }
                        pub mod math {
                            pub mod MathLayouter;
                            pub mod MathLayouterBase;
                            pub mod DelimiterLayouter;
                            pub mod AccentLayouter;
                            pub mod BarLayouter;
                            pub mod BoxLayouter;
                            pub mod BorderBoxLayouter;
                            pub mod ArrayLayouter;
                            pub mod FractionLayouter;
                            pub mod FunctionLayouter;
                            pub mod GroupCharacterLayouter;
                            pub mod LimLayouter;
                            pub mod LimLowerLayouter;
                            pub mod LimUpperLayouter;
                            pub mod PhantLayouter;
                            pub mod MatrixLayouter;
                            pub mod NaryLayouter;
                            pub mod RadLayouter;
                            pub mod SupSubBaseLayouter;
                            pub mod SSubSupLayouter;
                            pub mod SSubLayouter;
                            pub mod SSupLayouter;
                            pub mod SPreLayouter;
                            pub mod SepShapeFactory;
                            pub mod MathContentLayouter;
                            pub mod AlignOMathParaHandler;
                        }
                        pub mod equation {
                            pub mod EquationResult;
                            pub mod EquationParser;
                            pub mod EquationFactory;
                            pub mod EquationLayouter;
                            pub mod EquationContentLayouter;
                            pub mod EquationAlignment;
                            pub mod IEquationLayouter;
                            pub mod EquationALayouter;
                            pub mod EquationBLayouter;
                            pub mod EquationILayouter;
                            pub mod EquationXLayouter;
                            pub mod EquationFLayouter;
                            pub mod EquationRLayouter;
                            pub mod EquationLinearLayouter;
                            pub mod EquationDLayouter;
                            pub mod EquationLLayouter;
                            pub mod EquationOLayouter;
                            pub mod EquationSLayouter;
                        }
                    }
                    pub mod pageview;
                    pub mod ShareView {
                        pub mod ShareViewRunPropFilter;
                    }
                }
                pub mod render {
                    pub mod renderer {
                        pub mod PageRender;
                        pub mod TableRender;
                        pub mod BRCBorderRender;
                    }
                }
                pub mod rendercommand {
                    pub mod RenderCommand;
                    pub mod TextCommand;
                    pub mod DecoCommand;
                    pub mod Serialize;
                    pub mod Stage3Normalize;
                    pub mod StyleRuns;
                    pub mod StyleCommands;
                    pub mod PageCommands;
                }
                pub mod linegolden {
                    pub mod LineNormalize;
                    pub mod LineCompare;
                    pub mod LineCorpus;
                    pub mod LiveCorpus;
                }
                pub mod api {
                    pub mod stage1;
                    pub mod DocumentOpen;
                    pub mod SaxDocumentBridge;
                    pub mod DocumentSession;
                    pub mod StyleSnapshot;
                    pub mod WordStatistics;
                    pub mod SessionCaret;
                    pub mod SelectionRange;
                    pub mod SelectionRects;
                    pub mod SelectionFill;
                    pub mod SessionEdit;
                    pub mod LayoutDocument;
                    pub mod RenderDocx;
                    pub mod RenderPainter;
                    pub mod Viewport;
                }
                pub mod io {
                    pub mod tools {
                        pub mod ColorTransform;
                    }
                    // rust: ★M-STAGE2 P1-D1 / T2 —— `IDocumentCallback` 的依赖。
                    pub mod WriterIOException;
                    // rust: ★T7-8 —— `core::TextTool` 依赖 `io::sprm::Character`
                    //   （`TextTool.rs:39`）。`io/sprm/` **真实存在**（含 `mod.rs`），
                    //   但 `io` 在 lib.rs 为内联块 ⇒ 未编译，导致 E0433。
                    pub mod sprm;
                    pub mod reader {
                        // 01_function_importer_mode Class C: constructable empty shells.
                        pub mod xml03;
                        pub mod xml07 {
                            pub mod Xml07Context;
                            // rust: ★M-STAGE1 P2-12 —— xml07 子系统按 §48.2 拓扑逐层做实
                            //   第 1 层（无依赖）：constant / pi / cl
                            pub mod constant {
                                pub mod CONST;
                            }
                            pub mod pi {
                                pub mod IProcInfo;
                                pub mod PiOLE;
                                pub mod PiSectPr;
                                pub mod PiBlipFill;
                                pub mod PiInk;
                                pub mod PiChart;
                            }
                            pub mod cl {
                                pub mod ClRefInfo;
                                pub mod ClInfo;
                            }
                            // rust: ★只登记 `co::ICoreObject`（trait）。
                            //   `co::CoOLE` / `co::CoreObject` / `cl::ClRefInfo` **未登记**：
                            //   三者此前从未编译，登记后暴露 16 个错误
                            //   （`VersionState` 未登记、`CoreObject::getNext` 返回
                            //   `Box<dyn>` 而 trait 声明 `&dyn`、`ClRefInfo::mNext` 值语义等）
                            //   ⇒ 按 §45.2 清单不登记，列入 §48.2 第 1 层工单。
                            pub mod co {
                                pub mod ICoreObject;
                            }
                        }
                        pub mod ImporterBase;
                        pub mod face {
                            pub mod IReaderBase;
                            pub mod IDocxReader;
                            // rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 ——
                            //   `TextDocument.rs:574` 的就地影子 `pub trait IEbookReader {}`；
                            //   真实文件 9 行、零依赖。
                            pub mod IEbookReader;
                        }
                        // rust: ★M-STAGE2 P1-D1 / T1 —— `TextDocument.rs:573` 的影子
                        //   `pub trait IRtfReader {}`；真实文件 6 行、零依赖。
                        pub mod rtf {
                            pub mod IRtfReader;
                        }
                        pub mod TextboxInfoTool;
                        pub mod helper {
                            pub mod IOHelper;
                            pub mod PresetType2Shadow;
                            pub mod ParagraphContext;
                            // rust: ★M-STAGE1 P2-14 —— 原文件为模板引擎破损桩
                            //   （悬空 `>` 泛型 / 19 todo! / 未登记 ⇒ 长期不可编译），
                            //   本轮修正泛型 + 登记 + 管线部分真实实现 + 其余诚实 deferral。
                            pub mod TablePropCorrector;
                        }
                        pub mod tools {
                            pub mod FinishReadingWork;
                        }
                        pub mod docReader {
                            pub mod corrector {
                                pub mod DocumentCorrector;
                                pub mod section {
                                    // rust: ★M-STAGE1 P2-4 —— 由 auto-generated 桩做实（3 int + 3 对访问器）
                                    pub mod NodeContext;
                                    // rust: ★M-STAGE1 P2-5 —— 由 auto-generated 桩做实（475 行 Java）
                                    pub mod HdrFtrContext;
                                }
                            }
                            // rust: ★M-STAGE1 P2 拆环 —— `core/FontTable.rs` 需要
                            //   `docReader::importer::FontTableImporter::toFontInfo(Ffn)`；
                            //   Java 中该**静态方法确实定义在 docReader 侧**
                            //   （`docReader/importer/FontTableImporter.java:30`），
                            //   与 docx 侧的同名类是**两个不同的类**。
                            pub mod importer {
                                pub mod FontTableImporter;
                            }
                        }
                        // rust: ★M-STAGE1 P2-10 —— `dmlReader::context` 树
                        //   （原全仓未登记）。只登记闭包已闭合的部分：
                        //   DML context 与已完成的 importer 基类按实际编译面登记。
                        pub mod dmlReader {
                            // Java DmlImporter.anchor/inline -> PositioningContext.
                            pub mod PositioningContext;
                            pub mod DmlImporterBase;
                            pub mod FillBaseImporter;
                            pub mod SolidFillImporter;
                            pub mod BlipFillImporter;
                            pub mod PictureImporter;
                            pub mod DmlShapeImporter;
                            pub mod DmlCommonImporter;
                            pub mod DmlGroupShapeImporter;
                            pub mod ThemeImporter;
                            pub mod DmlImporter;
                            pub mod GRFImporter;
                            pub mod PresetTextWarpImporter;
                            pub mod ShapePosImporter;
                            pub mod PatternFillImporter;
                            pub mod GradFillImporter;
                            pub mod LineImporter;
                            pub mod GeometryImporter;
                            pub mod InnerShdwImporter;
                            pub mod ShadowImporter;
                            pub mod OuterShdwImporter;
                            pub mod PrstShdwImporter;
                            pub mod TextImporter;
                            pub mod GlowImporter;
                            pub mod DuotoneImporter;
                            pub mod ClrFromImporter;
                            pub mod helper {
                                pub mod EffectHelper;
                                pub mod TextShapeType;
                                pub mod LineImporterHelper;
                                pub mod MetricHelper;
                                pub mod HSLHelper;
                                pub mod ColorsByName;
                                pub mod ColorsTrans;
                                pub mod LnRefHelper;
                                pub mod FillRefHelper;
                                pub mod EffectRefHelper;
                                pub mod PrstShdwMap;
                            }
                            pub mod context {
                                pub mod Element;
                                pub mod StartTag;
                                pub mod EndTag;
                                pub mod Branch;
                                pub mod ThemeContext;
                                pub mod TxbxChainItem;
                                pub mod TxbxChain;
                                pub mod TxbxChains;
                                pub mod Tile;
                                pub mod ColorTransform;
                                pub mod ScrgbClr;
                                pub mod SrgbClr;
                                pub mod SysClr;
                                pub mod SchemeClr;
                                pub mod PrstClr;
                                pub mod ColorChoice;
                                pub mod ClrFrom;
                                pub mod Duotone;
                                pub mod Gs;
                                pub mod FgClr;
                                pub mod BgClr;
                                pub mod SolidFill;
                                pub mod InnerShdw;
                                pub mod Shadow;
                                pub mod OuterShdw;
                                pub mod PrstShdw;
                                pub mod LnRef;
                                pub mod FillRef;
                                pub mod EffectRef;
                                pub mod Glow;
                                pub mod Cxn;
                                pub mod Cxnlst;
                                pub mod FormulaType;
                                pub mod Fmla;
                                pub mod Gds;
                                pub mod DmlShapeCtxt;
                                pub mod DmlSubDocCtxt;
                            }
                        }
                        pub mod vmlReader {
                            pub mod importer {
                                pub mod vml {
                                    pub mod helper {
                                        pub mod VmlHelper;
                                        pub mod VmlStyle;
                                    }
                                    pub mod producer {
                                        pub mod ShapeAttrConverter;
                                        pub mod ShapePosConverter;
                                        pub mod ImageDataConverter;
                                    }
                                    pub mod ShapeCoreImporter;
                                    pub mod ShapeImporter;
                                    pub mod NormalShapeImporter;
                                    pub mod LineImporter;
                                    pub mod RoundRectImporter;
                                    pub mod GroupShapeImporter;
                                    pub mod ShapeTypeImporter;
                                    pub mod VmlImporter;
                                }
                            }
                        }
                        pub mod docxReader {
                            pub mod helper {
                                pub mod DocxReaderHelper;
                                // rust: ★M-STAGE1 P2-11 —— 已实（100 行 / 0 todo），登记以解除 marker
                                pub mod ShapeIDTool;
                            }
                            pub mod DocxReader;
                            pub mod importer {
                                // rust: ★M-STAGE2 P1-D1 —— 脚注/尾注导入子系统真实移植（第 7 轮）。
                                pub mod FootnoteEndnoteDocumentImporter;
                                pub mod FootnoteDocumentImporter;
                                pub mod EndnoteDocumentImporter;
                            pub mod BookmarkImporter;
                                pub mod helpers {
                                    pub mod BuildinStyleSheet;
                                    pub mod ConvertHelper;
                                    pub mod WmlUnit;
                                    pub mod ImporterHelper;
                                    pub mod BorderConvertSimple;
                                    pub mod ShareConvertHelper;
                                    pub mod RunPropsConverter;
                                    pub mod RunPropsConverterSimple;
                                    pub mod ThemeFontTool;
                                    pub mod ParaPropsConverterSimple;
                                    pub mod ParaPropsConverter;
                                    // rust: ★M-STAGE1 P5 —— ★★★假的编译面消除（根因，见 §66）：
                                    //   下列模块在 `helpers/mod.rs` 里被声明，但**从未在 lib.rs 登记**
                                    //   ⇒ 语法错误探针证明**根本没编译**。
                                    //   `WordBuildinStyleNames`（573 行 / 0 todo）与
                                    //   `BlipFillInfos`（603 行 / 2 todo）是**已写好却从未编译**的代码。
                                    pub mod WordBuildinStyleNames;
                                    pub mod BlipFillInfos;
                                    // rust: ★M-STAGE2 P1-D1 —— 脚注/尾注子系统真实移植（第 7 轮）。
                                    pub mod UUFootnoteEndnoteId;
                                    pub mod TablePropertyBaseConverterSimple;
                                    pub mod CellPropertyConverterSimple;
                                }
                                pub mod StyleId2Sti;
                                pub mod MoveRangeImporter;
                                pub mod NumIdRedirection;
                                pub mod IOStyle;
                                pub mod StylesImporter;
                                // rust: ★M-STAGE1 P2-2 —— `FontTableImporter`（docx 侧）由桩做实后登记。
                                //   ★与 `docReader::importer::FontTableImporter` 是**两个不同的类**
                                //   （Java 同名不同包），互不冲突。
                                pub mod FontTableImporter;
                                pub mod num {
                                    pub mod AbstractNumIdRedirection;
                                    pub mod IOLvl;
                                    pub mod IOLvlOverride;
                                    pub mod IONum;
                                    pub mod IOAbstractNum;
                                    pub mod NumberingLevelImporter;
                                    pub mod NumImporter;
                                    pub mod AbstractNumImporter;
                                    pub mod NumberingImporter;
                                }
                                // rust: ★M-STAGE1 P2-3 —— 解除 `compile-surface:` 影子
                                pub mod DocumentImporter;
                                pub mod document {
                                    pub mod SubDocumentImporter;
                                    pub mod MainDocumentImporter;
                                    pub mod HdrFtrDocumentImporter;
                                    // rust: ★M-STAGE1 P2-7 —— textbox 家族由 auto-generated 桩做实
                                    pub mod TextboxDocumentImporter;
                                    pub mod MainTextboxDocumentImporter;
                                    pub mod HeaderTextboxDocumentImporter;
                                }
                                // rust: ★M-STAGE1 P2 —— `SettingsImporter` 的依赖闭包。
                                //   按 §24.4 判据：闭包内类型先做实并登记，最后才动
                                //   `SettingsImporter` 本身与 `di03_seam` 影子。
                                // rust: ★M-STAGE1 P2-8 —— theme 家族
                                //   `FontSchemeImporter` 的闭包（FontCollection /
                                //   SupplementalFont / ThemeFontScheme / FontInfo /
                                //   ConvertHelper）已全部做实 ⇒ **登记并可测**。
                                pub mod theme {
                                    pub mod FontSchemeImporter;
                                    // rust: ★M-STAGE1 P2-9 —— `Theme` 闭包已闭合 ⇒ 登记
                                    pub mod ThemeImporter;
                                }
                                pub mod settings {
                                    pub mod NoLineBreaksImporter;
                                    // rust: ★M-STAGE1 P2 —— 闭包**最后一个成员**。
                                    pub mod CapationsImporter;
                                    // rust: ★M-STAGE1 P2 —— SettingsImporter 的加密 provider 枚举（全仓原缺失）。
                                    pub mod CryptProvType;
                                    // rust: ★M-STAGE1 P2 —— SettingsImporter 闭包成员（全仓原缺失）。
                                    pub mod StylePaneFormatFilterImporter;
                                    pub mod SmartTagTypeImporter;
                                    pub mod ClrSchemeMappingImporter;
                                    // rust: ★M-STAGE1 P2 —— 闭包已完整（§25–§29）⇒ 登记本体。
                                    //   Java 1774 行 / 112 方法；实体正分段做实。
                                    pub mod SettingsImporter;
                                }
                            }
                        }
                    }
                }
                // rust: ★M-STAGE2 P1-D1 —— 假的编译面消除。
                //   `VersionState`（629 行 / 215 `// java` / 1 todo）是真实移植产物，
                //   此前只在 `writer/mod.rs` 声明、从未进入 lib.rs 内联树 ⇒ 不编译。
                pub mod VersionState;
                    // rust: ★T7-5b —— 同上：`data` 改为文件驱动（入口修复）。
                    pub mod data;
            }
        }
    }
}

/// Placeholder for generated `crate::VersionState` imports.
#[derive(Default, Clone, Copy, Debug)]
pub struct VersionState;
