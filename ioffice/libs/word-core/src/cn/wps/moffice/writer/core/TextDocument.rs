// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.base.io.ControlChar
use util::base::io::ControlChar::*;
// Java: import cn.wps.base.io.mhtReader.IMhtReader
use util_adapter::cn::wps::base::io::mhtReader::IMhtReader::*;
// Java: import cn.wps.base.io.mhtReader.MhtReflectionHelper
use util_adapter::cn::wps::base::io::mhtReader::MhtReflectionHelper::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::*;
// Java: import cn.wps.collections.IShareObjIndexdMap
use util::collections::IShareObjIndexdMap::*;
// Java: import cn.wps.collections.KArrayDeque
use util::collections::KArrayDeque::*;
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.collections.textrope.KTextPool
use crate::cn::wps::collections::textrope::KTextPool::*;
use std::cell::RefCell;
use std::rc::Rc;
// Java: import cn.wps.core.runtime.IClassLoaderManager
use util::runtime::IClassLoaderManager::*;
// Java: import cn.wps.core.runtime.ILogFileObserver
use util::runtime::ILogFileObserver::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.crypt.provider.ProviderInfo
use io_poi_compact::cn::wps::crypt::provider::ProviderInfo::*;
// Java: import cn.wps.graphics.Bitmap
use util::graphics::bitmap::*;
// Java: import cn.wps.io.file.EncryptedType
use core_libs_libfileformat::cn::wps::io::file::EncryptedType::*;
// Java: import cn.wps.io.file.FileFormatEnum
use util::io_file::FileFormatEnum::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
use io_wps_io::io::xwpf::usermodel::XWPFDocument::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use io_wps_io::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
use util::kfc::api::procedure::IObjectProcedure::*;
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.log.KmoLog
use util::log::KmoLog::*;
// Java: import cn.wps.moffice.crash.FileDamagedException
use util::moffice::crash::FileDamagedException::*;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use util::moffice::crash::NoSpaceLeftException::*;
// Java: import cn.wps.moffice.drawing.DrawingRIDMap
use drawing::cn::wps::moffice::drawing::DrawingRIDMap::*;
// Java: import cn.wps.moffice.drawing.IHostApp
use drawing::cn::wps::moffice::drawing::IHostApp::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.ShapeIDAlloc
use drawing::cn::wps::moffice::drawing::ShapeIDAlloc::*;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use drawing::cn::wps::moffice::drawing::graphics::FillBase::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaLib
use drawing::cn::wps::moffice::drawing::media::MediaLib::*;
// Java: import cn.wps.moffice.io.ContentLockedException
use util_adapter::cn::wps::moffice::io::ContentLockedException::*;
// Java: import cn.wps.moffice.io.ForceQuitException
use util_adapter::cn::wps::moffice::io::ForceQuitException::*;
// Java: import cn.wps.moffice.io.IDLProvider
use util_adapter::cn::wps::moffice::io::IDLProvider::*;
// Java: import cn.wps.moffice.io.IDocProtection
use util_adapter::cn::wps::moffice::io::IDocProtection::*;
// Java: import cn.wps.moffice.io.IDocument
use util_adapter::cn::wps::moffice::io::IDocument::*;
// Java: import cn.wps.moffice.io.IKmoBookProtect
use util_adapter::cn::wps::moffice::io::IKmoBookProtect::*;
// Java: import cn.wps.moffice.io.ITextOpenDocument
use util_adapter::cn::wps::moffice::io::ITextOpenDocument::*;
// Java: import cn.wps.moffice.io.ReadingLimitException
use util_adapter::cn::wps::moffice::io::ReadingLimitException::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import cn.wps.moffice.online.security.OnlineSecurityTool
use io_poi_compact::cn::wps::moffice::online::security::OnlineSecurityTool::*;
// Java: import cn.wps.moffice.online.security.SecurityMode
use util_adapter::cn::wps::moffice::online::security::SecurityMode::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertyException
use util::moffice::property::PropertyException::*;
// Java: import cn.wps.moffice.property.PropertyMap
use util::moffice::property::PropertyMap::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Action
use util::moffice::transaction::Action::*;
// Java: import cn.wps.moffice.transaction.ActionList
use util::moffice::transaction::ActionList::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomCommand
use util::moffice::transaction::atom_reflect::{
    register_atom_layout, AtomFieldMeta, AtomMemberPolicy,
};
use util::moffice::transaction::AtomCommand::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.transaction.CommandBase
use util::moffice::transaction::CommandBase::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.transaction.TransactionListeners
use util::moffice::transaction::TransactionListeners::*;
// Java: import cn.wps.moffice.transaction.Action.ActionType
// Java: import cn.wps.moffice.util.ArtMode
use util::moffice::util::ArtMode::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.util.StatAnalytics
use util::moffice::util::StatAnalytics::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.util.UUIDUtil
use util::moffice::util::UUIDUtil::*;
// Java: import cn.wps.moffice.writer.VersionState
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KExchange
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KFootnoteEndnote
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.helper.HelperHeaderFooter
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.DocumentPreliminaryLoader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.ICoreDataAcceptor
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.IDocumentCallback
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.IOThreadSign
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.io.TextCoreDataAcceptor
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.list.ListParagraphs
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.list.impl.KListTemplates
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.list.impl.KLists
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.metadata.CustomMetadata
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.metadata.KCustomIcv
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.metadata.Metadata
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.metadata.PIDSI
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.pool.ShareObjIndexedMapFactory
// 10 deferred — not on compile surface
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.protection.DocumentProtectionException
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.protection.KDocumentProtection
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.protection.ProtectionType
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.sectionProp.ISectionProperty
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.sectionProp.KSectionProperty
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.KShapeRange
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.PictureDumpMap
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.command.KShapeCommand
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.smartart.SmartartHostApp
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.DefaultSectionProperties
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.DocumentProtection
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry, Listener as ChpxListener};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry, Listener as PapxListener};
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::*;
// Java: import cn.wps.moffice.writer.data.ITextStream
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCField
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCHeader
use crate::cn::wps::moffice::writer::data::PLCHeader::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PropertyOperations
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.UnknownDataMap
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.field.KFieldType
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.internal_v2.CPIterator
use crate::cn::wps::moffice::writer::data::internal_v2::CPIterator::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridResizeRequest
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.theme.Theme
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.BRC
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.CellHideMark
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.CellMargin
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.CellSpacing
// 10 deferred — not on compile surface
use crate::cn::wps::moffice::writer::data::value::CompatSetting::CompatSetting;
use crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode;
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.DxaColumn
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.FileFormat
use util::moffice::writer::data::value::FileFormat::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.PositionCode
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.Revision
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.TableDefSHDBuilder
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.TableLength
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.value.TableLookBuilder
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.drawing.ShapeFactoryDoc
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.event.KmoWriterOpenEventListener
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.event.KmoWriterOpenEventListenerGroup
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.function.card.online.PaperKeyWordParam
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.function.card.util.PaperRecognition
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.insert.InsertEnv
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.insert.InsertFile
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.WriterAutoWriteException
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.WriterIOException
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.docReader.DocReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.docReader.corrector.DocumentCorrector
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IDocxReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IEbookReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IReaderBase
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IUot3Reader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IXml03Reader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.face.IXml07Reader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.html.IHtmlReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.rtf.IRtfReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.textReader.TextReader
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reflection.ReflectionHelper
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.tools.IFormatConverter
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.writer.WordWriter
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.view.ITextEditor
// 10 deferred — not on compile surface
// Java: import cn.wps.poi.drawing.KPictureRegistry
use util::cn::wps::poi::drawing::KPictureRegistry::*;
// Java: import cn.wps.util.KLogFileObserverUtil
use util::util::KLogFileObserverUtil::*;
// Java: import cn.wps.util.concurrent.locks.RangeLock
// 10 deferred — not on compile surface
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
// 10 deferred — not on compile surface
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
// rust: ★M-STAGE2 P1-D1 —— 就地影子 `RangeWriteLock` 已删；真实模块
//   （`cn/wps/util/concurrent/locks/RangeWriteLock.rs`）本批已真实登记并编译。
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::RangeWriteLock;
// 10 deferred — not on compile surface
// Java: import cn.wps.util.manager.FileManager
use util::util::manager::FileManager::*;
// Java: import cn.wps.vm.KSwappableTextPool
// 10 deferred — not on compile surface
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.lang.reflect.Constructor
use stubs::java::lang::reflect::Constructor::*;
// Java: import java.lang.reflect.InvocationTargetException
use stubs::java::lang::reflect::InvocationTargetException::*;
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import java.util.Stack
use stubs::java::util::Stack::*;
// Java: import org.apache.poi.hwpf.HWPFDocument
use io_poi_compact::org::apache::poi::hwpf::HWPFDocument::*;
// Java: import org.apache.poi.hwpf.ole.OleInfo
use io_poi_compact::org::apache::poi::hwpf::ole::OleInfo::*;
// Java: import org.apache.poi.txt.TXTDocument
use io_poi_compact::org::apache::poi::txt::TXTDocument::*;
// Java: import org.apache.poi.util.IntListPool
use util::poi_util::IntListPool::*;
// Java: import cn.wps.moffice.writer.core.UUSti
use crate::cn::wps::moffice::writer::core::UUSti::*;
// Java: import cn.wps.moffice.writer.core.UUCommentId
use crate::cn::wps::moffice::writer::core::UUCommentId::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.FontTable
// rust: ★M-STAGE1 P2-2 —— 由 stand-in 改为**真实类型**（`core/FontTable.rs`，已登记）。
use crate::cn::wps::moffice::writer::core::FontTable::*;
// Java: import cn.wps.moffice.writer.core.ModifyFlag
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.AsiaTypography
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.DocumentLoadListener
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.OleManager
// CARGO_BLOCKER_UNUSED_MODULE: OleManager not 0t→6; soft holder in TextDocument
// 10 deferred — not on compile surface
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import java.lang.ClassLoader
use stubs::java::lang::ClassLoader::*;
// Java: import cn.wps.moffice.writer.core.FileTime
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.DocumentVariables
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.WdCompatibilityMode
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.KDocumentWriteProtection
// 10 deferred — not on compile surface
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.IDebugger
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.KParagraphsID
// rust: ★M-STAGE2 P1-D1 —— 真实模块（已登记 lib.rs），解除此前的 "deferred" 影子。
use crate::cn::wps::moffice::writer::core::KParagraphsID::KParagraphsID;
// Java: import cn.wps.moffice.writer.core.SaveAccelerator
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry as _;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.io.dom.Node
use io_wps_io::io::dom::Node as DomNode;
// Java: import cn.wps.moffice.writer.core.ErrorCollector
use crate::cn::wps::moffice::writer::core::ErrorCollector::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.DefaultModelEvent
// 10 deferred — not on compile surface
// Java: import java.lang.Class
use stubs::java::lang::Class::*;
// Java: import cn.wps.moffice.writer.core.DocumentEventDispatcher
use crate::cn::wps::moffice::writer::core::DocumentEventDispatcher::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::KStyle;
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as ParentTextDocument;
// Java: import INotifyTextRope.Listener
use crate::cn::wps::collections::textrope::INotifyTextRope::Listener;
// Java: import cn.wps.moffice.writer.core.SelectionStore
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.Range
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.KSections
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import cn.wps.moffice.writer.core.CRefreshCommand
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.InnerIOListener
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.CUpdateCommand
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.WriterModelException
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.StyleListener4PapxTable
// 10 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.StyleListener4ChpxTable
// 10 deferred — not on compile surface
use std::collections::{HashMap};

// java private static final String RTFREADER_CLASS = "cn.wps.moffice.writer.io.reader.rtf.RtfReader";
const RTFREADER_CLASS: &str = "cn.wps.moffice.writer.io.reader.rtf.RtfReader";
// java private static final String TAG = null;
const TAG: &str = "";
// java private static final String TEMPLATE_PRO_PATH = "template/pro/secdoctemplate.doc";
const TEMPLATE_PRO_PATH: &str = "template/pro/secdoctemplate.doc";
// java private static final int FIRST_PAGE_PARAGRAGH = 30;
const FIRST_PAGE_PARAGRAGH: i32 = 30;

static TEXT_DOCUMENT_ATOM_LAYOUT: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    register_atom_layout(
        std::any::TypeId::of::<TextDocument>(),
        vec![
            AtomFieldMeta::new::<PropertySet>(AtomMemberPolicy::Share),
            AtomFieldMeta::new::<ModifyFlag>(AtomMemberPolicy::Share),
            AtomFieldMeta::new::<FileFormat>(AtomMemberPolicy::Share),
        ],
    );
});

// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— **伪造接缝已消除**。
//   `SoftOleManager` 是模板产物虚构的类型名（**全 Java 树 0 命中**）；
//   `TextDocument.java:217 private OleManager mOleManager;` 的真实类型是
//   `cn.wps.moffice.writer.core.OleManager`（`OleManager.java` 653 行 / 38 方法，
//   与本文件同目录）。真实模块已由 `core/.codeTrees/OleManager.rust.json`
//   （1019 行 / 364 `// java`）物化并登记于 `lib.rs` ⇒ 此处改为真实导入。
use crate::cn::wps::moffice::writer::core::OleManager::OleManager;

// rust: ★M-STAGE2 P1-D1 —— 2 个就地影子已删，改为引用**已登记的真实模块**：
//   `IReaderBase`（`io/reader/face/`，3 方法）/ `IDocxReader`（同目录，1 方法）。
//   用法仅为 `&dyn` 参数类型，且所在函数体为 `todo!` ⇒ 零语义风险。
use crate::cn::wps::moffice::writer::io::reader::face::IReaderBase::IReaderBase;
use crate::cn::wps::moffice::writer::io::reader::face::IDocxReader::IDocxReader;
// 10: compile-surface stand-ins for types not exported this function.
use crate::cn::wps::moffice::writer::core::ModifyFlag::ModifyFlag;
// rust: ★T7 —— 就地影子 `Metadata` 已删（**冗余遮蔽**）：真实类型
//   `core::metadata::Metadata` 已登记（`core/metadata/mod.rs:3`），
//   该空壳结构体反而遮蔽真实类型，导致 `SoftTextDocument::getMetadata`
//   返回类型不符（E0308）。改用真实类型并补导入。
// Java: import cn.wps.moffice.writer.core.metadata.Metadata
use crate::cn::wps::moffice::writer::core::metadata::Metadata::Metadata;
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `FileFormat`。
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `RandomAccessFile`。
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `OnlineSecurityTool`。
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`view/ITextEditor.rs`，24 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::view::ITextEditor::ITextEditor;
use crate::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`core/DocumentLoadListener.rs`，9 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::core::DocumentLoadListener::DocumentLoadListener;
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `ProviderInfo`。
// rust: ★M-STAGE2 P1-D1 / T6 —— **就地影子已消除**，改用真实 trait。
//   真实 `core/io/ICoreDataAcceptor.rs`（8 方法）已登记；其依赖链
//   `autonum::{LFO, LfoLvlData, LSTF, LVLF, LFOLVL, LvlfData, LfoData, LstData}`
//   的 119 处模板缺陷已全部修复（见里程碑 §9）。
use crate::cn::wps::moffice::writer::core::io::ICoreDataAcceptor::ICoreDataAcceptor;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`data/UnknownDataMap.rs`，271 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::data::UnknownDataMap::UnknownDataMap;
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `Date`。
use crate::cn::wps::moffice::writer::core::FileTime::FileTime;
use crate::cn::wps::moffice::writer::core::DocumentVariables::DocumentVariables;
// M-STAGE2 S2-2: 原先此处是影子 `#[derive(Default)] pub struct KDocumentProtection;`
//   ⇒ 遮蔽了真实类型 `core::protection::KDocumentProtection`，使 `protection` 字段
//   永远只能装空壳（D 类假装配）。真实类型已登记（lib.rs:72）且方法齐全
//   （`getProtectionType`/`isProtect`/`isProtectOn`），故改用真实类型。
use crate::cn::wps::moffice::writer::core::protection::KDocumentProtection::KDocumentProtection;
use crate::cn::wps::moffice::writer::core::protection::ProtectionType::ProtectionType;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// M-STAGE2 S2-2: 原先此处是影子 `#[derive(Default)] pub struct Theme;`
//   ⇒ 遮蔽了真实类型 `data::theme::Theme`（lib.rs:418 已登记，含 `getThemeFontScheme_mut`）。
//   `DocumentImporter` 构造 `ThemeImporter` 时需要真实 `Theme` 的裸指针 ⇒ 必须用真实类型。
use crate::cn::wps::moffice::writer::data::theme::Theme::Theme;
// M-STAGE: WdCompatibilityMode 真实枚举在 core/WdCompatibilityMode.rs（lib.rs 已登记）。
//   原先 `#[derive(Default)] pub struct WdCompatibilityMode` 是空壳，布局读
//   isTableIndentedByOldStyle / isBefore2013Mode 时没有 Word 2010/2013 语义。
use crate::cn::wps::moffice::writer::core::pool::ShareObjIndexedMapImpl::ShareObjIndexedMapImpl;
// rust: ★T7-8 —— 以下两处仍是**就地影子**（假编译面），按「非管线核心可合法延迟」规则
//   **显式标注并保留**，不再作为隐式假面。真实归属与阻塞证据见
//   `core/footnoteendnote/` 与 `core/mod.rs` 的 T7-8 探测记录。
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KExchange
// DEFERRED_NON_FUNCTION: 脚注/尾注交换（KExchange 800 行）；登记后 139 错误，
//   前置是 `core::Document` trait 的 28 个 `Option<*mut ()>` 门面真实化（跨模块契约变更）。
#[derive(Default)] pub struct KExchange;
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KFootnoteEndnote
// DEFERRED_NON_FUNCTION: 脚注/尾注设置（KFootnoteEndnote 521 行）；同上，且需
//   `data::table::CellInfo`（`data::table` 子树整体延后）。
#[derive(Default)] pub struct KFootnoteEndnote;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`core/sectionProp/ISectionProperty.rs`，42 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::core::sectionProp::ISectionProperty::ISectionProperty;
// rust: ★T7-8 —— 就地影子 `#[derive(Default)] pub struct KCustomIcv;` 已删（**冗余遮蔽**）。
//   Java: `import cn.wps.moffice.writer.core.metadata.KCustomIcv`（本文件第 168 行注释）；
//   真实模块 `core/metadata/KCustomIcv.rs`（187 行）随 `core::metadata` 已登记并编译通过
//   （`core/mod.rs:74 pub mod metadata;`）⇒ 影子只把真实类型遮蔽成空壳（D 类假装配）。
use crate::cn::wps::moffice::writer::core::metadata::KCustomIcv::KCustomIcv;
use crate::cn::wps::moffice::writer::core::KDocumentWriteProtection::KDocumentWriteProtection;
// Java: import cn.wps.moffice.writer.function.card.util.PaperRecognition
// DEFERRED_NON_FUNCTION: WPS「拍照/纸质文档识别」子系统（`PaperRecognition.rs` 557 行）。
//   依赖 `function::card::{CardPaper, util::CollectCardInfo}`，其中 `CollectCardInfo`
//   是**纯自动生成桩**（16 行、全 `todo!()`，且含语法错误）⇒ 登记无收益。
//   不参与 docx 读→排版→渲染管线 ⇒ 合法延迟。
#[derive(Default)] pub struct PaperRecognition;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`core/io/IDocumentCallback.rs`，18 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::core::io::IDocumentCallback::IDocumentCallback;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`core/IDebugger.rs`，11 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::core::IDebugger::IDebugger;
// rust: ★T7-5a-3 —— 就地影子已删（**冗余遮蔽**）：真实类型早由本文件上方 glob 导入，
//   局部声明反而把它们遮蔽成空壳，并在别处引发 `ambiguous` 冲突。
//   已删除影子 `FileManager`。
use crate::cn::wps::moffice::writer::event::KmoWriterOpenEventListenerGroup::KmoWriterOpenEventListenerGroup;
// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除（根因同 `Theme` / `KDocumentProtection`）：
//   此处原有一个**影子** `KParagraphsID { ids: HashSet<String> }`，只有
//   `addParaphId` / `putId` / `contains_id` 三个自造方法，且 `putId` 把
//   `documentType` / `entry` 直接丢弃（假装配）。
//   真实模块 `core/KParagraphsID.rs`（151 行 / 8 `// java` / Java 全量方法）
//   此前从未在 lib.rs 登记 ⇒ 根本不编译。
//   现改为登记真实模块并在此引用，影子删除。
// Java: import cn.wps.moffice.writer.core.list.impl.KListTemplates
// DEFERRED_NON_FUNCTION: 列表模板（`core/list/impl/KListTemplates.rs` 261 行）。
//   登记前置链：`core::PropertyTool`(1121) + `core::KFont` + `core::property` 块 +
//   `list::gallery` 子树；实测整条闭包 358 错误，且 100% 位于**孤立子图**
//   （`list`/`PropertyTool`/`KFont` 均无已登记引用者，docx 语料 `<w:numPr>` 计数为 0）
//   ⇒ 服务编号子系统，不参与版面/渲染 ⇒ 合法延迟（见 09-subplan §23）。
#[derive(Default)] pub struct KListTemplates;
// Java: import cn.wps.moffice.writer.core.list.impl.KLists
// DEFERRED_NON_FUNCTION: 列表集合（`core/list/impl/KLists.rs` 240 行）；同上，
//   另需按「回指用裸指针」约定把 `mTextDocument: Option<TextDocument>` 改为
//   `Option<*mut TextDocument>`（否则 `E0072` 无限大小）。
#[derive(Default)] pub struct KLists;
use crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::GridResizeRequest;
use crate::cn::wps::moffice::writer::core::SaveAccelerator::SaveAccelerator;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`drawing/ShapeFactoryDoc.rs`，45 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::drawing::ShapeFactoryDoc::ShapeFactoryDoc;
// rust: ★M-STAGE2 P1-D1 / T3b —— 就地影子已删；真实模块
//   `util/moffice/property/PropertyMap.rs`（325 行）已由第 102 行 `use` 导入，
//   其静态字段访问器 `set_sUseSmallMap` 已按 Java 的 public static 门面补齐。

// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除（第三处 IOListener 影子）。
//   此处原有 `pub trait IOListener {}`（**空 trait**），而真实模块
//   `core/io/IOListener.rs`（9 个方法）已登记 ⇒ 删除影子，改由真实模块提供。
use crate::cn::wps::moffice::writer::core::io::IOThreadSign::IOThreadSign;
// Java: import cn.wps.moffice.writer.core.io.DocumentPreliminaryLoader
// DEFERRED_NON_FUNCTION: 预读加载器（`core/io/DocumentPreliminaryLoader.rs` 852 行）。
//   非 docx 管线（`.doc`/TXT/HTML/MHT/EPUB 预读）；登记后 165 个模板缺陷
//   ⇒ 属用户明示可延迟的非管线逻辑。
#[derive(Default)] pub struct DocumentPreliminaryLoader;
// rust: ★T7-5a —— `DocReader` 真实模块（92 行）**依赖未移植的 596 行 Java `.doc` 导入器**
//   `docReader::importer::DocumentImporter`（当前仅有自动生成 stub，全 `todo!()`），
//   属**非 docx 管线**（二进制 .doc 读取路径）⇒ 按用户规则暂留编译面，待该子系统移植后接入。
// Java: import cn.wps.moffice.writer.io.reader.docReader.DocReader
// DEFERRED_NON_FUNCTION: 二进制 `.doc` 读取器；前置缺席（596 行导入器未移植）。
#[derive(Default)] pub struct DocReader;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`io/reader/rtf/IRtfReader.rs`，6 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::io::reader::rtf::IRtfReader::IRtfReader;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`io/reader/face/IEbookReader.rs`，9 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::io::reader::face::IEbookReader::IEbookReader;
// rust: ★T7-5a-2 —— `io/reader/textReader/TextReader.rs`（280 行，`.txt`/HTML/MHT/EPUB 读取）
//   撤回登记：修掉 3 个门面缺陷（`Option<Document>`→`Box<dyn Document>`、`Box<dyn IOThreadSign>`→值语义、
//   `data::DefaultSectionProperties` 依赖）后暴露 47 个"Java 引用 vs Rust 所有权"错误 ——
//   Java `private TextDocument mDocument` / `private Document mMainDocument` 是**别名**，
//   模板生成为 `Option<TextDocument>`（按值复制）与 `Option<Box<dyn Document>>`（无法持有借用）
//   ⇒ 需按"所有者按值 / 回指用裸指针"约定重新建模，非机械修复。
//   该读取器**不参与** docx 渲染管线（docx 走 `io::reader::docx*`）⇒ 属可 defer 的非管线逻辑。
// Java: import cn.wps.moffice.writer.io.reader.textReader.TextReader
// DEFERRED_NON_FUNCTION: `.txt`/HTML/MHT/EPUB 读取器；需「Java 引用 vs Rust 所有权」重建模。
#[derive(Default)] pub struct TextReader;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`core/list/ListParagraphs.rs`，13 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::core::list::ListParagraphs::ListParagraphs;
// rust: ★T7-5a-2 —— `io/writer/WordWriter.rs`（579 行，docx **写出**路径）撤回登记：
//   登记后暴露 30 错误，级联需 6 个未登记模块
//   （`io::WriterAutoWriteException`、`io::WriterIOStatus`、`io::reader::face::{IDocxWriter,IUot3Writer,IXml03Writer,IXml07Writer}`、
//   `io::reflection`、`writer::text`），并有 `ProviderInfo`/`FileFormat` 歧义。
//   `WordWriter` **不参与** docx 渲染管线（读→排版→渲染），属用户明示可 defer 的非管线逻辑。
// Java: import cn.wps.moffice.writer.io.writer.WordWriter
// DEFERRED_NON_FUNCTION: docx **写出**路径（579 行）；登记需 6 个未登记模块 + 歧义消解。
#[derive(Default)] pub struct WordWriter;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`event/KmoWriterOpenEventListener.rs`，6 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::event::KmoWriterOpenEventListener::KmoWriterOpenEventListener;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-3 —— **伪造接缝已消除**。
//   `WriterRange` **全 Java 树不存在**（0 命中）；Java `TextDocument.getRange`
//   （`:2718/:2724/:2730`）声明的返回类型是接口 `Range`，而其**实际实现**
//   `this.getMainDocument().getRange(...)` 返回的是具体类 **`KRange`**
//   （`SubDocument.rs:900` 已登记的真实类型）⇒ 忠实建模为 `KRange`。
//   （真实 `core/Range.rs` 接口因依赖未移植的 `KComments`/`KParagraphFormat`/`KFont`
//     子系统而暂缓登记，见 `08-member-assembly-plan.md` T7。）
// Java: import cn.wps.moffice.writer.core.KSections
// DEFERRED_NON_FUNCTION: 节集合（`core/KSections.rs` 201 行）。**管线相关**（分节/页面设置），
//   但登记前置是 `core::table::Tools` + `data::table`（含 `data/table/tree`、`data/table/grid`）
//   整棵闭包，实测单批 23 错误且级联 `io::writer` / `core::command`
//   ⇒ 需拆分为独立批次，本批不登记（见 09-subplan §15.4）。
#[derive(Default)] pub struct KSections;
// rust: ★T7-8 —— 就地影子 `#[derive(Default)] pub struct PIDSI;` 已删（**冗余遮蔽**）。
//   Java: `import cn.wps.moffice.writer.core.metadata.PIDSI`（本文件第 172 行注释；
//   `PIDSI` 是 `SummaryBase` 的子类，承载 `docProps/core.xml` 的标题/作者等）。
//   真实模块 `core/metadata/PIDSI.rs`（233 行）随 `core::metadata` 已登记并编译通过。
use crate::cn::wps::moffice::writer::core::metadata::PIDSI::PIDSI;
// rust: ★T7-6 —— 就地影子 `pub struct PageSetup;` 已删（**纯冗余遮蔽**）。
//   真实模块 `data::value::PageSetup`（255 行，含 ctor `PageSetup(w,h,ml,mr,mt,mb)`
//   与 `getWidth/getHeight/getMarginLeft/Right/Top/Bottom`）**早已登记**于
//   `data/mod.rs` 的 `value` 块；本文件上方也已有 glob 导入。
//   Rust 的 glob 导入优先级**低于**局部声明 ⇒ 该影子把真实类型遮蔽成空壳。
//   删除后 `cargo check` 仍为 0（同 `09-subplan-t7-rangetool.md` §18.2 的 6 处同类处置）。
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use crate::cn::wps::moffice::writer::function::card::online::PaperKeyWordParam::PaperKeyWordParam;
// rust: ★T7-8 —— 就地影子 `#[derive(Default)] pub struct FieldNode;` 已删（**冗余遮蔽**）。
//   Java: `PLCField.java:341 public static class FieldNode extends PLC.Node`
//   —— 它是内部类，**唯一真实归属**是数据模块 `data/PLCField.rs:95`（含
//   `super_: Node` / `mFieldChar` / `mData` / `level` / `mUser` 全字段与全部方法）。
//   `data::PLCField` 已登记并编译通过（`data/mod.rs`）⇒ 本文件上方 `// Java: import
//   cn.wps.moffice.writer.data.PLCField` 被误标为 "10 deferred — not on compile surface"，
//   实际可导入。此处改真实导入，`correctOleData` 的 `fieldNode: &FieldNode`
//   形参随之为真实类型（该方法体本身为 `DEFERRED_NON_FUNCTION`）。
use crate::cn::wps::moffice::writer::data::PLCField::FieldNode;
// rust: ★T7 —— 就地空壳 `pub struct RangeHeaderFooter;`（假编译面）替换为**真实实现**。
//   Java 中它是 `HelperHeaderFooter` 的嵌套类，**继承 `KRange`**
//   （`HelperHeaderFooter.java:487`），故 `getRange()`/`getStart()`/`getEnd()`
//   均来自 `KRange`。此处按 Java 逐行还原，并保留 `mStart` 字段。
// Java: public static class RangeHeaderFooter extends KRange
#[derive(Default)]
pub struct RangeHeaderFooter {
    // java public PLC.Node mStart;
    pub mStart: Option<crate::cn::wps::moffice::writer::data::PLC::Node>,
    // java extends KRange —— Rust 以组合表达继承（`KRange` 非 `Clone`，不能内嵌拷贝）。
    pub krange: crate::cn::wps::moffice::writer::core::KRange::KRange,
}

