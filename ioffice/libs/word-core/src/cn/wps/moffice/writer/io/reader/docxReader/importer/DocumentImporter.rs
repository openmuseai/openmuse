#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MathArgmentType
use util::base::io::MathArgmentType::*;
// Java: import cn.wps.base.io.SubDocType
use util::base::io::SubDocType::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.graphics.PictureInfo
use util::graphics::PictureInfo::*;
// Java: import cn.wps.graphics.PictureParser
// soft: PictureParser 依赖 core_libs_picparser（crate 自身 10 个预存编译错误，依赖不可用）；媒体/图片路径已 DEFERRED 停车
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::*;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import cn.wps.graphics.bitmap.IPicturePool
use util::graphics::bitmap::IPicturePool::*;
// Java: import cn.wps.io.opc.CustomXmlInfo
use io_wps_io::cn::wps::io::opc::CustomXmlInfo::*;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::*;
// Java: import cn.wps.io.xwpf.IDmlImporter
use io_wps_io::cn::wps::io::xwpf::IDmlImporter::IDmlImporter;
// Java: import cn.wps.io.xwpf.IDmlTextImporter
use io_wps_io::cn::wps::io::xwpf::IDmlTextImporter::IDmlTextImporter;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::*;
// Java: import cn.wps.io.xwpf.IVmlImporter
use io_wps_io::cn::wps::io::xwpf::IVmlImporter::IVmlImporter;
// Java: import cn.wps.io.xwpf.MathIOType
use io_wps_io::cn::wps::io::xwpf::MathIOType::*;
// Java: import cn.wps.io.xwpf.element_handler.ParagraphSimple
use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
// Java: import cn.wps.io.xwpf.element_handler.RubyContent
use io_wps_io::cn::wps::io::xwpf::element_handler::RubyContent::RubyContent;
// Java: import cn.wps.io.xwpf.element_handler.RubyPr
use io_wps_io::cn::wps::io::xwpf::element_handler::RubyPr::RubyPr;
// Java: import cn.wps.io.xwpf.element_handler.SectionHeadFooterProps
// rust: ★M-STAGE1 P2 策略 B —— 该类型实体已在 wps-io 完整实现且**已登记**
//   （`element_handler/mod.rs:76`），零依赖闭包 ⇒ 解除该假编译面并导入真类型。
use io_wps_io::cn::wps::io::xwpf::element_handler::SectionHeadFooterProps::SectionHeadFooterProps;
// Java: import cn.wps.io.xwpf.element_handler.numbering.NumLvlType
use io_wps_io::cn::wps::io::xwpf::element_handler::numbering::NumLvlType::NumLvlType;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
use io_wps_io::cn::wps::io::xwpf::element_handler::prop::PropHandlerHelper::PropHandlerHelper;
// Java: import cn.wps.io.xwpf.element_handler.styles.StylePrModel
use io_wps_io::cn::wps::io::xwpf::element_handler::styles::StylePrModel::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFDocument::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFHeaderFooter
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFHeaderFooter::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFRelation
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFRelation::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFSettings
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFSettings::*;
// Java: import cn.wps.moffice.drawing.GRF
use drawing::cn::wps::moffice::drawing::GRF::GRF;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::OLE;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeFactory
use drawing::cn::wps::moffice::drawing::ShapeFactory::ShapeFactory;
// Java: import cn.wps.moffice.drawing.ShapePos
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
// Java: import cn.wps.moffice.drawing.graphics.Picture
// compile-surface: use drawing::cn::wps::moffice::drawing::graphics::Picture::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
// compile-surface: use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
// Java: import cn.wps.moffice.io.IDLProvider
use util_adapter::cn::wps::moffice::io::IDLProvider::*;
// Java: import cn.wps.moffice.kfs.File
// compile-surface: use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileOutputStream
// compile-surface: use kfs_kfs_app::cn::wps::moffice::kfs::FileOutputStream::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.FileUtil
// rust: ★M-STAGE1 P2 —— 真实 `util::moffice::util::FileUtil` **缺少** `newFolder`
//   （Java 原文 `FileUtil.java:942` 有），且其依赖的 `stubs::java::io::File`
//   是**纯 no-op 平台垫片**（`exists`/`mkdirs` 恒 false）⇒ 在本 crate 无法忠实实现。
//   故此处**保留** di03_seam 内的 `FileUtil` 影子（仅提供 `newFolder` no-op），
//   并把该缺口登记为待补项，而不是编造 stub 行为。
// compile-surface: use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.writer.VersionState
// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除：`VersionState` 已在 lib.rs 登记，
//   其 55 个模板错误已全部修实（`impl ISharedInfo for …`、可变静态改为
//   `Mutex<Box<dyn ISharedInfo + Send + Sync>>`、字段去 `Option`、
//   `sSharedInfo` 加锁转发），§3.8 门禁 PASS ⇒ 解除占位，改真实引用。
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KParagraphsID
// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除。
//   真实模块 `core/KParagraphsID.rs` 已在 lib.rs 登记、影子已从 TextDocument.rs 删除
//   ⇒ 解除此 marker，改为真实引用。
use crate::cn::wps::moffice::writer::core::KParagraphsID::KParagraphsID;
// Java: import cn.wps.moffice.writer.core.KRange
// compile-surface: use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.io.IOListener
// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除。
//   真实模块 `core/io/IOListener.rs` 已在 lib.rs 登记、`wc04_seam` 影子已删除
//   ⇒ 解除此 marker，改为真实引用。
use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;
// Java: import cn.wps.moffice.writer.core.metadata.Metadata
// compile-surface: use crate::cn::wps::moffice::writer::core::metadata::Metadata::*;
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
// compile-surface: use crate::cn::wps::moffice::writer::core::shape::ShapeTool::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::*;
// Java: import cn.wps.moffice.writer.data.PLCField
// compile-surface: use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
// compile-surface: use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.DmlTextImporter
// compile-surface: use crate::cn::wps::moffice::writer::io::reader::dmlReader::DmlTextImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.context.ThemeContext
// rust: ★M-STAGE1 P2-10 —— 已解除该 marker（真实现见 dmlReader/context/ThemeContext.rs）
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::ThemeContext::ThemeContext;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.helper.ShapeIDTool
// rust: ★M-STAGE1 P2-11 —— 已解除该 marker（真实现见 helper/ShapeIDTool.rs）
use crate::cn::wps::moffice::writer::io::reader::docxReader::helper::ShapeIDTool::ShapeIDTool;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.CommentsDocumentImporter
// compile-surface: use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::CommentsDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.HdrFtrDocumentImporter
// rust: ★M-STAGE1 P2-6 —— 已解除该 marker（真实现见 document/HdrFtrDocumentImporter.rs）
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::HdrFtrDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.HeaderTextboxDocumentImporter
// rust: ★M-STAGE1 P2-7 —— 已解除该 marker（真实现见 document/HeaderTextboxDocumentImporter.rs）
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::HeaderTextboxDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.MainDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::MainDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.MainTextboxDocumentImporter
// rust: ★M-STAGE1 P2-7 —— 已解除该 marker（真实现见 document/MainTextboxDocumentImporter.rs）
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::MainTextboxDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.TextboxDocumentImporter
// rust: ★M-STAGE1 P2-7 —— 已解除该 marker（真实现见 document/TextboxDocumentImporter.rs）
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::TextboxDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.BlipFillInfos
// compile-surface: use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BlipFillInfos::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.CellPropertyConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::CellPropertyConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ParaPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.RunPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::RunPropsConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.TablePropertyBaseConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::TablePropertyBaseConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.num.NumberingImporter
// rust: ★编号导入改为真实 `num::NumberingImporter`，与文档共享 StyleId2Sti / NumIdRedirection。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::num::NumberingImporter::NumberingImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.settings.SettingsImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::SettingsImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.theme.ThemeImporter
use crate::cn::wps::moffice::writer::data::theme::Theme::Theme;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::theme::ThemeImporter::ThemeImporter;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.TablePropCorrector
// rust: ★M-STAGE1 P2-14 —— 原 marker 记录了「helper TablePropCorrector.rs 语法损坏」。
//   本轮已**修正泛型签名 + 登记**（悬空 `>` / 19 todo! 全部处理），
//   故解除该假编译面并导入真类型（§58.7：解除后须做「真实使用」验证）。
use crate::cn::wps::moffice::writer::io::reader::helper::TablePropCorrector::TablePropCorrector;
// Java: import cn.wps.moffice.writer.io.reader.vmlReader.VmlReader
// compile-surface: use crate::cn::wps::moffice::writer::io::reader::vmlReader::VmlReader::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import cn.wps.moffice.writer.io.reflection.ReflectionHelper
// compile-surface: use crate::cn::wps::moffice::writer::io::reflection::ReflectionHelper::*;
// Java: import cn.wps.moffice.writer.io.writer.doc.ICmtCustDatasReader
// compile-surface: use crate::cn::wps::moffice::writer::io::writer::doc::ICmtCustDatasReader::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import org.xml.sax.Attributes
// rust: ★该 marker 原本指向 **错误的类型**（伪阻塞，§56.5）。
//   Java 原文 `:97 import org.xml.sax.Attributes;` 是 **SAX 接口**；
//   其忠实 Rust 建模是 `ConvertHelper::Attributes` **trait**
//   （`ConvertHelper.rs:50`，已由上方 `:174` 的 glob 导入）。
//   `kfc_kfc_alpha::...::token::Attributes` 是**另一个结构体**（HTML 词法 token），
//   `stubs::org::xml::sax::Attributes` 则是 `new()` 里 `todo!()` 的破损桩。
//   ⇒ 无需导入，marker 予以消除。
// Java: import org.xml.sax.SAXException
// rust: ★M-STAGE2 P1-D1 —— Java 侧 `SAXException` **仅出现在 `throws` 子句**
//   （`:195 onImportParagraphStart` / `:703 onImportSettingsChild`），方法体内无使用；
//   Rust 无受检异常对应物 ⇒ `// rust: no-stmt`。原桩 `stubs::org::xml::sax::SAXException`
//   已随 `libs/core-libs/stubs` 的 `org` 目录移除，此处不再引用。
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StylesImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StylesImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.FontTableImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::FontTableImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.EndnoteDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::EndnoteDocumentImporter::EndnoteDocumentImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.FootnoteDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::FootnoteDocumentImporter::FootnoteDocumentImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.MoveRangeImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::MoveRangeImporter::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// compile-surface: use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import cn.wps.io.dom.Node
// compile-surface: use io_wps_io::cn::wps::io::dom::Node::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.BookmarkImporter
// rust: ★M-STAGE2 P1-D1 —— 真实模块 `importer/BookmarkImporter.rs` 已登记并接线（第 5 轮）。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::BookmarkImporter::BookmarkImporter;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};
use std::cell::RefCell;
use std::rc::Rc;

mod di03_seam {
    use super::*;
    #[derive(Default)] pub struct CommentsDocumentImporter;
    #[derive(Default)] pub struct VmlReader;
    impl VmlReader {
        pub fn clearShapeTypes(_n: i32) {}
    }
    #[derive(Default)] pub struct PLCField;
    pub struct BlipFillInfos;
    impl BlipFillInfos { pub fn dispose() {} }
    #[derive(Default)] pub struct FileUtil;
    impl FileUtil { pub fn newFolder(_p: Option<&String>) {} }
    #[derive(Default)] pub struct SeamParagraphSimple;
    pub trait IPicturePool {}
    pub trait IDrawingContainer {}
    pub trait IMediaLib {}
    pub trait IDmlImporter {}
    pub trait IDmlTextImporter {}
    pub trait IVmlImporter {}
    pub struct SeamSaxAttributes;
    pub struct SeamPoiEntry;
}
use di03_seam::*;
// rust: ★M-STAGE1 P2-1 —— 原先此处 `pub use di03_seam::{NumLvlType, XWPFSettings};`
//   把**影子** `XWPFSettings` 再导出。`XWPFSettings` 现已改用 wps-io 的**真实类型**
//   （见文件顶部已解除的那些 marker）⇒ 从再导出中移除，避免遮蔽真实类型。
//   `NumLvlType` 仍属 B 类（仓库内暂无该类型），暂留影子。
// rust: 实测该再导出**无任何消费方**（全仓 grep `DocumentImporter::XWPFSettings` / `::NumLvlType` = 0）。
pub 
use crate::cn::wps::moffice::writer::core::UUSti::UUSti;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::UUNumberingId;


// java public class DocumentImporter extends ImporterBase implements IDocumentImporter
#[derive(Default)]
pub struct DocumentImporter {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
    // java protected TextDocument mDocument;
    // typeoracle: Java stores the same TextDocument as KStyles; tests keep owner, importer aliases
    pub mDocument: Option<*mut TextDocument>,
    // java private IOListener mIoListener;
    mIoListener: Option<Box<dyn IOListener>>,
    // java private StylesImporter mStylesImporter;
    pub mStylesImporter: Option<StylesImporter>,
    // java private SettingsImporter mSettingsImporter;
    mSettingsImporter: Option<SettingsImporter>,
    // java private FontTableImporter mFontTableImporter;
    mFontTableImporter: Option<FontTableImporter>,
    // java private NumberingImporter mNumberingImporter;
    mNumberingImporter: Option<NumberingImporter>,
    // java private ThemeImporter mThemeImporter;
    mThemeImporter: Option<ThemeImporter>,
    // java private ParaPropsConverterSimple mParaPropsConverter;
    pub mParaPropsConverter: Option<ParaPropsConverterSimple>,
    // java private CellPropertyConverterSimple mCellPropertyConverter;
    mCellPropertyConverter: Option<CellPropertyConverterSimple>,
    // java private RunPropsConverterSimple mRunPropsConverter;
    mRunPropsConverter: Option<RunPropsConverterSimple>,
    // java private TablePropertyBaseConverterSimple mTablePropsConverter;
    mTablePropsConverter: Option<TablePropertyBaseConverterSimple>,
    // java protected MainDocumentImporter mMainDocumentImporter;
    pub(crate) mMainDocumentImporter: Option<MainDocumentImporter>,
    // java private TextboxDocumentImporter mTextboxDocumentImporter;
    mTextboxDocumentImporter: Option<MainTextboxDocumentImporter>,
    // java private TextboxDocumentImporter mHdrFtrTextboxDocumentImporter;
    mHdrFtrTextboxDocumentImporter: Option<HeaderTextboxDocumentImporter>,
    // java protected HdrFtrDocumentImporter mHdrFtrDocumentImporter;
    pub(crate) mHdrFtrDocumentImporter: Option<HdrFtrDocumentImporter>,
    // java private CommentsDocumentImporter mCommentsDocumentImporter;
    mCommentsDocumentImporter: Option<SubDocumentImporter>,
    // java private EndnoteDocumentImporter mEndnoteDocumentImporter;
    mEndnoteDocumentImporter: Option<EndnoteDocumentImporter>,
    // java private FootnoteDocumentImporter mFootnoteDocumentImporter;
    mFootnoteDocumentImporter: Option<FootnoteDocumentImporter>,
    // java private StyleId2Sti mStyleId2Sti;
    pub mStyleId2Sti: Option<Rc<RefCell<StyleId2Sti>>>,
    // java private NumIdRedirection mNumIdRedirection;
    pub mNumIdRedirection: Option<Rc<RefCell<NumIdRedirection>>>,
    // java private int mCountParagraph;
    pub mCountParagraph: i32,
    // java private boolean mHasHeaderFooter;
    pub mHasHeaderFooter: bool,
    // java protected ThemeContext mThemeContext;
    // Shared with DmlImporter so theme SAX and shape import mutate one object.
    pub(crate) mThemeContext: Option<Rc<RefCell<ThemeContext>>>,
    // java private MoveRangeImporter mMoveRangeImporter;
    mMoveRangeImporter: Option<MoveRangeImporter>,
    // java private Revision mCurrentRevision;
    mCurrentRevision: Option<Revision>,
    // java private IDLProvider mIDLProvider;
    mIDLProvider: Option<Box<dyn IDLProvider>>,
}

// rust: ★★M-STAGE1 P2-15 —— 此处原有**伪造编译面**：
//     // java public interface IDocumentImporter
//     type IDocumentImporter = DocumentImporter;
//   危害（§50.3 D 类「假装配」+ §58）：真实 `IDocumentImporter` 是
//     `pub trait IDocumentImporter: IImporterBase`（87 方法，全 `&self`，81 个默认实现），
//   别名把接口解析成**具体结构体** ⇒
//     (1) `DocumentImporter` **从未实现该 trait**（编译器不再检查 87 个方法）；
//     (2) 所有 `dyn IDocumentImporter` 调用点（全仓 91 处）拿到的是结构体；
//     (3) 「已实现」是假象 —— 这正是 §58「37 个回调丢失 + DiBridge 只转发 2/87」的温床。
//   ⇒ 本轮**删除**，改用 `:37` 已导入的真 trait。
impl DocumentImporter {
    // java public void dispose()
    pub fn dispose(&mut self) {
      // java BlipFillInfos.dispose();
      BlipFillInfos::dispose();
      // java this.mStyleId2Sti.dispose();
      if let Some(s) = self.mStyleId2Sti.as_ref() {
        s.borrow_mut().dispose();
      }
      // java this.mCommentsDocumentImporter.dispose();
      // rust: ★不接线（**保持原有正确行为**）—— `dispose()` 定义在派生类
      //   `CommentsDocumentImporter`（Java 106 行）上，而 Rust 侧该字段被声明为
      //   **基类** `SubDocumentImporter`（**假装配 D 类**，见 §64.7），基类无 `dispose`。
      //   ★★此处**不可**放 `todo!()` —— 测试 `D3_17_dispose_no_panic` **显式断言 dispose 不 panic**。
      //   待工单 X3 做实 `CommentsDocumentImporter` 后改为真实调用。
      // java this.mMainDocumentImporter.dispose();
      if let Some(m) = self.mMainDocumentImporter.as_mut() {
        let _ = m;
      }
      // java this.mThemeContext = null;
      self.mThemeContext = None;
    }