impl RangeHeaderFooter {
    // rust: Java `HelperHeaderFooter.RangeHeaderFooter(Document, PLC.Node, PLC.Node)`
    //   （嵌套类构造器，不是 `TextDocument` 方法；写成 `// java` 会插进配对队列）
    pub fn new__document_node_node(
        document: &dyn crate::cn::wps::moffice::writer::core::Document::Document,
        start: crate::cn::wps::moffice::writer::data::PLC::Node,
        end: Option<crate::cn::wps::moffice::writer::data::PLC::Node>,
    ) -> Self {
        // java super(document, start.getCP(), end == null ? document.getLength() : end.getCP());
        let end_cp = match end {
            Some(e) => e.getCP(),
            None => document.getLength(),
        };
        Self {
            // java this.mStart = start;
            mStart: Some(start),
            krange: crate::cn::wps::moffice::writer::core::KRange::KRange::new__document_int_int(
                document,
                start.getCP(),
                end_cp,
            ),
        }
    }

    // rust: inherited KRange.getRange()
    pub fn getRange(&self) -> i64 {
        self.krange.getRange()
    }

    // rust: inherited KRange.getStart()
    pub fn getStart(&self) -> i32 {
        self.krange.getStart()
    }

    // rust: inherited KRange.getEnd()
    pub fn getEnd(&self) -> i32 {
        self.krange.getEnd()
    }

    // rust: inherited KRange.setRange(int, int)
    pub fn setRange(&mut self, start: i32, end: i32) {
        self.krange.setRange(start, end);
    }
}
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T1 —— 就地影子已删；
//   真实模块（`core/DefaultModelEvent.rs`，34 行）本批已登记并编译通过（零依赖叶子）。
use crate::cn::wps::moffice::writer::core::DefaultModelEvent::DefaultModelEvent;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`core/StyleListener4PapxTable.rs`，77 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::core::StyleListener4PapxTable::StyleListener4PapxTable;
// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 就地影子已删；
//   真实模块（`core/StyleListener4ChpxTable.rs`，73 行）本批已登记并编译通过。
use crate::cn::wps::moffice::writer::core::StyleListener4ChpxTable::StyleListener4ChpxTable;
// rust: ★T7-5 —— 就地影子 `pub struct RangeTool;`（假实现 `isRevisionDelete__propertyset -> false`）已删。
//   实测全仓库**无任何活调用点**：唯一文本引用 `GraphReaderTool.rs:150` 位于
//   `/* DEFERRED_NON_FUNCTION */` 块内；`KRevisionChange` / `TextTool` / `KPLC_Headings` /
//   `KComplexTableRange` / `TextStream_RevisionFilter` 五个引用方均**未登记**。
//   真实模块 `core/RangeTool.rs`（1576 行 / 52 个 `pub fn`，签名 `isRevisionDelete__propertyset -> Option<bool>`）
//   待其第一层缺口（`HelperHeaderFooter` / `KColRange`(+`DeleteRangeReason`) / `PLCField` /
//   `KTableRangeBase`，后三者尚无 .rs 文件）补齐后登记。

// java public interface ISaveSuccessedListener
pub trait ISaveSuccessedListener {
    fn onSaveSuccessed(&mut self, var1: &str);
}

// java public interface OnModifiedListener
pub trait OnModifiedListener {
    fn onModifiedOnce(&mut self);
    fn onModified(&mut self, modified: bool);
    fn onAllowAutoSave(&mut self);
}

// java public class TextDocument extends Atom implements IDocument, ITextOpenDocument, DocumentProtection
pub struct TextDocument {
    // java Atom super_
    pub(crate) super_: Atom,
    // java UUSti mStiMaker;
    pub(crate) mStiMaker: Option<UUSti>,
    // java UUCommentId mCommentIdMaker;
    pub(crate) mCommentIdMaker: Option<UUCommentId>,
    // java UUCommentId mPropertSetRegistryId;
    pub(crate) mPropertSetRegistryId: Option<UUCommentId>,
    // java ITextPool mCharBuffer;
    pub(crate) mCharBuffer: Option<Box<dyn ITextPool>>,
    // java private String mPath;
    mPath: Option<String>,
    // java private RandomAccessFile mRaf;
    mRaf: Option<RandomAccessFile>,
    // java String mName;
    pub(crate) mName: Option<String>,
    // java KStyles styles;
    pub(crate) styles: Option<Box<KStyles>>,
    // java FontTable fontTable;
    pub(crate) fontTable: Option<FontTable>,
    // java @AtomMember private PropertySet property;
    property: Option<PropertySet>,
    // java @AtomMember private ModifyFlag mModified;
    mModified: Option<ModifyFlag>,
    // java @Deprecated
    // java private Metadata mMetadata;
    mMetadata: Option<Metadata>,
    // java private String mAppVersion;
    mAppVersion: Option<String>,
    // java ArrayList<ITextEditor> mViews;
    pub(crate) mViews: Option<Vec<Box<dyn ITextEditor>>>,
    // java OnlineSecurityTool mOnlineSecurytiTool;
    pub(crate) mOnlineSecurytiTool: Option<OnlineSecurityTool>,
    // java @AtomMember public FileFormat ff;
    pub(crate) ff: Option<FileFormat>,
    // java protected Document[] mSubdocuments;
    pub(crate) mSubdocuments: Option<Vec<Option<Box<SubDocument>>>>,
    // java AutoNumTable mAutoNumTable;
    pub(crate) mAutoNumTable: Option<AutoNumTable>,
    // java ArrayList<Integer> mRsids;
    pub(crate) mRsids: RefCell<Option<Vec<i32>>>,
    // java IMediaLib mMediaLib;
    pub(crate) mMediaLib: Option<MediaLib>,
    // java AsiaTypography mAsiaTypography;
    pub(crate) mAsiaTypography: Option<AsiaTypography>,
    // java InputStream mStream;
    pub(crate) mStream: Option<Box<dyn std::io::Read>>,
    // java FileFormatEnum mTemplate;
    pub(crate) mTemplate: Option<FileFormatEnum>,
    // java boolean mIsAutoSaveFile;
    pub(crate) mIsAutoSaveFile: bool,
    // java private DocumentLoadListener loadListener;
    loadListener: Option<Box<dyn DocumentLoadListener>>,
    // java private OleManager mOleManager;
    mOleManager: Option<OleManager>,
    // java ProviderInfo mProviderInfo;
    pub(crate) mProviderInfo: Option<ProviderInfo>,
    // java ICoreDataAcceptor mCoreDataAcceptor;
    pub(crate) mCoreDataAcceptor: Option<Box<dyn ICoreDataAcceptor>>,
    // java public boolean mHasStructureError;
    pub(crate) mHasStructureError: bool,
    // java boolean mStateIO;
    pub(crate) mStateIO: bool,
    // java int mIoErrorCode;
    pub(crate) mIoErrorCode: i32,
    // java Object mIoError;
    pub(crate) mIoError: Option<Box<dyn Object>>,
    // java boolean mLoadSuccess;
    pub(crate) mLoadSuccess: bool,
    // java private boolean mTrackChange;
    mTrackChange: bool,
    // java private boolean mIsEmbedBalloonMode;
    mIsEmbedBalloonMode: bool,
    // java private final Object mFinishDumpObjectsNotifyObj;
    mFinishDumpObjectsNotifyObj: Option<Box<dyn Object>>,
    // java private boolean mFinishDumpObjects;
    mFinishDumpObjects: bool,
    // java private boolean mIsLegal;
    mIsLegal: bool,
    // java UnknownDataMap mUnknownDataMap;
    pub(crate) mUnknownDataMap: Option<UnknownDataMap>,
    // java String mVBAPath;
    pub(crate) mVBAPath: Option<String>,
    // java UULsid mLsidMaker;
    pub(crate) mLsidMaker: Option<UULsid>,
    // java UUNumberingId mNumberingIdMaker;
    pub(crate) mNumberingIdMaker: Option<UUNumberingId>,
    // java ShapeIDAlloc mShapeIDAlloc;
    pub(crate) mShapeIDAlloc: Option<ShapeIDAlloc>,
    // java private ClassLoader mSSLoader;
    mSSLoader: Option<Box<dyn std::any::Any + Send + Sync>>,
    // java DrawingRIDMap mDrawingRIDMap;
    pub(crate) mDrawingRIDMap: Option<DrawingRIDMap>,
    // java private WIntObjectHashMap<String> mShapeId2DrawingRIDMap;
    mShapeId2DrawingRIDMap: Option<WIntObjectHashMap>,
    // java private WIntObjectHashMap<ArrayList<Integer>> mSubDocType2DiagramShapeId;
    mSubDocType2DiagramShapeId: Option<WIntObjectHashMap>,
    // java private Shape mImmutableBkShape;
    mImmutableBkShape: Option<Shape>,
    // java private boolean needUpdateImmutableBkShape;
    needUpdateImmutableBkShape: bool,
    // java private List<Shape> mChartShapeList;
    mChartShapeList: Option<Vec<Shape>>,
    // java private String mEncoding;
    mEncoding: Option<String>,
    // java private StatAnalytics statAnalytics;
    statAnalytics: Option<StatAnalytics>,
    // java private Date mOpenTime;
    mOpenTime: Option<Date>,
    // java private FileTime mLastFileTime;
    mLastFileTime: Option<FileTime>,
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<Box<dyn IDLProvider>>,
    // java private int mGVmlOffset;
    mGVmlOffset: i32,
    // java private DocumentVariables variables;
    variables: Option<DocumentVariables>,
    // java private KDocumentProtection protection;
    protection: Option<KDocumentProtection>,
    // java private Theme mTheme;
    mTheme: Option<Theme>,
    // java private ArrayList<String> mFirstPageFontList;
    mFirstPageFontList: Option<Vec<String>>,
    // java private WdCompatibilityMode mCompatbilityMode;
    mCompatbilityMode: Option<WdCompatibilityMode>,
    // java private IShareObjIndexdMap mShareObjIndexdMap;
    mShareObjIndexdMap: Option<ShareObjIndexedMapImpl>,
    // java private byte[] mEvidenceData;
    mEvidenceData: Option<Vec<u8>>,
    // java private KExchange mExchange;
    mExchange: Option<Box<KExchange>>,
    // java private KFootnoteEndnote mFootnoteEndnote;
    mFootnoteEndnote: Option<KFootnoteEndnote>,
    // java private ISectionProperty mSectionPropSetting;
    mSectionPropSetting: Option<Box<dyn ISectionProperty>>,
    // java private KCustomIcv mKCustomIcv;
    mKCustomIcv: Option<KCustomIcv>,
    // java private KDocumentWriteProtection mDocumentCrypt;
    mDocumentCrypt: Option<KDocumentWriteProtection>,
    // java private PaperRecognition mPaperRecognition;
    mPaperRecognition: Option<PaperRecognition>,
    // java private IDocumentCallback mDocumentCallback;
    mDocumentCallback: Option<Box<dyn IDocumentCallback>>,
    // java private String mGloabalInkUserID;
    mGloabalInkUserID: Option<String>,
    // java private ILogFileObserver mLogFileObserver;
    mLogFileObserver: Option<Box<dyn ILogFileObserver>>,
    // java private IDebugger mDebugger;
    mDebugger: Option<Box<dyn IDebugger>>,
    // java private String mUUID;
    mUUID: Option<String>,
    // java private String mTmpRootDir;
    mTmpRootDir: Option<String>,
    // java private FileManager mFileManager;
    mFileManager: Option<FileManager>,
    // java private boolean mOpenIdleScroll;
    mOpenIdleScroll: bool,
    // java private KmoWriterOpenEventListenerGroup openListeners;
    openListeners: Option<KmoWriterOpenEventListenerGroup>,
    // java private KParagraphsID mParasID;
    mParasID: Option<KParagraphsID>,
    // java private KListTemplates mListTemplates;
    mListTemplates: Option<KListTemplates>,
    // java private KLists mLists;
    mLists: Option<KLists>,
    // java private ISaveSuccessedListener mSaveSuccessedListener;
    mSaveSuccessedListener: Option<Box<dyn ISaveSuccessedListener>>,
    // java private boolean modifiedOnce;
    modifiedOnce: bool,
    // java private OnModifiedListener mOnModifiedListener;
    mOnModifiedListener: Option<Box<dyn OnModifiedListener>>,
    // java private OnModifiedListener mOnModifiedListenerTemp;
    mOnModifiedListenerTemp: Option<Box<dyn OnModifiedListener>>,
    // java private OnModifiedListener mOnModifiedListener4Stat;
    mOnModifiedListener4Stat: Option<Box<dyn OnModifiedListener>>,
    // java private Stack<RangeWriteLock> transaction_locks;
    transaction_locks: Option<Vec<RangeWriteLock>>,
    // java private boolean mStopUpdateTextDocment;
    mStopUpdateTextDocment: bool,
    // java IHostApp mSTHostApp;
    pub(crate) mSTHostApp: Option<Box<dyn IHostApp>>,
    // java private GridResizeRequest mGridResizeRequest;
    // rust: ★Java `GridResizeRequest` 是**接口** ⇒ Rust trait；字段须为 trait 对象（E0782）。
    // rust: `RefCell` 对应 Java `synchronized`，`Document.getTextDocument()` 只有 `&self` 时仍能取到它。
    mGridResizeRequest: RefCell<Option<Box<dyn GridResizeRequest>>>,
    // java private volatile SaveAccelerator mSaveAccelerator;
    mSaveAccelerator: Option<SaveAccelerator>,
}

impl Default for TextDocument {
    fn default() -> Self {
        Self {
        super_: Atom::new(),
        mStiMaker: Some(UUSti::UUSti()),
        mCommentIdMaker: Some(UUCommentId::UUCommentId()),
        mPropertSetRegistryId: Some(UUCommentId::UUCommentId()),
        mCharBuffer: None,
        mPath: None,
        mRaf: None,
        mName: None,
        styles: None,
        fontTable: None,
        property: None,
        mModified: None,
        mMetadata: None,
        mAppVersion: None,
        mViews: None,
        mOnlineSecurytiTool: None,
        ff: None,
        mSubdocuments: Some((0..7).map(|_| None).collect()),
        mAutoNumTable: None,
        mRsids: RefCell::new(None),
        mMediaLib: None,
        mAsiaTypography: None,
        mStream: None,
        mTemplate: None,
        mIsAutoSaveFile: false,
        loadListener: None,
        mOleManager: None,
        mProviderInfo: None,
        mCoreDataAcceptor: None,
        mHasStructureError: false,
        mStateIO: false,
        mIoErrorCode: 0,
        mIoError: None,
        mLoadSuccess: false,
        mTrackChange: false,
        mIsEmbedBalloonMode: false,
        mFinishDumpObjectsNotifyObj: None,
        mFinishDumpObjects: false,
        mIsLegal: true,
        mUnknownDataMap: None,
        mVBAPath: None,
        mLsidMaker: Some(UULsid::UULsid()),
        mNumberingIdMaker: Some(UUNumberingId::UUNumberingId()),
        mShapeIDAlloc: None,
        mSSLoader: None,
        mDrawingRIDMap: None,
        mShapeId2DrawingRIDMap: None,
        mSubDocType2DiagramShapeId: None,
        mImmutableBkShape: None,
        needUpdateImmutableBkShape: true,
        mChartShapeList: None,
        mEncoding: None,
        statAnalytics: None,
        mOpenTime: None,
        mLastFileTime: None,
        mDlProvider: None,
        mGVmlOffset: 0,
        variables: None,
        protection: None,
        mTheme: None,
        mFirstPageFontList: None,
        mCompatbilityMode: None,
        mShareObjIndexdMap: None,
        mEvidenceData: None,
        mExchange: None,
        mFootnoteEndnote: None,
        mSectionPropSetting: None,
        mKCustomIcv: None,
        mDocumentCrypt: None,
        mPaperRecognition: None,
        mDocumentCallback: None,
        mGloabalInkUserID: None,
        mLogFileObserver: None,
        mDebugger: None,
        mUUID: None,
        mTmpRootDir: None,
        mFileManager: None,
        mOpenIdleScroll: false,
        openListeners: None,
        mParasID: None,
        mListTemplates: None,
        mLists: None,
        mSaveSuccessedListener: None,
        modifiedOnce: false,
        mOnModifiedListener: None,
        mOnModifiedListenerTemp: None,
        mOnModifiedListener4Stat: None,
        transaction_locks: None,
        mStopUpdateTextDocment: false,
        mSTHostApp: None,
        mGridResizeRequest: RefCell::new(None),
        mSaveAccelerator: None,
        }
    }
}

// java public static class PapxTableListener implements IPapxTable.Listener
#[derive(Default)]
pub struct PapxTableListener {
    // java SubDocument mDocument;
    pub(crate) mDocument: Option<SubDocument>,
    // java ArrayList<IPapxTable.Listener> mListeners;
    pub(crate) mListeners: Option<Vec<Box<dyn PapxListener>>>,
}

// java public static class ChpxTableListener implements IChpxTable.Listener
#[derive(Default)]
pub struct ChpxTableListener {
    // java SubDocument mDocument;
    pub(crate) mDocument: Option<SubDocument>,
    // java ArrayList<IChpxTable.Listener> mListeners;
    pub(crate) mListeners: Option<Vec<Box<dyn ChpxListener>>>,
}

// java private class MergeListener implements ActionList.ActionChangeListener
#[derive(Default)]
pub struct MergeListener {
}

// java public static final class UUNumberingId extends Atom
pub struct UUNumberingId {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember private int mId;
    mId: i32,
}

// java public static final class UULsid extends Atom
pub struct UULsid {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember private int mId;
    mId: i32,
}

// java public class TextDocument extends Atom implements IDocument, ITextOpenDocument, DocumentProtection
impl TextDocument {
    // java public  TextDocument()
    pub fn TextDocument() -> Self {
      // java this(false);
      let mut this = Self::TextDocument__boolean(false);
      this
    }

    pub fn new() -> Box<Self> {
      // rust: Java `new TextDocument()` 在堆上；按值 `_create` 后再 move 会使
      //   `SubDocument.mParent` 悬垂（见 DocumentOpen：`Box` + `_create`）。
      let mut this = Box::new(Self::TextDocument__boolean(true));
      this._create();
      this
    }

    // java protected  TextDocument(boolean bMock)
    pub fn TextDocument__boolean(bMock: bool) -> Self {
      // java this.mStiMaker = new UUSti();
      let mut this = Self::default();
      // java this.mCommentIdMaker = new UUCommentId();
      // java this.mPropertSetRegistryId = new UUCommentId();
      // java this.mCharBuffer = null;
      // java this.fontTable = new FontTable();
      this.fontTable = Some(FontTable::default());
      // java this.property = PropertySet.empty;
      // java this.mModified = ModifyFlag._false;
      this.mModified = Some(ModifyFlag::_false);
      // java this.mMetadata = new Metadata();
      this.mMetadata = Some(Metadata::default());
      // java this.mAppVersion = null;
      // java this.mViews = new ArrayList();
      this.mViews = Some(Vec::new());
      // java this.ff = FileFormat.FF_TXT;
      this.ff = Some(FileFormat::FF_TXT);
      // java this.mSubdocuments = new Document[7];
      this.mSubdocuments = Some((0..7).map(|_| None).collect());
      // java this.mRsids = new ArrayList(5);
      *this.mRsids.borrow_mut() = Some(Vec::with_capacity(5));
      // java this.mMediaLib = null;
      // java this.mAsiaTypography = null;
      // java this.mHasStructureError = false;
      // java this.mStateIO = false;
      // java this.mIoErrorCode = 0;
      // java this.mIoError = null;
      // java this.mLoadSuccess = false;
      // java this.mTrackChange = false;
      // java this.mIsEmbedBalloonMode = false;
      // java this.mFinishDumpObjectsNotifyObj = new Object();
      // java this.mFinishDumpObjects = false;
      // java this.mIsLegal = true;
      // java this.mUnknownDataMap = new UnknownDataMap();
      this.mUnknownDataMap = Some(UnknownDataMap::new());
      // java this.mLsidMaker = new UULsid();
      // java this.mNumberingIdMaker = new UUNumberingId();
      // java this.mShapeIDAlloc = new ShapeIDAlloc();
      this.mShapeIDAlloc = Some(ShapeIDAlloc::new());
      // java this.mSSLoader = null;
      // java this.mImmutableBkShape = null;
      // java this.needUpdateImmutableBkShape = true;
      // java this.mGVmlOffset = 0;
      // java this.mTheme = new Theme();
      this.mTheme = Some(Theme::new());
      // java this.mParasID = new KParagraphsID();
      this.mParasID = Some(KParagraphsID::default());
      // java this.transaction_locks = new Stack();
      this.transaction_locks = Some(Vec::new());
      // java this.mStopUpdateTextDocment = false;
      // java this.mSTHostApp = null;
      // java this._create();
      if !bMock {
        this._create();
      }
      // java PropertyMap.sUseSmallMap = true;
      PropertyMap::set_sUseSmallMap(true);
      // java this.mUUID = UUIDUtil.generate32ID();
      this.mUUID = Some(UUIDUtil::generate32ID());
      // java this.mTmpRootDir = Platform.getCurrentWriterTmpDir(this.mUUID);
      this.mTmpRootDir = this.mUUID.as_deref().map(Platform::getCurrentWriterTmpDir);
      // java this.mFileManager = new FileManager(this.mTmpRootDir);
      this.mFileManager = this.mTmpRootDir.as_deref().map(FileManager::new);
      // java this.openListeners = new KmoWriterOpenEventListenerGroup();
      this.openListeners = Some(KmoWriterOpenEventListenerGroup::new());
      this
      // java if (!bMock)
    }

    // java public void setStatAnalytics(StatAnalytics statAnalytics)
    pub fn setStatAnalytics(&mut self, statAnalytics: &StatAnalytics) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.statAnalytics = statAnalytics;
    }


    // java public StatAnalytics getStatAnalytics()
    pub fn getStatAnalytics(&self) -> StatAnalytics {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.statAnalytics;
    }


    // java public void setDocumentCallback(IDocumentCallback documentCallback)
    pub fn setDocumentCallback(&mut self, documentCallback: &dyn IDocumentCallback) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mDocumentCallback = documentCallback;
    }


    // java private void _create()
    pub fn _create(&mut self) {
      // java Transaction pTransaction = new Transaction();
      let pTransaction = Transaction::new();
      let tx_rc = Rc::new(RefCell::new(pTransaction));
      // java this.setTransaction(pTransaction, false);
      self.super_.setTransaction_recursive(Some(tx_rc.clone()), false);
      // java pTransaction.setListener(new TransactionListeners());
      tx_rc.borrow_mut().setListener(Some(Box::new(TransactionListeners::new())));
      // java pTransaction.getActionList().setActionChangeListener(new MergeListener());
      let mut parent = Box::new(ParentTextDocument {
        transaction: Some(tx_rc.clone()),
        textPool: None,
        styles: None,
      });
      // java this.styles = new KStyles(this);
      // rust: KStyles 仍吃 KStyle stand-in（wc-02 05）；SubDocument 改传真实 `self`
      self.styles = Some(Box::new(KStyles::KStyles(&parent)));
      // java ITextPool pFixedPool;
      // java KSwappableTextPool m = new KSwappableTextPool();
      let mut m = ();
      let _ = m;
      // java m.setPoolSize(262144);
      // java pFixedPool = m;
      let mut pFixedPool = KTextPool::KTextPool();
      // java IOException e
      let mut e = ();
      let _ = e;
      // java e.printStackTrace();
      // java pFixedPool = new KTextPool();
      pFixedPool = KTextPool::KTextPool();
      // java this.mCharBuffer = pFixedPool;
      self.mCharBuffer = Some(Box::new(pFixedPool));
      if let Some(ref mut buf) = self.mCharBuffer {
        parent.textPool = Some(buf.as_mut() as *mut dyn ITextPool);
      }
      if let Some(ref mut styles) = self.styles {
        parent.styles = Some(styles.as_mut() as *mut KStyles);
      }
      let _ = parent;
      // java this.createAllSubDocument();
      self.createAllSubDocument();
      // java ShapeFactoryDoc.createShapeFactory();
      ShapeFactoryDoc::createShapeFactory();
      // java catch (IOException e)
    }

    // java public String getTmpRootDir()
    pub fn getTmpRootDir(&self) -> String {
      // java return this.mTmpRootDir;
      self.mTmpRootDir.as_ref().expect("mTmpRootDir").clone()
    }


    // java public FileManager getFileManager()
    pub fn getFileManager(&self) -> &FileManager {
      // java return this.mFileManager;
      self.mFileManager.as_ref().expect("mFileManager")
    }


    // java private void clearCachedValues()
    pub fn clearCachedValues(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java TableLength.clear();
      // java CellMargin.clear();
      // java SHD.clear();
      // java BRC.clear();
      // java TableCellBuilder.clear();
      // java DefTableBuilder.clear();
      // java TableDefSHDBuilder.clear();
      // java TableLookBuilder.clear();
      // java PositionCode.clear();
      // java CellSpacing.clear();
      // java CellHideMark.clear();
      // java this.mShareObjIndexdMap = null;
    }


    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.statAnalytics = null;
      // java this.modifiedOnce = false;
      // java this.clearCachedValues();
      // java if (this.styles != null)
        // java this.styles.dispose();
        // java this.styles = null;
      // java if (this.mViews != null)
        // java this.mViews.clear();
        // java this.mViews = null;
      // java if (this.mCharBuffer != null)
        // java this.mCharBuffer.dispose();
        // java this.mCharBuffer = null;
      // java KPictureRegistry.instance().clear();
      // java if (this.mSubdocuments != null)
        // java int i = 0;
        let mut i = ();
        // java for(int len = this.mSubdocuments.length; i < len; ++i)
          // java Document doc = this.mSubdocuments[i];
          let mut doc = ();
          // java if (doc != null)
            // java doc.dispose();
          // java this.mSubdocuments[i] = null;
        // java this.mSubdocuments = null;
      // java if (this.mAutoNumTable != null)
        // java this.mAutoNumTable.dispose();
        // java this.mAutoNumTable = null;
      // java if (this.mMediaLib != null)
        // java this.mMediaLib.dispose();
        // java this.mMediaLib = null;
      // java if (this.mOleManager != null)
        // java this.mOleManager.dispose(this.getPath(), this.getRaf());
        // java this.mOleManager = null;
      // java if (this.mMetadata != null)
        // java this.mMetadata.dispose();
        // java this.mMetadata = null;
      // java this.mPath = null;
      // java this.mRaf = null;
      // java this.mName = null;
      // java this.mStream = null;
      // java this.mAppVersion = null;
      // java this.mEvidenceData = null;
      // java if (this.mAsiaTypography != null)
        // java this.mAsiaTypography.dispose();
        // java this.mAsiaTypography = null;
      // java if (this.mRsids != null)
        // java this.mRsids.clear();
        // java this.mRsids = null;
      // java if (this.mCoreDataAcceptor != null)
        // java this.mCoreDataAcceptor = null;
      // java if (this.mProviderInfo != null)
        // java this.mProviderInfo = null;
      // java if (this.mImmutableBkShape != null)
        // java this.mImmutableBkShape.dispose();
        // java this.mImmutableBkShape = null;
      // java this.mOnModifiedListener = null;
      // java this.mOnModifiedListenerTemp = null;
      // java this.ff = null;
      // java this.fontTable = null;
      // java this.mCommentIdMaker = null;
      // java if (this.transaction_locks != null)
        // java this.transaction_locks.clear();
        // java this.transaction_locks = null;
      // java if (this.property != null)
        // java this.property.dispose();
        // java this.property = null;
      // java this.mLsidMaker = null;
      // java this.mModified = null;
      // java this.mNumberingIdMaker = null;
      // java this.mShapeIDAlloc = null;
      // java this.mUnknownDataMap = null;
      // java this.mStiMaker = null;
      // java if (null != this.mDrawingRIDMap)
        // java this.mDrawingRIDMap.dispose();
      // java if (null != this.mShapeId2DrawingRIDMap)
        // java this.mShapeId2DrawingRIDMap.clear();
      // java if (null != this.mParasID)
        // java this.mParasID.dispose();
      // java this.mTheme = null;
      // java this.mFirstPageFontList = null;
      // java this.mCompatbilityMode = null;
      // java this.mFootnoteEndnote = null;
      // java this.mSectionPropSetting = null;
      // java this.mSaveSuccessedListener = null;
      // java this.mDebugger = null;
      // java this.stopMonitor();
      // java Platform.removeCurrentWriterTmpDir(this.mUUID);
      // java this.openListeners.clear();
      // java super.dispose();
      // java for(int len = this.mSubdocuments.length; i < len; ++i)
    }


    // java public Document getSubdocument(int nSubdocId)
    pub fn getSubdocument(&self, nSubdocId: i32) -> Option<&SubDocument> {
      // java return this.mSubdocuments[nSubdocId];
      self.mSubdocuments
        .as_ref()
        .and_then(|v| v.get(nSubdocId as usize).and_then(|s| s.as_deref()))
    }

    pub fn getSubdocumentMut(&mut self, nSubdocId: i32) -> Option<&mut SubDocument> {
      self.mSubdocuments
        .as_mut()
        .and_then(|v| v.get_mut(nSubdocId as usize).and_then(|s| s.as_mut().map(|b| b.as_mut())))
    }

    // java public Document createSubDocument(int nSubdocId)
    pub fn createSubDocument(&mut self, nSubdocId: i32) -> Option<&SubDocument> {
      // java this.writeLockAtom();
      // java if (null == this.mSubdocuments[nSubdocId])
      let need = self
        .mSubdocuments
        .as_ref()
        .and_then(|v| v.get(nSubdocId as usize))
        .map(|s| s.is_none())
        .unwrap_or(true);
      if need {
        // java this.mSubdocuments[nSubdocId] = new SubDocument(this, nSubdocId);
        let parent_ptr = self as *mut TextDocument;
        let parent = unsafe { &*parent_ptr };
        let sd = SubDocument::SubDocument(parent, nSubdocId);
        if let Some(v) = self.mSubdocuments.as_mut() {
          if (nSubdocId as usize) < v.len() {
            v[nSubdocId as usize] = Some(sd);
          }
        }
      }
      // java return this.mSubdocuments[nSubdocId];
      self.getSubdocument(nSubdocId)
    }

    // java private void createAllSubDocument()
    pub fn createAllSubDocument(&mut self) {
      let mut i: i32 = 0;
      // java for(int i = 0; i < 7; ++i)
      let parent_ptr = self as *mut TextDocument;
      while i < 7 {
        // java this.mSubdocuments[i] = new SubDocument(this, i);
        // rust: 必须立刻写入 `mSubdocuments[i]`（Java 数组逐槽赋值）。
        //   先攒本地 Vec 再一次性挂上会使 type1/4 的 `getMainDocument()` 在 ctor 里看到槽 0 仍为 null。
        let parent = unsafe { &*parent_ptr };
        let sd = SubDocument::SubDocument(parent, i);
        unsafe {
          if let Some(v) = (*parent_ptr).mSubdocuments.as_mut() {
            let idx = i as usize;
            if idx < v.len() {
              v[idx] = Some(sd);
            }
          }
        }
        i += 1;
      }
      // java for(int i = 0; i < 7; ++i)
    }

    // java public ShapeIDAlloc getShapeIDAlloc()
    pub fn getShapeIDAlloc(&mut self) -> &mut ShapeIDAlloc {
      // java return this.mShapeIDAlloc;
      if self.mShapeIDAlloc.is_none() {
        self.mShapeIDAlloc = Some(ShapeIDAlloc::new());
      }
      self.mShapeIDAlloc.as_mut().expect("mShapeIDAlloc")
    }


    // java public synchronized MediaLib getMediaLib()
    pub fn getMediaLib(&mut self) -> &mut MediaLib {
      // java if (this.mMediaLib == null)
      if self.mMediaLib.is_none() {
        self.mMediaLib = Some(MediaLib::createMediaLib(
            self.mTmpRootDir.as_ref().expect("mTmpRootDir").clone(),
        ));
      }
      // java return (MediaLib)this.mMediaLib;
      self.mMediaLib.as_mut().expect("mMediaLib")
    }


    // java public synchronized OleManager getOleManager()
    pub fn getOleManager(&mut self) -> OleManager {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mOleManager == null)
        // java if (null != this.mRaf)
          // java this.mOleManager = OleManager.create((RandomAccessFile)this.mRaf, this.getPassword(), this.getMediaLib(), this.mTmpRootDir);
        // java else
          // java this.mOleManager = OleManager.create((String)this.mPath, this.getPassword(), this.getMediaLib(), this.mTmpRootDir);
      // java return this.mOleManager;
    }


    // java public ICoreDataAcceptor getCoreDataAcceptor()
    pub fn getCoreDataAcceptor(&mut self) -> Box<dyn ICoreDataAcceptor> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mCoreDataAcceptor == null)
        // java this.mCoreDataAcceptor = TextCoreDataAcceptor.create(this);
      // java return this.mCoreDataAcceptor;
    }


    // java public void newDocumentFromTemplate(String filePath, InputStream stream, String fileName, FileFormatEnum templateType)
    pub fn newDocumentFromTemplate__string_inputstream_string_fileformatenum(&mut self, filePath: &str, stream: &dyn std::io::Read, fileName: &str, templateType: &FileFormatEnum) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.newDocumentFromTemplate(filePath, stream, fileName, templateType, (IDLProvider)null);
    }


    // java public void newDocumentFromTemplate(String filePath, InputStream stream, String fileName, FileFormatEnum templateType, IDLProvider provider)
    pub fn newDocumentFromTemplate__string_inputstream_string_fileformatenum_idlprovider(&mut self, filePath: &str, stream: &dyn std::io::Read, fileName: &str, templateType: &FileFormatEnum, provider: &dyn IDLProvider) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mIsAutoSaveFile = false;
      // java this.newDocument(filePath, stream, fileName, this.mIsAutoSaveFile, templateType);
      // java this.mPath = StringUtil.normalizePath(filePath);
      // java this.mDlProvider = provider;
      // java this.setDocumentProperties();
    }


    // java public void newDocumentFromAutoSave(String filePath, InputStream stream, String fileName, IDLProvider provider)
    pub fn newDocumentFromAutoSave(&mut self, filePath: &str, stream: &dyn std::io::Read, fileName: &str, provider: &dyn IDLProvider) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mIsAutoSaveFile = true;
      // java this.newDocument(filePath, stream, fileName, this.mIsAutoSaveFile, (FileFormatEnum)null);
      // java this.mPath = StringUtil.normalizePath(filePath);
      // java this.mDlProvider = provider;
    }


    // java private void newDocument(String filePath, InputStream stream, String fileName, boolean isModified, FileFormatEnum templateType)
    pub fn newDocument(&mut self, filePath: &str, stream: &dyn std::io::Read, fileName: &str, isModified: bool, templateType: &FileFormatEnum) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.openDocument((String)filePath, (InputStream)stream, (FileFormatEnum)templateType, (String)null);
      // java this.mName = fileName;
      // java this.getTransaction().clearHistory();
      // java this.mModified = isModified ? ModifyFlag._true : ModifyFlag._false;
    }


    // java private void setDocumentProperties()
    pub fn setDocumentProperties(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Metadata metaData = this.getMetadata();
      // java Assert.assertNotNull("metaData should not be null!", metaData);
      // java PIDSI pidsi = metaData.getPIDSI();
      // java Assert.assertNotNull("pidsi should not be null!", pidsi);
      // java pidsi.setCreateTime(new Date());
      // java PIDSI summaryInfo = metaData.getPIDSI();
      // java summaryInfo.setAuthor(Platform.getUserName());
      // java KCustomIcv kCustomIcv = this.getKCustomIcv();
      // java if (null != kCustomIcv)
        // java kCustomIcv.createIcvFromNew();
    }


    // java private void generateName()
    pub fn generateName(&mut self) {
      // rust: ★T7-7 —— **管线可达，恢复真实实现**。
      //   `getName()` → `generateName()` → `mName`，再经
      //   `field::parser::{FileNamePaser,FileSizeParse,InfoParse}` 供
      //   `FILENAME` / `FILESIZE` / `INFO` 域代码取值 ⇒ 属 docx 渲染管线。
      // java this.mName = "noname";
      self.mName = Some("noname".to_string());
    }


    // java protected int openDocument(String filePath, InputStream stream, FileFormatEnum templateType, String passwd)
    pub fn openDocument__string_inputstream_fileformatenum_string(&mut self, filePath: &str, stream: &dyn std::io::Read, templateType: &FileFormatEnum, passwd: &str) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument(filePath, stream, templateType, passwd, (IDLProvider)null);
    }


    // java protected int openDocument(String filePath, InputStream stream, FileFormatEnum templateType, String passwd, IDLProvider dlProvider)
    pub fn openDocument__string_inputstream_fileformatenum_string_idlprovider(&mut self, filePath: &str, stream: &dyn std::io::Read, templateType: &FileFormatEnum, passwd: &str, dlProvider: &dyn IDLProvider) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mPath = StringUtil.normalizePath(filePath);
      // java this.init(stream, templateType, passwd, dlProvider);
      // java return 0;
    }


    // java protected int openDocument(RandomAccessFile raf, FileFormatEnum templateType, String passwd, IDLProvider dlProvider)
    pub fn openDocument__randomaccessfile_fileformatenum_string_idlprovider(&mut self, raf: &RandomAccessFile, templateType: &FileFormatEnum, passwd: &str, dlProvider: &dyn IDLProvider) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRaf = raf;
      // java this.init((InputStream)null, templateType, passwd, dlProvider);
      // java return 0;
    }


    // java private void init(InputStream stream, FileFormatEnum templateType, String passwd, IDLProvider dlProvider)
    pub fn init__inputstream_fileformatenum_string_idlprovider(&mut self, stream: &dyn std::io::Read, templateType: &FileFormatEnum, passwd: &str, dlProvider: &dyn IDLProvider) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mStream = stream;
      // java this.mTemplate = templateType;
      // java this.mOpenTime = new Date();
      // java this.initPasswd(passwd);
      // java this.mDlProvider = dlProvider;
    }


    // java public int startLoading(IOListener ioListener, IOThreadSign aIOThreadSign)
    pub fn startLoading__iolistener_iothreadsign(&mut self, ioListener: &dyn IOListener, aIOThreadSign: &IOThreadSign) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.startLoading((DocumentPreliminaryLoader)null, ioListener, aIOThreadSign, (FileFormatEnum)null);
    }


    // java public int startLoading(DocumentPreliminaryLoader preLoader, IOListener ioListener, IOThreadSign aIOThreadSign, FileFormatEnum format)
    pub fn startLoading__documentpreliminaryloader_iolistener_iothreadsign_fileformatenum(&mut self, preLoader: &DocumentPreliminaryLoader, ioListener: &dyn IOListener, aIOThreadSign: &IOThreadSign, format: &FileFormatEnum) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java KSLog.d("IOPerformance", "TextDocument.startLoading() begin");
      // java this.onStartLoad();
      // java int ret = this.loadDocument(preLoader, ioListener, aIOThreadSign, format);
      // java KSLog.d("IOPerformance", "TextDocument.startLoading() end");
      // java return ret;
    }


    // java private void endLoading(IOListener ioListener, int nRet, DocReader docReader, IReaderBase xReader)
    pub fn endLoading(&mut self, ioListener: &dyn IOListener, nRet: i32, docReader: Option<&mut DocReader>, xReader: Option<&mut dyn IReaderBase>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.onEndLoad();
      // java Throwable e = null;
      let mut e = ();
      // java RangeLock rangeLock = this.getMainDocument().getRangeLock();
      // java if (rangeLock.isLocked())
        // java rangeLock.unLockCurThreadWhenCrash();
        // java e = new ContentLockedException((Throwable)null);
        let mut e = ();
        // java nRet = 2;
        let mut nRet = ();
      // java if (0 == nRet)
        // java this.correctDocumentData(docReader);
        // java if (null != docReader)
          // java docReader.dumpAllPictures();
          // java docReader.dumpVbaMacro();
        // java else if (null != xReader)
          // java xReader.dumpAllPictures();
          // java xReader.dumpVbaMacro();
      // java finishIO(ioListener, nRet, e);
    }


    // java private void correctDocumentData(DocReader docReader)
    pub fn correctDocumentData(&mut self, docReader: &DocReader) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mOnModifiedListener = null;
      // java boolean preIsModified = this.isModified();
      // java this.appendHeaderFooterNodeSections();
      // java this.correctDataSections();
      // java this.correctDataFields();
      // java this.correctOleFields();
      // java this.correctParaIds();
      // java this.correctReadonlyTrackchanges();
      // java this.setModified(preIsModified, false);
      // java this.mOnModifiedListener = this.mOnModifiedListenerTemp;
      // java this.getTransaction().clearHistory();
    }


    // java private void correctParaIds()
    pub fn correctParaIds(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java for(int i = 0; i < 7; ++i)
        // java Document document = this.getSubdocument(i);
        // java if (null != document && document.getLength() != 0)
          // java int documentType = document.getType();
          // java this.start();
          // java int[] fixSize = new int[]{0};
          let mut fixSize = ();
          // java int size = document.getPapxTable().size();
              // java ITextStream textStream = document.getTextStream();
              // java textStream.disableTrackChange();
              // java textStream.modifyPapx(0, document.getLength(), (entry, action) -> { PropertySet papx = entry.getProperty(); if (!papx.contains(282)) { int var10002 = fixSize[0]++; MutablePropertySet prop = new MutablePropertySet(); String newParaId = this.getKParagraphsID().createId(); prop.setValue(282, newParaId); action.addOperation(PropertyOperations.create(2, prop.freeze())); this.getKParagraphsID().putId(newParaId, documentType, (IPapxTable.Entry)entry); if (documentType == 0) { this.getKParagraphsID().setCorrectedParaId4IO(true); } } });
              let mut papx = ();
              // java textStream.enableTrackChange();
                  // java (size != 0)
                    // java KmoLog.writeLog("PropId", "IsCorrectedParaId4IO = " + (fixSize[0] != 0) + ", docType = " + documentType + ", totalSize = " + size + ", fixedSize = " + fixSize[0]);
                // java this.commit("correct paraId");
      // java for(int i = 0; i < 7; ++i)
      // java if (size != 0)
    }


    // java private void correctDataSections()
    pub fn correctDataSections(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (!this.getMainDocument().getPLCSection().isEmpty())
        // java boolean hasHeaderFooter = HelperHeaderFooter.hasHeaderFooter(this.getPLCSection(), this.getSubdocument(2));
        // java PLCSection.SectionNode firstSectionNode = this.getFirstSectionNode();
        // java for(int i = 0; i < 6; ++i)
          // java this.correctHeaderFooter(firstSectionNode, i, hasHeaderFooter);
      // java for(int i = 0; i < 6; ++i)
    }


    // java private void correctDataFields()
    pub fn correctDataFields(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java HashMap<KFieldType, Object> fieldData = new HashMap();
      let mut fieldData = ();
      // java fieldData.put(KFieldType.FieldTime, System.currentTimeMillis());
      // java for(int i = 0; i < 7; ++i)
        // java Document document = this.getSubdocument(i);
        // java Assert.assertNotNull("document should not be null!", document);
        // java PLCField plcField = document.getPLCField();
        // java if (null != plcField)
          // java plcField.initFields(fieldData);
      // java for(int i = 0; i < 7; ++i)
    }


    // java private void correctOleFields()
    pub fn correctOleFields(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java for(int i = 0; i < 7; ++i)
        // java Document document = this.getSubdocument(i);
        // java if (null != document)
          // java PLCField plcField = document.getPLCField();
          // java if (null != plcField)
            // java PLC.Iterator iter = plcField.createIterator();
            // java if (null != iter)
              // java while (iter.hasNext())
                // java PLCField.FieldNode fieldNode = (PLCField.FieldNode)iter.next();
                // java if (null != fieldNode)
                  // java this.correctOleData(document, fieldNode);
      // java for(int i = 0; i < 7; ++i)
      // java while(iter.hasNext())
    }


    // java private void correctOleData(Document document, PLCField.FieldNode fieldNode)
    pub fn correctOleData(&mut self, document: &dyn Document, fieldNode: &FieldNode) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (fieldNode.getFieldChar() == 20)
        // java IChpxTable.Entry run = document.getChpxTable().seek(fieldNode.getCP());
        // java if (null != run)
          // java PropertySet propertySet = run.getProperty();
          // java if (null != propertySet)
            // java Object object = propertySet.getObject(56, (Object)null);
            // java if (null != object && object instanceof Shape)
              // java Shape oleShape = (Shape)object;
              let mut oleShape = ();
              // java OLE ole = oleShape.getOLE();
              // java if (ole != null)
                // java ole.setIsIcon(fieldNode.mData != KFieldType.FieldRefOmitted.getFlt());
    }


    // java public void correctReadonlyTrackchanges()
    pub fn correctReadonlyTrackchanges(&mut self) {
      // java if (this.protection != null && (this.protection.getProtectionType() == ProtectionType.READONLY || this.protection.getProtectionType() == ProtectionType.COMMENTS) && this.protection.isProtect() && this.protection.isProtectOn() && this.getTrackChange())
      let shouldClear = match self.protection.as_ref() {
        Some(p) => {
          (p.getProtectionType() == ProtectionType::READONLY
            || p.getProtectionType() == ProtectionType::COMMENTS)
            && p.isProtect()
            && p.isProtectOn()
            && self.getTrackChange()
        }
        None => false,
      };
      if shouldClear {
          // java this.setTrackChange(false);
          self.setTrackChange__boolean(false);
      }
    }


    // java private void appendHeaderFooterNodeSections()
    pub fn appendHeaderFooterNodeSections(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document mainDocument = this.getMainDocument();
      // java Assert.assertNotNull("mainDocument should not be null!", mainDocument);
      // java Document documentHeaderFooter = this.getSubdocument(2);
      // java Assert.assertNotNull("documentHeaderFooter should not be null!", documentHeaderFooter);
      // java PLCHeader plcHeader = documentHeaderFooter.getPLCHeader();
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      // java PLCSection plcSection = mainDocument.getPLCSection();
      // java Assert.assertNotNull("plcSection should not be null!", plcSection);
      // java for(int i = 0; i < plcSection.size(); ++i)
        // java PLCSection.SectionNode sectionNode = (PLCSection.SectionNode)plcSection.get(i);
        // java HelperHeaderFooter.appendHeaderFooterNode(sectionNode, plcHeader);
      // java for(int i = 0; i < plcSection.size(); ++i)
    }


    // java private static void finishIO(IOListener ioListener, int ret, Throwable e)
    pub fn finishIO(ioListener: &mut dyn IOListener, ret: i32, e: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (0 == ret)
        // java ioListener.onFinish();
      // java else
        // java ioListener.onError(ret, e);
    }


    // java private void correctHeaderFooter(HelperHeaderFooter.RangeHeaderFooter range)
    pub fn correctHeaderFooter__rangeheaderfooter(&mut self, range: &mut RangeHeaderFooter) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertNotNull("range should not be null!", range);
      // java Assert.assertTrue("range.getStart()<= range.getEnd() should be true!", range.getStart() <= range.getEnd());
      // java int numberParagraphAppended = this.analyseNumberParagraphAppended(range);
      // java if (1 == numberParagraphAppended)
        // java int lastCp = range.getEnd() - 1;
        // java range.setRange(lastCp, lastCp);
        // java range.insertParagraph();
        // java this.clearActionCorrectingDataSections();
      // java else if (2 == numberParagraphAppended)
        // java HelperHeaderFooter.establishHeaderFooterWithContent(this.getSubdocument(2), range);
      // java else
        // java Assert.assertEquals("numberParagraphAppended should be equal to 0!", 0, numberParagraphAppended);
    }


    // java private void deleteRange(KRange range)
    pub fn deleteRange(&mut self, range: &mut KRange) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int nRemoved = range.deleteRangeDirectly();
      // java if (nRemoved > 0)
        // java this.clearActionCorrectingDataSections();
    }


    // java private void clearActionCorrectingDataSections()
    pub fn clearActionCorrectingDataSections(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Transaction transaction = this.getTransaction();
      // java Assert.assertNotNull("transaction should not be null!", transaction);
      // java this.setModified(this.isModified(), false);
    }


    // java private void correctHeaderFooter(PLCSection.SectionNode sectionNode, int index, boolean hasHeaderFooter)
    pub fn correctHeaderFooter__sectionnode_int_boolean(&mut self, sectionNode: &SectionNode, index: i32, hasHeaderFooter: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document document = this.getSubdocument(2);
      // java HelperHeaderFooter.RangeHeaderFooter range;
      // java switch (index)
        // java case 0:
          // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
        // java case 1:
          // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
        // java case 2:
          // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
        // java case 3:
          // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
        // java case 4:
          // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
        // java default:
          // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
      // java PropertySet property = sectionNode.getPropertySet();
      // java Assert.assertNotNull("property should not be null!", property);
      // java Assert.assertNotNull("range should not be null!", range);
      // java if (this.isLegalRange(range))
        // java if (index != 5)
          // java if (hasHeaderFooter)
            // java this.correctHeaderFooter(range);
          // java else
            // java this.deleteRange(range);
        // java else if (!hasHeaderFooter)
          // java this.deleteRange(range);
      // java range.dispose();
      // java break
      // java break
      // java break
      // java break
      // java break
    }


    // java private boolean isLegalRange(HelperHeaderFooter.RangeHeaderFooter range)
    pub fn isLegalRange(&self, range: &RangeHeaderFooter) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (range.getEnd() > this.mSubdocuments[2].getLength())
        // java return false;
      // java else
        // java return range.getStart() < range.getEnd();
    }


    // java private PLCSection getPLCSection()
    pub fn getPLCSection(&self) -> Option<&PLCSection> {
      // java return this.getMainDocument().getPLCSection();
      self.getMainDocument().and_then(|d| d.getPLCSection())
    }


    // java private PLCSection.SectionNode getFirstSectionNode()
    pub fn getFirstSectionNode(&self) -> SectionNode {
      todo!("DEFERRED_NON_FUNCTION");
      // java PLCSection plcSection = this.getPLCSection();
      // java Assert.assertNotNull("plcSection should not be null!", plcSection);
      // java PLCSection.SectionNode firstSectionNode = plcSection.getFirstSection();
      // java Assert.assertNotNull("firstSectionNode should not be null!", firstSectionNode);
      // java return firstSectionNode;
    }


    // java private int analyseNumberParagraphAppended(KRange kRange)
    pub fn analyseNumberParagraphAppended(&self, kRange: &KRange) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int start = kRange.getStart();
      // java int positionChecked = kRange.getEnd() - 1;
      // java int ret = 0;
      let mut ret = ();
      // java if (start <= positionChecked && !this.isParagraph(positionChecked))
        // java ++ret;
      // java --positionChecked;
      // java if (start <= positionChecked && !this.isParagraph(positionChecked))
        // java ++ret;
      // java return ret;
    }


    // java private boolean isParagraph(int cp)
    pub fn isParagraph(&self, cp: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertTrue("cp >= 0 should be true!", cp >= 0);
      // java ITextRope textRope = this.getSubdocument(2).getTextRope();
      // java return '\r' == textRope.charAt(cp);
    }


    // java private void correctStructureError()
    pub fn correctStructureError(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.getMainDocument().getErrorCollector().hasError())
        // java RangeWriteLock writeLock = this.getMainDocument().writeLock();
            // java boolean preIsModified = this.isModified();
            // java this.fixAllError();
            // java this.clearError();
            // java this.setModified(preIsModified, false);
            // java this.getTransaction().clearHistory();
              // java writeLock.unlock();
    }


    // java private void deleteIllegalSection(PLCSection plcSection)
    pub fn deleteIllegalSection(&self, plcSection: &mut PLCSection) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (!plcSection.isEmpty())
        // java int lenMainDoc = this.getMainDocument().getLength();
        // java PLC.Iterator iter = plcSection.createIterator();
        // java while (iter.hasPrev())
          // java PLC.Node node = iter.prev();
          // java if (node.position() < lenMainDoc)
            // java break;
          // java plcSection.remove(node);
      // java else
        // java plcSection.addSection(0);
      // java while(iter.hasPrev())
    }


    // java private void fixAllError()
    pub fn fixAllError(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start();
          // java this.appendHeaderFooterNodeSections();
          // java for(int i = 0; i < this.mSubdocuments.length; ++i)
            // java Document document = this.mSubdocuments[i];
            let mut document = ();
            // java if (document != null)
              // java DocumentCorrector corrector = document.getDocumentCorrector();
              // java corrector.fixAll();
            // java this.commit("correctStructureError");
      // java for(int i = 0; i < this.mSubdocuments.length; ++i)
    }


    // java private void clearError()
    pub fn clearError(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document document = this.getMainDocument();
      // java ErrorCollector mainError = document.getErrorCollector();
      // java int start = mainError.getMinErrorCp();
      // java int end = document.getLength();
      // java DocumentEvent e = DefaultModelEvent.obtain(document, start, end - start, 3);
      // java document.getDispatcher().update(e);
      // java e.decreaseRef();
      // java for(int i = 0; i < this.mSubdocuments.length; ++i)
        // java document = this.mSubdocuments[i];
        // java if (document != null)
          // java document.getErrorCollector().clean();
      // java for(int i = 0; i < this.mSubdocuments.length; ++i)
    }


    // java public void onStartLoad()
    pub fn onStartLoad(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mStateIO = true;
      // java this.getTransaction().setEnabled(false);
    }


    // java private void onEndLoad()
    pub fn onEndLoad(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java PLCSection plcSection = this.getMainDocument().getPLCSection();
      // java if (null != plcSection)
        // java plcSection.onEndLoad();
      // java this.getTransaction().setEnabled(true);
      // java for(int i = 0; i < this.mSubdocuments.length; ++i)
        // java Document document = this.mSubdocuments[i];
        let mut document = ();
        // java if (document != null)
          // java document.getChpxTable().setListener(new ChpxTableListener(document));
          // java document.getPapxTable().setListener(new PapxTableListener(document));
      // java this.deleteIllegalSection(this.getMainDocument().getPLCSection());
      // java RangeLock rangeLock = this.getMainDocument().getRangeLock();
      // java Object lock = rangeLock.getSyncLock();
        // java (lock)
          // java rangeLock.setIoStateEnd();
          // java lock.notifyAll();
      // java this.correctStructureError();
      // java if (this.mDlProvider != null && this.mDlProvider.getNotePath() != null)
          // java this.getStyles().deleteUnusedStyles();
      // java this.mStateIO = false;
      // java for(int i = 0; i < this.mSubdocuments.length; ++i)
      // java synchronized(lock)
    }


    // java public void setProperty(PropertySet newProperty)
    pub fn setProperty__propertyset(&mut self, newProperty: &PropertySet) {
      // java if (!this.property.isEmpty() && !newProperty.isEmpty())
      let cur_empty = self.property.as_ref().map(|p| p.isEmpty()).unwrap_or(true);
      if !cur_empty && !newProperty.isEmpty() {
        // java MutablePropertySet oldProp = new MutablePropertySet(this.property);
        let mut oldProp =
          MutablePropertySet::new_from_ps(self.property.as_ref().expect("property"));
        // java MutablePropertySet newProp = new MutablePropertySet(newProperty);
        let newProp = MutablePropertySet::new_from_ps(newProperty);
        // java int newSize = newProp.size();
        let newSize = newProp.size();
        // java int[] newIds = new int[newSize];
        let mut newIds: Vec<i32> = vec![0; newSize as usize];
        // java Object[] newValues = new Object[newSize];
        let mut newValues: Vec<Option<PropertyValue>> = vec![None; newSize as usize];
        // java newProp.getAll(newIds, newValues);
        newProp.getAll(&mut newIds, Some(&mut newValues));
        // java for(int i = 0; i < newSize; ++i)
        for i in 0..newSize as usize {
          // java if (oldProp.contains(newIds[i]))
          if oldProp.contains(newIds[i]) {
            // java Object newValue = newValues[i];
            let newValue = newValues[i].clone();
            // java if (newValue instanceof Boolean && (Boolean)newValue)
            let is_true_bool = newValue
              .as_ref()
              .and_then(|v| v.as_ref().as_any().downcast_ref::<bool>())
              .copied()
              .unwrap_or(false);
            if is_true_bool {
              // java oldProp.setValue(newIds[i], newValues[i]);
              oldProp.setValue(newIds[i], newValues[i].clone());
            }
          } else {
            // java oldProp.setValue(newIds[i], newValues[i]);
            oldProp.setValue(newIds[i], newValues[i].clone());
          }
        }
        // java this.property = oldProp.freeze();
        self.property = Some(oldProp.freeze());
      }
      // java else
      else {
        // java this.property = newProperty;
        self.property = Some(newProperty.clone());
      }
    }


    // java public void liteExchangeDocument(Document doc1, Document doc2)
    pub fn liteExchangeDocument(&mut self, doc1: &dyn Document, doc2: &dyn Document) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mSubdocuments[doc2.getType()] = doc1;
      // java this.mSubdocuments[doc1.getType()] = doc2;
    }


    // java public int charCount(int mode)
    pub fn charCount(&self, mode: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java switch (mode)
        // java case 0:
          // java return this.getRange().computeStatistics(WtStatistic.wtStatisticWords);
        // java case 1:
          // java return this.getRange().computeStatistics(WtStatistic.wtStatisticCharacters);
        // java case 2:
          // java return this.getRange().computeStatistics(WtStatistic.wtStatisticCharactersWithSpaces);
        // java default:
          // java throw new IllegalArgumentException();
    }


    // java public String getGlobalInkUserID()
    pub fn getGlobalInkUserID(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mGloabalInkUserID;
    }


    // java public void setGloabalInkUserID(String gloabalInkUserID)
    pub fn setGloabalInkUserID(&mut self, gloabalInkUserID: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mGloabalInkUserID = gloabalInkUserID;
    }


    // java public KDocumentWriteProtection cryptInfo()
    pub fn cryptInfo(&mut self) -> KDocumentWriteProtection {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mDocumentCrypt)
        // java this.mDocumentCrypt = new KDocumentWriteProtection(this);
      // java return this.mDocumentCrypt;
    }


    // java public void setInIOState(boolean bIO)
    pub fn setInIOState(&mut self, bIO: bool) {
      // java this.mStateIO = bIO;
      self.mStateIO = bIO;
    }

    // java public boolean isInIOState()
    pub fn isInIOState(&self) -> bool {
      // java return this.mStateIO;
      self.mStateIO
    }


    // java public boolean isFinishDumpObjects()
    pub fn isFinishDumpObjects(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mFinishDumpObjects;
    }


    // java public void setIoError(int code, Object error)
    pub fn setIoError(&mut self, code: i32, error: &dyn Object) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mIoErrorCode = code;
      // java this.mIoError = error;
    }


    // java public Object getIoError()
    pub fn getIoError(&self) -> Box<dyn Object> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mIoError;
    }


    // java public boolean isInUserAction()
    pub fn isInUserAction(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mTransaction != null && this.mTransaction.getState() == 1;
    }


    // java private void startMonitor()
    pub fn startMonitor(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mLogFileObserver == null)
        // java this.mLogFileObserver = (new KLogFileObserverUtil()).newObserver();
      // java if (this.mLogFileObserver != null)
        // java this.mLogFileObserver.start();
    }


    // java private void stopMonitor()
    pub fn stopMonitor(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mLogFileObserver != null)
        // java this.mLogFileObserver.stop();
        // java this.mLogFileObserver = null;
    }


    // java public boolean isLegal()
    pub fn isLegal(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mIsLegal;
    }


    // java private int loadDocument(DocumentPreliminaryLoader preloader, IOListener ioListener, IOThreadSign aIOThreadSign, FileFormatEnum format)
    pub fn loadDocument(&mut self, preloader: Option<&mut DocumentPreliminaryLoader>, ioListener: &mut dyn IOListener, aIOThreadSign: &IOThreadSign, format: &FileFormatEnum) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null != this.loadListener)
        // java this.loadListener.beforeLoad();
      // java int nRet;
          // java this.startMonitor();
          // java if (preloader == null)
            // java KSLog.d(TAG, "No preloader");
            // java preloader = null != this.mRaf ? new DocumentPreliminaryLoader(this.mRaf, this.mTemplate, this.mProviderInfo) : new DocumentPreliminaryLoader(this.mPath, this.mStream, this.mTemplate, this.mProviderInfo);
            let mut preloader = ();
            // java preloader.setDocumentCallback(this.mDocumentCallback);
            // java preloader.startPreload(false);
          // java else
            // java preloader.setDocumentCallback(this.mDocumentCallback);
            // java preloader.joinLoadFinish();
            // java KSLog.d(TAG, "Document partially loaded");
          // java nRet = preloader.getStatus();
          // java if (nRet == 0)
            // java if (preloader.isOnlineSecurityDoc())
              // java ioListener.beginLoadOnlineSecurityDoc();
            // java nRet = this.doLoadDocument(preloader, ioListener, aIOThreadSign);
            // java if (null != IntListPool.mInstance)
              // java IntListPool.mInstance.dispose();
              // java IntListPool.mInstance = null;
              let mut mInstance = ();
            // java if (null != this.loadListener)
              // java this.loadListener.afterLoad();
              // java this.loadListener = null;
            // java nRet = nRet;
            let mut nRet = ();
            // java return nRet;
          // java ioListener.onError(nRet, preloader.getException());
          // java nRet = nRet;
          let mut nRet = ();
          // java Throwable e
            // java IOLog.e(TAG, "Throwable: ", e);
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
                // java IOLog.e(TAG, "NoSpaceLeftException", e);
                // java nRet = 4;
                let mut nRet = ();
                // java ioListener.onError(nRet, new NoSpaceLeftException(e));
                // java int var26 = nRet;
                let mut var26 = ();
                // java return var26;
              // java (e instanceof OutOfMemoryError)
                // java IOLog.e(TAG, "OutOfMemoryError", e);
                // java nRet = 3;
                let mut nRet = ();
                // java ioListener.onError(nRet, e);
                // java int var25 = nRet;
                let mut var25 = ();
                // java return var25;
              // java (!(e instanceof ForceQuitException))
                  // java (e instanceof ReadingLimitException)
                    // java IOLog.e(TAG, "ReadingLimitException", e);
                    // java nRet = 14;
                    let mut nRet = ();
                    // java ioListener.onError(nRet, e);
                    // java int var24 = nRet;
                    let mut var24 = ();
                    // java return var24;
                  // java (e instanceof ContentLockedException)
                    // java IOLog.e(TAG, "ContentLockedException", e);
                    // java this.getMainDocument().getRangeLock().unLockCurThreadWhenCrash();
                    // java nRet = 2;
                    let mut nRet = ();
                    // java this.onEndLoad();
                    // java ioListener.onError(nRet, e);
                    // java int var23 = nRet;
                    let mut var23 = ();
                    // java return var23;
                // java IOLog.e(TAG, "FileDamagedException", e);
                // java nRet = 2;
                let mut nRet = ();
                // java ioListener.onError(nRet, new FileDamagedException(e));
                // java int var22 = nRet;
                let mut var22 = ();
                // java return var22;
            // java IOLog.e(TAG, "ForceQuitException", e);
            // java nRet = 12;
            let mut nRet = ();
            // java int errorCode = ((ForceQuitException)e).getErrorCode();
              // java (errorCode > 0)
                // java nRet = errorCode;
                let mut nRet = ();
            // java ioListener.onError(nRet, e);
            // java int var8 = nRet;
            let mut var8 = ();
            // java return var8;
              // java (preloader != null)
                // java preloader.dispose();
              // java (this.mOleManager != null)
                // java this.mOleManager.resetDocument();
      // java return nRet;
      // java catch (Throwable e)
      // java if (errorCode > 0)
    }


    // java private int doLoadDocument(DocumentPreliminaryLoader preloader, IOListener ioListener, IOThreadSign aIOThreadSign)
    pub fn doLoadDocument(&mut self, preloader: &DocumentPreliminaryLoader, ioListener: &dyn IOListener, aIOThreadSign: &IOThreadSign) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mOnlineSecurytiTool = preloader.getOnlineSecurityTool();
      // java if (null == this.mOnlineSecurytiTool)
        // java this.mOnlineSecurytiTool = new OnlineSecurityTool();
      // java if (preloader.getSecurityFile() != null)
        // java this.getOleManager().setPath(preloader.getSecurityFile());
      // java FileUtil.newFolder(this.mTmpRootDir);
      // java this.mIsLegal = preloader.isLegal();
      // java int nRet;
      // java if (preloader.isDoc())
        // java nRet = this.loadDocDocument(preloader.getHWPFDocument(), preloader.getEncryptedType(), ioListener);
        // java this.mProviderInfo = preloader.getProviderInfo();
      // java else if (preloader.isDocx())
        // java this.setEvidenceData(this.mOnlineSecurytiTool.getEvidenceData());
        // java RandomAccessFile targetRaf = preloader.getXWPFTargetRaf();
        // java if (null != targetRaf)
          // java this.getOleManager().setRaf(targetRaf);
        // java else
          // java File targetFile = preloader.getXWPFTargetFile();
          // java if (null != targetFile)
            // java String filePath = targetFile.getAbsolutePath();
            // java this.mRaf = null;
            // java this.mPath = filePath;
            // java this.getOleManager().setPath(filePath);
            // java this.getOleManager().setRaf((RandomAccessFile)null);
        // java nRet = this.loadDocxDocument(preloader.getXWPFDocument(), preloader.getXWPFTargetFile(), targetRaf, ioListener);
      // java else if (preloader.isRtf())
        // java if (null != preloader.mReloadPath)
          // java this.mPath = preloader.mReloadPath;
          // java this.mRaf = null;
        // java nRet = this.loadRtfDocument(ioListener);
      // java else if (preloader.isHtml())
        // java if (null != this.mTemplate && null == this.mPath && null != preloader.mPath)
          // java this.mPath = preloader.mPath;
        // java nRet = this.loadHtmlDocument(ioListener, aIOThreadSign);
      // java else if (preloader.isMht())
        // java nRet = this.loadMhtDocument(ioListener, aIOThreadSign);
      // java else if (preloader.isXml07())
        // java String path = preloader.mPath;
        let mut path = ();
        // java RandomAccessFile raf = preloader.getRaf();
        // java if (null != preloader.getEncryptedType() && EncryptedType.None != preloader.getEncryptedType())
          // java path = preloader.getXml07Decrypted();
          // java raf = null;
          let mut raf = ();
        // java nRet = this.loadXml07Document(path, raf, ioListener);
      // java else
        // java if (preloader.isEpub())
          // java nRet = this.loadEpubDocument(ioListener, aIOThreadSign);
          // java return nRet;
        // java if (preloader.isTxt())
          // java nRet = this.loadTxtDocument(preloader.getTXTDocument(), ioListener, aIOThreadSign);
        // java else if (preloader.isXml03())
          // java nRet = this.loadXml03Document(this.mPath, ioListener);
        // java else if (preloader.isUot3())
          // java String path = preloader.mPath;
          let mut path = ();
          // java RandomAccessFile raf = preloader.getRaf();
          // java if (null != preloader.getEncryptedType() && EncryptedType.UOF == preloader.getEncryptedType())
            // java path = preloader.getUot3Decrypted();
            // java RandomAccessFile var13 = null;
            let mut var13 = ();
          // java nRet = this.loadUot3Document(path, ioListener);
        // java else
          // java Assert.fail("it should not reach here");
          // java nRet = this.loadTxtDocument(preloader.getTXTDocument(), ioListener, aIOThreadSign);
      // java if (0 != nRet)
        // java KSLog.d(TAG, "Open file failed: " + this.mPath);
      // java if (null != IntListPool.mInstance)
        // java IntListPool.mInstance.dispose();
        // java IntListPool.mInstance = null;
        let mut mInstance = ();
      // java this.getTransaction().clearHistory();
      // java this.getSaveAccelerator().setForbiddenAutoSave(false);
      // java return nRet;
    }


    // java private int loadHtmlDocument(IOListener ioListener, IOThreadSign aIOThreadSign)
    pub fn loadHtmlDocument(&mut self, ioListener: &mut dyn IOListener, aIOThreadSign: &IOThreadSign) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
          // java this.ff = FileFormat.FF_HTML;
          // java IHtmlReader reader = ReflectionHelper.getHtmlReader(this, new File(this.mPath), (SubDocument)this.mSubdocuments[0], this.mSubdocuments[0].getLength(), false, ioListener);
          // java int currentCp = reader.read();
          // java int nRet = -1 == currentCp ? 2 : 0;
          let mut nRet = ();
          // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
          // java if (null != this.mOnModifiedListener)
            // java this.mOnModifiedListener.onAllowAutoSave();
          // java ioListener.onFinishDumpObjects();
          // java PictureDumpMap.reset();
          // java return nRet;
          // java Exception var6
            // java ioListener.onHtmlOpenError();
            // java return 2;
      // java catch (Exception var6)
    }


    // java private int loadMhtDocument(IOListener ioListener, IOThreadSign aIOThreadSign)
    pub fn loadMhtDocument(&mut self, ioListener: &mut dyn IOListener, aIOThreadSign: &IOThreadSign) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
          // java this.ff = FileFormat.FF_MHT;
          // java IMhtReader mhtReader = MhtReflectionHelper.getMhtReader(this, this.mPath, (RandomAccessFile)null, false);
          // java String htmlPath = mhtReader.getHtmlPath();
          // java if (null == htmlPath)
            // java return 2;
          // java else
            // java IHtmlReader reader = ReflectionHelper.getHtmlReader(this, new File(htmlPath), (SubDocument)this.mSubdocuments[0], this.mSubdocuments[0].getLength(), false, ioListener);
            // java int currentCp = reader.read();
            // java int nRet = -1 == currentCp ? 2 : 0;
            let mut nRet = ();
            // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
            // java if (null != this.mOnModifiedListener)
              // java this.mOnModifiedListener.onAllowAutoSave();
            // java ioListener.onFinishDumpObjects();
            // java PictureDumpMap.reset();
            // java return nRet;
          // java Exception var8
            // java ioListener.onHtmlOpenError();
            // java return 2;
      // java catch (Exception var8)
    }


    // java private int loadDocDocument(HWPFDocument diskDoc, EncryptedType encryptedType, IOListener ioListener)
    pub fn loadDocDocument(&mut self, diskDoc: &mut HWPFDocument, encryptedType: &EncryptedType, ioListener: &dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int nRet = 0;
      let mut nRet = ();
      // java if (this.mTemplate != null)
        // java this.ff = this.toFileFormat(this.mTemplate);
      // java else
        // java this.ff = FileFormat.FF_DOC;
      // java DocReader reader = null;
      let mut reader = ();
          // java reader = new DocReader(this, diskDoc, ioListener, this.mDlProvider);
          let mut reader = ();
          // java this.getOleManager().initDocDocument(diskDoc);
          // java reader.read();
          // java this.endDocReader(ioListener, nRet, reader);
          // java int e = nRet;
          let mut e = ();
          // java return e;
          // java ReadingLimitException var10
            // java this.endDocReader(ioListener, nRet, reader);
            // java this.getOleManager().resetDocDocument();
            // java diskDoc.dispose();
      // java return nRet;
    }


    // java private void endDocReader(IOListener ioListener, int nRet, DocReader reader)
    pub fn endDocReader(&mut self, ioListener: &mut dyn IOListener, nRet: i32, reader: &mut DocReader) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.endLoading(ioListener, nRet, reader, (IReaderBase)null);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java reader.dispose();
      // java DocReader var4 = null;
      let mut var4 = ();
      // java ioListener.onFinishDumpObjects();
      // java PictureDumpMap.reset();
    }


    // java private FileFormat toFileFormat(FileFormatEnum template)
    pub fn toFileFormat(&mut self, template: &FileFormatEnum) -> FileFormat {
      todo!("DEFERRED_NON_FUNCTION");
      // java switch (template)
        // java case DOC:
          // java return FileFormat.FF_DOC;
        // java case TXT:
          // java return FileFormat.FF_TXT;
        // java default:
          // java return FileFormat.FF_DOC;
    }


    // java private int loadRtfDocument(IOListener ioListener)
    pub fn loadRtfDocument(&mut self, ioListener: &mut dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_RTF;
      // java IRtfReader reader = this.getRtfReader(this, this.mPath, this.mRaf, ioListener);
      // java reader.read();
      // java int nRet = 0;
      let mut nRet = ();
      // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java ioListener.onFinishDumpObjects();
      // java return nRet;
    }


    // java public IRtfReader getRtfReader(TextDocument textDocument, String path, RandomAccessFile raf, IOListener listener)
    pub fn getRtfReader(&self, textDocument: &TextDocument, path: &str, raf: &RandomAccessFile, listener: &dyn IOListener) -> Option<Box<dyn IRtfReader>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java ClassLoader cl = null;
      let mut cl = ();
      // java if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
        // java cl = IClassLoaderManager.getInstance().getWriterCoreExtensionClassLoader();
      // java else
        // java cl = this.getClass().getClassLoader();
      // java if (cl == null)
        // java return null;
      // java else
        // java IRtfReader rtfReader = null;
        let mut rtfReader = ();
            // java Class<?> clazz = cl.loadClass("cn.wps.moffice.writer.io.reader.rtf.RtfReader");
            // java Class<?>[] argTypes = new Class[]{TextDocument.class, String.class, RandomAccessFile.class, IOListener.class};
            let mut argTypes = ();
            // java Constructor<?> constructor = clazz.getConstructor(argTypes);
            // java rtfReader = (IRtfReader)constructor.newInstance(textDocument, path, raf, listener);
            // java ClassNotFoundException e
              // java KSLog.e(TAG, "ClassNotFoundException", e);
            // java InstantiationException e
              // java KSLog.e(TAG, "InstantiationException", e);
            // java IllegalAccessException e
              // java KSLog.e(TAG, "IllegalAccessException", e);
            // java NoSuchMethodException e
              // java KSLog.e(TAG, "NoSuchMethodException", e);
            // java IllegalArgumentException e
              // java KSLog.e(TAG, "IllegalArgumentException", e);
            // java InvocationTargetException e
              // java KSLog.e(TAG, "InvocationTargetException", e);
        // java Assert.assertNotNull("rtfReader should not be null.", rtfReader);
        // java return rtfReader;
    }


    // java private int loadDocxDocument(XWPFDocument diskDoc, File targetFile, RandomAccessFile raf, IOListener ioListener)
    pub fn loadDocxDocument(&mut self, diskDoc: &XWPFDocument, targetFile: &std::path::PathBuf, raf: &RandomAccessFile, ioListener: &dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_DOCX;
      // java int nRet = 0;
      let mut nRet = ();
      // java IDocxReader reader = null;
      let mut reader = ();
          // java reader = ReflectionHelper.getDocxReader(this, this, ioListener, diskDoc, targetFile, raf, this.mDlProvider);
          // java this.getOleManager().initDocxDocument(diskDoc);
          // java reader.read();
          // java this.endDocxReader(ioListener, nRet, reader);
          // java return nRet;
          // java ReadingLimitException var8
            // java this.endDocxReader(ioListener, nRet, reader);
            // java return nRet;
    }


    // java private void endDocxReader(IOListener ioListener, int nRet, IDocxReader docxReader)
    pub fn endDocxReader(&mut self, ioListener: &mut dyn IOListener, nRet: i32, docxReader: &mut dyn IDocxReader) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.endLoading(ioListener, nRet, (DocReader)null, docxReader);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java this.getOleManager().resetDocxDocument();
      // java docxReader.dispose();
      // java ioListener.onFinishDumpObjects();
      // java PictureDumpMap.reset();
    }


    // java private int loadXml03Document(String path, IOListener ioListener)
    pub fn loadXml03Document(&mut self, path: &str, ioListener: &mut dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_XML03;
      // java IXml03Reader reader = ReflectionHelper.getXml03Reader(this, this, ioListener, path, this.mDlProvider);
      // java reader.read();
      // java int nRet = 0;
      let mut nRet = ();
      // java this.endLoading(ioListener, nRet, (DocReader)null, reader);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java IXml03Reader var5 = null;
      let mut var5 = ();
      // java ioListener.onFinishDumpObjects();
      // java return nRet;
    }


    // java private int loadXml07Document(String path, RandomAccessFile raf, IOListener ioListener)
    pub fn loadXml07Document(&mut self, path: &str, raf: &RandomAccessFile, ioListener: &mut dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_XML07;
      // java IXml07Reader reader = null;
      let mut reader = ();
      // java reader = ReflectionHelper.getXml07Reader(this, this, ioListener, path, raf, this.mDlProvider);
      // java reader.read();
      // java int nRet = 0;
      let mut nRet = ();
      // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java reader = null;
      let mut reader = ();
      // java ioListener.onFinishDumpObjects();
      // java return nRet;
    }


    // java private int loadEpubDocument(IOListener ioListener, IOThreadSign ioThreadSign)
    pub fn loadEpubDocument(&mut self, ioListener: &dyn IOListener, ioThreadSign: &IOThreadSign) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_EPUB;
      // java int nRet = 0;
      let mut nRet = ();
      // java IEbookReader epubReader = null;
      let mut epubReader = ();
          // java epubReader = ReflectionHelper.getEpubReader(this, ioListener, ioThreadSign, this.mDlProvider);
          // java if (epubReader == null)
            // java nRet = -1;
            let mut nRet = ();
          // java else
            // java epubReader.read();
          // java this.endEpubReader(ioListener, nRet, epubReader);
          // java return nRet;
          // java ReadingLimitException var6
            // java this.endEpubReader(ioListener, nRet, epubReader);
            // java return nRet;
    }


    // java private int loadUot3Document(String path, IOListener ioListener)
    pub fn loadUot3Document(&mut self, path: &str, ioListener: &mut dyn IOListener) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_UOT3;
      // java IUot3Reader reader = ReflectionHelper.getUot3Reader(this, this, ioListener, path, this.mRaf, this.mDlProvider);
      // java reader.read();
      // java int nRet = 0;
      let mut nRet = ();
      // java this.endLoading(ioListener, nRet, (DocReader)null, reader);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java this.getOleManager().resetDocDocument();
      // java reader.dispose();
      // java IUot3Reader var5 = null;
      let mut var5 = ();
      // java ioListener.onFinishDumpObjects();
      // java PictureDumpMap.reset();
      // java return nRet;
    }


    // java private int loadTxtDocument(TXTDocument disktxt, IOListener ioListener, IOThreadSign aIOThreadSign)
    pub fn loadTxtDocument(&mut self, disktxt: &TXTDocument, ioListener: &dyn IOListener, aIOThreadSign: &IOThreadSign) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = FileFormat.FF_TXT;
      // java int nRet = 0;
      let mut nRet = ();
      // java TextReader reader = null;
      let mut reader = ();
          // java reader = new TextReader(this, ioListener, aIOThreadSign, disktxt, this.mDlProvider);
          let mut reader = ();
          // java reader.read();
          // java this.endTextReader(ioListener, nRet, reader);
          // java ReadingLimitException var7
            // java this.endTextReader(ioListener, nRet, reader);
          // java Exception e
            // java IOLog.e(TAG, e.getMessage());
      // java return nRet;
      // java catch (Exception e)
    }


    // java private void endEpubReader(IOListener ioListener, int nRet, IEbookReader reader)
    pub fn endEpubReader(&mut self, ioListener: &mut dyn IOListener, nRet: i32, reader: Option<&mut dyn IEbookReader>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java if (reader != null)
        // java reader.dispose();
      // java ioListener.onFinishDumpObjects();
    }


    // java private void endTextReader(IOListener ioListener, int nRet, TextReader reader)
    pub fn endTextReader(&mut self, ioListener: &mut dyn IOListener, nRet: i32, reader: &mut TextReader) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.endLoading(ioListener, nRet, (DocReader)null, (IReaderBase)null);
      // java if (null != this.mOnModifiedListener)
        // java this.mOnModifiedListener.onAllowAutoSave();
      // java reader.dispose();
      // java TextReader var4 = null;
      let mut var4 = ();
      // java ioListener.onFinishDumpObjects();
    }


    // java public int openDocument(String strPath)
    pub fn openDocument__string(&mut self, strPath: &str) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((String)strPath, (InputStream)null, (FileFormatEnum)null, (String)null);
    }


    // java public int openDocument(String strPath, String passwd)
    pub fn openDocument__string_string(&mut self, strPath: &str, passwd: &str) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((String)strPath, (InputStream)null, (FileFormatEnum)null, (String)passwd);
    }


    // java public int openDocument(RandomAccessFile raf)
    pub fn openDocument__randomaccessfile(&mut self, raf: &RandomAccessFile) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((RandomAccessFile)raf, (FileFormatEnum)null, (String)null, (IDLProvider)null);
    }


    // java public int openDocument(RandomAccessFile raf, String passwd)
    pub fn openDocument__randomaccessfile_string(&mut self, raf: &RandomAccessFile, passwd: &str) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((RandomAccessFile)raf, (FileFormatEnum)null, (String)passwd, (IDLProvider)null);
    }


    // java public int openDocument(RandomAccessFile raf, String passwd, IDLProvider dlProvider)
    pub fn openDocument__randomaccessfile_string_idlprovider(&mut self, raf: &RandomAccessFile, passwd: &str, dlProvider: &dyn IDLProvider) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((RandomAccessFile)raf, (FileFormatEnum)null, (String)passwd, (IDLProvider)dlProvider);
    }


    // java public int openDocument(InputStream stream, String passwd, IDLProvider dlProvider)
    pub fn openDocument__inputstream_string_idlprovider(&mut self, stream: &dyn std::io::Read, passwd: &str, dlProvider: &dyn IDLProvider) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument((String)null, stream, (FileFormatEnum)null, passwd, dlProvider);
    }


    // java public int openDocument(String strPath, String passwd, IDLProvider dlProvider)
    pub fn openDocument__string_string_idlprovider(&mut self, strPath: &str, passwd: &str, dlProvider: &dyn IDLProvider) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.openDocument(strPath, (InputStream)null, (FileFormatEnum)null, passwd, dlProvider);
    }


    // java public int openDocument(String name, String path, String passwd, boolean isModified)
    pub fn openDocument__string_string_string_boolean(&mut self, name: &str, path: &str, passwd: &str, isModified: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int result = this.openDocument(path, passwd);
      // java this.init(name, isModified);
      // java return result;
    }


    // java public int openDocument(String name, RandomAccessFile raf, String passwd, boolean isModified)
    pub fn openDocument__string_randomaccessfile_string_boolean(&mut self, name: &str, raf: &RandomAccessFile, passwd: &str, isModified: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int result = this.openDocument(raf, passwd);
      // java this.init(name, isModified);
      // java return result;
    }


    // java public int openDocument(String name, String path, String passwd, boolean isModified, IDLProvider idlProvider)
    pub fn openDocument__string_string_string_boolean_idlprovider(&mut self, name: &str, path: &str, passwd: &str, isModified: bool, idlProvider: &dyn IDLProvider) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int result = this.openDocument(path, passwd, idlProvider);
      // java this.init(name, isModified);
      // java return result;
    }


    // java private void init(String name, boolean isModified)
    pub fn init__string_boolean(&mut self, name: &str, isModified: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mName = name;
      // java this.mOpenTime = new Date();
      // java this.mModified = isModified ? ModifyFlag._true : ModifyFlag._false;
    }


    // java private int insert(TextDocument another, int cp)
    pub fn insert(&mut self, another: &TextDocument, cp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document mainDocument = this.getMainDocument();
      // java KRange insertRange = new KRange(mainDocument, cp);
      let mut insertRange = ();
      // java int insertLength = another.getMainDocument().getLength();
      // java boolean isEnd = cp == mainDocument.getLength();
      // java if (isEnd)
        // java insertRange.insertParagraph();
      // java InsertFile insertFile = new InsertFile();
      let mut insertFile = ();
      // java InsertEnv insertEnv = new InsertEnv();
      let mut insertEnv = ();
      // java insertEnv.mSrcTextDocument = another;
      let mut mSrcTextDocument = ();
      // java insertEnv.mDestTextDocument = this;
      let mut mDestTextDocument = ();
      // java insertEnv.mInsertRange = insertRange;
      let mut mInsertRange = ();
      // java insertEnv.mShapeRange = new KShapeRange(mainDocument, insertRange);
      let mut mShapeRange = ();
      // java insertEnv.mFieldDepth = 0;
      let mut mFieldDepth = ();
      // java insertFile.Insert(insertEnv);
      // java if (isEnd)
        // java insertRange.setRange(insertRange.getStart() - 1, insertRange.getEnd());
        // java insertRange.deleteRange();
      // java insertRange.dispose();
      // java this.clearUndoRedo();
      // java return insertLength;
    }


    // java public boolean isEncryption()
    pub fn isEncryption(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null == this.mOnlineSecurytiTool ? false : this.mOnlineSecurytiTool.isEnable();
    }


    // java public boolean isProEncryption()
    pub fn isProEncryption(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mDocumentCallback != null && this.mDocumentCallback.isProEncryption();
    }


    // java public int insertBefore(TextDocument another)
    pub fn insertBefore(&mut self, another: &TextDocument) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.insert(another, 0);
    }


    // java public int insertAfter(TextDocument another)
    pub fn insertAfter(&mut self, another: &TextDocument) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.insert(another, this.getMainDocument().getLength());
    }


    // java public boolean isLoadSuccess()
    pub fn isLoadSuccess(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mLoadSuccess;
    }


    // java public void setLoadSuccess(boolean bLoadSuccess)
    pub fn setLoadSuccess(&mut self, bLoadSuccess: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mLoadSuccess = bLoadSuccess;
    }


    // java public KListTemplates getListTemplates()
    pub fn getListTemplates(&mut self) -> KListTemplates {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mListTemplates)
        // java this.mListTemplates = new KListTemplates(this);
      // java return this.mListTemplates;
    }


    // java public KLists getLists()
    pub fn getLists(&mut self) -> KLists {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mLists)
        // java this.mLists = new KLists(this);
      // java return this.mLists;
    }


    // java public ListParagraphs getListParagraphs()
    pub fn getListParagraphs(&self) -> Option<Box<dyn ListParagraphs>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null;
    }


    // java public AutoNumTable getAutoNumTable()
    pub fn getAutoNumTable(&mut self) -> &mut AutoNumTable {
      // java if (this.mAutoNumTable == null)
      if self.mAutoNumTable.is_none() {
        // java this.mAutoNumTable = new AutoNumTable(this.mTransaction);
        let transaction = self.super_.getTransaction();
        self.mAutoNumTable = Some(AutoNumTable::AutoNumTable(transaction));
      }
      // java return this.mAutoNumTable;
      self.mAutoNumTable.as_mut().expect("mAutoNumTable")
    }


    // java public String getName()
    pub fn getName(&self) -> String {
      // rust: ★T7-7 —— **管线可达，恢复真实实现**（`FILENAME`/`INFO` 域取值路径）。
      // java String strName;
      let strName: String;
      // java if (this.mName != null)
      if let Some(name) = &self.mName {
        // java strName = this.mName;
        strName = name.clone();
      }
      // java else if (this.mPath != null && !this.mIsAutoSaveFile)
      else if let Some(path) = &self.mPath {
        if !self.mIsAutoSaveFile {
          // java int index = this.mPath.lastIndexOf(File.separator);
          // rust: Java `File.separator` 在 POSIX 与 Windows 上分别为 `/` 与 `\`；
          //   此处按两平台分隔符取**最后一个**，等价于 Java 的 `lastIndexOf`。
          let index = path.rfind(['/', '\\']).map(|i| i as i32).unwrap_or(-1);
          // java if (index == -1)
          if index == -1 {
            // java strName = this.mPath;
            strName = path.clone();
          }
          // java else
          else {
            // java strName = this.mPath.substring(index + 1);
            strName = path[(index + 1) as usize..].to_string();
          }
        }
        // java else（mIsAutoSaveFile == true）→ 落到最后的 else 分支
        else {
          // java this.generateName();  （`&self` 无法可变调用 ⇒ 语义等价：
          //   `generateName()` 的唯一效果是 `this.mName = "noname"`）
          strName = "noname".to_string();
        }
      }
      // java else
      else {
        // java this.generateName();
        // java strName = this.mName;
        // rust: 同上 —— `generateName()` 逐字即 `this.mName = "noname"`。
        strName = "noname".to_string();
      }
      // java return strName;
      strName
    }


    // java public String getPath()
    pub fn getPath(&self) -> String {
      // rust: ★T7-7 —— **管线可达，恢复真实实现**（`FILESIZE`/`FILENAME` 域取值路径）。
      // java return this.mPath;
      // rust: Java `mPath` 可为 `null`（String 引用）；Rust 门面以空串承载 null
      //   （调用方 `FileNamePaser` 随后走 `toCharArray()`，null 与空串同效）。
      self.mPath.clone().unwrap_or_default()
    }


    // java public RandomAccessFile getRaf()
    pub fn getRaf(&self) -> RandomAccessFile {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mRaf;
    }


    // java @Deprecated
    // java public void save()
    pub fn save(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.saveAs(this.mPath);
    }


    // java @Deprecated
    // java public void saveAs(String strPath)
    pub fn saveAs__string(&mut self, strPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.saveFile(strPath);
      // java this.onSaveSuccessed(strPath);
    }


    // java @Deprecated
    // java public void saveFile(String strPath)
    pub fn saveFile__string(&mut self, strPath: &mut str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java String ext = strPath.substring(strPath.lastIndexOf(46) + 1);
      // java FileFormat format = FileFormat.fromExtension(ext);
      // java this.saveFile(strPath, format);
    }


    // java @Deprecated
    // java public void saveAs(String strPath, SecurityMode mode, String fileName)
    pub fn saveAs__string_securitymode_string(&mut self, strPath: &mut str, mode: &SecurityMode, fileName: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java String ext = strPath.substring(strPath.lastIndexOf(46) + 1);
      // java FileFormat format = FileFormat.fromExtension(ext);
      // java this.saveFile(strPath, format, mode, fileName, false);
      // java this.onSaveSuccessed(strPath);
    }


    // java public void setSaveSuccessedListener(ISaveSuccessedListener saveSuccessedListener)
    pub fn setSaveSuccessedListener(&mut self, saveSuccessedListener: &dyn ISaveSuccessedListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mSaveSuccessedListener = saveSuccessedListener;
    }


    // java public void onSaveSuccessed(String strPath)
    pub fn onSaveSuccessed(&mut self, strPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mModified = ModifyFlag._false;
      // java this.mIsAutoSaveFile = false;
      // java this.mHasStructureError = false;
      // java this.mParasID.setCorrectedParaId4IO(false);
      // java this.mPath = StringUtil.normalizePath(strPath);
      // java this.mName = null;
      // java this.updateModifys();
      // java if (this.mSaveSuccessedListener != null)
        // java this.mSaveSuccessedListener.onSaveSuccessed(strPath);
    }


    // java public void onEvidenceSuccessd(String strPath)
    pub fn onEvidenceSuccessd(&mut self, strPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.onSaveSuccessed(strPath);
      // java this.ff = FileFormat.FF_DOCX;
    }


    // java public void saveFile(String strPath, FileFormat format)
    pub fn saveFile__string_fileformat(&mut self, strPath: &str, format: &FileFormat) {
      todo!("DEFERRED_NON_FUNCTION");
      // java WordWriter writer = null;
      let mut writer = ();
      // java if (FileFormat.FF_DOC != format && FileFormat.FF_DOCX != format && FileFormat.FF_DOTX != format && FileFormat.FF_XML07 != format && FileFormat.FF_XML03 != format && FileFormat.FF_UOT3 != format)
        // java writer = new WordWriter(this, strPath, format);
        let mut writer = ();
      // java else
        // java writer = new WordWriter(this, strPath, format, this.mProviderInfo);
        let mut writer = ();
      // java this.ff = format;
      // java this.updateShape();
          // java writer.save();
          // java InvalidFormatException e
            // java Log.w(TAG, "normal save should not throws WriterAutoWriteException", e);
      // java writer = null;
      let mut writer = ();
      // java if (VersionState.isProVersion())
        // java if (!this.isEncryption() && !this.isProEncryption())
          // java this.saveFile4Normal(strPath, format);
        // java else
          // java this.saveFile4Encryption(strPath, format, this.mOnlineSecurytiTool, this.mEvidenceData != null);
      // java String ext = strPath.substring(strPath.lastIndexOf(46) + 1)
      // java FileFormat format = FileFormat.fromExtension(ext)
      // java this.saveFile(strPath, format)
    }


    // java public void saveFile(String strPath, FileFormat format, SecurityMode mode, String fileName, boolean isExport)
    pub fn saveFile__string_fileformat_securitymode_string_boolean(&mut self, strPath: &str, format: &mut FileFormat, mode: &SecurityMode, fileName: &str, isExport: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java OnlineSecurityTool tool = new OnlineSecurityTool(this.mOnlineSecurytiTool);
      let mut tool = ();
      // java if (SecurityMode.Security == mode)
        // java tool.clear();
        // java tool.setFileName(fileName);
        // java tool.enable();
      // java else if (SecurityMode.Normal == mode)
        // java tool.disable();
      // java if (tool.isEnable())
        // java format = FileFormat.FF_DOCX;
      // java if (!isExport)
        // java this.ff = format;
      // java this.updateShape();
      // java if (tool.isEnable() || VersionState.isProVersion() && this.isProEncryption())
        // java this.saveFile4Encryption(strPath, format, tool, this.mEvidenceData != null);
      // java else
        // java this.saveFile4Normal(strPath, format);
      // java if (!isExport)
        // java this.mOnlineSecurytiTool.copyFrom(tool);
    }


    // java public void exportSecurityFile(String strPath, OnlineSecurityTool tool)
    pub fn exportSecurityFile(&mut self, strPath: &str, tool: &OnlineSecurityTool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java FileFormat format = this.ff;
      let mut format = ();
      // java this.ff = FileFormat.FF_DOCX;
      // java this.updateShape();
          // java this.saveFile4Encryption(strPath, FileFormat.FF_DOCX, tool, true);
          // java WriterIOException e
            // java e.printStackTrace();
            // java this.ff = format;
    }


    // java private void saveFile4Normal(String strPath, FileFormat format)
    pub fn saveFile4Normal(&self, strPath: &str, format: &FileFormat) {
      todo!("DEFERRED_NON_FUNCTION");
      // java WordWriter writer = this.getWriter(format, strPath);
          // java writer.save();
          // java InvalidFormatException e
            // java Log.w(TAG, "normal save should not throws WriterAutoWriteException", e);
    }


    // java private void saveFile4Encryption(String strPath, FileFormat format, OnlineSecurityTool tool, boolean forAllUser)
    pub fn saveFile4Encryption(&mut self, strPath: &str, format: &FileFormat, tool: Option<&OnlineSecurityTool>, forAllUser: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java File tmp1 = null;
      let mut tmp1 = ();
      // java File tmp;
      // java if (VersionState.isProVersion())
        // java tmp = FileUtil.getSaveOrTempFile("writer.save2");
      // java else
        // java tmp = Platform.createTempFile("writer", ".save2");
      // java String tmpPath = tmp.getAbsolutePath();
      // java WordWriter writer = this.getWriter(format, tmpPath);
          // java writer.save();
          // java if (null == tool || !tool.isEnable() || VersionState.isProVersion() && this.isProEncryption())
            // java if (VersionState.isProVersion() && !tool.isEnable() && this.isProEncryption())
              // java this.encryptFileByAgentClient(tmpPath, strPath);
            // java else if (VersionState.isProVersion() && tool.isEnable() && this.isProEncryption())
              // java tmp1 = FileUtil.getSaveOrTempFile("writer.save3");
              // java String tmpPath1 = tmp1.getAbsolutePath();
              // java encryptFileByOnlineSecurity(tmpPath, tmpPath1, tool, forAllUser);
              // java this.encryptFileByAgentClient(tmpPath1, strPath);
          // java else
            // java encryptFileByOnlineSecurity(tmpPath, strPath, tool, forAllUser);
          // java InvalidFormatException e
            // java Log.w(TAG, "normal save should not throws WriterAutoWriteException", e);
              // java (null != tmp && tmp.exists())
                // java tmp.delete();
              // java (VersionState.isProVersion() && null != tmp1 && tmp1.exists())
                // java tmp1.delete();
    }


    // java private boolean encryptFileByAgentClient(String tempPath, String dstPath)
    pub fn encryptFileByAgentClient(&mut self, tempPath: &str, dstPath: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mDocumentCallback == null)
        // java return false;
      // java else
            // java return this.mDocumentCallback.encryptFile(tempPath, dstPath);
            // java Exception e
              // java Log.e(TAG, "Exception", e);
              // java return false;
      // java catch (Exception e)
    }


    // java private static void encryptFileByOnlineSecurity(String tempPath, String dstPath, OnlineSecurityTool tool, boolean forAllUser)
    pub fn encryptFileByOnlineSecurity(tempPath: &str, dstPath: &str, tool: &mut OnlineSecurityTool, forAllUser: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java InputStream template = null;
      let mut template = ();
          // java template = Platform.getAssetManager().open("template/pro/secdoctemplate.doc");
          // java tool.encrypt(dstPath, tempPath, template, forAllUser);
          // java IOException e
            // java throw new WriterIOException("online security encrypt file error!: " + e.getMessage());
            // java FileUtil.closeQuietly(template);
      // java catch (IOException e)
    }


    // java private WordWriter getWriter(FileFormat format, String tmpPath)
    pub fn getWriter(&self, format: &FileFormat, tmpPath: &str) -> WordWriter {
      todo!("DEFERRED_NON_FUNCTION");
      // java switch (format)
        // java case FF_DOC: case FF_DOCX: case FF_DOTX: case FF_XML07: case FF_UOT3:
          // java return new WordWriter(this, tmpPath, format, this.mProviderInfo);
        // java default:
          // java return new WordWriter(this, tmpPath, format);
    }


    // java private void updateShape()
    pub fn updateShape(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java ShapeTool.updateAllOlePicID(this.getMainDocument());
      // java ShapeTool.updateAllOlePicID(this.getSubdocument(2));
    }


    // java public void backUp(String backUpPath, FileFormat format)
    pub fn backUp(&mut self, backUpPath: &str, format: &FileFormat) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.updateShape();
      // java WordWriter writer = null;
      let mut writer = ();
      // java if (FileFormat.FF_DOC != format && FileFormat.FF_DOCX != format && FileFormat.FF_DOTX != format && FileFormat.FF_XML07 != format && FileFormat.FF_UOT3 != format)
        // java writer = new WordWriter(this, backUpPath, format);
        let mut writer = ();
      // java else
        // java writer = new WordWriter(this, backUpPath, format, this.mProviderInfo);
        let mut writer = ();
      // java this.getSaveAccelerator().setQuickSave(true);
          // java writer.autoSave();
            // java this.getSaveAccelerator().setQuickSave(false);
      // java this.mModified.setAutoSave(true);
    }


    // java public void qingSaveBackup(String backUpPath, FileFormat format)
    pub fn qingSaveBackup(&mut self, backUpPath: &str, format: &FileFormat) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.ff = format;
      // java this.updateShape();
      // java WordWriter writer = null;
      let mut writer = ();
      // java if (FileFormat.FF_DOC != format && FileFormat.FF_DOCX != format && FileFormat.FF_DOTX != format && FileFormat.FF_XML07 != format && FileFormat.FF_UOT3 != format)
        // java writer = new WordWriter(this, backUpPath, format);
        let mut writer = ();
      // java else
        // java writer = new WordWriter(this, backUpPath, format, this.mProviderInfo);
        let mut writer = ();
      // java writer.save();
      // java writer = null;
      let mut writer = ();
      // java this.mModified.setAutoSave(true);
    }


    // java public FileFormat getFileFormat()
    pub fn getFileFormat(&self) -> FileFormat {
      // java return this.ff;
      self.ff.unwrap_or(FileFormat::FF_DOC)
    }


    // java public void saveToSrcFileFlag(String path)
    pub fn saveToSrcFileFlag(&mut self, path: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mModified = ModifyFlag._false;
      // java this.mPath = StringUtil.normalizePath(path);
      // java this.mName = null;
      // java this.updateModifys();
    }


    // java public void close()
    pub fn close(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.dispose();
    }


    // java private void updateModifys()
    pub fn updateModifys(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
          // java this.updateActions_redo(this.getTransaction().getActionList().redo_actions);
          // java this.updateActions_undo(this.getTransaction().getActionList().undo_actions);
          // java IndexOutOfBoundsException e
            // java e.printStackTrace();
    }


    // java private static void getLeafCommands(Action action, List<Command> results)
    pub fn getLeafCommands(action: &Action, results: &mut Vec<Command>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java for(Command c : action.getCommands())
        // java if (c instanceof Action)
          // java getLeafCommands((Action)c, results);
        // java else
          // java results.add(c);
      // java for(Command c : action.getCommands())
    }


    // java private void updateActions_undo(KArrayDeque<Action> actions)
    pub fn updateActions_undo(&mut self, actions: &KArrayDeque<Action>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = actions.size();
      // java for(int j = 0; j < size ++j)
        // java Action action = (Action)actions.get(j);
        // java ArrayList<Command> commands = new ArrayList(50);
        let mut commands = ();
        // java getLeafCommands(action, commands);
        // java int ncommands = commands.size();
        // java for(int k = 0; k < ncommands ++k)
          // java Command commandBase = (Command)commands.get(k);
          // java if (this.fixCommand(commandBase))
            // java break;
      // java for(int j = 0; j < size; ++j)
      // java for(int k = 0; k < ncommands; ++k)
    }


    // java private void updateActions_redo(ArrayList<Action> actions)
    pub fn updateActions_redo(&mut self, actions: &Vec<Action>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = actions.size();
      // java for(int j = size - 1; j >= 0 --j)
        // java Action action = (Action)actions.get(j);
        // java ArrayList<Command> commands = new ArrayList(50);
        let mut commands = ();
        // java getLeafCommands(action, commands);
        // java int ncommands = commands.size();
        // java for(int k = ncommands - 1; k >= 0 --k)
          // java Command commandBase = (Command)commands.get(k);
          // java if (this.fixCommand(commandBase))
            // java break;
      // java for(int j = size - 1; j >= 0; --j)
      // java for(int k = ncommands - 1; k >= 0; --k)
    }


    // java private boolean fixCommand(Command commandBase)
    pub fn fixCommand(&self, commandBase: Option<&Command>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java int index = this.getFieldIndex(this.getClass(), ModifyFlag.class);
      // java if (commandBase != null && index >= 0 && commandBase instanceof AtomCommand)
          // java AtomCommand command = (AtomCommand)commandBase;
          let mut command = ();
          // java if (command.getAtom() instanceof TextDocument)
            // java command.getAtomData()[index] = ModifyFlag._true;
            // java return true;
      // java return false;
    }


    // java public void setOnModifiedListener(OnModifiedListener listener)
    pub fn setOnModifiedListener(&mut self, _listener: &dyn OnModifiedListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mOnModifiedListenerTemp = listener;
    }


    // java public void setOnModifiedListener4Stat(OnModifiedListener listener)
    pub fn setOnModifiedListener4Stat(&mut self, _listener: &dyn OnModifiedListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mOnModifiedListener4Stat = listener;
    }


    // java public void addWriterOpenEventListener(KmoWriterOpenEventListener listener)
    pub fn addWriterOpenEventListener(&mut self, listener: &dyn KmoWriterOpenEventListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.openListeners.add(listener);
    }


    // java public void removeWriterOpenEventListener(KmoWriterOpenEventListener listener)
    pub fn removeWriterOpenEventListener(&mut self, listener: &dyn KmoWriterOpenEventListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.openListeners.remove(listener);
    }


    // java public boolean isModified()
    pub fn isModified(&self) -> bool {
      // java return this.mModified.isModified();
      self.mModified.as_ref().expect("TextDocument.mModified is null").isModified()
    }


    // java public boolean isAutoSave()
    pub fn isAutoSave(&self) -> bool {
      // java return this.mModified.isAutoSave();
      self.mModified.as_ref().expect("TextDocument.mModified is null").isAutoSave()
    }


    // java public void setModified(boolean bModified)
    pub fn setModified__boolean(&mut self, bModified: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.setModified(bModified, true);
    }


    // java public void setModified(boolean bModified, boolean bBackup)
    pub fn setModified__boolean_boolean(&mut self, bModified: bool, bBackup: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.isModified() != bModified)
            // java if (bBackup)
              // java this.start(false);
              // java this.writeLockAtom();
                // java (bBackup)
                  // java this.commit("", false);
        // java this.doModified(bModified);
    }


    // java private void doModified(boolean bModified)
    pub fn doModified(&mut self, bModified: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.notify_modified(bModified);
      // java this.mModified = bModified ? ModifyFlag._true : ModifyFlag._false;
    }


    // java private void notify_modified(boolean modified)
    pub fn notify_modified(&mut self, modified: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mOnModifiedListener != null)
        // java if (!this.modifiedOnce && modified)
          // java this.mOnModifiedListener.onModifiedOnce();
          // java this.modifiedOnce = true;
        // java this.mOnModifiedListener.onModified(modified);
      // java if (null != this.mOnModifiedListener4Stat)
        // java this.mOnModifiedListener4Stat.onModified(modified);
    }


    // java private void submitModified(boolean bModified)
    pub fn submitModified(&mut self, bModified: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.getTransaction().getCurrentAction() != null)
        // java if (this.getTransaction().getCurrentAction().getCommandCount() > 0)
          // java this.writeLockAtom();
          // java this.doModified(bModified);
    }


    // java public byte[] getEvidenceData()
    pub fn getEvidenceData(&self) -> Vec<u8> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mEvidenceData;
    }


    // java public void setEvidenceData(byte[] evidenceData)
    pub fn setEvidenceData(&mut self, evidenceData: &Vec<u8>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mEvidenceData = evidenceData;
    }


    // java @Deprecated
    // java public Metadata getMetadata()
    pub fn getMetadata(&self) -> Metadata {
      // java return this.mMetadata;
      // rust: Java 返回同一引用；`Metadata` 无 `Clone`。按值门面以 `Default` 承载
      //   null；非空字段仍住在 `mMetadata`，调用方要引用请后续加 `getMetadata_mut`。
      self.mMetadata.as_ref().map_or_else(Metadata::default, |_| Metadata::default())
    }


    // java public String getAppVersion()
    pub fn getAppVersion(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mAppVersion;
    }


    // java public void setAppVersion(String version)
    pub fn setAppVersion(&mut self, version: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mAppVersion = version;
    }


    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Option<&SubDocument> {
      // java return this.mSubdocuments[0];
      self.getSubdocument(0)
    }

    pub fn getMainDocumentMut(&mut self) -> Option<&mut SubDocument> {
      self.getSubdocumentMut(0)
    }

    pub fn getStylesMut(&mut self) -> Option<&mut KStyles> {
      self.styles.as_deref_mut()
    }

    // java public void updateDocument()
    pub fn updateDocument(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i = ();
      // java for(int n = this.mSubdocuments.length; i < n; ++i)
        // java Document doc = this.mSubdocuments[i];
        let mut doc = ();
        // java int len = null == doc ? 0 : doc.getLength();
        // java if (null != doc && len > 0)
          // java DocumentEventDispatcher dispatcher = doc.getDispatcher();
          // java DocumentEvent e = DefaultModelEvent.obtain(doc, 0, len, 4);
          // java dispatcher.update(e);
          // java e.decreaseRef();
      // java for(int n = this.mSubdocuments.length; i < n; ++i)
    }


    // java public Bitmap getContentThumb()
    pub fn getContentThumb(&self) -> Box<dyn Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getMainDocument().getContentThumb();
    }


    // java public PropertySet getProperty()
    pub fn getProperty(&self) -> PropertySet {
      // java return this.property;
      return self.property.clone().unwrap_or_else(|| (*empty).clone());
    }


    // java public void setProperty(int nProp, Object value)
    pub fn setProperty__int_object(&mut self, nProp: i32, value: &dyn Object) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start();
          // java this.writeLockAtom();
          // java MutablePropertySet mp = new MutablePropertySet();
          let mut mp = ();
          // java mp.add(this.property);
          // java mp.setValue(nProp, value);
          // java this.property = mp.freeze();
          // java Document document = this.getMainDocument();
          // java DocumentEvent e = DefaultModelEvent.obtain(document, 0, document.getLength(), 3);
          // java document.getDispatcher().update(e);
          // java e.decreaseRef();
            // java this.commit("");
    }


    // java public KStyles getStyles()
    pub fn getStyles(&self) -> Option<&KStyles> {
      // java return this.styles;
      self.styles.as_deref()
    }

    // java public FontTable getFontTable()
    pub fn getFontTable(&self) -> FontTable {
      // java return this.fontTable;
      return self.fontTable.clone().expect("fontTable");
    }

    /// Rust shared-reference adapter for Java `getFontTable()` callers that
    /// mutate the returned table (for example `ThemeFontTool.addFontByName`).
    pub fn getFontTable_mut(&mut self) -> Option<&mut FontTable> {
      // java return this.fontTable;
      self.fontTable.as_mut()
    }


    // java public Theme getTheme()
    pub fn getTheme(&self) -> Theme {
      // java return this.mTheme;
      // rust: Java 返回**引用**；`Theme` 是引用型对象（`ThemeImporter` 会就地改写它）。
      //   真实 `Theme` 未实现 `Clone`（含 `POIXMLDocumentPart` 等非克隆字段），
      //   故按值返回无法复现引用语义 ⇒ 需要引用时请用 `getTheme_mut()`（返回 `*mut Theme`）。
      Theme::new()
    }

    // rust: M-STAGE2 S2-2 —— `Theme` 的 Java 引用语义访问器。
    //   `ThemeImporter::ThemeImporter(*mut Theme)` 需要文档内 `mTheme` 的**同一份**存储。
    pub fn getTheme_mut(&mut self) -> Option<*mut Theme> {
      // java return this.mTheme;
      self.mTheme.as_mut().map(|t| t as *mut Theme)
    }


    // java public KStyle addBuiltinStyles(int sti)
    pub fn addBuiltinStyles(&mut self, sti: i32) -> Option<KStyle> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (sti == 4095)
        // java return null;
      // java else
        // java KStyle pStyle = this.styles.getStyleById(sti, false);
        // java if (pStyle != null)
          // java return pStyle;
        // java else
          // java pStyle = (KStyle)KStyles.getBuiltinStyles().get(sti);
          // java if (pStyle == null)
            // java return null;
          // java else
            // java pStyle = (KStyle)pStyle.clone();
            // java pStyle.setTextDocument(this);
            // java this.styles.add(pStyle);
            // java this.addBuiltinStyles(pStyle.getBase());
            // java this.addBuiltinStyles(pStyle.getNext());
            // java this.addBuiltinStyles(pStyle.getLink());
            // java return pStyle;
    }


    // java public final ArrayList<Integer> getRsids()
    pub fn getRsids(&self) -> Option<std::cell::RefMut<'_, Vec<i32>>> {
      // java return this.mRsids;
      std::cell::RefMut::filter_map(self.mRsids.borrow_mut(), Option::as_mut).ok()
    }


    // java  void _saveSelection(Action pAction, String which)
    pub fn _saveSelection(&self, pAction: Option<&Action>, which: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (pAction != null && this.getTransaction().getLevel() == 1)
          // java int size = this.mViews.size();
          // java for(int i = 0; i < size; ++i)
            // java ITextEditor pEditor = (ITextEditor)this.mViews.get(i);
            // java pEditor.saveSelection(pAction, which);
      // java for(int i = 0; i < size; ++i)
    }


    // java  void _restoreSelection(Action pAction, String which)
    pub fn _restoreSelection(&self, pAction: Option<&Action>, which: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (pAction != null && this.getTransaction().getLevel() == 0)
          // java int size = this.mViews.size();
          // java for(int i = 0; i < size; ++i)
            // java ITextEditor pEditor = (ITextEditor)this.mViews.get(i);
            // java pEditor.restoreSelection(pAction, which);
      // java for(int i = 0; i < size; ++i)
    }


    // java  void waitSync()
    pub fn waitSync(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.getTransaction().getLevel() == 0 && !this.getMainDocument().isLocked())
        // java int size = this.mViews.size();
        // java for(int i = 0; i < size; ++i)
          // java ITextEditor pEditor = (ITextEditor)this.mViews.get(i);
          // java pEditor.waitSync();
      // java for(int i = 0; i < size; ++i)
    }


    // java public void addTransactionListener(Transaction.Listener listener)
    pub fn addTransactionListener(
        &self,
        listener: Box<dyn util::moffice::transaction::Transaction::Listener>,
    ) {
      // java ((TransactionListeners)this.getTransaction().getListener()).add(listener);
      let Some(tx) = self.super_.getTransaction() else {
        return;
      };
      let mut tx = tx.borrow_mut();
      if tx.listener_mut().is_some() {
        tx.listener_mut().unwrap().addListener(listener);
      } else {
        let mut listeners = TransactionListeners::new();
        listeners.add(listener);
        tx.setListener(Some(Box::new(listeners)));
      }
    }


    // java public void removeTransactionListener(Transaction.Listener listener)
    pub fn removeTransactionListener(&self, listener: &dyn Listener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java ((TransactionListeners)this.getTransaction().getListener()).remove(listener);
    }


    // java public void start()
    pub fn start(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start(true);
    }


    // java public void submit(CommandBase cmd)
    pub fn submit(&self, cmd: &CommandBase) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getTransaction().submitCommand(cmd);
    }


    // java public void commit(String description)
    pub fn commit__string(&mut self, description: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.commit(description, true);
    }


    // java public void update(Object obj)
    pub fn update(&self, obj: &dyn Object) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getTransaction().update(obj);
    }


    // java public void start(boolean bLock)
    pub fn start__boolean(&mut self, bLock: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start(bLock, "");
    }


    // java public void start(String description)
    pub fn start__string(&mut self, description: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start(true, description);
    }


    // java private void transaction_lock()
    pub fn transaction_lock(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Date before = null;
      let mut before = ();
      // java this.transaction_locks.push(this.getMainDocument().writeLock());
    }


    // java private void transaction_unlock()
    pub fn transaction_unlock(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Log.d(TAG, "release write_lock for transaction end...");
      // java ((RangeWriteLock)this.transaction_locks.pop()).unlock();
    }


    // java public void start(boolean bLock, String description)
    pub fn start__boolean_string(&mut self, bLock: bool, description: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start(bLock, description, ActionType.NORMAL, (Action.CallBack)null);
    }


    // java public void startInner(boolean bLock, Action.ActionType type)
    pub fn startInner(&mut self, bLock: bool, r#type: &ActionType) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (0 == this.getTransaction().getLevel())
        // java this.start(bLock, "", ActionType.NORMAL, (Action.CallBack)null);
      // java else
        // java if (bLock)
          // java this.transaction_lock();
        // java this.getTransaction().startInner(type);
    }


    // java public void start(boolean bLock, String description, Action.ActionType type, Action.CallBack callBack)
    pub fn start__boolean_string_actiontype_callback(&mut self, bLock: bool, description: &str, r#type: &ActionType, callBack: &dyn CallBack) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.waitSync();
      // java if (bLock)
        // java this.transaction_lock();
      // java this.getTransaction().start(description, type, callBack);
      // java if (this.getTransaction().getLevel() == 1)
        // java this._saveSelection(this.getTransaction().getCurrentAction(), "before");
    }


    // java public void commit(String description, boolean bUnlock)
    pub fn commit__string_boolean(&mut self, description: &str, bUnlock: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.commit(description, bUnlock, false);
    }


    // java public void commit(String description, boolean bUnlock, boolean forceCommit)
    pub fn commit__string_boolean_boolean(&mut self, description: &str, bUnlock: bool, forceCommit: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (1 == this.getTransaction().getLevel())
        // java this.getSaveAccelerator().restoreForbiddenAutoSave();
      // java this.submitModified(true);
      // java if (this.getTransaction().getLevel() == 1)
        // java this._saveSelection(this.getTransaction().getCurrentAction(), "after");
      // java this.getTransaction().commit(description, forceCommit);
      // java if (bUnlock)
        // java this.transaction_unlock();
    }


    // java public boolean getInterruptNext()
    pub fn getInterruptNext(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getTransaction().getActionList().getInterruptNext();
    }


    // java public void setInterruptNext(boolean interruptNext)
    pub fn setInterruptNext(&self, interruptNext: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getTransaction().getActionList().setInterruptNext(interruptNext);
    }


    // java public void rollback()
    pub fn rollback(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.transaction_lock();
          // java this._restoreSelection(this.getTransaction().getCurrentAction(), "before");
          // java this.getTransaction().rollback();
            // java this.transaction_unlock();
    }


    // java public void rollback(boolean bRollback)
    pub fn rollback__boolean(&mut self, bRollback: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.transaction_lock();
      // java this._restoreSelection(this.getTransaction().getCurrentAction(), "before");
      // java if (bRollback)
        // java this.getTransaction().rollback();
      // java else
        // java this.getTransaction().clearCurrentAction();
      // java this.transaction_unlock();
    }


    // java public void clearRedo()
    pub fn clearRedo(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.transaction_lock();
          // java this.getTransaction().clearRedo();
            // java this.transaction_unlock();
    }


    // java public boolean canExecuteUndoOrRedo()
    pub fn canExecuteUndoOrRedo(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getTransaction().getState() == 0;
    }


    // java public void undo()
    pub fn undo(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.waitSync();
      // java boolean modifiedBefore = this.isModified();
      // java Action pAction = null;
      let mut pAction = ();
      // java this.transaction_lock();
          // java if (this.getTransaction().getLevel() == 0)
            // java pAction = this.getTransaction().undo();
            // java this.transaction_unlock();
      // java this.getSaveAccelerator().restoreForbiddenAutoSave();
      // java if (pAction != null)
        // java this._restoreSelection(pAction, "before");
        // java boolean modifiedAfter = this.isModified();
        // java if (modifiedAfter != modifiedBefore)
          // java this.notify_modified(modifiedAfter);
        // java else if (null != this.mOnModifiedListener4Stat)
          // java this.mOnModifiedListener4Stat.onModified(modifiedAfter);
    }


    // java public String dumpForException()
    pub fn dumpForException(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java StringBuilder sb = new StringBuilder();
      let mut sb = ();
      // java sb.append("mStateIO = ");
      // java sb.append(this.mStateIO);
      // java if (this.mIoErrorCode > 0)
        // java sb.append(",errorCode = " + this.mIoErrorCode);
      // java sb.append("\n");
      // java if (!this.mStateIO)
        // java int size = this.mViews.size();
        // java if (size > 0)
          // java ITextEditor pEditor = (ITextEditor)this.mViews.get(0);
          // java Action pAction = this.getTransaction().getLastAction();
          // java if (null != pAction)
            // java SelectionStore store = (SelectionStore)pAction.getUser(pEditor.hashCode() + "after");
            // java if (store != null)
              // java int start = store.getStart();
              // java int end = store.getEnd();
              // java sb.append("current selection type = ");
              // java sb.append(store.selType);
              // java sb.append(" start = ");
              // java sb.append(start);
              // java sb.append(" end = ");
              // java sb.append(end);
              // java sb.append(" docType = " + store.documentType);
                  // java Document document = this.getSubdocument(store.documentType);
                  // java sb.append(" doclen = " + document.getLength());
                  // java sb.append(" papxEntry maxCp = " + document.getPapxTable().end().getPrev().position());
                  // java sb.append(" chpxEntry maxCp = " + document.getChpxTable().end().getPrev().position());
                  // java Throwable var10
              // java sb.append("\n");
            // java String actonInfo = this.getTransaction().getActionList().toString();
            // java sb.append(actonInfo);
            // java Exception var9
      // java return sb.toString();
      // java catch (Throwable var10)
      // java catch (Exception var9)
    }


    // java public void redo()
    pub fn redo(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.waitSync();
      // java boolean modifiedBefore = this.isModified();
      // java this.transaction_lock();
      // java Action pAction;
          // java pAction = this.getTransaction().redo();
            // java this.transaction_unlock();
      // java this.getSaveAccelerator().restoreForbiddenAutoSave();
      // java this._restoreSelection(pAction, "after");
      // java boolean modifiedAfter = this.isModified();
      // java if (modifiedAfter != modifiedBefore && this.mOnModifiedListener != null)
          // java this.mOnModifiedListener.onModified(modifiedAfter);
      // java if (null != this.mOnModifiedListener4Stat)
        // java this.mOnModifiedListener4Stat.onModified(modifiedAfter);
    }


    // java public boolean canUndo()
    pub fn canUndo(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getTransaction().canUndo();
    }


    // java public boolean canRedo()
    pub fn canRedo(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getTransaction().canRedo();
    }


    // java public void clearUndoRedo()
    pub fn clearUndoRedo(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getTransaction().clearHistory();
    }


    // java public ArrayList<ITextEditor> getViews()
    pub fn getViews(&self) -> Vec<Box<dyn ITextEditor>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mViews;
    }


    // java public AsiaTypography getAsiaTypography()
    pub fn getAsiaTypography(&mut self) -> &mut AsiaTypography {
      // java if (null == this.mAsiaTypography)
      if self.mAsiaTypography.is_none() {
        // java this.mAsiaTypography = new AsiaTypography(this);
        // rust: ★ Java `new AsiaTypography(this)` 传**宿主文档引用**；Rust ctor 取 `&TextDocument`，
        //   而此处正处 `&mut self` ⇒ 先取裸指针再转共享引用，避免与字段赋值借用冲突。
        let __self_ptr: *const TextDocument = self;
        let mut __asia = AsiaTypography::AsiaTypography(unsafe { &*__self_ptr });
        // java this.mAsiaTypography.init();
        __asia.init();
        self.mAsiaTypography = Some(__asia);
      }
      // java return this.mAsiaTypography;
      self.mAsiaTypography.as_mut().unwrap()
    }


    // java public String getPassword()
    pub fn getPassword(&self) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mProviderInfo == null ? null : this.mProviderInfo.strPassword;
    }


    // java public boolean hasReadPassword()
    pub fn hasReadPassword(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null != this.mProviderInfo && null != this.mProviderInfo.strPassword && this.mProviderInfo.strPassword.length() > 0;
    }


    // java public void setPasswd(String passwd)
    pub fn setPasswd(&mut self, passwd: Option<&str>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (passwd == null)
        // java if (this.mProviderInfo != null)
          // java this.setModified(true, false);
          // java this.mProviderInfo = null;
      // java else
        // java this.setModified(true, false);
        // java if (this.mProviderInfo == null)
          // java this.initPasswd(passwd);
        // java else
          // java this.mProviderInfo.strPassword = passwd;
    }


    // java private void initPasswd(String passwd)
    pub fn initPasswd(&mut self, passwd: Option<&str>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (passwd == null)
        // java this.mProviderInfo = null;
      // java else
        // java this.mProviderInfo = new ProviderInfo();
        // java if (passwd.length() <= 15)
          // java this.mProviderInfo.init_STD(ProviderInfo.CRYPTDOCUMENT_WPS, passwd, (byte[])null);
        // java else
          // java this.mProviderInfo.init_Other(passwd);
    }


    // java public int getEncryptType()
    pub fn getEncryptType(&self) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mProviderInfo == null ? ProviderInfo.ENCRYPT_STD : this.mProviderInfo.nEncryptType;
    }


    // java @Deprecated
    // java public final ITextPool getCharBuffer()
    pub fn getCharBuffer(&self) -> Option<*mut dyn ITextPool> {
      // java return this.mCharBuffer;
      self.getTextPool()
    }

    // java public final ITextPool getTextPool()
    pub fn getTextPool(&self) -> Option<*mut dyn ITextPool> {
      // java return this.mCharBuffer;
      self.mCharBuffer.as_ref().map(|b| {
        b.as_ref() as *const dyn ITextPool as *mut dyn ITextPool
      })
    }

    // java @Deprecated
    // java public int getLength()
    pub fn getLength(&self) -> i32 {
      // java return this.getMainDocument().getLength();
      self.getMainDocument().map(|d| d.getLength()).unwrap_or(0)
    }


    // java @Deprecated
    // java public Range getRange(int start, int end)
    pub fn getRange__int_int(&self, start: i32, end: i32) -> KRange {
      // java return this.getMainDocument().getRange(start, end);
      self.getMainDocument().map(|d| d.getRange__int_int(start, end)).unwrap_or_default()
    }


    // java @Deprecated
    // java public Range getRange(int start)
    pub fn getRange__int(&self, start: i32) -> KRange {
      // java return this.getRange(start, start);
      self.getRange__int_int(start, start)
    }


    // java @Deprecated
    // java public Range getRange()
    pub fn getRange(&self) -> KRange {
      // java return this.getRange(0, this.getLength());
      self.getRange__int_int(0, self.getLength())
    }


    // java public String getDigest()
    pub fn getDigest(&self) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java int defaultCount = 4000;
      let mut defaultCount = ();
      // java Document mainDoc = this.getSubdocument(0);
      // java Assert.assertNotNull(mainDoc);
      // java int mainDocLength = mainDoc.getLength();
      // java if (mainDocLength <= 1)
        // java return "";
      // java else
        // java StringBuilder result = new StringBuilder();
        let mut result = ();
        // java int start = 0;
        let mut start = ();
        // java int size = Math.min(mainDocLength, 4000);
        // java int end = start + size;
        let mut end = ();
        // java for(char[] text = new char[size]; result.length() < 200 && start < mainDocLength end = start + size)
          // java int count = mainDoc.getText(start, end, text, 0);
          // java if (count <= 0)
            // java break;
          // java int count2 = TextTool.filterMath(text, count, mainDoc, start);
          // java TextTool.filterText(text, 0, count2, result);
          // java start += count;
        // java boolean hasVisibleChar = false;
        let mut hasVisibleChar = ();
        // java boolean hasTable = false;
        let mut hasTable = ();
        // java int nLen = result.length();
        // java for(int i = 0; i < nLen; ++i)
          // java char c = result.charAt(i);
          // java if (Character.isLetterOrDigit(result.charAt(i)))
            // java hasVisibleChar = true;
            let mut hasVisibleChar = ();
            // java break;
          // java if (c == 7)
            // java hasTable = true;
            let mut hasTable = ();
        // java return (this.getMediaLib().size() > 0 || hasTable) && !hasVisibleChar ? null : result.substring(0, Math.min(200, result.length()));
      // java for(char[] text = new char[size]; result.length() < 200 && start < mainDocLength; end = start + size)
      // java for(int i = 0; i < nLen; ++i)
    }


    // java public KSections getSections()
    pub fn getSections(&self) -> KSections {
      todo!("DEFERRED_NON_FUNCTION");
      // java return new KSections(this);
    }


    // java public boolean isEmbedBalloonMode()
    pub fn isEmbedBalloonMode(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mIsEmbedBalloonMode;
    }


    // java public void setIsEmbedBalloonMode(boolean mIsEmbedBalloonMode)
    pub fn setIsEmbedBalloonMode(&mut self, mIsEmbedBalloonMode: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mIsEmbedBalloonMode = mIsEmbedBalloonMode;
    }


    // java public void setTrackChange(boolean trackChange)
    pub fn setTrackChange__boolean(&mut self, trackChange: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* Java 原文（逐语句保留）：
      // java if (!VersionState.isProVersion())
        // java if (this.mTrackChange == trackChange)
          // java return;
        // java if (!this.mStateIO)
          // java Assert.assertTrue("mTrackChange != trackChange should be true", this.mTrackChange != trackChange);
          // java this.setModified(true, false);
        // java this.mTrackChange = trackChange;
      // java else
        // java this.setTrackChange(trackChange, true);
      */
      // rust: ★M-STAGE1 阻塞记录 —— 本方法必经 `VersionState.isProVersion()`，
      //   而 `VersionState`（`writer/VersionState.rs`，含 `isProVersion`）**未登记进 `lib.rs`**：
      //   登记它会引入 55 个错误（`this` 未定义 ×N、`DefaultVersionProxy::new` 缺失），
      //   因该文件自身仍是 auto-generated 空壳（见 §24 的登记-回滚记录）。
      //   ⇒ 按 §20.4 判据保持 deferred，**不用假实现假装绿**。
    }


    // java public void setTrackChange(boolean trackChange, boolean modify)
    pub fn setTrackChange__boolean_boolean(&mut self, trackChange: bool, modify: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* Java 原文（逐语句保留）：
      // java if (this.mTrackChange != trackChange)
        // java if (!this.mStateIO && modify)
          // java Assert.assertTrue("mTrackChange != trackChange should be true", this.mTrackChange != trackChange);
          // java this.setModified(true, false);
        // java this.mTrackChange = trackChange;
      */
      // rust: 阻塞原因同上（`VersionState` 未登记）。本方法本体不依赖 `VersionState`，
      //   但其唯一调用者 `setTrackChange(boolean)` 的 else 分支依赖 ⇒ 一并保持 deferred。
    }


    // java public boolean getTrackChange()
    pub fn getTrackChange(&self) -> bool {
      // java return this.mTrackChange;
      return self.mTrackChange;
    }


    // java public boolean hasComments()
    pub fn hasComments(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mSubdocuments[3].getLength() > 0;
    }


    // java public boolean hasRevisions()
    pub fn hasRevisions(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java int docSize = this.mSubdocuments.length;
      let mut docSize = ();
      // java for(int i = 0; i < docSize; ++i)
        // java Document subDoc = this.mSubdocuments[i];
        let mut subDoc = ();
        // java CPIterator<IChpxTable.Entry> chpxIter = subDoc.getChpxTable().createCpIterator();
        // java while (chpxIter.hasNext())
          // java ITable.Entry chpx = chpxIter.next();
          // java PropertySet set = chpx.getProperty();
          // java if (set.getValue(45) != null)
            // java return true;
          // java if (set.getValue(46) != null)
            // java return true;
          // java Boolean bool = (Boolean)set.getValue(47);
          // java if (bool != null && bool)
            // java return true;
        // java CPIterator<IPapxTable.Entry> paraIter = subDoc.getPapxTable().createCpIterator();
        // java while (paraIter.hasNext())
          // java ITable.Entry parapx = paraIter.next();
          // java PropertySet set = parapx.getProperty();
          // java Boolean bool = (Boolean)set.getValue(238);
          // java if (bool != null && bool)
            // java return true;
          // java if (set.getValue(363) != null)
            // java return true;
      // java return false;
      // java for(int i = 0; i < docSize; ++i)
      // java while(chpxIter.hasNext())
      // java while(paraIter.hasNext())
    }


    // java public String getRevisionsAuthor()
    pub fn getRevisionsAuthor(&self) -> Option<String> {
      // java return Platform.getUserName();
      Platform::getUserName()
    }


    // java public void setRevisionsAuthor(String author)
    pub fn setRevisionsAuthor(&mut self, author: Option<&str>) {
      // java Platform.setUserName(author);
      Platform::setUserName(author);
    }


    // java public UnknownDataMap getUnknownDataMap()
    pub fn getUnknownDataMap(&self) -> UnknownDataMap {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mUnknownDataMap;
    }


    // java public String getVBAPath()
    pub fn getVBAPath(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mVBAPath;
    }


    // java public void setVBAPath(String vbaPath)
    pub fn setVBAPath(&mut self, vbaPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mVBAPath = vbaPath;
    }


    // java public UUSti getUUSti()
    pub fn getUUSti(&mut self) -> Option<&mut UUSti> {
      // java return this.mStiMaker;
      self.mStiMaker.as_mut()
    }

    // java public UUCommentId getUUCommentId()
    pub fn getUUCommentId(&self) -> UUCommentId {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mCommentIdMaker;
    }


    // java public UUCommentId getUUPropertySetRegistryId()
    pub fn getUUPropertySetRegistryId(&self) -> UUCommentId {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mPropertSetRegistryId;
    }


    // java public synchronized void addBuiltinTableStyle(int styleID)
    pub fn addBuiltinTableStyle(&mut self, styleID: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java KStyles styles = this.getStyles();
      // java if (null == styles.getStyleById(styleID, false))
        // java KStyle style = KStyles.getBuiltinTableStyle(this, styleID);
        // java if (style != null)
          // java this.addBuiltinTableStyle(style.getBase());
          // java styles.add((KStyle)style.clone());
    }


    // java public UUNumberingId getUUNumberingId()
    pub fn getUUNumberingId(&mut self) -> Option<&mut UUNumberingId> {
      // java return this.mNumberingIdMaker;
      self.mNumberingIdMaker.as_mut()
    }

    // java public UULsid getUULsid()
    pub fn getUULsid(&mut self) -> &mut UULsid {
      // java return this.mLsidMaker;
      self.mLsidMaker.as_mut().expect("mLsidMaker")
    }


    // java public ClassLoader getSSClassLoader()
    pub fn getSSClassLoader(&mut self) -> Box<dyn std::any::Any + Send + Sync> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mSSLoader == null)
        // java if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
          // java this.mSSLoader = IClassLoaderManager.getInstance().getSsClassLoader();
        // java else
          // java this.mSSLoader = TextDocument.class.getClassLoader();
      // java return this.mSSLoader;
    }


    // java public DrawingRIDMap getDrawingRIDMap()
    pub fn getDrawingRIDMap(&mut self) -> DrawingRIDMap {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mDrawingRIDMap == null)
        // java this.mDrawingRIDMap = new DrawingRIDMap();
      // java return this.mDrawingRIDMap;
    }


    // java public WIntObjectHashMap<String> getShapeId2DrawingRID()
    pub fn getShapeId2DrawingRID(&mut self) -> WIntObjectHashMap {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mShapeId2DrawingRIDMap)
        // java this.mShapeId2DrawingRIDMap = new WIntObjectHashMap();
      // java return this.mShapeId2DrawingRIDMap;
    }


    // java public ArrayList<Integer> getDiagramShapeId(int subDocType)
    pub fn getDiagramShapeId(&mut self, subDocType: i32) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mSubDocType2DiagramShapeId)
        // java this.mSubDocType2DiagramShapeId = new WIntObjectHashMap();
      // java ArrayList<Integer> arrayList = (ArrayList)this.mSubDocType2DiagramShapeId.get(subDocType);
      // java if (null == arrayList)
        // java arrayList = new ArrayList();
        let mut arrayList = ();
        // java this.mSubDocType2DiagramShapeId.put(subDocType, arrayList);
      // java return arrayList;
    }


    // java public boolean isDisplayBackground()
    pub fn isDisplayBackground(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.property.getBoolean(400, false);
    }


    // java public synchronized void requestUpdateBkShape()
    pub fn requestUpdateBkShape(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.needUpdateImmutableBkShape = true;
    }


    // java public synchronized Shape getImmutableBkShape()
    pub fn getImmutableBkShape(&mut self) -> Shape {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.needUpdateImmutableBkShape)
        // java boolean isInIOState = this.isInIOState();
        // java this.needUpdateImmutableBkShape = false;
        // java if (null != this.mImmutableBkShape)
          // java this.mImmutableBkShape.dispose();
        // java Shape bkShape = this.getBackground();
        // java if (null != bkShape)
              // java this.mImmutableBkShape = bkShape.cloneLightWeight();
              // java CloneNotSupportedException e
                // java e.printStackTrace();
        // java else
          // java this.mImmutableBkShape = null;
          // java if (isInIOState)
            // java this.needUpdateImmutableBkShape = true;
      // java return this.mImmutableBkShape;
    }


    // java public Shape getBackground()
    pub fn getBackground(&self) -> Option<Shape> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getMainDocument().getDrawingContainer().isBKShapesEmpty() ? null : (Shape)this.getMainDocument().getDrawingContainer().getBKShapes().get(0);
    }


    // java private int getShapeProprytyID(FillBase fill)
    pub fn getShapeProprytyID(&self, fill: Option<&FillBase>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int id = -1;
      let mut id = ();
      // java if (fill != null)
        // java int type = fill.getFillType();
        // java switch (type)
          // java case 0:
            // java id = 464;
            let mut id = ();
          // java case 1: case 2: case 3:
            // java id = 466;
            let mut id = ();
          // java case 4: case 5: case 6: case 7: case 8:
            // java id = 465;
            let mut id = ();
          // java default:
            // java id = 464;
            let mut id = ();
      // java return id;
      // java break
      // java break
      // java break
    }


    // java private void addShapeCommand(Shape shape, FillBase fill)
    pub fn addShapeCommand(&mut self, shape: &Shape, fill: Option<&FillBase>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int id = this.getShapeProprytyID(fill);
      // java int oldId = id;
      let mut oldId = ();
      // java FillBase oldFill = shape.getFill();
      // java if (oldFill != null)
        // java oldId = this.getShapeProprytyID(oldFill);
        // java if (fill == null)
          // java id = oldId;
          let mut id = ();
      // java if (oldId == id)
        // java KShapeCommand shapeCommand = new KShapeCommand(shape, id, oldFill, fill);
        let mut shapeCommand = ();
        // java this.submit(shapeCommand);
      // java else
        // java KShapeCommand shapeCommand = new KShapeCommand(shape, id, (Object)null, fill);
        let mut shapeCommand = ();
        // java this.submit(shapeCommand);
        // java shapeCommand = new KShapeCommand(shape, oldId, oldFill, (Object)null);
        let mut shapeCommand = ();
        // java this.submit(shapeCommand);
    }


    // java public void setBackground(FillBase fillBase)
    pub fn setBackground(&mut self, fillBase: Option<&FillBase>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Shape shape = this.getBackground();
      // java if (shape == null)
        // java shape = this.getMainDocument().getDrawingContainer().addBKShape();
      // java if (shape != null)
        // java if (fillBase == null && shape.getFill() == null)
          // java return;
        // java this.start();
            // java this.addShapeCommand(shape, fillBase);
            // java shape.setFill(fillBase);
            // java this.setModified(true);
            // java CRefreshCommand command = new CRefreshCommand(this.getMainDocument());
            // java this.submit(command);
            // java command.execute();
            // java Object displayBKShape = this.property.getValue(400);
            // java if (displayBKShape == null || !(Boolean)displayBKShape)
              // java MutablePropertySet mSettings = new MutablePropertySet(this.property);
              let mut mSettings = ();
              // java mSettings.setBoolean(400, true);
              // java this.property = mSettings.freeze();
              // java this.commit("set background");
    }


    // java public void setStopUpdateTextDocment(boolean bStop)
    pub fn setStopUpdateTextDocment(&mut self, bStop: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mStopUpdateTextDocment = bStop;
    }


    // java public boolean isStopUpdateTextDocment()
    pub fn isStopUpdateTextDocment(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mStopUpdateTextDocment;
    }


    // java public void onFinishDumpObjects()
    pub fn onFinishDumpObjects(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
        // java (this.mFinishDumpObjectsNotifyObj)
          // java this.mFinishDumpObjects = true;
          // java this.mFinishDumpObjectsNotifyObj.notifyAll();
          // java this.openListeners.docDidFinishReadingEverything();
    }


    // java public void waitFinishDumpObjects()
    pub fn waitFinishDumpObjects(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
        // java (this.mFinishDumpObjectsNotifyObj)
          // java while (!this.mFinishDumpObjects)
                // java this.mFinishDumpObjectsNotifyObj.wait();
                // java InterruptedException e
                  // java e.printStackTrace();
      // java while(!this.mFinishDumpObjects)
    }


    // java public void setIoThreadIdToRangeLock(long id)
    pub fn setIoThreadIdToRangeLock(&self, id: i64) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i = ();
      // java for(int n = this.mSubdocuments.length; i < n; ++i)
        // java Document doc = this.mSubdocuments[i];
        let mut doc = ();
        // java if (null != doc)
          // java RangeLock rangeLock = doc.getRangeLock();
          // java rangeLock.setIoThreadId(id);
      // java for(int n = this.mSubdocuments.length; i < n; ++i)
    }


    // java public void setChartShapes(List<Shape> shapes)
    pub fn setChartShapes(&mut self, shapes: &Vec<Shape>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mChartShapeList = shapes;
    }


    // java public List<Shape> getChartShapes()
    pub fn getChartShapes(&self) -> Vec<Shape> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mChartShapeList;
    }


    // java public void beforeSectionChanged()
    pub fn beforeSectionChanged(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = this.mViews.size();
      // java for(int i = 0; i < size; ++i)
        // java ITextEditor pEditor = (ITextEditor)this.mViews.get(i);
        // java pEditor.beforeSectionChanged();
      // java for(int i = 0; i < size; ++i)
    }


    // java public void afterSectionChanged()
    pub fn afterSectionChanged(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = this.mViews.size();
      // java for(int i = 0; i < size; ++i)
        // java ITextEditor pEditor = (ITextEditor)this.mViews.get(i);
        // java pEditor.afterSectionChanged();
      // java for(int i = 0; i < size; ++i)
    }


    // java public String getEncoding()
    pub fn getEncoding(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mEncoding;
    }


    // java public void setEncoding(String encoding)
    pub fn setEncoding(&mut self, encoding: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertNotNull("encoding should not be null", encoding);
      // java this.mEncoding = encoding;
    }


    // java public void setLoadListener(DocumentLoadListener loadListener)
    pub fn setLoadListener(&mut self, loadListener: &dyn DocumentLoadListener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.loadListener = loadListener;
    }


    // java public boolean isContainMathEquations()
    pub fn isContainMathEquations(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = 7;
      let mut size = ();
      // java for(int i = 0; i < size; ++i)
        // java Document subDocument = this.mSubdocuments[i];
        let mut subDocument = ();
        // java if (null != subDocument && subDocument.isContainMathEquations())
          // java return true;
      // java return false;
      // java for(int i = 0; i < size; ++i)
    }


    // java public boolean hasMainDocumentShape()
    pub fn hasMainDocumentShape(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document document = this.getMainDocument();
      // java if (document != null)
        // java PLCShapeAnchor shapeAnchor = document.getShapeAnchors();
        // java if (shapeAnchor != null && !shapeAnchor.isEmpty())
          // java return true;
      // java return false;
    }


    // java public boolean hasDocField()
    pub fn hasDocField(&self) -> bool {
      // java for(int i = 0; i < 7; ++i)
      let mut i: i32 = 0;
      while i < 7 {
        // java Document subDoc = this.getSubdocument(i);
        if let Some(subDoc) = self.getSubdocument(i) {
          // java if (subDoc != null && subDoc.getDocumentFileds() != null && !subDoc.getDocumentFileds().isEmpty())
          if subDoc
            .getDocumentFileds()
            .map(|f| !f.isEmpty())
            .unwrap_or(false)
          {
            // java return true;
            return true;
          }
        }
        i += 1;
      }
      // java return false;
      false
    }


    // java public boolean setFileTime(boolean isSavedAs)
    pub fn setFileTime(&mut self, isSavedAs: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertNotNull("mMetadata should not be null!", this.mMetadata);
      // java PIDSI pidsi = this.mMetadata.getPIDSI();
      // java Assert.assertNotNull("pidsi should not be null!", pidsi);
      // java this.backupFileTime(pidsi);
      // java if (isSavedAs)
        // java this.setFileTimeSavedAs(pidsi);
      // java else
        // java this.setFileTimeSaved(pidsi);
      // java return true;
    }


    // java private void setFileTimeSaved(PIDSI pidsi)
    pub fn setFileTimeSaved(&mut self, pidsi: &mut PIDSI) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertNotNull("mOpenTime should not be null!", this.mOpenTime);
      // java Date currentDate = new Date();
      let mut currentDate = ();
      // java long editTime = currentDate.getTime() - this.mOpenTime.getTime();
      // java Long currentEditTime = pidsi.getEditTime();
      // java if (null != currentEditTime)
        // java editTime += currentEditTime;
      // java pidsi.setEditTime(editTime);
      // java pidsi.setLastSaveTime(currentDate);
      // java this.mOpenTime = currentDate;
    }


    // java private void setFileTimeSavedAs(PIDSI pidsi)
    pub fn setFileTimeSavedAs(&mut self, pidsi: &mut PIDSI) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Date currentDate = new Date();
      let mut currentDate = ();
      // java pidsi.setCreateTime(currentDate);
      // java pidsi.setEditTime(0L);
      // java pidsi.setLastSaveTime(currentDate);
    }


    // java private void backupFileTime(PIDSI pidsi)
    pub fn backupFileTime(&mut self, pidsi: &PIDSI) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Long editTime = pidsi.getEditTime();
      // java Date createTime = pidsi.getCreateTime();
      // java Date lastSaveTime = pidsi.getLastSaveTime();
      // java long editTimel = 0L;
      let mut editTimel = ();
      // java if (null != editTime)
        // java editTimel = editTime;
        let mut editTimel = ();
      // java this.mLastFileTime = new FileTime(editTimel, createTime, lastSaveTime, this.mOpenTime);
    }


    // java public void recoverFileTime()
    pub fn recoverFileTime(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Assert.assertNotNull("mLastFileTime should not be null!", this.mLastFileTime);
      // java Assert.assertNotNull("mMetadata should not be null!", this.mMetadata);
      // java Date openTime = this.mLastFileTime.getFileOpenTime();
      // java long editTime = this.mLastFileTime.getEditTime();
      // java Date createTime = this.mLastFileTime.getCreateTime();
      // java Date lastSaveTime = this.mLastFileTime.getLastSaveTime();
      // java PIDSI pidsi = this.mMetadata.getPIDSI();
      // java pidsi.setEditTime(editTime);
      // java if (null != openTime)
        // java this.mOpenTime = openTime;
      // java if (null != createTime)
        // java pidsi.setCreateTime(createTime);
      // java if (null != lastSaveTime)
        // java pidsi.setLastSaveTime(lastSaveTime);
    }


    // java public void updateCore4Save(boolean isSavedAs)
    pub fn updateCore4Save(&self, isSavedAs: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java KCustomIcv kCustomIcv = this.getKCustomIcv();
      // java if (null != kCustomIcv)
        // java kCustomIcv.updateIcv(isSavedAs);
    }


    // java public DocumentVariables getDocumentVariables()
    pub fn getDocumentVariables(&mut self) -> DocumentVariables {
      todo!("DEFERRED_NON_FUNCTION");
        // java (this)
          // java if (this.variables == null)
            // java this.variables = new DocumentVariables(this);
      // java return this.variables;
      // java synchronized(this)
    }


    // java public KDocumentProtection getDocumentProtection()
    pub fn getDocumentProtection(&self) -> KDocumentProtection {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getDocumentProtection(true);
    }


    // java public KDocumentProtection getDocumentProtection(boolean create)
    pub fn getDocumentProtection__boolean(&mut self, create: bool) -> KDocumentProtection {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (create)
          // java (this)
            // java if (this.protection == null)
              // java this.protection = new KDocumentProtection(this);
      // java return this.protection;
      // java synchronized(this)
    }

    // Java: getDocumentProtection(true) returns the document-owned, mutable KDocumentProtection.
    // Rust: this reference adapter preserves Java object identity for SettingsImporter.
    pub fn getDocumentProtection_mut(&mut self, create: bool) -> Option<&mut KDocumentProtection> {
      if create && self.protection.is_none() {
        self.protection = Some(KDocumentProtection::KDocumentProtection(self));
      }
      self.protection.as_mut()
    }


    // java public OnlineSecurityTool getOnlineSecurityTool()
    pub fn getOnlineSecurityTool(&self) -> OnlineSecurityTool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mOnlineSecurytiTool;
    }


    // java private void fillFirstPageFont()
    pub fn fillFirstPageFont(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document mainDoc = this.getMainDocument();
      // java RangeReadLock readLock = mainDoc.readLock();
          // java int endCp = this.getMainFirstPageEndCp(mainDoc);
          // java int EMPTY_DOC_LENGTH = 1;
          let mut EMPTY_DOC_LENGTH = ();
          // java if (endCp > 1)
            // java Set<String> firstPageFontSet = new HashSet();
            let mut firstPageFontSet = ();
            // java IChpxTable chpxTable = mainDoc.getChpxTable();
            // java CPIterator iter = chpxTable.createCpIterator();
            // java KRange range = mainDoc.getRange(0, 0);
            // java while (iter.hasNext())
              // java iter.next();
              // java int start = iter.getStart();
              // java int end = iter.getEnd();
              // java if (start >= endCp)
                // java break;
              // java if (end - start != 1 || !ControlChar.isParagraphBreak(mainDoc.charAt(start)))
                // java range.setRange(start, end);
                // java range.fillUsedFontNameList(firstPageFontSet);
            // java range.dispose();
            // java this.mFirstPageFontList = new ArrayList(firstPageFontSet);
            // java return;
            // java readLock.unlock();
      // java while(iter.hasNext())
    }


    // java private int getMainFirstPageEndCp(Document mainDoc)
    pub fn getMainFirstPageEndCp(&self, mainDoc: &dyn Document) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java IPapxTable papxTable = mainDoc.getPapxTable();
      // java int size = papxTable.size();
      // java int cp = 0;
      let mut cp = ();
      // java if (size <= 30)
        // java cp = mainDoc.getLength();
      // java else
        // java int count = 0;
        let mut count = ();
        // java for(CPIterator<IPapxTable.Entry> iter = papxTable.createCpIterator(); iter.hasNext() && count < 30 ++count)
          // java iter.next();
          // java cp = iter.getEnd();
      // java return cp;
      // java for(CPIterator<IPapxTable.Entry> iter = papxTable.createCpIterator(); iter.hasNext() && count < 30; ++count)
    }


    // java public ArrayList<String> getFirstPageFontList()
    pub fn getFirstPageFontList(&mut self) -> Vec<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.fillFirstPageFont();
      // java if (null == this.mFirstPageFontList)
        // java this.mFirstPageFontList = new ArrayList();
      // java return this.mFirstPageFontList;
    }


    // java public boolean hasAudioComment()
    pub fn hasAudioComment(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java RangeReadLock lock = this.getMainDocument().readLock();
      // java boolean var5;
          // java PLCCommentRef commentRef = this.getMainDocument().getCommentRef();
          // java if (null == commentRef || commentRef.size() <= 0)
            // java return false;
          // java PLC.Iterator iter = commentRef.createIterator();
          // java PLCCommentRef.CommentRefNode node;
          // java do (node.getCustomData() == null)
              // java if (!iter.hasNext())
                // java return false;
              // java node = (PLCCommentRef.CommentRefNode)iter.next();
          // java var5 = true;
          let mut var5 = ();
            // java lock.unlock();
      // java return var5;
      // java while(node.getCustomData() == null)
    }


    // java public WdCompatibilityMode getCompatibilityMode()
    pub fn getCompatibilityMode(&mut self) -> WdCompatibilityMode {
      // java if (null == this.mCompatbilityMode)
      if self.mCompatbilityMode.is_none() {
        // java CompatSetting[] compatSettingArr = (CompatSetting[])this.property.getValue(542);
        let compatSettingArr = self.property.as_ref().and_then(|p| {
          p.getValue(542).and_then(|v| v.as_any().downcast_ref::<Vec<CompatSetting>>().cloned())
        });
        // java if (compatSettingArr != null)
        if let Some(compatSettingArr) = compatSettingArr {
          // java int i = 0;
          let mut i: i32 = 0;
          // java for(int count = compatSettingArr.length; i < count; ++i)
          let count = compatSettingArr.len() as i32;
          while i < count {
            // java CompatSetting setting = compatSettingArr[i];
            let setting = &compatSettingArr[i as usize];
            // java if ("compatibilityMode".equalsIgnoreCase(setting.getName()))
            if setting.getName().as_deref().is_some_and(|n| n.eq_ignore_ascii_case("compatibilityMode")) {
              // java this.mCompatbilityMode = WdCompatibilityMode.convertValue(setting.getVal());
              if let Some(val) = setting.getVal() {
                self.mCompatbilityMode = Some(WdCompatibilityMode::convertValue(&val));
              }
            }
            i += 1;
          }
        }
        // java if (null == this.mCompatbilityMode)
        if self.mCompatbilityMode.is_none() {
          // java this.mCompatbilityMode = WdCompatibilityMode.wdWord2010;
          self.mCompatbilityMode = Some(WdCompatibilityMode::wdWord2010);
        }
      }
      // java return this.mCompatbilityMode;
      self.mCompatbilityMode.unwrap()
    }


    // java public int type()
    pub fn r#type(&mut self) -> i32 {
      // java return 1;
      return 1;
    }

    // java public boolean isDirty()
    pub fn isDirty(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.isModified();
    }


    // java public boolean isSecurityFile()
    pub fn isSecurityFile(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null != this.mPath)
        // java OnlineSecurityTool tool = new OnlineSecurityTool();
        let mut tool = ();
        // java return tool.isWpsSecurityFile(this.mPath);
      // java else
        // java return false;
    }


    // java public void setSlimDirty()
    pub fn setSlimDirty(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.setModified(true, false);
    }


    // java public int open(String path, String password, Object context, IDLProvider dlProvider, boolean ignoreSecurity)
    pub fn open__string_string_object_idlprovider_boolean(&mut self, path: &str, password: &str, context: &dyn Object, dlProvider: &dyn IDLProvider, ignoreSecurity: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
          // java if (!ignoreSecurity)
            // java OnlineSecurityTool tool = new OnlineSecurityTool();
            let mut tool = ();
            // java if (tool.isWpsSecurityFile(path))
              // java return 5;
          // java this.openDocument(path, password, dlProvider);
          // java InnerIOListener listener = new InnerIOListener(this);
          let mut listener = ();
              // java this.startLoading(listener, new IOThreadSign());
              // java IOException var8
                // java return 2;
          // java listener.onFinishDumpObjects();
          // java if (14 == listener.getErrorCode())
            // java return 7;
          // java else if (6 != listener.getErrorCode() && 7 != listener.getErrorCode())
            // java if (12 == listener.getErrorCode())
              // java return 4;
            // java else
              // java return 0 != listener.getErrorCode() ? 2 : 1;
          // java else
            // java return 3;
          // java Throwable var9
            // java return 2;
      // java catch (Throwable var9)
    }


    // java public int open(String path, String password, Object context, IDLProvider dlProvider)
    pub fn open__string_string_object_idlprovider(&mut self, path: &str, password: &str, context: &dyn Object, dlProvider: &dyn IDLProvider) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.open(path, password, context, dlProvider, false);
    }


    // java public boolean isEmptySlides()
    pub fn isEmptySlides(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return false;
    }


    // java public void setWritePassword(String password)
    pub fn setWritePassword(&mut self, password: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java KSAssert.fail("文档暂时没有编辑密码支持");
    }


    // java public boolean hasWritePassword()
    pub fn hasWritePassword(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return false;
    }


    // java public boolean save(String path)
    pub fn save__string(&mut self, path: Option<&str>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null != path && path.length() > 0)
          // java FileFormat format = this.getFileExtFormat(path);
          // java this.doFormatConvert(this, format);
          // java File tmp = Platform.createTempFile("wri", ".tmp");
              // java this.saveFile(tmp.getAbsolutePath(), format);
              // java WriterIOException var5
                // java return false;
          // java return FileUtil.moveFile(tmp.getAbsolutePath(), path);
      // java else
        // java return false;
      // java this.saveAs(this.mPath)
    }


    // java public IDocProtection getDocProtection()
    pub fn getDocProtection(&self) -> Box<dyn IDocProtection> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getDocumentProtection();
    }


    // java public IKmoBookProtect getBookProtection()
    pub fn getBookProtection(&self) -> Option<Box<dyn IKmoBookProtect>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null;
    }


    // java private FileFormat getFileExtFormat(String path)
    pub fn getFileExtFormat(&self, path: &mut str) -> FileFormat {
      todo!("DEFERRED_NON_FUNCTION");
      // java String ext = path.substring(path.lastIndexOf(46) + 1);
      // java return FileFormat.fromExtension(ext);
    }


    // java private void doFormatConvert(TextDocument document, FileFormat fileFormat)
    pub fn doFormatConvert(&mut self, document: Option<&TextDocument>, fileFormat: &FileFormat) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (document != null)
        // java IFormatConverter converter = null;
        let mut converter = ();
        // java if (fileFormat == FileFormat.FF_DOC && document.getFileFormat() == FileFormat.FF_DOCX)
          // java converter = ReflectionHelper.getFormatConverter(document);
          // java converter.convertToDoc(document);
        // java else if (fileFormat == FileFormat.FF_DOCX && document.getFileFormat() == FileFormat.FF_DOC)
          // java converter = ReflectionHelper.getFormatConverter(document);
          // java converter.convertToDocX(document);
    }


    // java public IShareObjIndexdMap getShareObjIndexdMap()
    pub fn getShareObjIndexdMap(&mut self) -> ShareObjIndexedMapImpl {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mShareObjIndexdMap == null)
        // java this.mShareObjIndexdMap = ShareObjIndexedMapFactory.newMixClassInstance();
      // java return this.mShareObjIndexdMap;
    }


    // java public KExchange getFootnoteEndnoteExchange()
    pub fn getFootnoteEndnoteExchange(&mut self) -> KExchange {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mExchange)
        // java this.mExchange = new KExchange(this);
      // java return this.mExchange;
    }


    // java public KFootnoteEndnote getFootnoteEndnote()
    pub fn getFootnoteEndnote(&mut self) -> KFootnoteEndnote {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mFootnoteEndnote)
        // java this.mFootnoteEndnote = new KFootnoteEndnote(this);
      // java return this.mFootnoteEndnote;
    }


    // java public ISectionProperty getSectionProperty()
    pub fn getSectionProperty(&mut self) -> Box<dyn ISectionProperty> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mSectionPropSetting)
        // java this.mSectionPropSetting = new KSectionProperty(this);
      // java return this.mSectionPropSetting;
    }


    // java public IHostApp getSTHostApp()
    pub fn getSTHostApp(&self) -> Box<dyn IHostApp> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mSTHostApp;
    }


    // java public IHostApp getSTHostApp(String theme, Map<String, Integer> rid2MediaId)
    pub fn getSTHostApp__string_mapstring(&mut self, theme: &str, rid2MediaId: Option<&HashMap<String, i32>>) -> Box<dyn IHostApp> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mSTHostApp == null)
        // java this.mSTHostApp = new SmartartHostApp(this.getMediaLib(), theme, this.getMetadata().getPIDDSI().getDocVersion(), this.getMetadata().getPIDSI().getAppName());
        // java if (null != rid2MediaId)
          // java for(String key : rid2MediaId.keySet())
            // java this.mSTHostApp.hostTheme().addMediaItem(key, (Integer)rid2MediaId.get(key));
      // java return this.mSTHostApp;
      // java for(String key : rid2MediaId.keySet())
    }


    // java public void addOle(int mediaId, OleInfo docOleInfo)
    pub fn addOle(&self, mediaId: i32, docOleInfo: &OleInfo) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getOleManager().addOle(mediaId, docOleInfo);
    }


    // java public KCustomIcv getKCustomIcv()
    pub fn getKCustomIcv(&mut self) -> Option<KCustomIcv> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mKCustomIcv)
        // java if (null == this.mMetadata)
          // java return null;
        // java CustomMetadata customMetadata = this.mMetadata.getCustomMetadata();
        // java if (null == customMetadata)
          // java return null;
        // java this.mKCustomIcv = new KCustomIcv(customMetadata);
      // java return this.mKCustomIcv;
    }


    // java public boolean isProtectOn_readOnly()
    pub fn isProtectOn_readOnly(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java KDocumentProtection documentProtection = this.getDocumentProtection();
      // java ProtectionType protectionType = documentProtection.getProtectionType();
      // java return documentProtection.isProtectOn() && (protectionType == ProtectionType.READONLY || protectionType == ProtectionType.FORMS);
    }


    // java public boolean isProtectOn_TrackChangeCtrl()
    pub fn isProtectOn_TrackChangeCtrl(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java KDocumentProtection documentProtection = this.getDocumentProtection();
      // java ProtectionType protectionType = documentProtection.getProtectionType();
      // java return documentProtection.isProtectOn() && (protectionType == ProtectionType.READONLY || protectionType == ProtectionType.FORMS || protectionType == ProtectionType.COMMENTS || protectionType == ProtectionType.TRACKEDCHANGES);
    }


    // java public boolean isProtectOn()
    pub fn isProtectOn(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java KDocumentProtection documentProtection = this.getDocumentProtection();
      // java ProtectionType protectionType = documentProtection.getProtectionType();
      // java return documentProtection.isProtectOn() && (protectionType == ProtectionType.READONLY || protectionType == ProtectionType.FORMS || protectionType == ProtectionType.COMMENTS);
    }


    // java public boolean isThrowDocumentProtectionException()
    pub fn isThrowDocumentProtectionException(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getDocumentProtection().isThrowDocumentProtectionException();
    }


    // java public void beginSectionChange()
    pub fn beginSectionChange(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document document = this.getMainDocument();
      // java long range = RANGEFactory.getRANGE(0, document.getLength());
      // java CUpdateCommand.Callback callback = new CUpdateCommand.Callback() { public void beforeUpdate() { TextDocument.this.beforeSectionChanged(); } public void afterUpdate() { TextDocument.this.afterSectionChanged(); } };
      // java CUpdateCommand.UpdateAfterUndo cmd = new CUpdateCommand.UpdateAfterUndo(document, range, callback);
      // java this.submit(cmd);
    }


    // java public void endSectionChange()
    pub fn endSectionChange(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java Document document = this.getMainDocument();
      // java long range = RANGEFactory.getRANGE(0, document.getLength());
      // java CUpdateCommand.Callback callback = new CUpdateCommand.Callback() { public void beforeUpdate() { TextDocument.this.beforeSectionChanged(); } public void afterUpdate() { TextDocument.this.afterSectionChanged(); } };
      // java CUpdateCommand.UpdateAfterRedo cmd = new CUpdateCommand.UpdateAfterRedo(document, range, callback);
      // java cmd.execute();
      // java this.submit(cmd);
    }


    // java private PropertySet createInitialSepx(PageSetup pageSetup, int orientation)
    pub fn createInitialSepx(&mut self, pageSetup: Option<&mut PageSetup>, orientation: i32) -> PropertySet {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (pageSetup == null)
        // java pageSetup = this.createDefaultPageSetup(orientation);
      // java MutablePropertySet props = new MutablePropertySet();
      let mut props = ();
      // java props.setValue(676, pageSetup);
      // java return props.freeze();
    }


    // java public PageSetup createDefaultPageSetup(int orientation)
    pub fn createDefaultPageSetup(&mut self, orientation: i32) -> PageSetup {
      todo!("DEFERRED_NON_FUNCTION");
      // java PageSetup def = (PageSetup)DefaultSectionProperties.getProperties().getValue(676);
      // java if (def == null)
        // java throw new RuntimeException();
      // java else
        // java return orientation == 1 ? new PageSetup(def.getWidth(), def.getHeight(), def.getMarginLeft(), def.getMarginRight(), def.getMarginTop(), def.getMarginBottom()) : new PageSetup(def.getHeight(), def.getWidth(), def.getMarginTop(), def.getMarginBottom(), def.getMarginLeft(), def.getMarginRight());
    }


    // java private PropertySet getMergedSepx(PropertySet sepx)
    pub fn getMergedSepx(&self, sepx: &PropertySet) -> PropertySet {
      // java Revision revision = (Revision)sepx.getValue(727);
      let revision: Option<Revision> = sepx
        .getValue(727)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>())
        .cloned();
      // java if (revision != null && revision.getProperty() != null)
      if let Some(revision) = revision {
        if let Some(revProp) = revision.getProperty() {
          // java MutablePropertySet m = new MutablePropertySet(revision.getProperty());
          let mut m = MutablePropertySet::new_from_ps(&revProp);
          // java m.add(sepx);
          m.add(sepx);
          // java m.add(revision.getProperty());
          m.add(&revProp);
          // java return m.freeze();
          return m.freeze();
        }
      }
      // java else
      // java return sepx;
      sepx.clone()
    }


    // java private boolean orientation_equal(PageSetup pageSetup, PropertySet sepx)
    pub fn orientation_equal(&mut self, pageSetup: &PageSetup, sepx: &PropertySet) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return normalizeOrientation((float)pageSetup.getWidth(), (float)pageSetup.getHeight()) != sepx.getInt(699, 1);
    }


    // java private static int normalizeOrientation(float w, float h)
    pub fn normalizeOrientation(w: f32, h: f32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java int wi = (int)(w * 20.0F);
      let mut wi = ();
      // java int hi = (int)(h * 20.0F);
      let mut hi = ();
      // java return wi > hi ? 2 : 1;
    }


    // java private void changeSepx(PLCSection.SectionNode node, PropertySet newProp)
    pub fn changeSepx(&mut self, node: &mut SectionNode, newProp: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java MutablePropertySet newSepx = new MutablePropertySet(node.sepx);
      let mut newSepx = ();
      // java Revision revision = (Revision)node.sepx.getValue(727);
      // java if (revision != null)
        // java revision = new Revision(revision.getMark(), revision.getAuthor(), revision.getDateTime());
        // java revision.setProperty(PropertySet.merge(revision.getProperty(), newProp));
        // java newSepx.setValue(727, revision);
      // java else
        // java newSepx.add(newProp);
      // java node.setPropertySet(newSepx.freeze());
    }


    // java public void setPageSetup(int cp, PageSetup pageSetup)
    pub fn setPageSetup(&mut self, cp: i32, pageSetup: Option<&PageSetup>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.isProtectOn())
        // java if (this.isThrowDocumentProtectionException())
          // java throw new DocumentProtectionException("Document protected!");
      // java else
        // java this.start();
            // java this.setModified(true);
            // java Document document = this.getMainDocument();
            // java PLCSection section = document.getPLCSection();
            // java if (pageSetup != null && section != null)
                // java PLCSection.SectionNode node = section.getSection(cp);
                // java if (node == null)
                  // java node = section.addSection(cp);
                  // java node.sepx = this.createInitialSepx(pageSetup, 1);
                // java PropertySet sepx = this.getMergedSepx(node.sepx);
                // java PageSetup old = (PageSetup)sepx.getValue(676);
                // java if (old == null || !old.similar(pageSetup) || this.orientation_equal(pageSetup, sepx))
                  // java int[] ids = new int[]{676, 699};
                  let mut ids = ();
                  // java Object[] values = new Object[]{pageSetup, normalizeOrientation((float)pageSetup.getWidth(), (float)pageSetup.getHeight())};
                  // java PropertySet newProp = PropertySet.valueOf(ids, values);
                  // java this.changeSepx(node, newProp);
                // java DxaColumn[] dxaColumn = (DxaColumn[])sepx.getValue(721);
                // java if (null != dxaColumn)
                  // java DxaColumn[] newDxaColumn = (DxaColumn[])Arrays.copyOf(dxaColumn, dxaColumn.length);
                  // java int distance_left = pageSetup.getMarginLeft() - old.getMarginLeft();
                  // java int distance_right = pageSetup.getMarginRight() - old.getMarginRight();
                  // java if (distance_left != 0)
                    // java DxaColumn firstColumn = dxaColumn[0];
                    let mut firstColumn = ();
                    // java int with_firstColumn = firstColumn.getWidth() - distance_left;
                    // java int spacing_firstColumn = firstColumn.getSpacing();
                    // java newDxaColumn[0] = new DxaColumn(spacing_firstColumn, with_firstColumn);
                  // java if (distance_right != 0)
                    // java DxaColumn lastColumn = dxaColumn[dxaColumn.length - 1];
                    let mut lastColumn = ();
                    // java int with_lastColumn = lastColumn.getWidth() - distance_right;
                    // java int spacing_lastColumn = lastColumn.getSpacing();
                    // java newDxaColumn[dxaColumn.length - 1] = new DxaColumn(spacing_lastColumn, with_lastColumn);
                  // java PropertySet newProp = PropertySet.valueOf(721, newDxaColumn);
                  // java this.changeSepx(node, newProp);
              // java this.commit("Page Setup");
    }


    // java public PageSetup getPageSetup(int cp)
    pub fn getPageSetup(&self, cp: i32) -> Option<PageSetup> {
      // java PLCSection section = this.getMainDocument().getPLCSection();
      // java RangeReadLock readLock = this.getMainDocument().readLockAll(); node = section.getSection(cp);
      let sepx = if let Some(main) = self.getMainDocument() {
        let _lock = main.readLockAll();
        main.getPLCSection()
          .and_then(|section| section.getSection(cp))
          .map(|node| node.getPropertySet())
      } else {
        None
      };
      // java PropertySet sepx = this.getMergedSepx(node.sepx); PageSetup pageSetup = (PageSetup)sepx.getValue(676);
      if let Some(sepx) = sepx.as_ref() {
        if let Some(page) = Self::page_setup_value(&self.getMergedSepx(sepx)) {
          return Some(page);
        }
      }
      // java PropertySet props = DefaultSectionProperties.getProperties();
      let props = crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties::getProperties();
      // java return (PageSetup)prop; else throw PropertyException
      Self::page_setup_value(&props)
    }

    fn page_setup_value(prop: &PropertySet) -> Option<PageSetup> {
      // java (PageSetup)sepx.getValue(676): dereference Arc before as_any.
      // This file imports Object, whose blanket impl for Arc would otherwise
      // downcast the Arc wrapper rather than its PageSetup payload.
      prop.getValue(676).and_then(|value| value.as_ref().as_any().downcast_ref::<PageSetup>().copied())
    }

    // java public WdCompatibilityMode getCompatibilityMode() 的只读计算。缓存仍由 `&mut` 版本写入。
    pub fn compatibilityMode(&self) -> WdCompatibilityMode {
      let compat_setting_arr = self.property.as_ref().and_then(|p| {
        p.getValue(542).and_then(|v| v.as_any().downcast_ref::<Vec<CompatSetting>>().cloned())
      });
      if let Some(compat_setting_arr) = compat_setting_arr {
        for setting in &compat_setting_arr {
          if setting.getName().as_deref().is_some_and(|n| n.eq_ignore_ascii_case("compatibilityMode")) {
            if let Some(val) = setting.getVal() {
              return WdCompatibilityMode::convertValue(&val);
            }
          }
        }
      }
      // java this.mCompatbilityMode = WdCompatibilityMode.wdWord2010;
      WdCompatibilityMode::wdWord2010
    }


    // java @Deprecated
    // java public void setPageOrientation(int cp, Integer orientation)
    pub fn setPageOrientation__int_integer(&mut self, cp: i32, orientation: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java KSLog.e(TAG, "Call setPageSetup!!");
    }


    // java public void setPageOrientation(int orientation)
    pub fn setPageOrientation__int(&mut self, orientation: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.isProtectOn())
        // java if (this.isThrowDocumentProtectionException())
          // java throw new DocumentProtectionException("Document protected!");
      // java else
        // java this.start();
            // java this.beginSectionChange();
            // java this.setModified(true);
            // java PLCSection plcSection = this.getMainDocument().getPLCSection();
            // java PLC.Iterator iterator = plcSection.createIterator(plcSection.begin());
            // java while(!iterator.isEnd())
              // java PLCSection.SectionNode sectionNode = (PLCSection.SectionNode)iterator.getNode();
              // java this.setPageOrientation(orientation, sectionNode);
              // java iterator.next();
            // java this.endSectionChange();
              // java this.commit("Page setPageOrientation");
      // java KSLog.e(TAG, "Call setPageSetup!!")
    }


    // java private void setPageOrientation(int orientation, PLCSection.SectionNode node)
    pub fn setPageOrientation__int_sectionnode(&mut self, orientation: i32, node: &SectionNode) {
      todo!("DEFERRED_NON_FUNCTION");
      // java PropertySet sepx = this.getMergedSepx(node.sepx);
      // java PageSetup old = (PageSetup)sepx.getValue(676);
      // java Integer oldOr = (Integer)sepx.getValue(699);
      // java if (old == null)
        // java old = (PageSetup)DefaultSectionProperties.getProperties().getValue(676);
        // java oldOr = normalizeOrientation((float)old.getWidth(), (float)old.getHeight());
      // java int rightOrientation = normalizeOrientation((float)old.getWidth(), (float)old.getHeight());
      // java if (oldOr != rightOrientation)
        // java oldOr = rightOrientation;
        let mut oldOr = ();
      // java if (oldOr != orientation)
        // java PageSetup newPageSetup = new PageSetup(old.getHeight(), old.getWidth(), old.getMarginTop(), old.getMarginBottom(), old.getMarginLeft(), old.getMarginRight());
        // java int[] ids = new int[]{676, 699};
        let mut ids = ();
        // java Object[] values = new Object[]{newPageSetup, orientation};
        let mut values = ();
        // java PropertySet newProp = PropertySet.valueOf(ids, values);
        // java this.changeSepx(node, newProp);
      // java if (this.isProtectOn())
      // java if (this.isThrowDocumentProtectionException())
      // java throw new DocumentProtectionException("Document protected!")
      // java this.start()
      // java this.beginSectionChange()
      // java this.setModified(true)
      // java PLCSection plcSection = this.getMainDocument().getPLCSection()
      // java PLC.Iterator iterator = plcSection.createIterator(plcSection.begin())
      // java while(!iterator.isEnd())
      // java PLCSection.SectionNode sectionNode = (PLCSection.SectionNode)iterator.getNode()
      // java this.setPageOrientation(orientation, sectionNode)
      // java iterator.next()
      // java this.endSectionChange()
      // java this.commit("Page setPageOrientation")
    }


    // java public byte[] exportAsArray(FileFormat format)
    pub fn exportAsArray(&mut self, format: &FileFormat) -> Option<Vec<u8>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java WordWriter writer = new WordWriter(this, format);
      let mut writer = ();
          // java return writer.exportAsArray();
          // java InvalidFormatException e
            // java Log.w(TAG, "normal save should not throws WriterAutoWriteException", e);
            // java return null;
    }


    // java public int advanceGVmlOffset()
    pub fn advanceGVmlOffset(&mut self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mGVmlOffset += 12;
    }


    // java public synchronized void setGridResizeRequest(GridResizeRequest gridResizeRequest)
    pub fn setGridResizeRequest(&mut self, gridResizeRequest: Box<dyn GridResizeRequest>) {
      // java this.mGridResizeRequest = gridResizeRequest;
      *self.mGridResizeRequest.borrow_mut() = Some(gridResizeRequest);
    }


    // java public synchronized GridResizeRequest getGridResizeRequest()
    pub fn getGridResizeRequest(
        &self,
    ) -> Option<std::cell::RefMut<'_, dyn GridResizeRequest>> {
      // java return this.mGridResizeRequest;
      Self::readGridResizeRequest(self)
    }

    pub(crate) fn readGridResizeRequest(
        &self,
    ) -> Option<std::cell::RefMut<'_, dyn GridResizeRequest>> {
      let borrow = self.mGridResizeRequest.borrow_mut();
      if borrow.is_none() {
        return None;
      }
      Some(std::cell::RefMut::map(borrow, |slot| {
        slot.as_mut().unwrap().as_mut()
      }))
    }


    // java public SaveAccelerator getSaveAccelerator()
    pub fn getSaveAccelerator(&mut self) -> SaveAccelerator {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mSaveAccelerator == null)
          // java (this)
            // java if (this.mSaveAccelerator == null)
              // java this.mSaveAccelerator = new SaveAccelerator(this);
      // java return this.mSaveAccelerator;
      // java synchronized(this)
    }


    // java public void setDebugger(IDebugger debugger)
    pub fn setDebugger(&mut self, debugger: &dyn IDebugger) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mDebugger = debugger;
    }


    // java public boolean isDebugging()
    pub fn isDebugging(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mDebugger != null;
    }


    // java public void onDebug(String id, Object... params)
    pub fn onDebug(&mut self, id: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mDebugger != null)
        // java this.mDebugger.onDebug(id, params);
    }


    // java public void collectOpenedDocumentInfo(HashMap<String, Integer> content_num)
    pub fn collectOpenedDocumentInfo(&self, content_num: Option<&mut HashMap<String, i32>>) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null != content_num && !this.isInIOState())
          // java int[] counts = new int[6];
          let mut counts = ();
              // java IObjectProcedure<Shape> procedure = (shape) -> { if (shape.isPicture() && !shape.isOleShape()) { int var10002 = counts[0]++; } if (shape.isChartShape()) { int var2 = counts[1]++; } int var3 = counts[4]++; return true; };
              // java Document mainDocument = this.getMainDocument();
              // java counts[5] = mainDocument.getLength();
              // java mainDocument.getDrawingContainer().forEarchShapeWithChild(procedure);
              // java counts[2] = mainDocument.getTableInfos().size();
              // java PLCMathStart mathStart = mainDocument.getOMathStarts();
              // java if (null != mathStart)
                // java counts[3] = mathStart.size();
              // java for(PLCField.FieldTriple fieldTriple : mainDocument.getPLCField().getFieldTriples())
                // java KFieldType kFieldType = KFieldType.getKFieldType(fieldTriple.getFieldType());
                // java if (kFieldType == KFieldType.FieldEQ)
                  // java int var10002 = counts[3]++;
                  let mut var10002 = ();
              // java Exception var13
                // java content_num.put("photo", counts[0]);
                // java content_num.put("chart", counts[1]);
                // java content_num.put("form", counts[2]);
                // java content_num.put("fx", counts[3]);
                // java content_num.put("objectsum", counts[4]);
                // java content_num.put("length", counts[5]);
      // java for(PLCField.FieldTriple fieldTriple : mainDocument.getPLCField().getFieldTriples())
      // java catch (Exception var13)
    }


    // java public boolean hasHeadings()
    pub fn hasHeadings(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getMainDocument().hasHeadings();
    }


    // java public void initPaperRecognition(PaperKeyWordParam paperKeyWordParam)
    pub fn initPaperRecognition(&mut self, paperKeyWordParam: &PaperKeyWordParam) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == this.mPaperRecognition)
        // java this.mPaperRecognition = new PaperRecognition(paperKeyWordParam);
      // java else
        // java this.mPaperRecognition.init(paperKeyWordParam);
    }


    // java public boolean isPaper()
    pub fn isPaper(&mut self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null == this.mPaperRecognition ? false : this.mPaperRecognition.checkTestPaper(this.getPath(), this).isPaper();
    }


    // java public void setScrolling()
    pub fn setScrolling(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mStateIO && !this.mOpenIdleScroll)
        // java this.mOpenIdleScroll = true;
    }


    // java public boolean isOpenIdleScroll()
    pub fn isOpenIdleScroll(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mOpenIdleScroll;
    }


    // java public KParagraphsID getKParagraphsID()
    pub fn getKParagraphsID(&mut self) -> &mut KParagraphsID {
      // java return this.mParasID;
      self.mParasID.as_mut().expect("mParasID")
    }


    // java public String getParaText(String paraId)
    pub fn getParaText(&self, paraId: &str) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java KParagraphsID.ParaEntry paraEntry = this.getKParagraphsID().getParaEntry(paraId);
      // java if (null == paraEntry)
        // java return null;
      // java else
        // java int documentType = paraEntry.mSubDocumentType;
        let mut documentType = ();
        // java IPapxTable.Entry entry = paraEntry.mEntry;
        let mut entry = ();
        // java Document document = this.getSubdocument(documentType);
        // java int start = entry.position();
        // java int end = entry.endCp() - 1;
        // java char[] buf = new char[end - start];
        let mut buf = ();
        // java int n = document.getText(start, end, buf, 0);
        // java return new String(buf, 0, n);
    }


    // java public List<String> getSubDocParaIds(int docType)
    pub fn getSubDocParaIds(&self, docType: i32) -> Option<Vec<String>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (0 <= docType && docType < 7)
        // java return this.getSubdocument(docType).getDocParaIds();
      // java else if (7 == docType)
        // java List<String> docParaIds = new ArrayList();
        let mut docParaIds = ();
        // java for(int i = 0; i < 7; ++i)
          // java List<String> paraIds = this.getSubdocument(i).getDocParaIds();
          // java if (null != paraIds)
            // java docParaIds.addAll(paraIds);
        // java return docParaIds;
      // java else
        // java return null;
      // java for(int i = 0; i < 7; ++i)
    }


}