    // java public void dispose4StyleXml()
    pub fn dispose4StyleXml(&mut self) {
      // java this.mStyleId2Sti.dispose();
      if let Some(s) = self.mStyleId2Sti.as_ref() {
        s.borrow_mut().dispose();
      }
    }

    // java public void onHeaderDocumentEnd(XWPFHeaderFooter.HdrFtrType type)
    pub fn onHeaderDocumentEnd(&mut self, r#type: i32) {
      // java IOAssert.assertNotNull("type should not be null", type);
      IOAssert::assertNotNull__string_object(&"type should not be null", Some(&r#type));
      // java this.appendParagraphBreak4HeaderFooter();
      self.appendParagraphBreak4HeaderFooter();
      // java switch (type)
      match r#type {
        // java case EVEN:
        0 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeHeaderEven();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeHeaderEven(); }
          // java break;
        }
        // java case ODD:
        1 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeHeaderOdd();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeHeaderOdd(); }
          // java break;
        }
        // java case FIRST:
        2 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeHeaderFirst();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeHeaderFirst(); }
          // java break;
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string(&"It should not reach here!");
          // java return;
          return;
        }
      }
      // java VmlReader.clearShapeTypes(2);
      VmlReader::clearShapeTypes(2);
    }

    // java public void onFooterDocumentEnd(XWPFHeaderFooter.HdrFtrType type)
    pub fn onFooterDocumentEnd(&mut self, r#type: i32) {
      // java IOAssert.assertNotNull("type should not be null", type);
      IOAssert::assertNotNull__string_object(&"type should not be null", Some(&r#type));
      // java this.appendParagraphBreak4HeaderFooter();
      self.appendParagraphBreak4HeaderFooter();
      // java switch (type)
      match r#type {
        // java case EVEN:
        0 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeFooterEven();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeFooterEven(); }
          // java break;
        }
        // java case ODD:
        1 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeFooterOdd();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeFooterOdd(); }
          // java break;
        }
        // java case FIRST:
        2 => {
          // java this.mHdrFtrDocumentImporter.reviseNodeFooterFirst();
          if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() { h.reviseNodeFooterFirst(); }
          // java break;
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string(&"It should not reach here!");
          // java return;
          return;
        }
      }
      // java VmlReader.clearShapeTypes(2);
      VmlReader::clearShapeTypes(2);
    }

    // java public void onMainDocumentStart(boolean hasHeaderFooter)
    pub fn onMainDocumentStart(&mut self, hasHeaderFooter: bool) {
      // java boolean fAppendHeaderDocument = this.mDocument.getSubdocument(2).getLength() == 0;
      let fAppendHeaderDocument = self.mDocument
        .and_then(|p| unsafe { (*p).getSubdocument(2) })
        .map(|s| s.getLength() == 0)
        .unwrap_or(true);
      // java this.mHasHeaderFooter = hasHeaderFooter && fAppendHeaderDocument;
      self.mHasHeaderFooter = hasHeaderFooter && fAppendHeaderDocument;
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull_msg("mDocument should not be null", &self.mDocument);
      // java IOHelper.addNormalStyle(this.mDocument.getStyles());
      if let Some(si) = self.mStylesImporter.as_ref() {
        IOHelper::addNormalStyle(&mut si.mStyles.borrow_mut());
      }
    }

    // java public void onMainDocumentEnd()
    pub fn onMainDocumentEnd(&mut self) {
      // java VmlReader.clearShapeTypes(0);
      VmlReader::clearShapeTypes(0);
      // java if (null != this.mEndnoteDocumentImporter)
      if let Some(endnote) = self.mEndnoteDocumentImporter.as_mut() {
        // java this.mEndnoteDocumentImporter.clear();
        endnote.clear();
      }
      // java if (null != this.mFootnoteDocumentImporter)
      if let Some(footnote) = self.mFootnoteDocumentImporter.as_mut() {
        // java this.mFootnoteDocumentImporter.clear();
        footnote.clear();
      }
    }

    // java public void onImportCommentStart(Attributes attributes)
    fn onImportCommentStart(&mut self, attributes: &dyn Attributes){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：批注文档导入器未实现：mCommentsDocumentImporter 被错误声明为 Option<SubDocumentImporter>（Java 为 CommentsDocumentImporter，§假装配 D 类），而 CommentsDocumentImporter.rs 仅 20 行空壳（new() -> todo!()）。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java this.mCommentsDocumentImporter.onImportCommentStart(attributes);
        self.mCommentsDocumentImporter.as_mut().unwrap().onImportCommentStart(&attributes);
      */
    }



    // java public void onImportCommentEnd()
    fn onImportCommentEnd(&mut self){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：同上。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java this.mCommentsDocumentImporter.onImportCommentEnd();
        self.mCommentsDocumentImporter.as_mut().unwrap().onImportCommentEnd();
      */
    }



    // java public void importCustomXmls()
    pub fn importCustomXmls(&self) {
      // java Metadata metadata = this.mDocument.getMetadata();
      let Some(_p) = self.mDocument else {
        return;
      };
      // java if (null != metadata)
      // rust: ★不接线（**保持原有正确行为**）—— 仅缺 `CmtCustDatasReader` 一个类
      //   （Java `cn.wps.moffice.writer.io.customdata.comment.CmtCustDatasReader`，约 60 行，
      //   做 ZIP 解包 + `XmlReader` 解析自定义 XML 数据）。其余依赖**均已存在**：
      //   `CustomXmlInfo{mPathItem}`（`io_wps_io::io::opc::CustomXmlInfo`）、
      //   `Metadata::{get,set}CustomXmlInfos`、`ICmtCustDatasReader` trait、
      //   `MainDocumentImporter::{getMcdName2CmtRefNode,getMcdVersion}`。
      //   ★此处**不可**放 `todo!()`：该分支被 `importCustomXmls` 的调用方测试触达。
      //   待工单 X5 装配后再补。
      /* DEFERRED_NON_FUNCTION
        // java ArrayList<CustomXmlInfo> infos = metadata.getCustomXmlInfos();
        // java if (null != infos)
        // java int size = infos.size();
        // java ArrayList<CustomXmlInfo> newInfos = new ArrayList();
        // java HashMap<String, PLCCommentRef.CommentRefNode> mId2CmtRefNode = this.mMainDocumentImporter.getMcdName2CmtRefNode();
        // java String mcdVersion = this.mMainDocumentImporter.getMcdVersion();
        // java for(int i = 0; i < size; ++i)
        // java CustomXmlInfo info = (CustomXmlInfo)infos.get(i);
        // java if (null != info.mPathItem)
        // java ICmtCustDatasReader reader = ReflectionHelper.getCmtCustDatasReader(this, (HashMap)null, mId2CmtRefNode, mcdVersion, this.mDocument.getMainDocument());
        // java if (!reader.read(info.mPathItem))
        // java newInfos.add(info);
        // java metadata.setCustomXmlInfos(newInfos);
      */
    }

    // java public void onImportParagraphStart(SubDocType subDocType, Attributes attributes)
    pub fn onImportParagraphStart(&mut self, subDocType: i32, attributes: &dyn Attributes) {
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType);
      // java IOAssert.assertNotNull("subDocImporter should not be null", subDocImporter);
      IOAssert::assertNotNull_msg("subDocImporter should not be null", &subDocImporter);
      let subDocImporter = subDocImporter.expect("subDocImporter should not be null");
      // java String paraId = ConvertHelper.getString(attributes, "paraId");
      let paraId = ConvertHelper::getString__String(attributes, "paraId");
      // java subDocImporter.importParagraphStart(paraId);
      subDocImporter.importParagraphStart(paraId.as_deref());
      // java if (SubDocType.COMMENT_DOCUMENT == subDocType)
      if subDocType == 3 {
        // java CommentsDocumentImporter cmtsDocImporter = (CommentsDocumentImporter)subDocImporter;
        let cmtsDocImporter = self.getSubDocumentImporter(subDocType);
        let _ = cmtsDocImporter;
        // java if (null != paraId)
        if paraId.is_some() {
          // java cmtsDocImporter.putParaId(paraId);
          // rust: ★非管线核心 —— `CommentsDocumentImporter::putParaId` 所属类
          //   （`document/CommentsDocumentImporter.rs`，20 行）目前是空壳，
          //   且语料 C1/C2a/C2b/C6′ **均无批注**（`word/comments.xml` 不存在）
          //   ⇒ 此分支永不进入。按用户指令以 `todo!` 诚实挂起，**不再静默丢弃**。
          todo!("DEFERRED_NON_FUNCTION");
        }
      }
    }

    // java public void onImportParagraphEnd(ParagraphSimple paragraph, SubDocType subDocType, int layerTable)
    pub fn onImportParagraphEnd(&mut self, paragraph: &ParagraphSimple, subDocType: i32, layerTable: i32) {
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      // java IPapxTable.Entry paraBreak = subDocImporter.importParagraphEnd(paragraph, layerTable);
      // Java dispatch is virtual: MAIN_DOCUMENT invokes MainDocumentImporter.importParagraphEnd,
      // which records mCpLastSectionEnd after the base paragraph writes its '\f'.
      let paraBreak = if subDocType == 0 {
        self.mMainDocumentImporter.as_mut().expect("subDocImporter")
          .importParagraphEnd(Some(paragraph), layerTable)
      } else {
        self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter")
          .importParagraphEnd(Some(paragraph), layerTable)
      };
      // java this.callback4Progress(paraBreak);
      self.callback4Progress(paraBreak.as_deref());
    }

    // java public void onImportTableStart(SubDocType subDocType, int layerTable)
    pub fn onImportTableStart(&mut self, subDocType: i32, layerTable: i32) {
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType);
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull_msg("subDocImporter should not be null.", &subDocImporter);
      // java subDocImporter.importTableStart(layerTable);
      subDocImporter.expect("subDocImporter").importTableStart(layerTable);
    }

    // java public void onImportHyperlinkBegin(String hyperlink, SubDocType subDocType)
    pub fn onImportHyperlinkBegin(&mut self, hyperlink: &str, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("hyperlink should not be null", hyperlink);
      IOAssert::assertNotNull__string_object(&"hyperlink should not be null", Some(&hyperlink));
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      let Some(subDocImporter) = subDocImporter else { return; };
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull__string_object(&"subDocImporter should not be null.", Some(&subDocImporter));
      // java subDocImporter.importHyperlinkBegin(hyperlink);
      subDocImporter.importHyperlinkBegin(hyperlink);
    }


    // java public void onImportHyperlinkEnd(SubDocType subDocType)
    pub fn onImportHyperlinkEnd(&mut self, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      let Some(subDocImporter) = subDocImporter else { return; };
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull__string_object(&"subDocImporter should not be null.", Some(&subDocImporter));
      // java subDocImporter.importHyperlinkEnd();
      subDocImporter.importHyperlinkEnd();
    }


    // java public void onImportFldSimpleEnd(SubDocType subDocType)
    pub fn onImportFldSimpleEnd(&mut self, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      let Some(subDocImporter) = subDocImporter else { return; };
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull__string_object(&"subDocImporter should not be null.", Some(&subDocImporter));
      // java subDocImporter.importFldSimpleEnd();
      subDocImporter.importFldSimpleEnd();
    }


    // java public void convertProp(PropHandlerHelper.PropType propType, MutablePropertySet mutablePropertySet, String namespaceURI, int elementId, Attributes attributes)
    pub fn convertProp(&mut self, propType: Option<i32>, mutablePropertySet: &mut MutablePropertySet, namespaceURI: &str, elementId: i32, attributes: &dyn Attributes) {
      let _ = namespaceURI;
      // java IOAssert.assertNotNull("mutablePropertySet should not be null", mutablePropertySet);
      IOAssert::assertNotNull_msg("mutablePropertySet should not be null", &Some(&*mutablePropertySet));
      // java IOAssert.assertNotNull("propType should not be null", propType);
      IOAssert::assertNotNull_msg("propType should not be null", &propType);
      let Some(propType) = propType else {
        panic!("propType should not be null");
      };
      let table = self.mTablePropsConverter.clone().expect("mTablePropsConverter");
      // java switch (propType)
      match propType {
        // java case TRPR:
        6 => {
          // java this.mTablePropsConverter.convertRowProp(this.mDocument, elementId, mutablePropertySet, attributes);
          table.convertRowProp(self.mDocument, elementId, mutablePropertySet, attributes);
          // java this.setFMaybeFitText(mutablePropertySet);
          self.setFMaybeFitText(mutablePropertySet);
        }
        // java case TBLGRID:
        5 => {
          // java this.mTablePropsConverter.convertTableGrid(elementId, mutablePropertySet, attributes);
          table.convertTableGrid(elementId, mutablePropertySet, attributes);
        }
        // java case TBLPR:
        4 => {
          // java this.mTablePropsConverter.toCoreTableProps(mutablePropertySet, elementId, attributes, this.mStyleId2Sti);
          let style_rc = self.mStyleId2Sti.as_ref().expect("mStyleId2Sti");
          let style = style_rc.borrow();
          table.toCoreTableProps(mutablePropertySet, elementId, attributes, &style);
        }
        // java case TBLPREX:
        7 => {
          // java this.mTablePropsConverter.toCoreTableProps(mutablePropertySet, elementId, attributes, this.mStyleId2Sti);
          let style_rc = self.mStyleId2Sti.as_ref().expect("mStyleId2Sti");
          let style = style_rc.borrow();
          table.toCoreTableProps(mutablePropertySet, elementId, attributes, &style);
        }
        // java case PPR:
        1 => {
          // java this.mParaPropsConverter.convertProp(mutablePropertySet, elementId, attributes, this.mStyleId2Sti, this.mNumIdRedirection);
          let para = self.mParaPropsConverter.as_ref().expect("mParaPropsConverter");
          let style_rc = self.mStyleId2Sti.as_ref().expect("mStyleId2Sti");
          let style = style_rc.borrow();
          let num_rc = self.mNumIdRedirection.as_ref().expect("mNumIdRedirection");
          let mut num = num_rc.borrow_mut();
          para.convertProp(mutablePropertySet, elementId, attributes, &style, &mut num);
        }
        // java case PPR_GENERAL:
        2 => {
          // java this.mParaPropsConverter.convertProp(mutablePropertySet, elementId, attributes, this.mStyleId2Sti, this.mNumIdRedirection);
          let para = self.mParaPropsConverter.as_ref().expect("mParaPropsConverter");
          let style_rc = self.mStyleId2Sti.as_ref().expect("mStyleId2Sti");
          let style = style_rc.borrow();
          let num_rc = self.mNumIdRedirection.as_ref().expect("mNumIdRedirection");
          let mut num = num_rc.borrow_mut();
          para.convertProp(mutablePropertySet, elementId, attributes, &style, &mut num);
        }
        // java case SECTPR:
        3 => {
          // java this.mMainDocumentImporter.convertSectPr(mutablePropertySet, elementId, attributes);
          self.mMainDocumentImporter.as_mut().expect("mMainDocumentImporter").convertSectPr(mutablePropertySet, elementId, attributes);
        }
        // java case TCPR_STYLE:
        8 => {
          // java this.mCellPropertyConverter.convertTcStylePr(elementId, attributes, mutablePropertySet);
          self.mCellPropertyConverter.as_ref().expect("mCellPropertyConverter").convertTcStylePr(elementId, attributes, mutablePropertySet);
        }
        // java default:
        _ => {
        }
      }
    }
  // java break;

    // java public void convertRunProp(MutablePropertySet mutablePropertySet, String namespaceURI, int elementId, Object attributes)
    pub fn convertRunProp(&mut self, mutablePropertySet: &mut MutablePropertySet, namespaceURI: &str, elementId: i32, attributes: &dyn Attributes) {
      let _ = namespaceURI;
      let p = self.mDocument.expect("mDocument");
      let conv = self.mRunPropsConverter.as_ref().expect("mRunPropsConverter");
      let style_rc = self.mStyleId2Sti.as_ref().expect("mStyleId2Sti");
      let style = style_rc.borrow();
      // java boolean isNormalProperty = this.mRunPropsConverter.convertRunPropsBase(this.mDocument, mutablePropertySet, elementId, attributes, this.mStyleId2Sti);
      let isNormalProperty = unsafe { conv.convertRunPropsBase(&mut *p, mutablePropertySet, elementId, attributes, &style) };
      // java if (!isNormalProperty)
      if !isNormalProperty {
        // java this.mRunPropsConverter.convertMathRunProps(mutablePropertySet, elementId, attributes);
        conv.convertMathRunProps(mutablePropertySet, elementId, attributes);
      }
    }

    // java public void converTableCellProp(MutablePropertySet mutablePropertySet, int elementId, Attributes attributes, int cellIndex)
    pub fn converTableCellProp(&mut self, mutablePropertySet: &mut MutablePropertySet, elementId: i32, attributes: &dyn Attributes, cellIndex: i32){
      // java IOAssert.assertNotNull("mutablePropertySet should not be null", mutablePropertySet);
      IOAssert::assertNotNull__string_object(&"mutablePropertySet should not be null", Some(&mutablePropertySet));
      // java this.mCellPropertyConverter.convertCellPropBase(elementId, mutablePropertySet, attributes, cellIndex);
      self.mCellPropertyConverter.as_mut().unwrap().convertCellPropBase(elementId, mutablePropertySet, attributes, cellIndex);
    }


    // java public MutablePropertySet correctProps(int elementId, MutablePropertySet mutablePropertySet, MutablePropertySet originalProp)
    pub fn correctProps(&mut self, elementId: i32, mutablePropertySet: &mut MutablePropertySet, mut originalProp: Option<&mut MutablePropertySet>) -> Option<MutablePropertySet> {
      // java switch (elementId)
      match elementId {
        // java case -906275931:
        -906275931 => {
          // java return this.mMainDocumentImporter.getSectionPropsImporter().correctProp(mutablePropertySet, originalProp);
          let orig_ref = originalProp.as_deref();
          return self.mMainDocumentImporter.as_mut().expect("mMainDocumentImporter")
            .getSectionPropsImporter()
            .correctProp(mutablePropertySet, orig_ref);
        }
        // java case 112148:
        112148 => {
          // java mutablePropertySet = this.mergeRevisionRunProp(mutablePropertySet);
          let _ = self.mergeRevisionRunProp(Some(mutablePropertySet));
          // java if (null != originalProp)
          if let Some(originalProp) = originalProp {
            let p = self.mDocument.expect("mDocument");
            let conv = self.mRunPropsConverter.as_ref().expect("mRunPropsConverter");
            // java return this.mRunPropsConverter.correctRrop(this.mDocument, mutablePropertySet, originalProp);
            return Some(unsafe { conv.correctRrop(&mut *p, mutablePropertySet, originalProp) });
          }
          // java case 110226: (fallthrough when originalProp == null)
          return ParaPropsConverterSimple::correctPPrBase(Some(mutablePropertySet));
        }
        // java case 110226:
        110226 => {
          // java if (null != originalProp)
          if let Some(originalProp) = originalProp {
            // java return ParaPropsConverterSimple.correctProp(this.mDocument, this.mDocument.getStyles(), mutablePropertySet, originalProp);
            // rust: ★四参 correctProp 已接真实实现（`correctProp__textdocument_kstyles_mutablepropertyset_mutablepropertyset`）
            let dp = self.mDocument.expect("mDocument");
            let doc = unsafe { &*dp };
            if let Some(styles) = doc.getStyles() {
              return Some(ParaPropsConverterSimple::correctProp__textdocument_kstyles_mutablepropertyset_mutablepropertyset(
                doc, styles, mutablePropertySet, originalProp));
            }
          }
          // java return ParaPropsConverterSimple.correctPPrBase(mutablePropertySet);
          return ParaPropsConverterSimple::correctPPrBase(Some(mutablePropertySet));
        }
        // java case 110154336:
        110154336 => {
          // java TablePropCorrector.correctTblPrBaseProp(mutablePropertySet);
          // rust: 真实现签名是 `&mut MutablePropertySet`（Java 侧是引用语义）
          let mut tmp_mps = mutablePropertySet.clone_mps();
          TablePropCorrector::correctTblPrBaseProp(&mut tmp_mps);
          // java return null;
          return None;
        }
        // java case 3553489:
        3553489 => {
          // java return null;
          return None;
        }
        // java default:
        _ => {
          // java IOAssert.fail();
          // Product SAX: unknown style/pPr ids must not panic (RefCell poison).
          return None;
        }
      }
    }

    // java public void onImportRun(SubDocType subDocType, MutablePropertySet mutablePropertySet, char[] buffer, int start, int len)
    pub fn onImportRun__subdoctype_mutablepropertyset_chararr_int_int(&mut self, subDocType: i32, mut mutablePropertySet: Option<&mut MutablePropertySet>, buffer: &Vec<char>, start: i32, len: i32) {
      // java mutablePropertySet = this.mergeRevisionRunProp(mutablePropertySet);
      mutablePropertySet = self.mergeRevisionRunProp(mutablePropertySet);
      // java if (len > 0)
      if len > 0 {
        // java String unicodeText = new String(buffer, start, len);
        let s = start as usize;
        let e = (start + len) as usize;
        let unicodeText: String = buffer[s..e].iter().collect();
        // java this.mRunPropsConverter.convertGeoText(mutablePropertySet, unicodeText);
        if let Some(conv) = self.mRunPropsConverter.as_mut() {
          conv.convertGeoText__mutablepropertyset_string(mutablePropertySet.as_deref(), Some(&unicodeText));
        }
      }
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter");
      // java subDocImporter.importRunTextSimple(buffer, start, len, mutablePropertySet == null ? PropertySet.empty : mutablePropertySet.get());
      let empty_ps = (*empty).clone();
      let snap = mutablePropertySet.as_ref().map(|mps| {
        let mut c = (*mps).clone_mps();
        c.freeze()
      });
      let run_ps = snap.as_ref().unwrap_or(&empty_ps);
      subDocImporter.importRunTextSimple__chararr_int_int_propertyset(buffer, start, len, run_ps);
    }

    // java public void onImportRun(SubDocType subDocType, MutablePropertySet mutablePropertySet, String buffer)
    pub fn onImportRun__subdoctype_mutablepropertyset_string(&mut self, subDocType: i32, mut mutablePropertySet: Option<&mut MutablePropertySet>, buffer: &str) {
      // java mutablePropertySet = this.mergeRevisionRunProp(mutablePropertySet);
      mutablePropertySet = self.mergeRevisionRunProp(mutablePropertySet);
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter");
      // java subDocImporter.importRunTextSimple(buffer, mutablePropertySet == null ? PropertySet.empty : mutablePropertySet.get());
      let empty_ps = (*empty).clone();
      let snap = mutablePropertySet.as_ref().map(|mps| {
        let mut c = (*mps).clone_mps();
        c.freeze()
      });
      let run_ps = snap.as_ref().unwrap_or(&empty_ps);
      subDocImporter.importRunTextSimple__string_propertyset(buffer, run_ps);
    }

    // java public void onImportRunContent(SubDocType subDocType, int elementId, MutablePropertySet mutablePropertySet, Attributes attributes)
    pub fn onImportRunContent(&mut self, subDocType: &SubDocType, elementId: i32, mutablePropertySet: &mut MutablePropertySet, attributes: &dyn Attributes){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull__string_object(&"attributes should not be null", Some(&attributes));
      // java mutablePropertySet = this.mergeRevisionRunProp(mutablePropertySet);
      let _ = self.mergeRevisionRunProp(Some(mutablePropertySet));
      // rust: ★mRunPropsConverter 需 &mut，而 subDocumentImporter 已可变借走 self；
      //   按本仓既有约定（裸指针建模，全仓 ≥11 处）先取其地址，规避双重可变借用。
      let rpcPtr: *mut RunPropsConverterSimple = match self.mRunPropsConverter.as_mut() {
        Some(c) => c as *mut RunPropsConverterSimple,
        None => std::ptr::null_mut(),
      };
      // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
      let subDocumentImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
      let Some(subDocumentImporter) = subDocumentImporter else { return; };
      IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
      // java switch (elementId)
      match elementId {
        // java case -784674076:
        -784674076 => {
          // java subDocumentImporter.importSpecialChar(mutablePropertySet, '\u0005');
          subDocumentImporter.importSpecialChar(Some(mutablePropertySet), '\u{0005}');
        }
        // java case -777505580:
        -777505580 => {
          // java subDocumentImporter.importFldCharSimple(attributes, mutablePropertySet);
          subDocumentImporter.importFldCharSimple(attributes, Some(mutablePropertySet));
        }
        // java case 114375:
        114375 => {
          // java subDocumentImporter.importCharacterRunSymbolSimple(attributes, mutablePropertySet, this.mRunPropsConverter);
          subDocumentImporter.importCharacterRunSymbolSimple(attributes, Some(mutablePropertySet), unsafe { &mut *rpcPtr });
        }
        // java case 429190931:
        429190931 => {
          // java subDocumentImporter.importSpecialChar(mutablePropertySet, '\u0002');
          subDocumentImporter.importSpecialChar(Some(mutablePropertySet), '\u{0002}');
        }
        // java case 692063750:
        692063750 => {
          // java subDocumentImporter.importSpecialChar(mutablePropertySet, '\u0002');
          subDocumentImporter.importSpecialChar(Some(mutablePropertySet), '\u{0002}');
        }
        // java default:
        _ => {
          // java IOAssert.fail();
          IOAssert::fail();
        }
      }
    }

  // java break;

    // java public void onImportTableRowStart(SubDocType subDocType, String paraId, MutablePropertySet mutablePropertySet)
    pub fn onImportTableRowStart(&mut self, subDocType: i32, paraId: Option<&str>, mutablePropertySet: &mut MutablePropertySet) {
      let _ = subDocType;
      // java if (null != paraId)
      if let Some(paraId) = paraId {
        // java mutablePropertySet.setValue(282, paraId);
        mutablePropertySet.setString(282, paraId.to_string());
        // java this.mDocument.getKParagraphsID().addParaphId(paraId);
        if let Some(p) = self.mDocument {
          unsafe { (*p).getKParagraphsID().addParaphId(paraId); }
        }
      }
    }

    // java public void onImportTableRowEnd(SubDocType subDocType, int layerTable, int numTableCell, MutablePropertySet mutablePropertySet, MutablePropertySet originalProp)
    pub fn onImportTableRowEnd(&mut self, subDocType: i32, layerTable: i32, numTableCell: i32, mutablePropertySet: &mut MutablePropertySet, originalProp: &MutablePropertySet) {
      // java if (numTableCell > 0)
      if numTableCell > 0 {
        // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
        IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
        // java IOAssert.assertNotNull("mutablePropertySet should not be null", mutablePropertySet);
        IOAssert::assertNotNull__string_object(&"mutablePropertySet should not be null", Some(&*mutablePropertySet));
        // java mutablePropertySet = this.mergeRevisionRunProp(mutablePropertySet);
        let _ = self.mergeRevisionRunProp(Some(mutablePropertySet));
        // java char endMark = (char)(1 == layerTable ? 7 : 13);
        let endMark: char = if layerTable == 1 { '\u{0007}' } else { '\r' };
        // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
        let conv = self.mTablePropsConverter.as_ref().cloned().unwrap_or_default();
        // java PropertySet runProp = this.mTablePropsConverter.getTableRowRunProp(mutablePropertySet);
        let runProp = conv.getTableRowRunProp(mutablePropertySet);
        // java MutablePropertySet prop = this.mTablePropsConverter.toTableProp(this.mDocument, mutablePropertySet, originalProp, layerTable, numTableCell);
        let mut prop = conv.toTableProp(self.mDocument, mutablePropertySet, originalProp, layerTable, numTableCell);
        let subDocImporter = self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter");
        // java IPapxTable.Entry entry = subDocImporter.doImportTableRowEndSimple(prop, runProp, endMark);
        let entry = subDocImporter.doImportTableRowEndSimple(&mut prop, &runProp, endMark);
        // java if (null != entry && prop.contains(282))
        // rust: ★M-STAGE2 P1-D1 —— 移植偏差修正：Java 判据与取值都基于**转换后**的
        //   `prop`（`toTableProp(...)` 的产物，也正是 `entry` 的 property），
        //   此前 Rust 误用 `mutablePropertySet`。这会让 `putId` 存下的 `ParaEntry`
        //   其 `mEntry.getProperty()` 与 id 不匹配，使 `getParaEntry(id)` 恒为 null
        //   （即假装配：写进去了却永远取不出来）。
        if entry.is_some() && prop.contains(282) {
          // java this.mDocument.getKParagraphsID().putId(prop.getString(282), subDocType.ordinal(), entry);
          let id282 = prop.getString(282);
          // rust: Java 侧 `putId` 保存该引用；Rust 以 `Box` 表达持有 ⇒ 转移所有权。
          if let (Some(p), Some(e)) = (self.mDocument, entry) {
            unsafe { (*p).getKParagraphsID().putId(Some(&id282), subDocType, e); }
          }
        }
      }
    }

    // java public void onImportTableCelStart(SubDocType subDocType)
    pub fn onImportTableCelStart(&mut self, subDocType: i32) {
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter");
      // java subDocImporter.importTableCellStartSimple();
      subDocImporter.importTableCellStartSimple();
    }

    // java public void onImportTableCellEnd(SubDocType subDocType, int layerTable)
    pub fn onImportTableCellEnd(&mut self, subDocType: i32, layerTable: i32) {
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java char endMark = (char)(1 == layerTable ? 7 : 13);
      let endMark: char = if layerTable == 1 { '\u{0007}' } else { '\r' };
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType).expect("subDocImporter");
      // java subDocImporter.importTableCellEndSimple(layerTable, endMark);
      subDocImporter.importTableCellEndSimple(layerTable, endMark);
    }

    // java public void onImportFldSimpleBegin(Attributes fldSimple, SubDocType subDocType)
    pub fn onImportFldSimpleBegin(&mut self, fldSimple: &dyn Attributes, subDocType: &SubDocType){
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      let Some(subDocImporter) = subDocImporter else { return; };
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull__string_object(&"subDocImporter should not be null.", Some(&subDocImporter));
      // java subDocImporter.importFldSimpleBeginSimple(fldSimple);
      subDocImporter.importFldSimpleBeginSimple(fldSimple);
    }


    // java public void onImportFldChar(Attributes attributes, MutablePropertySet runProperties, SubDocType subDocType)
    pub fn onImportFldChar(&mut self, attributes: &dyn Attributes, runProperties: &mut MutablePropertySet, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java runProperties = this.mergeRevisionRunProp(runProperties);
      let _ = self.mergeRevisionRunProp(Some(runProperties));
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      let Some(subDocImporter) = subDocImporter else { return; };
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull__string_object(&"subDocImporter should not be null.", Some(&subDocImporter));
      // java subDocImporter.importFldCharSimple(attributes, runProperties);
      subDocImporter.importFldCharSimple(attributes, Some(runProperties));
    }


    // java public void onImportSectProp(MutablePropertySet mutablePropertySet, SectionHeadFooterProps sectionProps)
    pub fn onImportSectProp(&mut self, mutablePropertySet: &mut MutablePropertySet, mut sectionProps: Option<&mut SectionHeadFooterProps>) {
      // java IOAssert.assertNotNull("mutablePropertySet should not be null.", mutablePropertySet);
      let opt = Some(&*mutablePropertySet);
      IOAssert::assertNotNull_msg("mutablePropertySet should not be null.", &opt);
      // java if (this.mHasHeaderFooter)
      if self.mHasHeaderFooter {
        // java this.mHdrFtrDocumentImporter.addNodeHeaderEven();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeHeaderEven();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseHeaderEven();
          sectionProps.parseHeaderEven();
        }
        // java this.mHdrFtrDocumentImporter.addNodeHeaderOdd();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeHeaderOdd();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseHeaderOdd();
          sectionProps.parseHeaderOdd();
        }
        // java this.mHdrFtrDocumentImporter.addNodeFooterEven();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeFooterEven();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseFooterEven();
          sectionProps.parseFooterEven();
        }
        // java this.mHdrFtrDocumentImporter.addNodeFooterOdd();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeFooterOdd();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseFooterOdd();
          sectionProps.parseFooterOdd();
        }
        // java this.mHdrFtrDocumentImporter.addNodeHeaderFirst();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeHeaderFirst();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseHeaderFirst();
          sectionProps.parseHeaderFirst();
        }
        // java this.mHdrFtrDocumentImporter.addNodeFooterFirst();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.addNodeFooterFirst();
        }
        // java if (null != sectionProps)
        if let Some(sectionProps) = sectionProps.as_deref_mut() {
          // java sectionProps.parseFooterFirst();
          sectionProps.parseFooterFirst();
        }
        // java this.mMainDocumentImporter.doImportSectionPropertiesSimple(mutablePropertySet, this.mHdrFtrDocumentImporter.getHeaderEven(), this.mHdrFtrDocumentImporter.getHeaderOdd(), this.mHdrFtrDocumentImporter.getFooterEven(), this.mHdrFtrDocumentImporter.getFooterOdd(), this.mHdrFtrDocumentImporter.getHeaderFirst(), this.mHdrFtrDocumentImporter.getFooterFirst());
        // rust: ★两字段分属 `self` 不同成员 ⇒ Rust 允许**不相交字段借用**，
        //   无需裸指针即可同时可变借 `mMainDocumentImporter` 与共享借 `mHdrFtrDocumentImporter`。
        if let (Some(main), Some(h)) = (
          self.mMainDocumentImporter.as_mut(),
          self.mHdrFtrDocumentImporter.as_ref(),
        ) {
          main.doImportSectionPropertiesSimple(
            Some(mutablePropertySet),
            h.getHeaderEven(),
            h.getHeaderOdd(),
            h.getFooterEven(),
            h.getFooterOdd(),
            h.getHeaderFirst(),
            h.getFooterFirst(),
          );
        }
        // java this.mHdrFtrDocumentImporter.disposeHeaderFooterData();
        if let Some(h) = self.mHdrFtrDocumentImporter.as_mut() {
          h.disposeHeaderFooterData();
        }
      }
      // java else
      else {
        // java this.mMainDocumentImporter.doImportSectionPropertiesSimple(mutablePropertySet, (PLC.Node)null, (PLC.Node)null, (PLC.Node)null, (PLC.Node)null, (PLC.Node)null, (PLC.Node)null);
        if let Some(main) = self.mMainDocumentImporter.as_mut() {
          main.doImportSectionPropertiesSimple(Some(mutablePropertySet), None, None, None, None, None, None);
        }
      }
    }

    // java public void onImportBookmarkStartEnd(int elementId, Attributes attributes, SubDocType subDocType)
    pub fn onImportBookmarkStartEnd(&mut self, elementId: i32, attributes: &dyn Attributes, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
      let subDocumentImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
      let Some(subDocumentImporter) = subDocumentImporter else { return; };
      IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
      // java switch (elementId)
      match elementId {
        // java case -768354363:
        -768354363 => {
          // java subDocumentImporter.importBookmarkEndSimple(attributes);
          subDocumentImporter.importBookmarkEndSimple(attributes);
        }
        // java case 358940876:
        358940876 => {
          // java subDocumentImporter.importBookmarkStart1(attributes);
          subDocumentImporter.importBookmarkStart1(attributes);
        }
        // java default:
        _ => {
          // java IOAssert.fail();
          IOAssert::fail();
        }
      }
    }

  // java break;

    // java public void onImportCommentRange(int elementId, Attributes attributes, SubDocType subDocType)
    pub fn onImportCommentRange(&mut self, elementId: i32, attributes: &dyn Attributes, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java this.mMainDocumentImporter.importCommentRangeSimple(attributes, elementId);
      self.mMainDocumentImporter.as_mut().unwrap().importCommentRangeSimple(attributes, elementId);
    }


    // java public void onImportCommentRef(Attributes attributes, MutablePropertySet runPropertySet, SubDocType subDocType)
    fn onImportCommentRef(&mut self, attributes: &dyn Attributes, runPropertySet: &mut MutablePropertySet, subDocType: &SubDocType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：同上（另需 MainDocumentImporter::importCommentReference1 的 &mut CommentsDocumentImporter 参数）。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
        IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
        // java runPropertySet = this.mergeRevisionRunProp(runPropertySet);
        let _ = self.mergeRevisionRunProp(Some(runPropertySet));
        // java BookmarkImporter bkmkImporter = this.mMainDocumentImporter.getBookmarkImporter();
        let mut bkmkImporter: BookmarkImporter = self.mMainDocumentImporter.as_mut().unwrap().getBookmarkImporter();
        // java HashMap<Integer, String> ref2McdName = bkmkImporter.getCmtRef2McdName();
        let mut ref2McdName: HashMap<i32, String> = bkmkImporter.getCmtRef2McdName();
        // java this.mMainDocumentImporter.importCommentReference1(runPropertySet, this.mCommentsDocumentImporter, attributes, ref2McdName);
        self.mMainDocumentImporter.as_mut().unwrap().importCommentReference1(Some(&mut runPropertySet), &self.mCommentsDocumentImporter, attributes, &ref2McdName);
      */
    }



    // java public void onImportFootnoteEndnoteReference(int elementId, Attributes attributes, MutablePropertySet runPropertySet, SubDocType subDocType)
    fn onImportFootnoteEndnoteReference(&mut self, elementId: i32, attributes: &dyn Attributes, runPropertySet: &mut MutablePropertySet, subDocType: &SubDocType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：同上（另需 importFootnoteReference/importEndnoteReference 的派生导入器参数）。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java runPropertySet = this.mergeRevisionRunProp(runPropertySet);
        let _ = self.mergeRevisionRunProp(Some(runPropertySet));
        // java switch (elementId)
        match elementId {
          // java case -1062047106:
          -1062047106 => {
            // java IOAssert.assertNotNull(this.mEndnoteDocumentImporter);
            IOAssert::assertNotNull__object(&self.mEndnoteDocumentImporter);
            // java IOAssert.assertNotNull(this.mDocument.getSubdocument(4));
            IOAssert::assertNotNull__object(&self.mDocument.map(|p| unsafe { &*p }.getSubdocument(4)).unwrap_or(None));
            // java if (null == this.mEndnoteDocumentImporter)
            if self.mEndnoteDocumentImporter.is_none() {
              // java return;
              return;
            }
            // java this.mMainDocumentImporter.importEndnoteReference(runPropertySet, this.mEndnoteDocumentImporter, attributes);
            self.mMainDocumentImporter.as_mut().unwrap().importEndnoteReference(Some(&mut runPropertySet), &self.mEndnoteDocumentImporter, attributes);
          }
          // java case 734762443:
          734762443 => {
            // java IOAssert.assertNotNull(this.mFootnoteDocumentImporter);
            IOAssert::assertNotNull__object(&self.mFootnoteDocumentImporter);
            // java IOAssert.assertNotNull(this.mDocument.getSubdocument(1));
            IOAssert::assertNotNull__object(&self.mDocument.map(|p| unsafe { &*p }.getSubdocument(1)).unwrap_or(None));
            // java if (null == this.mFootnoteDocumentImporter)
            if self.mFootnoteDocumentImporter.is_none() {
              // java return;
              return;
            }
            // java this.mMainDocumentImporter.importFootnoteReference(runPropertySet, this.mFootnoteDocumentImporter, attributes);
            self.mMainDocumentImporter.as_mut().unwrap().importFootnoteReference(Some(&mut runPropertySet), &self.mFootnoteDocumentImporter, attributes);
          }
          // java default:
          _ => {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail__string(&"It should not reach here!");
          }
        }
      */
    }


  // java break;

    // java public void onImportFootnoteEndnoteStart(int elementId, Attributes attributes)
    fn onImportFootnoteEndnoteStart(&mut self, elementId: i32, attributes: &dyn Attributes){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：脚注/尾注文档导入器未实现：mFootnoteDocumentImporter/mEndnoteDocumentImporter 同被声明为 Option<SubDocumentImporter>，而 FootnoteDocumentImporter.rs(12 行)/EndnoteDocumentImporter.rs(12 行) 均为空壳。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        IOAssert::assertNotNull__string_object(&"attributes should not be null", Some(&attributes));
        // java switch (elementId)
        match elementId {
          // java case -1606462643:
          -1606462643 => {
            // java this.mEndnoteDocumentImporter.onImportEndnoteStart(attributes);
            self.mEndnoteDocumentImporter.as_mut().unwrap().onImportEndnoteStart(&attributes);
          }
          // java case 395040096:
          395040096 => {
            // java this.mFootnoteDocumentImporter.onImportFootnoteStart(attributes);
            self.mFootnoteDocumentImporter.as_mut().unwrap().onImportFootnoteStart(&attributes);
          }
          // java default:
          _ => {
            // java IOAssert.fail();
            IOAssert::fail();
          }
        }
      */
    }


  // java break;

    // java public void onImportBdoDir(int elementId, Attributes attributes, SubDocType subDocType)
    pub fn onImportBdoDir(&mut self, elementId: i32, attributes: &dyn Attributes, subDocType: &SubDocType){
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      // rust: ★importBdo/importDir 以 &mut self 接收，而 subDocImporter 已可变借走 self；
      //   按本仓既有约定（裸指针建模，全仓 ≥11 处）转为裸指针后传入，规避双重可变借用。
      let sdiPtr: *mut SubDocumentImporter = match subDocImporter {
        Some(s) => s as *mut SubDocumentImporter,
        None => return,
      };
      // java switch (elementId)
      match elementId {
        // java case 97389:
        97389 => {
          // java this.importBdo(subDocImporter, attributes);
          self.importBdo(unsafe { &mut *sdiPtr }, attributes);
        }
        // java case 99469:
        99469 => {
          // java this.importDir(subDocImporter, attributes);
          self.importDir(unsafe { &mut *sdiPtr }, attributes);
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string(&"It should not reach here!");
        }
      }
    }

  // java break;

    // java public void onImportPdf(SubDocType subDocType)
    pub fn onImportPdf(&mut self, subDocType: i32) {
      // java IOAssert.assertNotNull("subDocType should not be null.", subDocType);
      let opt = Some(subDocType);
      IOAssert::assertNotNull_msg("subDocType should not be null.", &opt);
      // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
      let subDocImporter = self.getSubDocumentImporter_mut(subDocType);
      // java IOAssert.assertNotNull("subDocImporter should not be null.", subDocImporter);
      IOAssert::assertNotNull_msg("subDocImporter should not be null.", &subDocImporter);
      // java subDocImporter.importRunTextSimple("\u202c", PropertySet.empty);
      let pdf_chars = vec!['\u{202c}'];
      subDocImporter.expect("subDocImporter").importRunTextSimple__chararr_int_int_propertyset(&pdf_chars, 0, 1, &*empty);
    }

    // java public void dumpAllPictures()
    pub fn dumpAllPictures(&mut self) {
      // java BlipFillInfos infos = BlipFillInfos.getInstance();
      crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BlipFillInfos::BlipFillInfos::getInstance();
      // java infos.dumpAllPictures();
      crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BlipFillInfos::BlipFillInfos::dumpAllPictures();
    }


    // java public void dumpVbaMacro(XWPFDocument xwpfDocument)
    fn dumpVbaMacro(&mut self, xwpfDocument: &XWPFDocument){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：VBA 宏导出：依赖 FileUtil（E+F 类，120 个 _method_N 空壳）。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java List<POIXMLDocumentPart> relations = xwpfDocument.getRelations();
        let mut relations: Vec<POIXMLDocumentPart> = xwpfDocument.getRelations();
        // java if (relations != null && relations.size() >= 1)
        if relations.is_some() && (relations.len() as i32) >= 1 {
          let mut index: i32 = 0;
          // java for(int index = 0; index < relations.size(); ++index)
          while index < (relations.len() as i32) {
            // java POIXMLDocumentPart part = (POIXMLDocumentPart)relations.get(index);
            let mut part: POIXMLDocumentPart = (relations.get(index)).downcast_ref::<POIXMLDocumentPart>().unwrap();
            // java if (part != null && part.getPackagePart() != null && XWPFRelation.VBA_PROJECT.getContentType().equals(part.getPackagePart().getContentType().toString()))
            if part.is_some() && part.getPackagePart().is_some() && XWPFRelation::VBA_PROJECT.getContentType().equals(part.getPackagePart().getContentType().toString()) {
              // java File tmp = Platform.createTempFile("vba", ".bin");
              let mut tmp: std::path::PathBuf = Platform::createTempFile__string_string(&"vba", &".bin");
              // java FileOutputStream outputStream = null;
              let mut outputStream: Box<dyn std::any::Any + Send + Sync> = Box::new(0i32);
              // java InputStream inputStream = null;
              let mut inputStream: Box<dyn std::io::Read> = Box::new(()) as Box<dyn std::io::Read>;
                  // java outputStream = new FileOutputStream(tmp);
                  outputStream = FileOutputStream::new(tmp);
                  // java inputStream = part.getPackagePart().getInputStream();
                  inputStream = part.getPackagePart().getInputStream();
                  // java if (inputStream == null)
                  if inputStream.is_none() {
                    // java return;
                    return;
                  }
                  // java int read = 0;
                  let mut read: i32 = 0;
                  // java byte[] buffer = new byte[4096];
                  let mut buffer: Vec<u8> = vec![Default::default(); 4096usize];
                  // java while((read = inputStream.read(buffer)) > 0)
                  while { read = inputStream.read(buffer); read } > 0 {
                    // java outputStream.write(buffer, 0, read);
                    outputStream.write(buffer, 0, read);
                  }
                  // java this.mDocument.setVBAPath(tmp.getAbsolutePath());
                  self.mDocument.setVBAPath(&tmp.getAbsolutePath());
                  // java FileNotFoundException var14
                  todo!();
                    // java IOLog.e("DocumentImporter", "create vba macro file error");
                    IOLog::e__string_string(&"DocumentImporter", &"create vba macro file error");
                    // java return;
                    return;
                  // java IOException var15
                  todo!();
                    // java IOLog.e("DocumentImporter", "write vba macro file error");
                    IOLog::e__string_string(&"DocumentImporter", &"write vba macro file error");
                    // java return;
                    return;
                    // java FileUtil.closeQuietly(outputStream);
                    FileUtil::closeQuietly__fileoutputstream(outputStream);
                    // java FileUtil.closeQuietly(inputStream);
                    FileUtil::closeQuietly__fileoutputstream(inputStream);
              // java return;
              return;
            }
          }
        }
      */
    }


    // java public void onImportCommentExStart(Attributes attributes)
    fn onImportCommentExStart(&mut self, attributes: &dyn Attributes){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心（被「假装配」结构缺陷阻塞）：批注引用节点：PLCCommentRef 仅 core/SubDocument.rs 的空 unit struct，无 CommentRefNode 内部类型，且 getCommentRefNode 不存在。
      //   语料 C1/C2a/C2b/C6′ 均无批注/脚注/尾注，不影响管线推进与文档渲染；
      //   依用户指令先 todo!("DEFERRED_NON_FUNCTION")，Java 原文与逐语句译文完整保留于下方 park 块。
      /* DEFERRED_NON_FUNCTION
        // java String paraIdParent = ConvertHelper.getString(attributes, "paraIdParent");
        let paraIdParent = ConvertHelper::getString__String(attributes, &"paraIdParent");
        // java if (null != paraIdParent)
        if paraIdParent.is_some() {
          // java PLCCommentRef.CommentRefNode cmtRefNodeParent = this.mMainDocumentImporter.getCommentRefNode(paraIdParent);
          let mut cmtRefNodeParent: PLCCommentRef::CommentRefNode = self.mMainDocumentImporter.getCommentRefNode(&paraIdParent);
          // java if (null != cmtRefNodeParent)
          if cmtRefNodeParent.is_some() {
            // java String paraId = ConvertHelper.getString(attributes, "paraId");
            let paraId = ConvertHelper::getString__String(attributes, &"paraId");
            // java if (null != paraId)
            if paraId.is_some() {
              // java PLCCommentRef.CommentRefNode cmtRefNode = this.mMainDocumentImporter.getCommentRefNode(paraId);
              let mut cmtRefNode: PLCCommentRef::CommentRefNode = self.mMainDocumentImporter.getCommentRefNode(&paraId);
              // java if (null != cmtRefNode)
              if cmtRefNode.is_some() {
                // java cmtRefNode.setParentRefNode(cmtRefNodeParent);
                cmtRefNode.setParentRefNode(cmtRefNodeParent);
              }
            }
          }
        }
      */
    }



    // java public void onImportFontStart(Attributes attributes)
    pub fn onImportFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("mFontTableImporter should not be null", this.mFontTableImporter);
      IOAssert::assertNotNull_msg("mFontTableImporter should not be null", &self.mFontTableImporter);
      // java this.mFontTableImporter.importFontStart(attributes);
      self.mFontTableImporter.as_mut().expect("mFontTableImporter").importFontStart(attributes);
    }

    // java public void onImportFontChild(int elementId, Attributes attributes)
    pub fn onImportFontChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("mFontTableImporter should not be null", this.mFontTableImporter);
      IOAssert::assertNotNull_msg("mFontTableImporter should not be null", &self.mFontTableImporter);
      // java this.mFontTableImporter.importFontChild(elementId, attributes);
      self.mFontTableImporter.as_mut().expect("mFontTableImporter").importFontChild(elementId, attributes);
    }

    // java public void onImportFontEnd()
    pub fn onImportFontEnd(&mut self) {
      // java IOAssert.assertNotNull("mFontTableImporter should not be null", this.mFontTableImporter);
      IOAssert::assertNotNull_msg("mFontTableImporter should not be null", &self.mFontTableImporter);
      // java this.mFontTableImporter.importFontEnd();
      self.mFontTableImporter.as_mut().expect("mFontTableImporter").importFontEnd();
    }

    // java public void onImportSettingsStart(XWPFSettings xwpfSettings)
    pub fn onImportSettingsStart(&mut self, xwpfSettings: &XWPFSettings) {
      // java IOAssert.assertNotNull("mSettingsImporter should not be null", this.mSettingsImporter);
      IOAssert::assertNotNull_msg("mSettingsImporter should not be null", &self.mSettingsImporter);
      // java this.mSettingsImporter.importSettingsStart(xwpfSettings);
      self.mSettingsImporter.as_mut().expect("mSettingsImporter").importSettingsStart(xwpfSettings);
    }

    // java public void onImportSettingsChild(int elementId, Attributes attributes)
    pub fn onImportSettingsChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("mSettingsImporter should not be null", this.mSettingsImporter);
      IOAssert::assertNotNull_msg("mSettingsImporter should not be null", &self.mSettingsImporter);
      // java this.mSettingsImporter.importSettingsSimpleChild(elementId, attributes);
      self.mSettingsImporter.as_mut().expect("mSettingsImporter").importSettingsSimpleChild(elementId, attributes);
    }

    // java public void onImportSettingsEnd()
    pub fn onImportSettingsEnd(&mut self) {
      // java IOAssert.assertNotNull("mSettingsImporter should not be null", this.mSettingsImporter);
      IOAssert::assertNotNull_msg("mSettingsImporter should not be null", &self.mSettingsImporter);
      // java this.mSettingsImporter.importSettingsEnd();
      self.mSettingsImporter.as_mut().expect("mSettingsImporter").importSettingsEnd();
    }

    // java public void onImportStyleDocDefaults(MutablePropertySet rPrSet, MutablePropertySet pPrSet)
    pub fn onImportStyleDocDefaults(&mut self, rPrSet: &mut MutablePropertySet, pPrSet: &mut MutablePropertySet) {
      // java this.mStylesImporter.importDocDefaults(rPrSet, pPrSet);
      self.mStylesImporter.as_ref().expect("mStylesImporter").importDocDefaults(Some(rPrSet), Some(pPrSet));
    }

    // java public void onImportStyleStart(Attributes attributes)
    pub fn onImportStyleStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("mStylesImporter should not be null", this.mStylesImporter);
      IOAssert::assertNotNull_msg("mStylesImporter should not be null", &self.mStylesImporter);
      // java this.mStylesImporter.importStyleStart(attributes);
      self.mStylesImporter.as_mut().expect("mStylesImporter should not be null").importStyleStart(attributes);
    }

    // java public void onImportStyleChild(int elementId, Attributes attributes)
    pub fn onImportStyleChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("mStylesImporter should not be null", this.mStylesImporter);
      IOAssert::assertNotNull_msg("mStylesImporter should not be null", &self.mStylesImporter);
      // java this.mStylesImporter.importStyleChild(elementId, attributes);
      self.mStylesImporter.as_mut().expect("mStylesImporter").importStyleChild(elementId, attributes);
    }

    // java public void onImportStyleTblStylePrEnd(StylePrModel stylePrModel)
    pub fn onImportStyleTblStylePrEnd(&mut self, stylePrModel: &StylePrModel) {
      // java IOAssert.assertNotNull("stylePrModel should not be null", stylePrModel);
      IOAssert::assertNotNull_msg("stylePrModel should not be null", &Some(stylePrModel));
      // java IOAssert.assertNotNull("mStylesImporter should not be null", this.mStylesImporter);
      IOAssert::assertNotNull_msg("mStylesImporter should not be null", &self.mStylesImporter);
      // java this.mStylesImporter.importTblStylePrEnd(stylePrModel);
      self.mStylesImporter.as_ref().expect("mStylesImporter").importTblStylePrEnd(Some(stylePrModel));
    }

    // java public void convertStyleTcStylePr(int elementId, Attributes attributes, MutablePropertySet dst)
    pub fn convertStyleTcStylePr(&mut self, elementId: i32, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &Some(attributes));
      // java IOAssert.assertNotNull("dst should not be null", dst);
      IOAssert::assertNotNull_msg("dst should not be null", &Some(&*dst));
      // java this.mCellPropertyConverter.convertTcStylePr(elementId, attributes, dst);
      self.mCellPropertyConverter.as_ref().expect("mCellPropertyConverter").convertTcStylePr(elementId, attributes, dst);
    }

    // java public void onImportStyleEnd(StylePrModel stylePrModel)
    pub fn onImportStyleEnd(&mut self, stylePrModel: &StylePrModel) {
      // java IOAssert.assertNotNull("stylePrModel should not be null", stylePrModel);
      IOAssert::assertNotNull_msg("stylePrModel should not be null", &Some(stylePrModel));
      // java IOAssert.assertNotNull("mStylesImporter should not be null", this.mStylesImporter);
      IOAssert::assertNotNull_msg("mStylesImporter should not be null", &self.mStylesImporter);
      // java this.mStylesImporter.importStyleEnd(stylePrModel);
      self.mStylesImporter.as_mut().expect("mStylesImporter").importStyleEnd(stylePrModel);
    }

    // java public void onImportStylesEnd()
    pub fn onImportStylesEnd(&mut self) {
      // java IOAssert.assertNotNull("mStylesImporter should not be null", this.mStylesImporter);
      IOAssert::assertNotNull_msg("mStylesImporter should not be null", &self.mStylesImporter);
      // java this.mStylesImporter.importStylesEnd();
      self.mStylesImporter.as_mut().expect("mStylesImporter").importStylesEnd();
      // Java passes the document's KStyles object into StylesImporter. The
      // Rust importer currently owns a value copy, so publish the parsed
      // styles at the same SAX boundary, before document.xml (and sectPr)
      // starts. Publishing only after XWPFDocument.parse loses Normal's font
      // size while convertDocumentGrid reads it.
      let imported = self.mStylesImporter.as_ref().expect("mStylesImporter").mStyles.borrow();
      let mut styles = Vec::new();
      let mut index = 0;
      while let Some(style) = imported.getStyleByIndex(index) {
        styles.push(style);
        index += 1;
      }
      let r_default = imported.getRPrDefault();
      let p_default = imported.getPPrDefault();
      drop(imported);
      if let Some(document) = self.mDocument {
        if let Some(destination) = unsafe { (*document).getStylesMut() } {
          destination.setRPrDefault(&r_default);
          destination.setPPrDefault(&p_default);
          for mut style in styles {
            destination.add(&mut style);
          }
        }
      }
    }

    // java public void onImportNumberingNumStart(int elementId, Attributes attributes)
    pub fn onImportNumberingNumStart(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingNumStart(elementId, attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingNumStart(elementId, attributes);
    }

    // java public void onImportNumberingNumChild(int elementId, Attributes attributes)
    pub fn onImportNumberingNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingNumChild(elementId, attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingNumChild(elementId, attributes);
    }

    // java public void onImportNumberingNumEnd()
    pub fn onImportNumberingNumEnd(&mut self) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingNumEnd();
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingNumEnd();
    }

    // java public void onImportNumberingLevelStart(NumLvlType lvlType, Attributes attributes)
    pub fn onImportNumberingLevelStart(&mut self, lvlType: &NumLvlType, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingLevelStart(lvlType, attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingLevelStart(lvlType, attributes);
    }

    // java public void onImportNumberingLevelChild(NumLvlType lvlType, int elementId, Attributes attributes)
    pub fn onImportNumberingLevelChild(&mut self, lvlType: &NumLvlType, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingLevelChild(lvlType, elementId, attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingLevelChild(lvlType, elementId, attributes);
    }

    // java public void onImportNumberingLevelEnd(NumLvlType lvlType, MutablePropertySet rPrBaseSet, MutablePropertySet pPrSet)
    pub fn onImportNumberingLevelEnd(&mut self, lvlType: &NumLvlType, rPrBaseSet: &MutablePropertySet, pPrSet: &MutablePropertySet) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingLevelEnd(lvlType, rPrBaseSet, pPrSet);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingLevelEnd(lvlType, rPrBaseSet, pPrSet);
    }

    // java public void onImportNumberingAbstractNumStart(Attributes attributes)
    pub fn onImportNumberingAbstractNumStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingAbstractNumStart(attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingAbstractNumStart(attributes);
    }

    // java public void onImportNumberingAbstractNumChild(int elementId, Attributes attributes)
    pub fn onImportNumberingAbstractNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingAbstractNumChild(elementId, attributes);
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingAbstractNumChild(elementId, attributes);
    }

    // java public void onImportNumberingAbstractNumEnd()
    pub fn onImportNumberingAbstractNumEnd(&mut self) {
      // java IOAssert.assertNotNull("mNumberingImporter should not be null", this.mNumberingImporter);
      IOAssert::assertNotNull_msg("mNumberingImporter should not be null", &self.mNumberingImporter);
      // java this.mNumberingImporter.importNumberingAbstractNumEnd();
      self.mNumberingImporter.as_mut().expect("mNumberingImporter").importNumberingAbstractNumEnd();
    }

    // java public void onImportOMathStart(SubDocType subDocType, boolean isInOMathPara, MutablePropertySet oMathParaPr)
    fn onImportOMathStart(&self, subDocType: &SubDocType, isInOMathPara: bool, oMathParaPr: &MutablePropertySet){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：依赖 MathIOType（wps-io 侧为空壳 pub struct MathIOType {}）。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importOMathStart(isInOMathPara, oMathParaPr);
        subDocumentImporter.importOMathStart(isInOMathPara, oMathParaPr);
      */
    }


    // java public void onImportOMathEnd(SubDocType subDocType)
    fn onImportOMathEnd(&self, subDocType: &SubDocType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importOMathEnd();
        subDocumentImporter.importOMathEnd();
      */
    }


    // java public void importMathElementsProp(SubDocType subDocType, MathIOType mathType, int elementId, Attributes attributes)
    fn importMathElementsProp(&self, subDocType: &SubDocType, mathType: &MathIOType, elementId: i32, attributes: &dyn Attributes){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathElementsProp(mathType, elementId, attributes);
        subDocumentImporter.importMathElementsProp(mathType, elementId, attributes);
      */
    }


    // java public void importMathArgmentEnd(SubDocType subDocType, MutablePropertySet propSet, MathIOType mathType, MathArgmentType argType)
    fn importMathArgmentEnd(&mut self, subDocType: &SubDocType, propSet: &mut MutablePropertySet, mathType: &MathIOType, argType: &MathArgmentType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java propSet = this.mergeRevisionRunProp(propSet);
        let _ = self.mergeRevisionRunProp(Some(propSet));
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathArgmentEnd(propSet, argType, mathType);
        subDocumentImporter.importMathArgmentEnd(propSet, argType, mathType);
      */
    }


    // java public void importMathObjectEnd(MutablePropertySet runPropSet, MathIOType mathType, SubDocType subDocType)
    fn importMathObjectEnd(&mut self, runPropSet: &mut MutablePropertySet, mathType: &MathIOType, subDocType: &SubDocType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("mathType should not be null", mathType);
        IOAssert::assertNotNull__string_object(&"mathType should not be null", Some(&mathType));
        // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
        IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
        // java runPropSet = this.mergeRevisionRunProp(runPropSet);
        let _ = self.mergeRevisionRunProp(Some(runPropSet));
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathObjectEndSimple(runPropSet, mathType);
        subDocumentImporter.importMathObjectEndSimple(runPropSet, mathType);
      */
    }


    // java public void onImportMathObjectBegin(SubDocType subDocType, MathIOType mathType)
    fn onImportMathObjectBegin(&self, subDocType: &SubDocType, mathType: &MathIOType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathObjectBegin(mathType);
        subDocumentImporter.importMathObjectBegin(mathType);
      */
    }


    // java public void onImportMathArgmentStart(SubDocType subDocType, MathIOType mathType, MathArgmentType argmentType)
    fn onImportMathArgmentStart(&self, subDocType: &SubDocType, mathType: &MathIOType, argmentType: &MathArgmentType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathArgmentStart(mathType, argmentType);
        subDocumentImporter.importMathArgmentStart(mathType, argmentType);
      */
    }


    // java public void onImportMathMatrixRowEnd(SubDocType subDocType)
    fn onImportMathMatrixRowEnd(&self, subDocType: &SubDocType){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：OMML 公式：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java subDocumentImporter.importMathMatrixRowEnd();
        subDocumentImporter.importMathMatrixRowEnd();
      */
    }


    // java public IVmlImporter getVmlImporter(SubDocType subDocType)
    fn getVmlImporter(&self, subDocType: &SubDocType)-> Box<dyn IVmlImporter> {
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：VML 导入器：依赖 VmlReader 真实装配（di03_seam 影子）。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
        IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
        // java return subDocumentImporter.getVmlImporter(this);
        return subDocumentImporter.getVmlImporter(self);
      */
    }


    // java public IDmlImporter getDmlImporter(SubDocType subDocType)
    fn getDmlImporter(&self, subDocType: &SubDocType)-> Option<Box<dyn IDmlImporter>> {
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：DML 导入器：依赖 DmlImporter 真实装配（di03_seam 影子）。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
        let mut subDocumentImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
        // java return null == subDocumentImporter ? null : subDocumentImporter.getDmlImporter(this, this.mThemeContext, (Xml07Context)null);
        return (if subDocumentImporter.is_none() { None } else { Some(subDocumentImporter.getDmlImporter(self, self.mThemeContext, None)) });
      */
    }


    // java public boolean hasContent()
    pub fn hasContent(&self) -> bool {
      // java return this.mDocument.getMainDocument().getLength() > 0;
      let Some(p) = self.mDocument else {
        return false;
      };
      unsafe {
        return (*p)
          .getMainDocument()
          .map(|m| m.getLength() > 0)
          .unwrap_or(false);
      }
    }

    // java public void onImportFontSchemeStart(Attributes attributes)
    pub fn onImportFontSchemeStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importFontSchemeStart(attributes);
      self.mThemeImporter.as_mut().expect("mThemeImporter").importFontSchemeStart(attributes);
    }

    // java public void onImportMajorFontStart()
    pub fn onImportMajorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importMajorFontStart();
      self.mThemeImporter.as_mut().expect("mThemeImporter").importMajorFontStart();
    }

    // java public void onImportMinorFontStart()
    pub fn onImportMinorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importMinorFontStart();
      self.mThemeImporter.as_mut().expect("mThemeImporter").importMinorFontStart();
    }

    // java public void onImportThemeCSFontStart(Attributes attributes)
    pub fn onImportThemeCSFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importCSFontStart(attributes);
      self.mThemeImporter.as_mut().expect("mThemeImporter").importCSFontStart(attributes);
    }

    // java public void onImportThemeEAFontStart(Attributes attributes)
    pub fn onImportThemeEAFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importEAFontStart(attributes);
      self.mThemeImporter.as_mut().expect("mThemeImporter").importEAFontStart(attributes);
    }

    // java public void onImportThemeLatinFontStart(Attributes attributes)
    pub fn onImportThemeLatinFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importLatinFontStart(attributes);
      self.mThemeImporter.as_mut().expect("mThemeImporter").importLatinFontStart(attributes);
    }

    // java public void onImportThemeFontStart(Attributes attributes)
    pub fn onImportThemeFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeImporter shoud not be null!", this.mThemeImporter);
      IOAssert::assertNotNull_msg("mThemeImporter shoud not be null!", &self.mThemeImporter);
      // java this.mThemeImporter.importFontStart(attributes);
      self.mThemeImporter.as_mut().expect("mThemeImporter").importFontStart(attributes);
    }

    // java public void onImportMoveRangeStart(int elementId, Attributes attributes)
    pub fn onImportMoveRangeStart(&mut self, elementId: i32, attributes: &dyn Attributes){
      // rust: ★★★已激活（原为 park + todo!）—— 原挂起理由「依赖 Revision 为空 unit struct」
      //   **已解除**：`Revision` 已由 compile-surface 空壳做实为真实值类型（§67），
      //   `MoveRangeImporter` 亦已按 Java 真实实现并登记进 `lib.rs`。
      // java if (null == this.mMoveRangeImporter)
      if self.mMoveRangeImporter.is_none() {
        // java this.mMoveRangeImporter = new MoveRangeImporter(this.mDocument);
        self.mMoveRangeImporter = Some(MoveRangeImporter::MoveRangeImporter(self.mDocument));
      }
      // java IOAssert.assertNotNull("mMoveRangeImporter should not be null!", this.mMoveRangeImporter);
      IOAssert::assertNotNull__string_object(&"mMoveRangeImporter should not be null!", Some(&self.mMoveRangeImporter));
      // java switch (elementId)
      match elementId {
        // java case -688961888:
        -688961888 => {
          // java this.mCurrentRevision = this.mMoveRangeImporter.importMoveFromRangeStart(attributes);
          self.mCurrentRevision = Some(self.mMoveRangeImporter.as_ref().expect("mMoveRangeImporter").importMoveFromRangeStart(attributes));
        }
        // java case 1452770417:
        1452770417 => {
          // java this.mCurrentRevision = this.mMoveRangeImporter.importMoveToRangeStart(attributes);
          self.mCurrentRevision = Some(self.mMoveRangeImporter.as_ref().expect("mMoveRangeImporter").importMoveToRangeStart(attributes));
        }
        // java default:
        _ => {
          // java IOAssert.fail();
          IOAssert::fail();
        }
      }
    }

  // java break;

    // java public void onImportMoveRangeEnd(int elementId, Attributes attributes)
    pub fn onImportMoveRangeEnd(&mut self, elementId: i32, attributes: &dyn Attributes){
      // rust: ★★★已激活（原为 park + todo!）—— 同 `onImportMoveRangeStart`，依赖已做实。
      // java this.mCurrentRevision = null;
      self.mCurrentRevision = None;
    }


    // java public IDmlTextImporter getDmlTextImporter(MutablePropertySet prop)
    fn getDmlTextImporter(&mut self, prop: &MutablePropertySet)-> Box<dyn IDmlTextImporter> {
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：DML 文本导入器：同上。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java return new DmlTextImporter(prop, this.mThemeContext, this.mode());
        return DmlTextImporter::new(prop, self.mThemeContext, self.super_.mode());
      */
    }


    // java public void onImportRuby(SubDocType subDocType, RubyPr rubyPr, RubyContent rubyBase, RubyContent rt)
    fn onImportRuby(&self, subDocType: &SubDocType, rubyPr: &RubyPr, rubyBase: &RubyContent, rt: &RubyContent){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：注音（Ruby）：依赖 RubyContent.mTmpText（wps-io 侧私有字段）。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("rubyPr should not be null!", rubyPr);
        IOAssert::assertNotNull__string_object(&"rubyPr should not be null!", Some(&rubyPr));
        // java IOAssert.assertNotNull("rubyBase should not be null!", rubyBase);
        IOAssert::assertNotNull__string_object(&"rubyBase should not be null!", Some(&rubyBase));
        // java IOAssert.assertNotNull("rubyPr should not be null!", rt);
        IOAssert::assertNotNull__string_object(&"rubyPr should not be null!", Some(&rt));
        // java if (null != rubyBase.mTmpText && null != rt.mTmpText)
        if rubyBase.mTmpText.is_some() && rt.mTmpText.is_some() {
          // java SubDocumentImporter subDocImporter = this.getSubDocumentImporter(subDocType);
          let mut subDocImporter: SubDocumentImporter = self.getSubDocumentImporter(&subDocType);
          // java subDocImporter.importRuby(rubyPr, rubyBase, rt);
          subDocImporter.importRuby(rubyPr, rubyBase, rt);
        }
      */
    }


    // java public void setThemePackagePart(POIXMLDocumentPart packagePart)
    pub fn setThemePackagePart(&mut self, packagePart: std::rc::Rc<POIXMLDocumentPart>){
        // java this.mThemeImporter.setThemePackagePart(packagePart);
        self.mThemeImporter.as_mut().expect("mThemeImporter").setThemePackagePart(packagePart);
    }


    // java public void onImportMedia(SubDocType mSubDocType, Attributes attributes)
    fn onImportMedia(&mut self, mSubDocType: &SubDocType, attributes: &dyn Attributes){
      todo!("DEFERRED_NON_FUNCTION");
      // rust: ★非管线核心：媒体导入：依赖无真实实现的 IMediaLib 接线。
      //   依用户指令「不影响管线推进的非管线核心逻辑可先 todo!("DEFERRED_NON_FUNCTION")」处置；
      //   Java 原文与逐语句译文完整保留于下方 park 块，依赖做实后原样激活。
      /* DEFERRED_NON_FUNCTION
        // java String url = ConvertHelper.getString(attributes, "url");
        let url = ConvertHelper::getString__String(attributes, &"url");
        // java String type = ConvertHelper.getString(attributes, "type");
        let mut r#type: String = ConvertHelper::getString__attributes_string(&attributes, &"type");
        // java if (null != url && null != type)
        if url.is_some() && r#type.is_some() {
          // java String newUrl = url;
          let mut newUrl: String = url;
          // java String notePath = this.mIDLProvider != null ? this.mIDLProvider.getNotePath() : null;
          let mut notePath: String = (if self.mIDLProvider.is_some() { self.mIDLProvider.getNotePath() } else { String::new() });
          // java if (!this.isEmpty(notePath))
          if !self.isEmpty(Some(&notePath)) {
            // java newUrl = notePath.substring(0, notePath.lastIndexOf(File.separator) + 1) + url;
            newUrl = notePath.substring(0, notePath.lastIndexOf(File::separator) + 1) + url;
            // java File newFile = new File(newUrl);
            let mut newFile: std::path::PathBuf = File::new(newUrl);
            // java if (!newFile.exists())
            if !newFile.exists() {
              // java return;
              return;
            }
          }
          // java if (type.equals("picture"))
          if r#type.equals("picture") {
            // java this.importPicture(newUrl);
            self.importPicture(&newUrl);
          }
          // java else if (type.equals("audio") || type.equals("vedio"))
          else if r#type.equals("audio") || r#type.equals("vedio") {
            // java this.importAudio(newUrl, notePath);
            self.importAudio(&newUrl, Some(&notePath));
          }
        }
      */
    }


    // java public void onImportDocFieldStartEnd(int elementId, Attributes attributes, SubDocType subDocType)
    pub fn onImportDocFieldStartEnd(&mut self, elementId: i32, attributes: &dyn Attributes, subDocType: &SubDocType){
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull__string_object(&"subDocType should not be null", Some(&subDocType));
      // java SubDocumentImporter subDocumentImporter = this.getSubDocumentImporter(subDocType);
      let subDocumentImporter = self.getSubDocumentImporter_mut(*subDocType as i32);
      // java IOAssert.assertNotNull("subDocumentImporter shoud not be null!", subDocumentImporter);
      let Some(subDocumentImporter) = subDocumentImporter else { return; };
      IOAssert::assertNotNull__string_object(&"subDocumentImporter shoud not be null!", Some(&subDocumentImporter));
      // java switch (elementId)
      match elementId {
        // java case 876929721:
        876929721 => {
          // java subDocumentImporter.importDocumentFieldEnd(attributes);
          subDocumentImporter.importDocumentFieldEnd(attributes);
        }
        // java case 928980672:
        928980672 => {
          // java subDocumentImporter.importDocFieldStart(attributes);
          subDocumentImporter.importDocFieldStart(attributes);
        }
        // java default:
        _ => {
          // java IOAssert.fail();
          IOAssert::fail();
        }
      }
    }

  // java break;

    // java public boolean isLock()
    pub fn isLock(&self) -> bool {
      // java if (this.mDocument == null)
      if self.mDocument.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Document mainDocument = this.mDocument.getMainDocument();
        let mainDocument = unsafe { (*self.mDocument.unwrap()).getMainDocument() };
        // java return mainDocument == null ? false : mainDocument.isLocked();
        return mainDocument.map(|m| m.isLocked()).unwrap_or(false);
      }
    }

}