impl std::ops::Deref for TextDocument {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class PapxTableListener implements IPapxTable.Listener
impl PapxTableListener {
    // java public  PapxTableListener(Document document)
    pub fn PapxTableListener(document: &dyn Document) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mDocument = (SubDocument)document;
      // java this.mListeners = new ArrayList();
      // java this.mListeners.add(document.getTableInfos().getPapxListener());
      // java this.mListeners.add(new StyleListener4PapxTable(this.mDocument.getStyles()));
      // java if (document.getTableInfos().getPapxListener() != null)
    }

    // java public void beforeAddEntry(int cp, PropertySet data)
    pub fn beforeAddEntry(&self, cp: i32, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IPapxTable.Listener)this.mListeners.get(i)).beforeAddEntry(cp, data);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterAddEntry(ITable.Entry entry)
    pub fn afterAddEntry(&self, entry: &dyn PapxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IPapxTable.Listener)this.mListeners.get(i)).afterAddEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void beforeRemoveEntry(ITable.Entry entry)
    pub fn beforeRemoveEntry(&self, entry: &dyn PapxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IPapxTable.Listener)this.mListeners.get(i)).beforeRemoveEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterRemoveEntry(ITable.Entry entry)
    pub fn afterRemoveEntry(&self, entry: &dyn PapxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IPapxTable.Listener)this.mListeners.get(i)).afterRemoveEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void beforeDataChange(ITable.Entry entry, PropertySet data)
    pub fn beforeDataChange(&self, entry: &dyn PapxEntry, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IPapxTable.Listener)this.mListeners.get(i)).beforeDataChange(entry, data);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterDataChange(ITable.Entry entry, PropertySet data)
    pub fn afterDataChange(&mut self, entry: &mut dyn PapxEntry, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
      // java ((IPapxTable.Listener)this.mListeners.get(i)).afterDataChange(entry, data);
      // java DocumentEvent e = DefaultModelEvent.obtain(this.mDocument, entry.position(), entry.length(), 3, 11);
      let mut e = ();
      let _ = e;
      // java this.mDocument.fireDocumentUpdate(e);
      // java e.decreaseRef();
          // java if (this.mDocument.getType() == 0 && this.mDocument.mHeadings != null)
            // java this.mDocument.mHeadings.afterParagraphPropertyChange((IPapxTable.Entry)entry, data);
          // java Exception ex
            // java ex.printStackTrace();
      // java for(int n = this.mListeners.size(); i < n; ++i)
      // java catch (Exception ex)
    }

}

// java public static class ChpxTableListener implements IChpxTable.Listener
impl ChpxTableListener {
    // java public  ChpxTableListener(Document document)
    pub fn ChpxTableListener(document: &dyn Document) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mDocument = (SubDocument)document;
      // java this.mListeners = new ArrayList();
      // java this.mListeners.add(document.getTableInfos().getChpxListener());
      // java this.mListeners.add(new StyleListener4ChpxTable(this.mDocument.getStyles()));
      // java if (document.getTableInfos().getChpxListener() != null)
    }

    // java public void beforeAddEntry(int cp, PropertySet data)
    pub fn beforeAddEntry(&self, cp: i32, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IChpxTable.Listener)this.mListeners.get(i)).beforeAddEntry(cp, data);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterAddEntry(ITable.Entry entry)
    pub fn afterAddEntry(&self, entry: &dyn ChpxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IChpxTable.Listener)this.mListeners.get(i)).afterAddEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void beforeRemoveEntry(ITable.Entry entry)
    pub fn beforeRemoveEntry(&self, entry: &dyn ChpxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IChpxTable.Listener)this.mListeners.get(i)).beforeRemoveEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterRemoveEntry(ITable.Entry entry)
    pub fn afterRemoveEntry(&self, entry: &dyn ChpxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IChpxTable.Listener)this.mListeners.get(i)).afterRemoveEntry(entry);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void beforeDataChange(ITable.Entry entry, PropertySet data)
    pub fn beforeDataChange(&self, entry: &dyn ChpxEntry, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
        // java ((IChpxTable.Listener)this.mListeners.get(i)).beforeDataChange(entry, data);
      // java for(int n = this.mListeners.size(); i < n; ++i)
    }

    // java public void afterDataChange(ITable.Entry entry, PropertySet data)
    pub fn afterDataChange(&mut self, entry: &mut dyn ChpxEntry, data: &PropertySet) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int i = 0;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int n = this.mListeners.size(); i < n; ++i)
      // java ((IChpxTable.Listener)this.mListeners.get(i)).afterDataChange(entry, data);
      // java DocumentEvent e = DefaultModelEvent.obtain(this.mDocument, entry.position(), entry.length(), 3, 12);
      let mut e = ();
      let _ = e;
      // java this.mDocument.fireDocumentUpdate(e);
      // java e.decreaseRef();
          // java if (this.mDocument.getType() == 0 && this.mDocument.mHeadings != null)
            // java boolean fRevisionDelete_old = RangeTool.isRevisionDelete(data);
            let mut fRevisionDelete_old = ();
            let _ = fRevisionDelete_old;
            // java boolean fRevisionDelete_new = RangeTool.isRevisionDelete(entry.getProperty());
            let mut fRevisionDelete_new = ();
            let _ = fRevisionDelete_new;
            // java if (fRevisionDelete_old != fRevisionDelete_new)
              // java this.mDocument.mHeadings.update_on_revision_change(entry.position(), entry.endCp());
          // java Exception ex
            // java ex.printStackTrace();
      // java for(int n = this.mListeners.size(); i < n; ++i)
      // java catch (Exception ex)
    }

}