// java public class DocumentImporter extends ImporterBase implements IDocumentImporter
impl DocumentImporter {
    // java public  DocumentImporter(KStyles kStyles, int mode)
    pub fn DocumentImporter__kstyles_int(kStyles: Option<KStyles>, mode: i32) -> Self {
      // java this(kStyles, mode, (Xml07Context)null, (Xml03Context)null);
      Self::DocumentImporter__kstyles_int_xml07context_xml03context(kStyles, mode, None, None)
    }

    // java public  DocumentImporter(KStyles kStyles, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn DocumentImporter__kstyles_int_xml07context_xml03context(
      kStyles: Option<KStyles>,
      mode: i32,
      xml07Context: Option<Xml07Context>,
      xml03Context: Option<Xml03Context>,
    ) -> Self {
      // java super(mode, xml07Context, xml03Context);
      let super_ = ImporterBase::ImporterBase(mode, xml07Context.clone(), xml03Context.clone());
      // java this.mEndnoteDocumentImporter = null;
      // java this.mFootnoteDocumentImporter = null;
      let mut this = Self {
        super_,
        mDocument: None,
        mIoListener: None,
        mStylesImporter: None,
        mSettingsImporter: None,
        mFontTableImporter: None,
        mNumberingImporter: None,
        mThemeImporter: None,
        mParaPropsConverter: None,
        mCellPropertyConverter: None,
        mRunPropsConverter: None,
        mTablePropsConverter: None,
        mMainDocumentImporter: None,
        mTextboxDocumentImporter: None,
        mHdrFtrTextboxDocumentImporter: None,
        mHdrFtrDocumentImporter: None,
        mCommentsDocumentImporter: None,
        mEndnoteDocumentImporter: None,
        mFootnoteDocumentImporter: None,
        mStyleId2Sti: None,
        mNumIdRedirection: None,
        mCountParagraph: 0,
        mHasHeaderFooter: false,
        mThemeContext: None,
        mMoveRangeImporter: None,
        mCurrentRevision: None,
        mIDLProvider: None,
      };
      // java IOAssert.assertNotNull("kStyles should not be null", kStyles);
      IOAssert::assertNotNull_msg("kStyles should not be null", &kStyles);
      let Some(kStyles) = kStyles else {
        panic!("kStyles should not be null");
      };
      // java this.mDocument = kStyles.getTextDocument();
      // rust: ★本构造器**故意不设** `mDocument`（不是静默丢弃，是**正确行为**）：
      //   - 直接调用方 `DocumentImporter__textdocument_...` 在**本构造器返回后立即**
      //     执行 `this.mDocument = Some(document as *mut TextDocument);` 覆盖之；
      //   - 另一调用方 `KStyle` 反射路径也不需要 `mDocument`。
      //   ★为何不在此处取址：`KStyles::getTextDocument()` 返回 **clone**（Java 返回引用），
      //   且 `mTextDocument` 是**值字段**；`kStyles` 随后被移入 `Rc<RefCell<>>`
      //   ⇒ 取其地址会**悬垂**。需先修 `KStyles` 的引用语义（工单 X4）。
      //   ★★教训：此处**不可**放 `todo!()` —— 该路径被 28 个测试触达（含主渲染管线）。
      // java FileUtil.newFolder(this.mDocument.getTmpRootDir());
      FileUtil::newFolder(None);
      // java this.mStyleId2Sti = new StyleId2Sti(this.mDocument.getUUSti());
      let style = Rc::new(RefCell::new(StyleId2Sti::StyleId2Sti(UUSti::UUSti())));
      this.mStyleId2Sti = Some(style.clone());
      // java this.mNumIdRedirection = new NumIdRedirection(this.mDocument.getUUNumberingId());
      this.mNumIdRedirection = Some(Rc::new(RefCell::new(NumIdRedirection::NumIdRedirection(UUNumberingId::new()))));
      // java this.mStylesImporter = new StylesImporter(kStyles, this.mStyleId2Sti);
      this.mStylesImporter = Some(StylesImporter::StylesImporter(
        std::rc::Rc::new(std::cell::RefCell::new(kStyles)),
        style,
      ));
      // java this.mThemeContext = new ThemeContext();
      this.mThemeContext = Some(Rc::new(RefCell::new(ThemeContext::default())));
      // java this.mParaPropsConverter = new ParaPropsConverterSimple(this.mode(), this.xml07Context(), this.xml03Context());
      this.mParaPropsConverter = Some(ParaPropsConverterSimple::ParaPropsConverterSimple(
        mode,
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      // java this.mRunPropsConverter = new RunPropsConverterSimple(this.mode(), this.xml07Context(), this.xml03Context());
      this.mRunPropsConverter = Some(RunPropsConverterSimple::RunPropsConverterSimple(
        mode,
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      // java this.mTablePropsConverter = new TablePropertyBaseConverterSimple(this.mode(), this.xml07Context(), this.xml03Context());
      let tableProps = TablePropertyBaseConverterSimple::TablePropertyBaseConverterSimple(
        mode,
        xml07Context.clone(),
        xml03Context.clone(),
      );
      // java this.mCellPropertyConverter = new CellPropertyConverterSimple(this.mTablePropsConverter, this.mode(), this.xml07Context(), this.xml03Context());
      this.mCellPropertyConverter = Some(CellPropertyConverterSimple::CellPropertyConverterSimple(
        tableProps.clone(),
        mode,
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      this.mTablePropsConverter = Some(tableProps);
      this
    }

    fn subdoc_mut(document: &mut TextDocument, id: i32) -> &mut SubDocument {
      document
        .mSubdocuments
        .as_mut()
        .and_then(|v| v.get_mut(id as usize))
        .and_then(|s| s.as_mut())
        .map(|b| b.as_mut())
        .expect("subdocument")
    }

    fn assemble_sub(
      document: &mut SubDocument,
      mode: i32,
      xml07Context: Option<Xml07Context>,
      xml03Context: Option<Xml03Context>,
      ioListener: Option<Box<dyn IOListener>>,
    ) -> SubDocumentImporter {
      SubDocumentImporter::SubDocumentImporter(
        document,
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        ioListener,
        false,
        mode,
        xml07Context,
        xml03Context,
      )
    }

    // java public  DocumentImporter(TextDocument document, IOListener ioListener, IDLProvider dlProvider, boolean initDrawingContainer, ThemeContext themeContext, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
      document: &mut TextDocument,
      ioListener: Option<Box<dyn IOListener>>,
      dlProvider: Option<Box<dyn IDLProvider>>,
      initDrawingContainer: bool,
      themeContext: Option<ThemeContext>,
      mode: i32,
      xml07Context: Option<Xml07Context>,
      xml03Context: Option<Xml03Context>,
    ) -> Self {
      // java this(document.getStyles(), mode, xml07Context, xml03Context);
      let parent = crate::cn::wps::moffice::writer::core::KStyle::TextDocument::default();
      let light_styles = KStyles::KStyles(&parent);
      let mut this = Self::DocumentImporter__kstyles_int_xml07context_xml03context(
        Some(light_styles),
        mode,
        xml07Context.clone(),
        xml03Context.clone(),
      );
      // java IOAssert.assertNotNull("ioListener should not be null", ioListener);
      IOAssert::assertNotNull_msg("ioListener should not be null", &ioListener);
      let Some(ioListener) = ioListener else {
        panic!("ioListener should not be null");
      };
      // java if (document.getMainDocument().getLength() == 0)
      let main_len = document.getMainDocument().map(|m| m.getLength()).unwrap_or(0);
      if main_len == 0 {
        // java document.setProperty(PropertySet.empty);
        document.setProperty__propertyset(&*empty);
      }
      this.mDocument = Some(document as *mut TextDocument);
      // java SubDocument mainDocument = (SubDocument)this.mDocument.createSubDocument(0);
      document.createSubDocument(0);
      // java IOAssert.assertNotNull("mainDocument should not be null", mainDocument);
      // java this.mIoListener = ioListener;
      this.mIoListener = Some(ioListener);
      // java this.mIDLProvider = dlProvider;
      this.mIDLProvider = dlProvider;
      // java this.mSettingsImporter = new SettingsImporter(this.mDocument, this.mIoListener, this.mIDLProvider, this.mode(), this.xml07Context(), this.xml03Context());
      // rust: ★`mIoListener` / `mIDLProvider` 传 `None` —— 二者在 Java 中**唯一**消费方是
      //   `setWriteProtection`（本轮仍为 `DEFERRED_NON_FUNCTION`），且
      //   `core.io.IOListener` 现已登记进 `lib.rs`（M-STAGE2 P1-D1 解除 DocumentImporter.rs:120 的 marker）。
      //   这是**已记录的缺口**，不是静默丢弃：其余 174 路 dispatch 与全部 correct* 均不读它们。
      this.mSettingsImporter = Some(SettingsImporter::SettingsImporter(
        document as *mut TextDocument,
        None,
        None,
        this.mode(),
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      // java this.mFontTableImporter = new FontTableImporter(this.mDocument.getFontTable(), this.mode(), this.xml07Context(), this.xml03Context());
      this.mFontTableImporter = Some(FontTableImporter::FontTableImporter(
        unsafe { &*document }.getFontTable(),
        this.mode(),
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      // java this.mNumberingImporter = new NumberingImporter(this.mDocument, this.mStyleId2Sti, this.mNumIdRedirection, this.mode(), this.xml07Context(), this.xml03Context());
      let style = this.mStyleId2Sti.clone().expect("mStyleId2Sti");
      let num = this.mNumIdRedirection.clone().expect("mNumIdRedirection");
      this.mNumberingImporter = Some(NumberingImporter::NumberingImporter(
        document as *mut TextDocument,
        style,
        num,
        this.mode(),
        xml07Context.clone(),
        xml03Context.clone(),
      ));
      // java this.mThemeImporter = new ThemeImporter(this.mDocument.getTheme());
      // M-STAGE2 S2-2: 原先此处是 `ThemeImporter::default()` —— **D 类假装配**：
      //   `Default` 使 `mFontSchemeImporter = None`，而 `ThemeImporter.importFontSchemeStart`
      //   等 8 个方法首个动作就是断言其非空 ⇒ 一旦走到主题导入即 panic。
      //   Java 是 `new ThemeImporter(this.mDocument.getTheme())`，故改用真实构造器。
      let theme_ptr: *mut Theme = document.getTheme_mut().unwrap_or(std::ptr::null_mut());
      this.mThemeImporter = Some(ThemeImporter::ThemeImporter(theme_ptr));
      let xml07 = xml07Context.clone();
      let xml03 = xml03Context.clone();
      // java this.mMainDocumentImporter = new MainDocumentImporter(mainDocument, this.mStyleId2Sti, this.mNumIdRedirection, this.mIoListener, dlProvider, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      this.mMainDocumentImporter = Some(MainDocumentImporter::MainDocumentImporter(
        Self::subdoc_mut(document, 0),
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        None,
        None,
        initDrawingContainer,
        mode,
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mTextboxDocumentImporter = new MainTextboxDocumentImporter((SubDocument)this.mDocument.createSubDocument(5), this.mStyleId2Sti, this.mNumIdRedirection, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(5);
      // rust: ★由 `assemble_sub`（只造基类 ⇒ **假装配**）改为**真构造器** `MainTextboxDocumentImporter`。
      this.mTextboxDocumentImporter = Some(MainTextboxDocumentImporter::MainTextboxDocumentImporter(
        Self::subdoc_mut(document, 5),
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        initDrawingContainer,
        this.mode(),
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mHdrFtrTextboxDocumentImporter = new HeaderTextboxDocumentImporter((SubDocument)this.mDocument.createSubDocument(6), this.mStyleId2Sti, this.mNumIdRedirection, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(6);
      // rust: ★由 `assemble_sub`（只造基类 ⇒ **假装配**）改为**真构造器** `HeaderTextboxDocumentImporter`。
      this.mHdrFtrTextboxDocumentImporter = Some(HeaderTextboxDocumentImporter::HeaderTextboxDocumentImporter(
        Self::subdoc_mut(document, 6),
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        initDrawingContainer,
        this.mode(),
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mHdrFtrDocumentImporter = new HdrFtrDocumentImporter((SubDocument)this.mDocument.createSubDocument(2), this.mStyleId2Sti, this.mNumIdRedirection, this.mIoListener, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(2);
      // rust: ★由 `assemble_sub`（只造基类 ⇒ **假装配**）改为**真构造器**。
      //   ★已记录偏差：Java 共享 `this.mStyleId2Sti` / `this.mNumIdRedirection`，
      //   而二者**非 `Clone`**，Rust 侧各 importer 各持一份 ⇒ 样式映射不互通
      //   （见 M_STAGE1_BASELINE §40）。`mIoListener` 同理（`Box` 不可共享）。
      this.mHdrFtrDocumentImporter = Some(HdrFtrDocumentImporter::HdrFtrDocumentImporter(
        Self::subdoc_mut(document, 2),
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        None,
        initDrawingContainer,
        this.mode(),
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mCommentsDocumentImporter = new CommentsDocumentImporter((SubDocument)this.mDocument.createSubDocument(3), this.mStyleId2Sti, this.mNumIdRedirection, this.mIoListener, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(3);
      this.mCommentsDocumentImporter = Some(Self::assemble_sub(Self::subdoc_mut(document, 3), mode, xml07.clone(), xml03.clone(), None));
      // java this.mFootnoteDocumentImporter = new FootnoteDocumentImporter(this.mDocument, this.mStyleId2Sti, this.mNumIdRedirection, this.mIoListener, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(1);
      // rust: ★M-STAGE2 P1-D1 —— 原先此处以 `assemble_sub` **伪造**为 `SubDocumentImporter`
      //   （类型错、styleId2Sti/numIdRedirection 被凭空新建、ioListener 被丢弃）。
      //   现改为真实构造 `FootnoteDocumentImporter`，并传入**真实的**
      //   `this.mStyleId2Sti` / `this.mNumIdRedirection`（已加 `Clone`，优于先例的凭空新建）。
      //   ★已记录偏差：Java 共享 `this.mIoListener`，而 `Box<dyn IOListener>` 不可共享，
      //   且 `this.mIoListener` 在 `DocumentImporter` 自身 :2456 处仍被读取 ⇒ 不可 `take`。
      //   与 `HdrFtrDocumentImporter` 先例一致传 `None`（仅损失该子文档的进度回调）。
      this.mFootnoteDocumentImporter = Some(FootnoteDocumentImporter::FootnoteDocumentImporter(
        document,
        this.mStyleId2Sti.as_ref().map(|r| r.borrow().clone()).unwrap_or_default(),
        this.mNumIdRedirection.as_ref().map(|r| r.borrow().clone()).unwrap_or_default(),
        None,
        initDrawingContainer,
        this.mode(),
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mEndnoteDocumentImporter = new EndnoteDocumentImporter(this.mDocument, this.mStyleId2Sti, this.mNumIdRedirection, this.mIoListener, initDrawingContainer, this.mode(), this.xml07Context(), this.xml03Context());
      document.createSubDocument(4);
      // rust: ★M-STAGE2 P1-D1 —— 同上，改为真实构造 `EndnoteDocumentImporter`。
      this.mEndnoteDocumentImporter = Some(EndnoteDocumentImporter::EndnoteDocumentImporter(
        document,
        this.mStyleId2Sti.as_ref().map(|r| r.borrow().clone()).unwrap_or_default(),
        this.mNumIdRedirection.as_ref().map(|r| r.borrow().clone()).unwrap_or_default(),
        None,
        initDrawingContainer,
        this.mode(),
        xml07.clone(),
        xml03.clone(),
      ));
      // java this.mThemeContext = null != themeContext ? themeContext : this.mThemeContext;
      if let Some(themeContext) = themeContext {
        this.mThemeContext = Some(Rc::new(RefCell::new(themeContext)));
      }
      // java this.mCountParagraph = 0;
      this.mCountParagraph = 0;
      // java this.mHasHeaderFooter = false;
      this.mHasHeaderFooter = false;
      this
    }

    // java public SubDocumentImporter getSubDocumentImporter(SubDocType subDocType)
    pub fn getSubDocumentImporter(&self, subDocType: i32) -> Option<&SubDocumentImporter> {
      let opt = Some(subDocType);
      // java IOAssert.assertNotNull("subDocType should not be null", subDocType);
      IOAssert::assertNotNull_msg("subDocType should not be null", &opt);
      // java IOAssert.assertNotNull("mMainDocumentImporter should not be null", this.mMainDocumentImporter);
      IOAssert::assertNotNull_msg("mMainDocumentImporter should not be null", &self.mMainDocumentImporter);
      if self.mMainDocumentImporter.is_none() {
        panic!("mMainDocumentImporter should not be null");
      }
      // java IOAssert.assertNotNull("mTextboxDocumentImporter should not be null", this.mTextboxDocumentImporter);
      IOAssert::assertNotNull_msg("mTextboxDocumentImporter should not be null", &self.mTextboxDocumentImporter);
      if self.mTextboxDocumentImporter.is_none() {
        panic!("mTextboxDocumentImporter should not be null");
      }
      // java SubDocumentImporter subDocImporter = null;
      let mut subDocImporter: Option<&SubDocumentImporter> = None;
      // java switch (subDocType)
      match subDocType {
        // java case MAIN_DOCUMENT:
        0 => {
          // java subDocImporter = this.mMainDocumentImporter;
          subDocImporter = self.mMainDocumentImporter.as_ref().map(|m| &m.super_);
        }
        // java case TEXTBOX_DOCUMENT:
        5 => {
          // java subDocImporter = this.mTextboxDocumentImporter;
          // rust: 真类型含 `Deref<Target = SubDocumentImporter>` ⇒ 显式降为基类引用。
          subDocImporter = self.mTextboxDocumentImporter.as_ref().map(|m| &m.super_.super_);
        }
        // java case HEADER_DOCUMENT:
        2 => {
          // java subDocImporter = this.mHdrFtrDocumentImporter;
          // rust: `HdrFtrDocumentImporter` 是**真类型**（含 `Deref<Target = SubDocumentImporter>`）
          //   ⇒ 显式降为基类引用。
          subDocImporter = self.mHdrFtrDocumentImporter.as_ref().map(|m| &m.super_);
        }
        // java case COMMENT_DOCUMENT:
        3 => {
          // java subDocImporter = this.mCommentsDocumentImporter;
          subDocImporter = self.mCommentsDocumentImporter.as_ref();
        }
        // java case ENDNOTE_DOCUMENT:
        4 => {
          // java subDocImporter = this.mEndnoteDocumentImporter;
          // rust: 真类型含 `Deref<Target = FootnoteEndnoteDocumentImporter>` ⇒ 显式降为基类引用。
          subDocImporter = self.mEndnoteDocumentImporter.as_ref().map(|m| &m.super_.super_);
        }
        // java case FOOTNOTE_DOCUMENT:
        1 => {
          // java subDocImporter = this.mFootnoteDocumentImporter;
          // rust: 同上。
          subDocImporter = self.mFootnoteDocumentImporter.as_ref().map(|m| &m.super_.super_);
        }
        // java case HEADERTEXTBOX_DOCUMENT:
        6 => {
          // java subDocImporter = this.mHdrFtrTextboxDocumentImporter;
          // rust: 真类型含 `Deref<Target = SubDocumentImporter>` ⇒ 显式降为基类引用。
          subDocImporter = self.mHdrFtrTextboxDocumentImporter.as_ref().map(|m| &m.super_.super_);
        }
        // java default:
        _ => {
          // java IOAssert.fail("Add more case for sub-document type: " + subDocType);
          IOAssert::fail_msg("Add more case for sub-document type");
          panic!("Add more case for sub-document type: {subDocType}");
        }
      }
      // java return subDocImporter;
      subDocImporter
    }

    // Java: IDmlImporter getDmlImporter(SubDocType). The pointer is the SAX bridge,
    // the same object later callbacks use, and it outlives this borrow of `inner`.
    pub fn open_dml_importer(
      &mut self,
      sub_doc_type: i32,
      document_importer: *const dyn io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter,
      document_importer_ref: Option<Rc<dyn io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter>>,
    ) -> Option<io_wps_io::cn::wps::io::xwpf::IDmlImporter::DmlImporterRef> {
      let theme = match self.mThemeContext.clone() {
        Some(theme) => theme,
        None => {
          let theme = Rc::new(RefCell::new(ThemeContext::default()));
          self.mThemeContext = Some(theme.clone());
          theme
        }
      };
      let xml07 = self.xml07Context().cloned();
      let mode = self.mode();
      let sub = self.getSubDocumentImporter_mut(sub_doc_type)?;
      Some(sub.cache_dml_importer(document_importer, document_importer_ref, theme, xml07, mode))
    }

    // Java: IVmlImporter getVmlImporter(SubDocType).
    pub fn open_vml_importer(
      &mut self,
      sub_doc_type: i32,
    ) -> Option<io_wps_io::cn::wps::io::xwpf::IVmlImporter::VmlImporterRef> {
      let mode = self.mode();
      let sub = self.getSubDocumentImporter_mut(sub_doc_type)?;
      Some(sub.cache_vml_importer(mode))
    }

    pub fn getSubDocumentImporter_mut(&mut self, subDocType: i32) -> Option<&mut SubDocumentImporter> {
      match subDocType {
        0 => self.mMainDocumentImporter.as_mut().map(|m| &mut m.super_),
        5 => self.mTextboxDocumentImporter.as_mut().map(|m| &mut m.super_.super_),
        2 => self.mHdrFtrDocumentImporter.as_mut().map(|m| &mut m.super_),
        3 => self.mCommentsDocumentImporter.as_mut(),
        4 => self.mEndnoteDocumentImporter.as_mut().map(|m| &mut m.super_.super_),
        1 => self.mFootnoteDocumentImporter.as_mut().map(|m| &mut m.super_.super_),
        6 => self.mHdrFtrTextboxDocumentImporter.as_mut().map(|m| &mut m.super_.super_),
        _ => {
          IOAssert::fail_msg("Add more case for sub-document type");
          panic!("Add more case for sub-document type: {subDocType}");
        }
      }
    }
  // java break;

    // java private void appendParagraphBreak4HeaderFooter()
    pub fn appendParagraphBreak4HeaderFooter(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java SubDocument hdrFtrDocument = this.mHdrFtrDocumentImporter.getSubDocument();
      let mut hdrFtrDocument: SubDocument = self.mHdrFtrDocumentImporter.getSubDocument();
      // java IOAssert.assertNotNull("hdrFtrDocument should not be null.", hdrFtrDocument);
      IOAssert::assertNotNull__string_object(&"hdrFtrDocument should not be null.", Some(&hdrFtrDocument));
      // java MutablePropertySet paraProperty = new MutablePropertySet(PropertySet.empty);
      let mut paraProperty: MutablePropertySet = MutablePropertySet::new(PropertySet::empty());
      // java KParagraphsID kParagraphsID = hdrFtrDocument.getTextDocument().getKParagraphsID();
      let mut kParagraphsID: KParagraphsID = hdrFtrDocument.getTextDocument().getKParagraphsID();
      // java String id = kParagraphsID.createId();
      let mut id: String = kParagraphsID.createId();
      // java paraProperty.setValue(282, id);
      paraProperty.setValue(282, id);
      // java kParagraphsID.addParaphId(id);
      kParagraphsID.addParaphId(id);
      // java IPapxTable.Entry entry = CoreHelper.appendParagraph(hdrFtrDocument, '\r', PropertySet.empty, paraProperty.freeze());
      let mut entry: Box<dyn Entry> = CoreHelper::appendParagraph__subdocument_char_propertyset_propertyset(&hdrFtrDocument, '\r', &PropertySet::empty(), &paraProperty.freeze());
      // java kParagraphsID.putId(id, hdrFtrDocument.getType(), entry);
      kParagraphsID.putId(id, hdrFtrDocument.getType(), entry);
      */
    }

    // java private void callback4Progress(IPapxTable.Entry paraBreak)
    pub fn callback4Progress(&mut self, paraBreak: Option<&dyn crate::cn::wps::moffice::writer::data::IPapxTable::Entry>) {
      // java int LAYOUT_FREQUENCY = 3;
      let LAYOUT_FREQUENCY: i32 = 3;
      let _ = LAYOUT_FREQUENCY;
      // java if (null != paraBreak && ++this.mCountParagraph / 3 >= 1 && null != this.mIoListener)
      if paraBreak.is_none() {
        return;
      }
      self.mCountParagraph += 1;
      if self.mCountParagraph / 3 >= 1 {
        if paraBreak.is_some() && self.mIoListener.is_some() {
          // java this.mIoListener.onLoadParas(paraBreak.position());
          let position = if let Some(p) = self.mDocument {
            unsafe { (*p).getMainDocument().map(|m| m.getLength()).unwrap_or(0) }
          } else {
            0
          };
          if let Some(listener) = self.mIoListener.as_mut() {
            listener.onLoadParas(position);
          }
          // java this.mCountParagraph = 0;
          self.mCountParagraph = 0;
        }
      }
    }

    // java private void importDir(SubDocumentImporter subDocImporter, Attributes attributes)
    pub fn importDir(&mut self, subDocImporter: &mut SubDocumentImporter, attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String text = "\u202b";
      let mut text: String = "\u{202b}".to_string();
      // java String type = ConvertHelper.getString(attributes);
      let mut r#type: String = ConvertHelper::getString__attributes(&attributes);
      // java String LTR = "ltr";
      let mut LTR: String = "ltr".to_string();
      // java if (null != type && type.equalsIgnoreCase("ltr"))
      if r#type.is_some() && r#type.equalsIgnoreCase("ltr") {
        // java text = "\u202a";
        text = "\u{202a}";
      }
      // java subDocImporter.importRunTextSimple(text, PropertySet.empty);
      subDocImporter.importRunTextSimple__string_propertyset(&text, &PropertySet::empty());
      */
    }

    // java private void importBdo(SubDocumentImporter subDocImporter, Attributes attributes)
    pub fn importBdo(&mut self, subDocImporter: &mut SubDocumentImporter, attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String text = "\u202d";
      let mut text: String = "\u{202d}".to_string();
      // java String type = ConvertHelper.getString(attributes);
      let mut r#type: String = ConvertHelper::getString__attributes(&attributes);
      // java String LTR = "ltr";
      let mut LTR: String = "ltr".to_string();
      // java if (null != type && type.equalsIgnoreCase("ltr"))
      if r#type.is_some() && r#type.equalsIgnoreCase("ltr") {
        // java text = "\u202e";
        text = "\u{202e}";
      }
      // java subDocImporter.importRunTextSimple(text, PropertySet.empty);
      subDocImporter.importRunTextSimple__string_propertyset(&text, &PropertySet::empty());
      */
    }

    // java private void setFMaybeFitText(MutablePropertySet mutablePropertySet)
    pub fn setFMaybeFitText(&mut self, mutablePropertySet: &MutablePropertySet) {
      // java Object value = mutablePropertySet.getValue(305);
      // java if (value != null)
      if !mutablePropertySet.contains(305) {
        return;
      }
      // java Integer intVal = (Integer)value;
      let intVal = mutablePropertySet.getInt(305);
      let Some(p) = self.mDocument else { return; };
      unsafe {
        // java if (intVal > 0 && !this.mDocument.getProperty().getBoolean(643, false))
        if intVal > 0 && !(*p).getProperty().getBoolean__int_boolean(643, false) {
          // java MutablePropertySet mp = new MutablePropertySet();
          let mut mp = MutablePropertySet::new();
          // java mp.add(this.mDocument.getProperty());
          mp.add(&(*p).getProperty());
          // java mp.setBoolean(643, true);
          mp.setBoolean(643, true);
          // java this.mDocument.setProperty(mp.freeze());
          (*p).setProperty__propertyset(&mp.freeze());
        }
      }
    }

    // java private MutablePropertySet mergeRevisionRunProp(MutablePropertySet mutablePropertySet)
    pub fn mergeRevisionRunProp<'a>(&self, mutablePropertySet: Option<&'a mut MutablePropertySet>) -> Option<&'a mut MutablePropertySet> {
      // java if (null == this.mCurrentRevision)
      if self.mCurrentRevision.is_none() {
        // java return mutablePropertySet;
        return mutablePropertySet;
      }
      todo!("DEFERRED_NON_FUNCTION");
      // java else
      // java if (null == mutablePropertySet)
      // java mutablePropertySet = new MutablePropertySet();
      let mutablePropertySet = mutablePropertySet;
      /* DEFERRED_NON_FUNCTION
      // java int revisionType = this.mCurrentRevision.getMark();
      // java switch (revisionType)
      // java case 1:
      // java if (null == mutablePropertySet.getValue(45))
      // java ConvertHelper.addDelRevision(this.mDocument, mutablePropertySet, this.mCurrentRevision);
      // java case 2:
      // java if (null == mutablePropertySet.getValue(46))
      // java ConvertHelper.addInsRevision(this.mDocument, mutablePropertySet, this.mCurrentRevision);
      // java default:
      // java return mutablePropertySet;
      mutablePropertySet
      */
    }
  // java break;

    // java private boolean isEmpty(String str)
    pub fn isEmpty(&self, str: Option<&str>) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return str == null || str.length() == 0;
      return str.is_none() || (str.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) == 0;
      */
    }

    // java private void importAudio(String url, String notePath)
    pub fn importAudio(&mut self, url: &str, notePath: Option<&str>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (notePath != null)
      if let Some(notePath) = notePath {
        // java String mediaTagPath = this.getMediaTagPath(url, notePath);
        let mut mediaTagPath: String = self.getMediaTagPath(&url, &mut notePath);
        // java SubDocument mainDoc = (SubDocument)this.mDocument.getMainDocument();
        let mut mainDoc: SubDocument = (self.mDocument.getMainDocument()).downcast_ref::<SubDocument>().unwrap();
        // java Shape shape = IOHelper.createShape(mainDoc.getDrawingContainer(), false);
        let mut shape: Shape = IOHelper::createShape__idrawingcontainer_boolean(&mainDoc.getDrawingContainer(), false);
        // java shape.setShapeID(ShapeIDTool.getNewID(mainDoc.getDrawingContainer()));
        shape.setShapeID(ShapeIDTool::getNewID__idrawingcontainer(Some(&mut mainDoc.getDrawingContainer())));
        // java shape.setShapeType(204);
        shape.setShapeType(204);
        // java shape.setInlineShape(true);
        shape.setInlineShape(true);
        // java shape.setRotWithShape(true);
        shape.setRotWithShape(true);
        // java shape.setInDggInfo(false);
        shape.setInDggInfo(false);
        // java ShapePos shapePos = new ShapePos();
        let mut shapePos: ShapePos = ShapePos::new();
        // java RectF rect = this.getSutablePicSize(mediaTagPath);
        let mut rect: RectF = self.getSutablePicSize(&mediaTagPath);
        // java shapePos.setShapeRect(rect);
        shapePos.setShapeRect(rect);
        // java shape.setShapePos(shapePos);
        shape.setShapePos(shapePos);
        // java GRF grf = new GRF();
        let mut grf: GRF = GRF::new();
        // java shape.setGRF(grf);
        shape.setGRF(grf);
        // java int mediaId = this.mDocument.getMediaLib().addMedia(mediaTagPath, mediaTagPath, MediaTypeEnum.PICTURE);
        let mut mediaId: i32 = self.mDocument.getMediaLib().addMedia(mediaTagPath, mediaTagPath, MediaTypeEnum::PICTURE);
        // java Picture pic = new Picture(mediaId);
        let mut pic: Picture = Picture::new(mediaId);
        // java shape.setPicture(pic);
        shape.setPicture(pic);
        // java char ch = (char)19;
        let mut ch: char = todo!("cast → char");
        // java this.appendFieldControlChar(mainDoc, ch, new MutablePropertySet());
        self.appendFieldControlChar(&mainDoc, ch, &MutablePropertySet::new());
        // java String fldCode = " " + KFieldType.FieldEmbed + " " + "Package" + " ";
        let mut fldCode: String = " " + KFieldType::FieldEmbed + " " + "Package" + " ";
        // java CoreHelper.appendText(mainDoc, fldCode, PropertySet.empty);
        CoreHelper::appendText__subdocument_char_propertyset(&mainDoc, fldCode, &PropertySet::empty());
        // java ch = (char)20;
        ch = todo!("cast → char");
        // java MutablePropertySet prop = this.prepareSepProp(mainDoc, url, "Package");
        let mut prop: MutablePropertySet = self.prepareSepProp(&mainDoc, &url, &"Package");
        // java this.appendFieldControlChar(mainDoc, ch, prop);
        self.appendFieldControlChar(&mainDoc, ch, &prop);
        // java ch = (char)(shape.isInDggInfo() ? 8 : 1);
        ch = todo!("cast → char");
        // java int cp = IOHelper.appendControlChar(mainDoc, ch, new MutablePropertySet());
        let mut cp: i32 = IOHelper::appendControlChar(&mainDoc, ch, &mut MutablePropertySet::new());
        // java ch = (char)21;
        ch = todo!("cast → char");
        // java this.appendFieldControlChar(mainDoc, ch, new MutablePropertySet());
        self.appendFieldControlChar(&mainDoc, ch, &MutablePropertySet::new());
        // java IOHelper.addTopLevelShape(mainDoc, cp, shape);
        IOHelper::addTopLevelShape(&mainDoc, cp, &shape);
      }
      */
    }

    // java private String getMediaTagPath(String url, String notePath)
    pub fn getMediaTagPath(&self, url: &str, notePath: &mut str) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java File urlFile = new File(url);
      let mut urlFile: std::path::PathBuf = File::new(url);
      // java String urlName = urlFile.getName();
      let mut urlName: String = urlFile.getName();
      // java String tagPath = notePath.substring(0, notePath.lastIndexOf(File.separator) + 1) + urlName + ".png";
      let mut tagPath: String = notePath.substring(0, notePath.lastIndexOf(File::separator) + 1) + urlName + ".png";
      // java File tagFile = new File(tagPath);
      let mut tagFile: std::path::PathBuf = File::new(tagPath);
      // java if (tagFile.exists())
      if tagFile.exists() {
        // java return tagFile.getAbsolutePath();
        return tagFile.getAbsolutePath();
      }
      // java else
      else {
        // java File file = Platform.createTempFile("audio", ".wav");
        let mut file: std::path::PathBuf = Platform::createTempFile__string_string(&"audio", &".wav");
        // java String newPath = file.getAbsolutePath();
        let mut newPath: String = file.getAbsolutePath();
        // java if (null != newPath)
        if newPath.is_some() {
          // java FileUtil.copyFile(url, newPath);
          FileUtil::copyFile__string_string(Some(&url), Some(&newPath));
        }
        // java file = Platform.createTempFile("audioPic", ".emf");
        file = Platform::createTempFile__string_string(&"audioPic", &".emf");
        // java FileUtil.writeToFile(file.getAbsolutePath(), readAudioPicFile());
        FileUtil::writeToFile__string_inputstream(&file.getAbsolutePath(), &Self::readAudioPicFile());
        // java return file.getAbsolutePath();
        return file.getAbsolutePath();
      }
      */
    }

    // java private void appendFieldControlChar(SubDocument document, char fldCh, MutablePropertySet props)
    pub fn appendFieldControlChar(&mut self, document: &SubDocument, fldCh: char, props: &MutablePropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int cp = IOHelper.appendControlChar(document, fldCh, props);
      let mut cp: i32 = IOHelper::appendControlChar(&document, fldCh, &mut props);
      // java PLCField plcField = document.getPLCField();
      let mut plcField: PLCField = document.getPLCField();
      // java PLCField.FieldNode fieldNode = plcField.addFieldChar(cp);
      let mut fieldNode: PLCField::FieldNode = plcField.addFieldChar(cp);
      // java fieldNode.setFieldChar(fldCh);
      fieldNode.setFieldChar(fldCh);
      // java if (19 == fldCh)
      if 19 == fldCh {
        // java fieldNode.mData = KFieldType.FieldEmbed.getFlt();
        fieldNode.mData = KFieldType::FieldEmbed.getFlt();
      }
      // java else if (21 == fldCh)
      else if 21 == fldCh {
        // java fieldNode.setFieldFlags(KRange.fieldFlags(fieldNode));
        fieldNode.setFieldFlags(KRange::fieldFlags(&fieldNode));
      }
      */
    }

    // java private MutablePropertySet prepareSepProp(SubDocument document, String fileName, String progId)
    pub fn prepareSepProp(&mut self, document: &SubDocument, fileName: &str, progId: &str) -> MutablePropertySet {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java MutablePropertySet prop = new MutablePropertySet();
      let mut prop: MutablePropertySet = MutablePropertySet::new();
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java prop.setBoolean(30, true);
      prop.setBoolean(30, true);
      // java prop.setBoolean(29, true);
      prop.setBoolean(29, true);
      // java OLE ole = this.createOLE(fileName, document, progId);
      let mut ole: OLE = self.createOLE(&fileName, &document, &progId);
      // java IDrawingContainer drawingContainer = document.getDrawingContainer();
      let mut drawingContainer: Box<dyn IDrawingContainer> = document.getDrawingContainer();
      // java Shape shape = ShapeFactory.getShapeFactory().createShape(drawingContainer);
      let mut shape: Shape = ShapeFactory::getShapeFactory().createShape(drawingContainer);
      // java shape.setShapeID(drawingContainer.newShapeID());
      shape.setShapeID(drawingContainer.newShapeID());
      // java shape.setOLE(ole);
      shape.setOLE(ole);
      // java prop.setValue(56, shape);
      prop.setValue(56, shape);
      // java return prop;
      return prop;
      */
    }

    // java private OLE createOLE(String fileName, Document document, String progId)
    pub fn createOLE(&mut self, fileName: &str, document: &dyn Document, progId: &str) -> OLE {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IDrawingContainer container = document.getDrawingContainer();
      let mut container: Box<dyn IDrawingContainer> = document.getDrawingContainer();
      // java IMediaLib mediaLib = container.getMediaLib();
      let mut mediaLib: Box<dyn IMediaLib> = container.getMediaLib();
      // java int mediaId = mediaLib.addMedia(fileName, fileName, MediaTypeEnum.OLE);
      let mut mediaId: i32 = mediaLib.addMedia(fileName, fileName, MediaTypeEnum::OLE);
      // java OLE ole;
      let mut ole: OLE = OLE::default();
      // java if (VersionState.isProVersion())
      if VersionState::isProVersion() {
        // java ole = new OLE(mediaId, mediaLib);
        ole = OLE::new(mediaId, mediaLib);
      }
      // java else
      else {
        // java ole = new OLE(mediaId);
        ole = OLE::new(mediaId);
      }
      // java ole.setProgId(progId);
      ole.setProgId(progId);
      // java return ole;
      return ole;
      */
    }

    // java private void importPicture(String url)
    pub fn importPicture(&self, url: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java SubDocument mainDoc = (SubDocument)this.mDocument.getMainDocument();
      let mut mainDoc: SubDocument = (self.mDocument.getMainDocument()).downcast_ref::<SubDocument>().unwrap();
      // java Shape shape = IOHelper.createShape(mainDoc.getDrawingContainer(), false);
      let mut shape: Shape = IOHelper::createShape__idrawingcontainer_boolean(&mainDoc.getDrawingContainer(), false);
      // java shape.setShapeID(ShapeIDTool.getNewID(mainDoc.getDrawingContainer()));
      shape.setShapeID(ShapeIDTool::getNewID__idrawingcontainer(Some(&mut mainDoc.getDrawingContainer())));
      // java shape.setIsFromDml(true);
      shape.setIsFromDml(true);
      // java GRF grf = new GRF();
      let mut grf: GRF = GRF::new();
      // java grf.setChild(false);
      grf.setChild(false);
      // java grf.setGroup(false);
      grf.setGroup(false);
      // java grf.setHaveSpt(true);
      grf.setHaveSpt(true);
      // java shape.setGRF(grf);
      shape.setGRF(grf);
      // java shape.setInDggInfo(false);
      shape.setInDggInfo(false);
      // java shape.setInlineShape(true);
      shape.setInlineShape(true);
      // java shape.setShapeType(204);
      shape.setShapeType(204);
      // java shape.setInlineShape(true);
      shape.setInlineShape(true);
      // java shape.setAnchorLock(true);
      shape.setAnchorLock(true);
      // java ShapePos pos = new ShapePos();
      let mut pos: ShapePos = ShapePos::new();
      // java pos.setScaleH(1.0F);
      pos.setScaleH(1);
      // java pos.setScaleV(1.0F);
      pos.setScaleV(1);
      // java PictureParser.rotateBitmapPathIfNeeded(url);
      PictureParser::rotateBitmapPathIfNeeded(&url);
      // java RectF rectf = this.getSutablePicSize(url);
      let mut rectf: RectF = self.getSutablePicSize(&url);
      // java pos.setShapeRect(rectf);
      pos.setShapeRect(rectf);
      // java shape.setShapePos(pos);
      shape.setShapePos(pos);
      // java shape.setIsVml(false);
      shape.setIsVml(false);
      // java int mediaId = this.mDocument.getMediaLib().addMedia(url, url, MediaTypeEnum.PICTURE);
      let mut mediaId: i32 = self.mDocument.getMediaLib().addMedia(url, url, MediaTypeEnum::PICTURE);
      // java Picture pic = new Picture(mediaId);
      let mut pic: Picture = Picture::new(mediaId);
      // java shape.setPicture(pic);
      shape.setPicture(pic);
      // java char ch = (char)(shape.isInDggInfo() ? 8 : 1);
      let mut ch: char = todo!("cast → char");
      // java int cp = IOHelper.appendControlChar(mainDoc, ch, new MutablePropertySet());
      let mut cp: i32 = IOHelper::appendControlChar(&mainDoc, ch, &mut MutablePropertySet::new());
      // java IOHelper.addTopLevelShape(mainDoc, cp, shape);
      IOHelper::addTopLevelShape(&mainDoc, cp, &shape);
      */
    }

    // java private RectF getSutablePicSize(String filePath)
    pub fn getSutablePicSize(&self, filePath: &str) -> RectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IPicturePool picturePool = Platform.getPicturePool();
      let mut picturePool: Box<dyn IPicturePool> = Platform::getPicturePool();
      // java PictureInfo info = picturePool.getInfoFromPool(filePath);
      let mut info: PictureInfo = picturePool.getInfoFromPool(filePath);
      // java RectF rectF = new RectF(0.0F, 0.0F, 100.0F, 100.0F);
      let mut rectF: RectF = RectF::new(0, 0, 100, 100);
      // java if (info == null)
      if info.is_none() {
        // java return rectF;
        return rectF;
      }
      // java else
      else {
        // java PageSetup defaultPageSetup = new PageSetup(11906, 16838, 1800, 1800, 1440, 1440);
        let mut defaultPageSetup: PageSetup = PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440);
        // java PointF size = ShapeTool.getSuitablePicSize(defaultPageSetup, (float)info.getWidth(), (float)info.getHeight());
        let mut size: PointF = ShapeTool::getSuitablePicSize__pagesetup_float_float(&defaultPageSetup, ((info.getWidth()) as f32), ((info.getHeight()) as f32));
        // java rectF.left = 0.0F;
        rectF.left = 0;
        // java rectF.right = size.x;
        rectF.right = size.x;
        // java rectF.top = 0.0F;
        rectF.top = 0;
        // java rectF.bottom = size.y;
        rectF.bottom = size.y;
        // java return rectF;
        return rectF;
      }
      */
    }

    // java public static byte[] readAudioPicFile()
    pub fn readAudioPicFile() -> Option<Vec<u8>> {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION
          // java InputStream ins = Platform.getAssetManager().open("audio.emf");
          let mut ins: Box<dyn std::io::Read> = Platform::getAssetManager().open("audio.emf");
          // java byte[] data = new byte[ins.available()];
          let mut data: Vec<u8> = vec![Default::default(); (ins.available()) as usize];
          // java ins.read(data);
          ins.read(data);
          // java ins.close();
          ins.close();
          // java return data;
          return data;
          // java catch (IOException e)
          todo!();
            // java e.printStackTrace();
            e.printStackTrace();
            // java return null;
            return None;
          */
    }

}

impl std::ops::Deref for DocumentImporter {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