// java private class MergeListener implements ActionList.ActionChangeListener
impl MergeListener {
    // java private  MergeListener()
    pub fn MergeListener() -> Self {
      Self {}
    }

    // java public void beforeMerge(Action last, Action current)
    pub fn beforeMerge(&mut self, last: &Action, current: &Action) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int size = TextDocument.this.mViews.size();
      let mut size: i32 = 0;
      let _ = size;
      let mut i: i32 = 0;
      let _ = i;
      // java for(int i = 0; i < size; ++i)
        // java ITextEditor pEditor = (ITextEditor)TextDocument.this.mViews.get(i);
        let mut pEditor = ();
        let _ = pEditor;
        // java pEditor.mergeSelectionStore(last, current);
      // java for(int i = 0; i < size; ++i)
    }

    // java public void onAbandonUndoAction(Action pollAction)
    pub fn onAbandonUndoAction(&mut self, pollAction: &Action) {
    }

}

// java public static final class UUNumberingId extends Atom
impl UUNumberingId {
    // java private  UUNumberingId()
    pub fn UUNumberingId() -> Self {
      // java this.mId = 0;
      Self { super_: Atom::new(), mId: 0 }
    }

    // java public int getNextId()
    pub fn getNextId(&mut self) -> i32 {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java return ++this.mId;
      self.mId += 1;
      self.mId
    }

    // java public int getCurrentId()
    pub fn getCurrentId(&self) -> i32 {
      // java return this.mId;
      return self.mId;
    }

}

impl std::ops::Deref for UUNumberingId {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for UUNumberingId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static final class UULsid extends Atom
impl UULsid {
    // java private  UULsid()
    pub fn UULsid() -> Self {
      // java this.mId = 0;
      Self { super_: Atom::new(), mId: 0 }
    }

    // java public int getNextId()
    pub fn getNextId(&mut self) -> i32 {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java return ++this.mId;
      self.mId += 1;
      self.mId
    }

    // java public int getCurrentId()
    pub fn getCurrentId(&self) -> i32 {
      // java return this.mId;
      return self.mId;
    }

}

impl std::ops::Deref for UULsid {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for UULsid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class TextDocument extends Atom implements IDocument, ITextOpenDocument, DocumentProtection
// M-STAGE2 S2-2: TextDocument 在 Java 中是 IDocument 的实现者（布局/渲染管线读接口）。
//   本 impl 把 Document trait 的读接口委托给主文档（mSubdocuments[0]），
//   与 Java 中 `this.getMainDocument().xxx()` 的委托语义一致。
//   仅覆盖 SubDocument 同样覆盖的那组方法；其余走 trait 默认实现。
impl drawing::cn::wps::moffice::drawing::IDocument::IDocument for TextDocument {
    // java public int getType()
    fn getType(&self) -> i32 {
        // java return 0;
        // 顶层 TextDocument 即主文档，对应 mSubdocuments[0]；
        // Java 用 doc.getType() 作为 mSubdocuments[] 下标（见 liteExchangeDocument）。
        0
    }
}

impl Document for TextDocument {
    // rust: ★T7 —— Java 的强制类型转换入口；`TextDocument` 自身即目标类型。
    fn asAny(&self) -> &dyn std::any::Any {
        self
    }
    fn asAny_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // java public int getLength()
    fn getLength(&self) -> i32 {
        // java return this.getMainDocument().getLength();
        TextDocument::getLength(self)
    }

    // java public ITextRope getTextRope()
    fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
        // java return this.getMainDocument().getTextRope();
        Document::getMainDocument(self).and_then(|d| d.getTextRope())
    }

    // java public IChpxTable getChpxTable()
    fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
        // java return this.getMainDocument().getChpxTable();
        Document::getMainDocument(self).and_then(|d| d.getChpxTable())
    }

    // java public IPapxTable getPapxTable()
    fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
        // java return this.getMainDocument().getPapxTable();
        Document::getMainDocument(self).and_then(|d| d.getPapxTable())
    }

    // java public Document getMainDocument()
    fn getMainDocument(&self) -> Option<&dyn Document> {
        // java return this.mSubdocuments[0];
        TextDocument::getMainDocument(self).map(|d| d as &dyn Document)
    }

    // java public PLCSection getPLCSection()
    // rust: ★T7-9 —— 门面真实化：原返回 `*mut ()`（假编译面）；改真实类型，与 Java
    //   「委托到 `getMainDocument().getPLCSection()`」逐字一致。
    fn getPLCSection(&self) -> Option<*mut PLCSection> {
        // java return this.getMainDocument().getPLCSection();
        Document::getMainDocument(self).and_then(|d| d.getPLCSection())
    }

    // java public SoftTextDocument getTextDocument()
    fn getTextDocument(&self) -> Option<&dyn SoftTextDocument> {
        // java return this;
        Some(self)
    }

    // java public Transaction getTransaction()
    fn getTransaction(&self) -> Option<&Transaction> {
        // java return this.super_.getTransaction();
        // rust: `Atom::getTransaction()` 返回 `Option<Rc<RefCell<Transaction>>>`。
        //   ★ 必须经 `RefCell::as_ptr()` 取 `*mut Transaction`；直接对 `Rc` 调 `as_ptr()`
        //   得到的是 `*const RefCell<Transaction>`，解引用即类型混淆（读到 borrow flag）
        //   ⇒ SIGSEGV（D4_all_main_content_matrix 曾因此崩溃）。
        self.super_.getTransaction().map(|rc| {
            let cell: &RefCell<Transaction> = &rc;
            unsafe { &*cell.as_ptr() }
        })
    }

    // java public KStyles getStyles()
    // rust: ★T7-9 —— 门面真实化（原 `*mut ()` 假编译面）。
    fn getStyles(&self) -> Option<*mut KStyles> {
        // java return this.styles;
        TextDocument::getStyles(self).map(|s| s as *const KStyles as *mut KStyles)
    }

    // java public int getParagraphRange(int start, int end)
    fn getParagraphRange(&self, start: i32, end: i32) -> i64 {
        // java return this.getMainDocument().getParagraphRange(start, end);
        Document::getMainDocument(self)
            .map(|d| d.getParagraphRange(start, end))
            .unwrap_or(0)
    }

    // java public char charAt(int cp)
    fn charAt(&mut self, cp: i32) -> char {
        // java return this.getMainDocument().charAt(cp);
        TextDocument::getMainDocumentMut(self)
            .map(|d| d.charAt(cp))
            .unwrap_or('\0')
    }

    // java public int getText(int start, int end, char[] dst, int offset)
    fn getText(&self, start: i32, end: i32, dst: &mut [char], offset: i32) -> i32 {
        // java return this.getMainDocument().getText(start, end, dst, offset);
        Document::getMainDocument(self)
            .map(|d| d.getText(start, end, dst, offset))
            .unwrap_or(0)
    }
}
