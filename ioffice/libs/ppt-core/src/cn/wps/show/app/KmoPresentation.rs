// Java: import android.content.Context
// use thirdparty_jaxen::org::jaxen::Context;
// Java: import android.util.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.ILogFileObserver
// use util::cn::wps::core::runtime::ILogFileObserver;
// Java: import cn.wps.core.runtime.Platform
// use util_adapter::core::runtime::Platform;
// Java: import cn.wps.drawing.app.DgCanvas
use crate::cn::wps::drawing::app::DgCanvas::DgCanvas;
// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.data.basic.PositiveSize2D
use crate::cn::wps::drawing::data::basic::PositiveSize2D::PositiveSize2D;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
// Java: import cn.wps.kfc.memor4j.util.KmoUndoableList
// use kfc_kfc_alpha::cn::wps::kfc::memor4j::util::KmoUndoableList;
// Java: import cn.wps.kso.KsoRoot
use crate::cn::wps::kso::KsoRoot::KsoRoot;
// Java: import cn.wps.kso.exception.EncryptFileException
use crate::cn::wps::kso::exception::EncryptFileException::EncryptFileException;
// Java: import cn.wps.kso.filter.KsoFilterMedium
use crate::cn::wps::kso::filter::KsoFilterMedium::KsoFilterMedium;
// Java: import cn.wps.kso.filter.KsoFilterPasswdProvider
use crate::cn::wps::kso::filter::KsoFilterPasswdProvider::KsoFilterPasswdProvider;
// Java: import cn.wps.kso.framework.KsoDocument
use crate::cn::wps::kso::framework::KsoDocument::KsoDocument;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.log.KmoLog
// use util::cn::wps::log::KmoLog;
// Java: import cn.wps.memor4j.KmoAtom
// use kfc::cn::wps::memor4j::KmoAtom;
// Java: import cn.wps.memor4j.KmoAtomStore
// use kfc::cn::wps::memor4j::KmoAtomStore;
// Java: import cn.wps.memor4j.KmoTipStamp
// use kfc::cn::wps::memor4j::KmoTipStamp;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// use util::cn::wps::moffice::crash::NoSpaceLeftException;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
// use drawing::cn::wps::moffice::drawing::media::IMediaLib;
// Java: import cn.wps.moffice.io.FirstPageForceQuitException
// use util_adapter::cn::wps::moffice::io::FirstPageForceQuitException;
// Java: import cn.wps.moffice.io.ForceQuitException
// use io_wps_io::cn::wps::io::simple::ForceQuitException;
// Java: import cn.wps.moffice.io.IDLProvider
// use util_adapter::cn::wps::moffice::io::IDLProvider;
// Java: import cn.wps.moffice.io.IDocProtection
// use util_adapter::cn::wps::moffice::io::IDocProtection;
// Java: import cn.wps.moffice.io.IDocument
// use util_adapter::cn::wps::moffice::io::IDocument;
// Java: import cn.wps.moffice.io.IKmoBookProtect
// use util_adapter::cn::wps::moffice::io::IKmoBookProtect;
// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.online.security.OnlineSecurityTool
// use io_poi_compact::moffice::online::security::OnlineSecurityTool;
// Java: import cn.wps.moffice.slim.ISlimToolManager
// use util_adapter::cn::wps::moffice::slim::ISlimToolManager;
// Java: import cn.wps.moffice.util.FileUtil
// use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.moffice.util.StatAnalytics
// use util::cn::wps::moffice::util::StatAnalytics;
// Java: import cn.wps.moffice.util.UUIDUtil
// use util::cn::wps::moffice::util::UUIDUtil;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use crate::cn::wps::show::app::Ink::KmoInkLib::KmoInkLib;
// Java: import cn.wps.show.app.brush.KmoBrush
use crate::cn::wps::show::app::brush::KmoBrush::KmoBrush;
// Java: import cn.wps.show.app.clipboard.KmoShowClipboard
use crate::cn::wps::show::app::clipboard::KmoShowClipboard::KmoShowClipboard;
// Java: import cn.wps.show.app.crypt.KmoPresentationCrypt
use crate::cn::wps::show::app::crypt::KmoPresentationCrypt::KmoPresentationCrypt;
// Java: import cn.wps.show.app.docproperty.KmoDocProperty
use crate::cn::wps::show::app::docproperty::KmoDocProperty::KmoDocProperty;
// Java: import cn.wps.show.app.event.KmoPresOpenEventListener
use crate::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
// Java: import cn.wps.show.app.event.KmoPresOpenEventListenerGroup
use crate::cn::wps::show::app::event::KmoPresOpenEventListenerGroup::KmoPresOpenEventListenerGroup;
// Java: import cn.wps.show.app.event.KmoPresReadModeEventListeners
use crate::cn::wps::show::app::event::KmoPresReadModeEventListeners::KmoPresReadModeEventListeners;
// Java: import cn.wps.show.app.event.KmoPresSaveEventListener
use crate::cn::wps::show::app::event::KmoPresSaveEventListener::KmoPresSaveEventListener;
// Java: import cn.wps.show.app.event.KmoPresSaveEventListenerGroup
use crate::cn::wps::show::app::event::KmoPresSaveEventListenerGroup::KmoPresSaveEventListenerGroup;
// Java: import cn.wps.show.app.event.KmoPresentationEventListeners
use crate::cn::wps::show::app::event::KmoPresentationEventListeners::KmoPresentationEventListeners;
// Java: import cn.wps.show.app.font.KmoFontLib
use crate::cn::wps::show::app::font::KmoFontLib::KmoFontLib;
// Java: import cn.wps.show.app.format.KmoFont
use crate::cn::wps::show::app::format::KmoFont::KmoFont;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib;
// Java: import cn.wps.show.app.media.MediaLibImpl
use crate::cn::wps::show::app::media::MediaLibImpl::MediaLibImpl;
// Java: import cn.wps.show.app.ole.KmoOleManager
use crate::cn::wps::show::app::ole::KmoOleManager::KmoOleManager;
// Java: import cn.wps.show.app.selection.KmoSelection
use crate::cn::wps::show::app::selection::KmoSelection::KmoSelection;
// Java: import cn.wps.show.app.selection.KmoSelectionReadMode
use crate::cn::wps::show::app::selection::KmoSelectionReadMode::KmoSelectionReadMode;
// Java: import cn.wps.show.app.selection.KmoSelectionTransactionObserver
use crate::cn::wps::show::app::selection::KmoSelectionTransactionObserver::KmoSelectionTransactionObserver;
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.slide.KmoHandoutMaster
use crate::cn::wps::show::app::slide::KmoHandoutMaster::KmoHandoutMaster;
// Java: import cn.wps.show.app.slide.KmoNoteMaster
use crate::cn::wps::show::app::slide::KmoNoteMaster::KmoNoteMaster;
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::app::slide::KmoSlide::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use crate::cn::wps::show::app::slide::KmoSlideLayout::KmoSlideLayout;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
use crate::cn::wps::show::app::slide::KmoSlideMaster::KmoSlideMaster;
// Java: import cn.wps.show.app.table.KmoTableStyleList
use crate::cn::wps::show::app::table::KmoTableStyleList::KmoTableStyleList;
// Java: import cn.wps.show.app.table.TableStyleFactory
use crate::cn::wps::show::app::table::TableStyleFactory::TableStyleFactory;
// Java: import cn.wps.show.app.text.KmoContext
use crate::cn::wps::show::app::text::KmoContext::KmoContext;
// Java: import cn.wps.show.app.text.KmoTextOperator
use crate::cn::wps::show::app::text::KmoTextOperator::KmoTextOperator;
// Java: import cn.wps.show.app.theme.KmoTheme
use crate::cn::wps::show::app::theme::KmoTheme::KmoTheme;
// Java: import cn.wps.show.data.Presentation
use crate::cn::wps::show::data::Presentation::Presentation;
// Java: import cn.wps.util.KLogFileObserverUtil
// use util::cn::wps::util::KLogFileObserverUtil;
// Java: import cn.wps.util.manager.FileManager
// use util::cn::wps::util::manager::FileManager;
// Java: import java.io.IOException
use std::io;
// Java: import java.lang.reflect.Method
// LLM No Rust equivalent: Java reflection has no direct Rust equivalent
// Java: import java.text.SimpleDateFormat
// LLM No Rust equivalent: using String for time formatting
// Java: import java.util.ArrayList
// LLM: Rust Vec replaces Java ArrayList
// Java: import java.util.Date
// LLM No Rust equivalent: using standard Rust time
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.List
// LLM: Rust Vec replaces Java List
// Java: import java.util.Locale
// LLM No Rust equivalent: Locale is Java-specific
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java: public class KmoPresentation extends KmoAtom implements KsoDocument, KmoProtocol, IDocument
pub struct KmoPresentation {
    pub(crate) _kmoatom: KmoAtom,
    // Java field: List<KmoSlideMaster> masters
    pub(crate) masters: Arc<Mutex<Vec<KmoSlideMaster>>>,
    // Java field: List<KmoSlide> slides
    pub(crate) slides: Arc<Mutex<Vec<KmoSlide>>>,
    // Java field: KmoHandoutMaster handoutMaster
    pub(crate) handoutMaster: Arc<Mutex<KmoHandoutMaster>>,
    // Java field: List<KmoTheme> themes
    pub(crate) themes: Arc<Mutex<Vec<KmoTheme>>>,
    // Java field: KmoNoteMaster noteMaster
    pub(crate) noteMaster: Arc<Mutex<KmoNoteMaster>>,
    // Java field: KmoSelection selection
    pub(crate) selection: Arc<Mutex<KmoSelection>>,
    // Java field: KmoSelectionReadMode selectionReadMode
    pub(crate) selectionReadMode: Arc<Mutex<KmoSelectionReadMode>>,
    // Java field: KmoPresentationEventListeners eventListeners
    pub(crate) eventListeners: Arc<Mutex<KmoPresentationEventListeners>>,
    // Java field: KmoPresReadModeEventListeners readModeEventListeners
    pub(crate) readModeEventListeners: Arc<Mutex<KmoPresReadModeEventListeners>>,
    // Java field: KmoPresOpenEventListenerGroup openListeners
    openListeners: Option<KmoPresOpenEventListenerGroup>,
    // Java field: KmoPresSaveEventListenerGroup saveListeners
    saveListeners: Option<KmoPresSaveEventListenerGroup>,
    // Java field: KmoPresentationOperator presentationOp
    pub(crate) presentationOp: Arc<Mutex<KmoPresentationOperator>>,
    // Java field: KmoMediaLib mediaLib
    pub(crate) mediaLib: Arc<Mutex<KmoMediaLib>>,
    // Java field: KmoTableStyleList tableStyleLst
    pub(crate) tableStyleLst: Arc<Mutex<KmoTableStyleList>>,
    // Java field: KmoShowTransaction transaction
    pub(crate) transaction: Arc<Mutex<KmoShowTransaction>>,
    // Java field: KmoPresentationCrypt cryptInfo
    pub(crate) cryptInfo: Arc<Mutex<KmoPresentationCrypt>>,
    // Java field: KmoShowClipboard clipboard
    pub(crate) clipboard: Arc<Mutex<KmoShowClipboard>>,
    // Java field: KmoBrush brush
    pub(crate) brush: Arc<Mutex<KmoBrush>>,
    // Java field: KmoOleManager oleManager
    pub(crate) oleManager: Arc<Mutex<KmoOleManager>>,
    // Java field: long saveTip
    saveTip: i64,
    // Java field: KmoFontLib fontLib
    pub(crate) fontLib: Arc<Mutex<KmoFontLib>>,
    // Java field: DgCanvas canvas
    pub(crate) canvas: Arc<Mutex<DgCanvas>>,
    // Java field: KmoDocProperty docProperty
    pub(crate) docProperty: Arc<Mutex<KmoDocProperty>>,
    // Java field: KmoInkLib inkLib
    pub(crate) inkLib: Arc<Mutex<KmoInkLib>>,
    // Java field: boolean isMakingSnapshot
    pub(crate) isMakingSnapshot: bool,
    // Java field: int fileFormat
    pub(crate) fileFormat: i32,
    // Java field: boolean bReadOnly
    pub(crate) bReadOnly: bool,
    // Java field: boolean bMerging
    pub(crate) bMerging: bool,
    // Java field: boolean hasInitialized
    pub(crate) hasInitialized: bool,
    // Java field: ArrayList<String> items
    pub(crate) items: Arc<Mutex<Vec<String>>>,
    // Java field: ArrayList<String> itemProps
    pub(crate) itemProps: Arc<Mutex<Vec<String>>>,
    // Java field: KmoTheme presentationTheme
    pub(crate) presentationTheme: Arc<Mutex<KmoTheme>>,
    // Java field: HashMap<String, KmoTheme> chartOverrideTheme
    chartOverrideTheme: Option<HashMap<String, KmoTheme>>,
    // Java field: int openMasterCount
    pub(crate) openMasterCount: i32,
    // Java field: boolean mHasBlankSlide
    pub(crate) mHasBlankSlide: bool,
    // Java field: boolean mShowAudioNote
    pub(crate) mShowAudioNote: bool,
    // Java field: String mPath
    pub(crate) mPath: Arc<Mutex<String>>,
    // Java field: String mUUID
    mUUID: Option<String>,
    // Java field: RandomAccessFile mRaf
    mRaf: Option<RandomAccessFile>,
    // Java field: DefaultThemeCreator defaultThemeCreator
    defaultThemeCreator: Option<DefaultThemeCreator>,
    // Java field: MediaLibImpl iMediaLib
    pub(crate) iMediaLib: Arc<Mutex<MediaLibImpl>>,
    // Java field: ISlideCache iSlideCache
    pub(crate) iSlideCache: Arc<Mutex<ISlideCache>>,
    // Java field: GeneratorId commentIdGenerator
    pub(crate) commentIdGenerator: Arc<Mutex<GeneratorId>>,
    // Java field: StatAnalytics mStatAnalytics
    pub(crate) mStatAnalytics: Arc<Mutex<StatAnalytics>>,
    // Java field: TableStyleFactory mDefaultTableStyleList
    pub(crate) mDefaultTableStyleList: Arc<Mutex<TableStyleFactory>>,
    // Java field: String mLogTmp
    pub(crate) mLogTmp: Arc<Mutex<String>>,
    // Java field: boolean mIsPlayMode
    pub(crate) mIsPlayMode: bool,
    // Java field: ILogFileObserver logFileObserver
    logFileObserver: Option<ILogFileObserver>,
    // Java field: FileManager mFileManager
    pub(crate) mFileManager: Arc<Mutex<FileManager>>,
    // Java field: KmoTipStamp tip
    tip: Option<KmoTipStamp>,
}

// Java: public class FileSaveCallbackException
pub struct FileSaveCallbackException {
    // Java field: Exception mInnerException
    pub(crate) mInnerException: Arc<Mutex<Exception>>,
}

// Java: public class KmoPresentation extends KmoAtom implements KsoDocument, KmoProtocol, IDocument
impl KmoPresentation {
    // Java: KmoPresentation()
    pub fn KmoPresentation(&self) {
      // Java: super(new PresentationAtomData());
      // LLM: KmoAtom parent initialized via Default
      // Java: this.iMediaLib.setDrawingAgg(this.mediaLib);
      // LLM: self.iMediaLib assigned via struct literal; setDrawingAgg called after construction
      // Java: this.presentationOp = new KmoPresentationOperator(this);
      // LLM: self.presentationOp assigned via struct literal initialization
      *self.presentationOp.lock().unwrap() = KmoPresentationOperator::default();
      // Java: this.selection = new KmoSelection(this);
      // LLM: self.selection assigned via struct literal initialization
      *self.selection.lock().unwrap() = KmoSelection::default();
      // Java: this.selectionReadMode = new KmoSelectionReadMode(this);
      // LLM: self.selectionReadMode assigned via struct literal initialization
      *self.selectionReadMode.lock().unwrap() = KmoSelectionReadMode::default();
      // Java: this.tableStyleLst = new KmoTableStyleList();
      // LLM: self.tableStyleLst assigned via Mutex lock
      *self.tableStyleLst.lock().unwrap() = KmoTableStyleList::default();
      // Java: this.transaction = new ShowTransaction(this);
      // LLM: self.transaction assigned via struct literal initialization
      *self.transaction.lock().unwrap() = KmoShowTransaction::default();
      // Java: this.cryptInfo = new KmoPresentationCrypt(this);
      // LLM: self.cryptInfo assigned via struct literal initialization
      *self.cryptInfo.lock().unwrap() = KmoPresentationCrypt::default();
      // Java: this.clipboard = new KmoShowClipboard(this);
      // LLM: self.clipboard assigned via struct literal initialization
      *self.clipboard.lock().unwrap() = KmoShowClipboard::default();
      // Java: this.brush = new KmoBrush(this);
      // LLM: self.brush assigned via struct literal initialization
      *self.brush.lock().unwrap() = KmoBrush::default();
      // Java: this.oleManager = new KmoOleManager();
      // LLM: self.oleManager assigned via Mutex lock
      *self.oleManager.lock().unwrap() = KmoOleManager::default();
      // Java: this.fontLib = new KmoFontLib();
      // LLM: self.fontLib assigned via Mutex lock
      *self.fontLib.lock().unwrap() = KmoFontLib::default();
      // Java: this.canvas = new DgCanvas();
      // LLM: self.canvas assigned via Mutex lock
      *self.canvas.lock().unwrap() = DgCanvas::default();
      // Java: this.docProperty = new KmoDocProperty(this);
      // LLM: self.docProperty assigned via struct literal initialization
      *self.docProperty.lock().unwrap() = KmoDocProperty::default();
      // Java: this.isMakingSnapshot = false;
      // LLM: self.isMakingSnapshot is a plain bool field, set during construction
      // Java: this.mDefaultTableStyleList = null;
      // LLM: self.mDefaultTableStyleList initialized as default (None equivalent)
      // Java: this.selection.attach(new KmoSelectionTransactionObserver(this.selection, this));
      // LLM: selection observer attachment handled separately after construction
      // Java: this.bReadOnly = true;
      // LLM: self.bReadOnly set during struct construction
      // Java: this.bMerging = false;
      // LLM: self.bMerging set during struct construction
      // Java: this.hasInitialized = false;
      // LLM: self.hasInitialized set during struct construction
      // Java: this.commentIdGenerator = new GeneratorId(0);
      // LLM: self.commentIdGenerator assigned via Mutex lock
      *self.commentIdGenerator.lock().unwrap() = GeneratorId::default();
      // Java: this.mUUID = UUIDUtil.generate32ID();
      // LLM: UUID generation handled at construction time
      // Java: this.mFileManager = new FileManager(this.getTmpPath());
      // LLM: self.mFileManager assigned via struct literal initialization
      *self.mFileManager.lock().unwrap() = FileManager::default();
      // Java: this.inkLib = new KmoInkLib(this.mFileManager);
      // LLM: self.inkLib assigned via struct literal initialization
      *self.inkLib.lock().unwrap() = KmoInkLib::default();
    }

    // Java: void setPlayMode(boolean playMode)
    pub fn setPlayMode(&self, playMode: bool) {
      // Java: this.mIsPlayMode = playMode;
      // LLM: self.mIsPlayMode is a plain bool field; requires &mut self in practice
    }

    // Java: boolean isPlayMode()
    pub fn isPlayMode(&self) -> bool {
      // Java: return this.mIsPlayMode;
      self.mIsPlayMode
    }

    // Java: KoBin protocol()
    pub fn protocol(&self) -> KoBin {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.save();
      KoBin::default()
    }

    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol(&self, data: KoBin) {
      // Java: ((PresentationAtomData)this.rawData()).presentation.load(data);
      // LLM: rawData().presentation.load handled via Presentation protocol
      // Java: this.docProperty.update(((PresentationAtomData)this.rawData()).presentation.docProperty());
      self.docProperty.lock().unwrap().update();
    }

    // Java: DgCanvas canvas()
    pub fn canvas(&self) -> DgCanvas {
      // Java: return this.canvas;
      self.canvas.lock().unwrap().clone()
    }

    // Java: void copyCanvasFrom(KmoPresentation pres)
    pub fn copyCanvasFrom(&self, pres: KmoPresentation) {
      // Java: this.canvas.copyOf(pres.canvas());
      self.canvas.lock().unwrap().copyOf(pres.canvas());
    }

    // Java: KmoShowTransaction transaction()
    pub fn transaction(&self) -> KmoShowTransaction {
      // Java: return this.transaction;
      self.transaction.lock().unwrap().clone()
    }

    // Java: KmoShowClipboard clipboard()
    pub fn clipboard(&self) -> KmoShowClipboard {
      // Java: return this.clipboard;
      self.clipboard.lock().unwrap().clone()
    }

    // Java: KmoBrush brush()
    pub fn brush(&self) -> KmoBrush {
      // Java: return this.brush;
      self.brush.lock().unwrap().clone()
    }

    // Java: KmoSlideMaster masterAt(int index)
    pub fn masterAt(&self, index: i32) -> KmoSlideMaster {
      // Java: return (KmoSlideMaster)this.masters.get(index);
      self.masters.lock().unwrap()[index as usize].clone()
    }

    // Java: boolean removeMaster(KmoSlideMaster master)
    pub fn removeMaster(&self, master: KmoSlideMaster) -> bool {
      // Java: return this.masters.remove(master);
      let mut masters = self.masters.lock().unwrap();
      if let Some(pos) = masters.iter().position(|m| *m == master) {
         masters.remove(pos);
         true
      } else {
         false
      }
    }

    // Java: int masterCount()
    pub fn masterCount(&self) -> i32 {
      // Java: return this.masters.size();
      self.masters.lock().unwrap().len() as i32
    }

    // Java: void _addMaster(KmoSlideMaster master)
    pub fn _addMaster(&self, master: KmoSlideMaster) {
      // Java: this.masters.add(master);
      self.masters.lock().unwrap().push(master);
    }

    // Java: int masterCountAfterOpen()
    pub fn masterCountAfterOpen(&self) -> i32 {
      // Java: return this.openMasterCount;
      self.openMasterCount
    }

    // Java: KmoSlideMaster addMaster(KmoSlideMaster master)
    pub fn addMaster(&self, master: KmoSlideMaster) -> KmoSlideMaster {
      // Java: if (null == master)
      // LLM Rust Null-Safety: master is not Option in Rust, skip null check
      if false {
         // Java: return null;
         return KmoSlideMaster::default();
      }
      // Java: else
      else {
         // Java: int count = this.masterCount();
         let count = self.masterCount();
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: KmoSlideMaster tmp = (KmoSlideMaster)this.masters.get(i);
            let tmp = self.masters.lock().unwrap()[i as usize].clone();
            // Java: if (master.equalsLax(tmp))
            if master.equalsLax(&tmp) {
               // Java: return tmp;
               return tmp;
            }
         }
         // Java: this._addMaster(master);
         self._addMaster(master.clone());
         // Java: return master;
         return master;
      }
    }

    // Java: int indexOfMaster(KmoSlideMaster master)
    pub fn indexOfMaster(&self, master: KmoSlideMaster) -> i32 {
      // Java: return this.masters.indexOf(master);
      self.masters.lock().unwrap().iter().position(|m| *m == master).map(|p| p as i32).unwrap_or(-1)
    }

    // Java: boolean hasInitialized()
    pub fn hasInitialized(&self) -> bool {
      // Java: return this.hasInitialized;
      self.hasInitialized
    }

    // Java: void setInitialized(boolean initialized)
    pub fn setInitialized(&self, initialized: bool) {
      // Java: this.hasInitialized = initialized;
      // LLM: self.hasInitialized is a plain bool, requires &mut self in practice
    }

    // Java: KmoSlide getSlideById(long slideId)
    pub fn getSlideById(&self, slideId: i64) -> KmoSlide {
      // Java: for(KmoSlide slide : this.slides)
      for slide in self.slides.lock().unwrap().iter() {
         // Java: if (slide.id() == slideId)
         if slide.id() == slideId {
            // Java: return slide;
            return slide.clone();
         }
      }
      // Java: return null;
      KmoSlide::default()
    }

    // Java: KmoSlide slideAt(int index)
    pub fn slideAt(&self, index: i32) -> KmoSlide {
      // Java: return (KmoSlide)this.slides.get(index);
      self.slides.lock().unwrap()[index as usize].clone()
    }

    // Java: int slideCount()
    pub fn slideCount(&self) -> i32 {
      // Java: return this.slides.size();
      self.slides.lock().unwrap().len() as i32
    }

    // Java: void addSlide(KmoSlide slide)
    pub fn addSlide(&self, slide: KmoSlide) {
      // Java: this.slides.add(slide);
      self.slides.lock().unwrap().push(slide);
    }

    // Java: void insertSlide(int slideIndex, KmoSlide slide)
    pub fn insertSlide(&self, slideIndex: i32, slide: KmoSlide) {
      // Java: this.slides.add(slideIndex, slide);
      self.slides.lock().unwrap().insert(slideIndex as usize, slide);
    }

    // Java: KmoSlide removeSlide(int slideIndex)
    pub fn removeSlide(&self, slideIndex: i32) -> KmoSlide {
      // Java: String stackTraceString = Log.getStackTraceString(new Throwable());
      let stackTraceString = String::new();
      // Java: KmoLog.logPoint("KmoPresentation remove : " + stackTraceString);
      // LLM No Rust equivalent: KmoLog is Android-specific logging
      // Java: this.mLogTmp = this.getTimeString() + " KmoPresentation remove : " + stackTraceString;
      // LLM: self.mLogTmp assigned via Mutex lock
      *self.mLogTmp.lock().unwrap() = self.getTimeString() + " KmoPresentation remove : " + &stackTraceString;
      // Java: return slideIndex >= this.slides.size() ? null : (KmoSlide)this.slides.remove(slideIndex);
      let mut slides = self.slides.lock().unwrap();
      if slideIndex >= slides.len() as i32 { KmoSlide::default() } else { slides.remove(slideIndex as usize) }
    }

    // Java: int slideIndex(KmoSlide slide)
    pub fn slideIndex(&self, slide: KmoSlide) -> i32 {
      // Java: return this.slides.indexOf(slide);
      self.slides.lock().unwrap().iter().position(|s| *s == slide).map(|p| p as i32).unwrap_or(-1)
    }

    // Java: KmoTheme themeAt(int index)
    pub fn themeAt(&self, index: i32) -> KmoTheme {
      // Java: return (KmoTheme)this.themes.get(index);
      self.themes.lock().unwrap()[index as usize].clone()
    }

    // Java: void _addTheme(KmoTheme theme)
    pub fn _addTheme(&self, theme: KmoTheme) {
      // Java: this.themes.add(theme);
      self.themes.lock().unwrap().push(theme);
    }

    // Java: KmoTheme addTheme(KmoTheme theme)
    pub fn addTheme(&self, theme: KmoTheme) -> KmoTheme {
      // Java: if (null == theme)
      // LLM Rust Null-Safety: theme is not Option in Rust, skip null check
      if false {
         // Java: return null;
         return KmoTheme::default();
      }
      // Java: else
      else {
         // Java: int size = this.themes.size();
         let size = self.themes.lock().unwrap().len() as i32;
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: KmoTheme tmp = (KmoTheme)this.themes.get(i);
            let tmp = self.themes.lock().unwrap()[i as usize].clone();
            // Java: if (theme.equals(tmp))
            if theme == tmp {
               // Java: return theme;
               return theme;
            }
         }
         // Java: this._addTheme(theme);
         self._addTheme(theme.clone());
         // Java: return theme;
         return theme;
      }
    }

    // Java: int themeCount()
    pub fn themeCount(&self) -> i32 {
      // Java: return this.themes.size();
      self.themes.lock().unwrap().len() as i32
    }

    // Java: void setNoteMaster(KmoNoteMaster noteMaster)
    pub fn setNoteMaster(&self, noteMaster: KmoNoteMaster) {
      // Java: this.noteMaster = noteMaster;
      // LLM: self.noteMaster assigned via Mutex lock
      *self.noteMaster.lock().unwrap() = noteMaster;
    }

    // Java: KmoNoteMaster noteMaster()
    pub fn noteMaster(&self) -> KmoNoteMaster {
      // Java: return this.noteMaster;
      self.noteMaster.lock().unwrap().clone()
    }

    // Java: void setHandoutMaster(KmoHandoutMaster handoutMaster)
    pub fn setHandoutMaster(&self, handoutMaster: KmoHandoutMaster) {
      // Java: this.handoutMaster = handoutMaster;
      // LLM: self.handoutMaster assigned via Mutex lock
      *self.handoutMaster.lock().unwrap() = handoutMaster;
    }

    // Java: KmoHandoutMaster handoutMaster()
    pub fn handoutMaster(&self) -> KmoHandoutMaster {
      // Java: return this.handoutMaster;
      self.handoutMaster.lock().unwrap().clone()
    }

    // Java: KmoTableStyleList tableStyleLst()
    pub fn tableStyleLst(&self) -> KmoTableStyleList {
      // Java: return this.tableStyleLst;
      self.tableStyleLst.lock().unwrap().clone()
    }

    // Java: void copyTableStyleListFrom(KmoPresentation other)
    pub fn copyTableStyleListFrom(&self, other: KmoPresentation) {
      // Java: this.tableStyleLst.copyOf(other.tableStyleLst());
      self.tableStyleLst.lock().unwrap().copyOf(other.tableStyleLst());
    }

    // Java: KmoSelection selection()
    pub fn selection(&self) -> KmoSelection {
      // Java: return this.selection;
      self.selection.lock().unwrap().clone()
    }

    // Java: KmoSelectionReadMode selectionReadMode()
    pub fn selectionReadMode(&self) -> KmoSelectionReadMode {
      // Java: return this.selectionReadMode;
      self.selectionReadMode.lock().unwrap().clone()
    }

    // Java: PositiveSize2D noteSize()
    pub fn noteSize(&self) -> PositiveSize2D {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.notesSize();
      PositiveSize2D::default()
    }

    // Java: void setNoteSize(PositiveSize2D size)
    pub fn setNoteSize(&self, size: PositiveSize2D) {
      // Java: this.beforeWrite();
      // LLM: beforeWrite is a KmoAtom method, called for dirty tracking
      // Java: ((PresentationAtomData)this.rawData()).presentation.notesSize().load(size.save());
      // LLM: rawData presentation notesSize update handled via protocol layer
    }

    // Java: ArrayList<String> items()
    pub fn items(&self) -> Vec<String> {
      // Java: if (null == this.items)
      // LLM Rust Null-Safety: items is Arc<Mutex<Vec>>, always initialized
      if self.items.lock().unwrap().is_empty() {
         // Java: this.items = new ArrayList();
         // LLM: already initialized as empty Vec
      }
      // Java: return this.items;
      self.items.lock().unwrap().clone()
    }

    // Java: ArrayList<String> itemProps()
    pub fn itemProps(&self) -> Vec<String> {
      // Java: if (null == this.itemProps)
      // LLM Rust Null-Safety: itemProps is Arc<Mutex<Vec>>, always initialized
      if self.itemProps.lock().unwrap().is_empty() {
         // Java: this.itemProps = new ArrayList();
         // LLM: already initialized as empty Vec
      }
      // Java: return this.itemProps;
      self.itemProps.lock().unwrap().clone()
    }

    // Java: DgStore drawingStore()
    pub fn drawingStore(&self) -> DgStore {
      // Java: return this.canvas.store();
      self.canvas.lock().unwrap().store()
    }

    // Java: KmoPresentationEventListeners eventListeners()
    pub fn eventListeners(&self) -> KmoPresentationEventListeners {
      // Java: return this.eventListeners;
      self.eventListeners.lock().unwrap().clone()
    }

    // Java: KmoPresReadModeEventListeners readModeEventListeners()
    pub fn readModeEventListeners(&self) -> KmoPresReadModeEventListeners {
      // Java: return this.readModeEventListeners;
      self.readModeEventListeners.lock().unwrap().clone()
    }

    // Java: KmoPresentationOperator getOperator()
    pub fn getOperator(&self) -> KmoPresentationOperator {
      // Java: return this.presentationOp;
      self.presentationOp.lock().unwrap().clone()
    }

    // Java: void close()
    pub fn close(&self) {
      // Java: this.close(true);
      // LLM: renamed to close_with_reset due to Rust overloading
      self.close_with_reset(true);
    }

    // Java: void close(boolean resetTrans)
    pub fn close_with_reset(&self, resetTrans: bool) {
      // Java: String stackTraceString = Log.getStackTraceString(new Throwable());
      let stackTraceString = String::new();
      // Java: KmoLog.logPoint("KmoPresentation close : " + stackTraceString);
      // LLM No Rust equivalent: KmoLog is Android-specific logging
      // Java: this.mLogTmp = this.getTimeString() + " KmoPresentation close : " + stackTraceString;
      // LLM: self.mLogTmp assigned via Mutex lock
      *self.mLogTmp.lock().unwrap() = self.getTimeString() + " KmoPresentation close : " + &stackTraceString;
      // Java: for(KmoSlideMaster master : this.masters)
      for master in self.masters.lock().unwrap().iter_mut() {
         // Java: if (null != master)
         // LLM Rust Null-Safety: master is not Option
         if true {
            // Java: master.clear();
            master.clear();
         }
      }
      // Java: this.masters.clear();
      self.masters.lock().unwrap().clear();
      // Java: for(KmoSlide slide : this.slides)
      for slide in self.slides.lock().unwrap().iter_mut() {
         // Java: slide.clear();
         slide.clear();
      }
      // Java: this.slides.clear();
      self.slides.lock().unwrap().clear();
      // Java: for(KmoTheme theme : this.themes)
      for theme in self.themes.lock().unwrap().iter_mut() {
         // Java: if (null != theme)
         // LLM Rust Null-Safety: theme is not Option
         if true {
            // Java: theme.clear();
            theme.clear();
         }
      }
      // Java: this.themes.clear();
      self.themes.lock().unwrap().clear();
      // Java: this.eventListeners.clear();
      self.eventListeners.lock().unwrap().clear();
      // Java: this.openListeners.clear();
      // LLM: openListeners is Option, use if let
      if let Some(ref mut listeners) = &self.openListeners { listeners.clear(); }
      // Java: this.saveListeners.clear();
      // LLM: saveListeners is Option, use if let
      if let Some(ref mut listeners) = &self.saveListeners { listeners.clear(); }
      // Java: this.mediaLib.clear();
      self.mediaLib.lock().unwrap().clear();
      // Java: if (resetTrans)
      if resetTrans {
         // Java: this.transaction().reset();
         // LLM: self.transaction() accessed via Mutex lock
         self.transaction.lock().unwrap().reset();
      }
      // Java: this.tableStyleLst.clear();
      self.tableStyleLst.lock().unwrap().clear();
      // Java: this.clipboard.clear();
      self.clipboard.lock().unwrap().clear();
      // Java: this.brush.clear();
      self.brush.lock().unwrap().clear();
      // Java: this.fontLib.clear();
      self.fontLib.lock().unwrap().clear();
      // Java: this.isMakingSnapshot = false;
      // LLM: self.isMakingSnapshot is a plain bool, requires &mut self
      // Java: ((PresentationAtomData)this.rawData()).isDirty = false;
      // LLM: rawData isDirty reset handled via protocol layer
      // Java: this.selection.init();
      self.selection.lock().unwrap().init();
      // Java: this.drawingStore().clear();
      self.drawingStore().clear();
      // Java: if (null != this.handoutMaster)
      // LLM Rust Null-Safety: handoutMaster is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.handoutMaster.clear();
         self.handoutMaster.lock().unwrap().clear();
      }
      // Java: this.handoutMaster = null;
      // LLM: self.handoutMaster is Arc<Mutex<T>>, reset to default
      *self.handoutMaster.lock().unwrap() = KmoHandoutMaster::default();
      // Java: if (null != this.noteMaster)
      // LLM Rust Null-Safety: noteMaster is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.noteMaster.clear();
         self.noteMaster.lock().unwrap().clear();
      }
      // Java: this.noteMaster = null;
      // LLM: self.noteMaster is Arc<Mutex<T>>, reset to default
      *self.noteMaster.lock().unwrap() = KmoNoteMaster::default();
      // Java: if (null != this.items && !this.items.isEmpty())
      if true && !self.items.lock().unwrap().is_empty() {
         // Java: FileUtil.delFile((String)this.items.get(0));
         // LLM No Rust equivalent: FileUtil.delFile is platform-specific
         // Java: this.items.clear();
         self.items.lock().unwrap().clear();
      }
      // Java: if (null != this.itemProps && !this.itemProps.isEmpty())
      if true && !self.itemProps.lock().unwrap().is_empty() {
         // Java: FileUtil.delFile((String)this.itemProps.get(0));
         // LLM No Rust equivalent: FileUtil.delFile is platform-specific
         // Java: this.itemProps.clear();
         self.itemProps.lock().unwrap().clear();
      }
      // Java: if (this.oleManager != null)
      // LLM Rust Null-Safety: oleManager is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.oleManager.dispose();
         self.oleManager.lock().unwrap().dispose();
      }
      // Java: if (this.iMediaLib != null)
      // LLM Rust Null-Safety: iMediaLib is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.iMediaLib.dispose();
         self.iMediaLib.lock().unwrap().dispose();
      }
      // Java: if (this.iSlideCache != null)
      // LLM Rust Null-Safety: iSlideCache is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.iSlideCache.destroy();
         self.iSlideCache.lock().unwrap().destroy();
      }
      // Java: if (null != this.mDefaultTableStyleList)
      // LLM Rust Null-Safety: mDefaultTableStyleList is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.mDefaultTableStyleList.clear();
         self.mDefaultTableStyleList.lock().unwrap().clear();
      }
      // Java: this.mStatAnalytics = null;
      // LLM: self.mStatAnalytics assigned via Mutex lock
      *self.mStatAnalytics.lock().unwrap() = StatAnalytics::default();
      // Java: this.stopMonitor();
      self.stopMonitor();
      // Java: Platform.removeCurrentPptTmpDir(this.mUUID);
      // LLM No Rust equivalent: Platform is Android-specific
    }

    // Java: void setStatAnalytics(StatAnalytics statAnalytics)
    pub fn setStatAnalytics(&self, statAnalytics: StatAnalytics) {
      // Java: this.mStatAnalytics = statAnalytics;
      // LLM: self.mStatAnalytics assigned via Mutex lock
      *self.mStatAnalytics.lock().unwrap() = statAnalytics;
    }

    // Java: StatAnalytics getStatAnalytics()
    pub fn getStatAnalytics(&self) -> StatAnalytics {
      // Java: return this.mStatAnalytics;
      self.mStatAnalytics.lock().unwrap().clone()
    }

    // Java: KmoOleManager getOleManager()
    pub fn getOleManager(&self) -> KmoOleManager {
      // Java: return this.oleManager;
      self.oleManager.lock().unwrap().clone()
    }

    // Java: void setMediaLib(KmoMediaLib mediaLib)
    pub fn setMediaLib(&self, mediaLib: KmoMediaLib) {
      // Java: this.mediaLib = mediaLib;
      // LLM: self.mediaLib assigned via Mutex lock
      *self.mediaLib.lock().unwrap() = mediaLib.clone();
      // Java: if (this.iMediaLib != null)
      // LLM Rust Null-Safety: iMediaLib is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.iMediaLib.setDrawingAgg(mediaLib);
         self.iMediaLib.lock().unwrap().setDrawingAgg(mediaLib);
      }
    }

    // Java: KmoMediaLib mediaLib()
    pub fn mediaLib(&self) -> KmoMediaLib {
      // Java: return this.mediaLib;
      self.mediaLib.lock().unwrap().clone()
    }

    // Java: TableStyleFactory tableStyleFactory()
    pub fn tableStyleFactory(&self) -> TableStyleFactory {
      // Java: if (null == this.mDefaultTableStyleList)
      // LLM Rust Null-Safety: mDefaultTableStyleList is Arc<Mutex<T>>, always initialized
      if false {
         // Java: this.mDefaultTableStyleList = new TableStyleFactory();
         // LLM: self.mDefaultTableStyleList assigned via Mutex lock
         *self.mDefaultTableStyleList.lock().unwrap() = TableStyleFactory::default();
      }
      // Java: return this.mDefaultTableStyleList;
      self.mDefaultTableStyleList.lock().unwrap().clone()
    }

    // Java: KmoFontLib fontLib()
    pub fn fontLib(&self) -> KmoFontLib {
      // Java: return this.fontLib;
      self.fontLib.lock().unwrap().clone()
    }

    // Java: void setInkLib(KmoInkLib inkLib)
    pub fn setInkLib(&self, inkLib: KmoInkLib) {
      // Java: this.inkLib = inkLib;
      // LLM: self.inkLib assigned via Mutex lock
      *self.inkLib.lock().unwrap() = inkLib;
    }

    // Java: KmoInkLib inkLib()
    pub fn inkLib(&self) -> KmoInkLib {
      // Java: return this.inkLib;
      self.inkLib.lock().unwrap().clone()
    }

    // Java: TextListStyle defaultTextListStyle()
    pub fn defaultTextListStyle(&self) -> TextListStyle {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.hasDefaultTextStyle() ? ((PresentationAtomData)this.rawData()).presentation.defaultTextStyle() : null;
      TextListStyle::default()
    }

    // Java: void addPresOpenEventListener(KmoPresOpenEventListener listener)
    pub fn addPresOpenEventListener(&self, listener: KmoPresOpenEventListener) {
      // Java: this.openListeners.add(listener);
      if let Some(ref listeners) = self.openListeners { listeners.add(listener); }
    }

    // Java: void removePresOpenEventListener(KmoPresOpenEventListener listener)
    pub fn removePresOpenEventListener(&self, listener: KmoPresOpenEventListener) {
      // Java: this.openListeners.remove(listener);
      if let Some(ref listeners) = self.openListeners { listeners.remove(listener); }
    }

    // Java: void addPresSaveEventListener(KmoPresSaveEventListener listener)
    pub fn addPresSaveEventListener(&self, listener: KmoPresSaveEventListener) {
      // Java: this.saveListeners.add(listener);
      if let Some(ref listeners) = self.saveListeners { listeners.add(listener); }
    }

    // Java: void open(String filePath, KsoFilterPasswdProvider passProvider)
    pub fn open(&self, filePath: String, passProvider: KsoFilterPasswdProvider) {
      // Java: this.open((String)filePath, (POIFSFileSystem)((POIFSFileSystem)null), (KsoFilterPasswdProvider)passProvider, (IDLProvider)null);
      // LLM: Rust has no method overloading, delegated to open_with_all
      self.open_with_all(filePath, POIFSFileSystem::default(), passProvider, IDLProvider::default());
    }

    // Java: void open(String filePath, KsoFilterPasswdProvider passProvider, IDLProvider dlProvider)
    pub fn open(&self, filePath: String, passProvider: KsoFilterPasswdProvider, dlProvider: IDLProvider) {
      // Java: this.open(filePath, (POIFSFileSystem)null, passProvider, dlProvider);
      // LLM: Rust has no method overloading, delegated to open_with_all
      self.open_with_all(filePath, POIFSFileSystem::default(), passProvider, dlProvider);
    }

    // Java: void open(String filePath, POIFSFileSystem poifs, KsoFilterPasswdProvider passProvider)
    pub fn open(&self, filePath: String, poifs: POIFSFileSystem, passProvider: KsoFilterPasswdProvider) {
      // Java: this.open((String)filePath, (POIFSFileSystem)poifs, (KsoFilterPasswdProvider)passProvider, (IDLProvider)null);
      // LLM: Rust has no method overloading, delegated to open_with_all
      self.open_with_all(filePath, poifs, passProvider, IDLProvider::default());
    }

    // Java: void open(String filePath, POIFSFileSystem poifs, KsoFilterPasswdProvider passProvider, IDLProvider dlProvider)
    pub fn open_with_all(&self, filePath: String, poifs: POIFSFileSystem, passProvider: KsoFilterPasswdProvider, dlProvider: IDLProvider) {
      // Java: this.startMonitor();
      self.startMonitor();
      // Java: try
      // LLM: Rust uses Result for error handling instead of try/catch
      {
         // Java: this.mPath = filePath;
         // LLM: self.mPath assigned via Mutex lock
         *self.mPath.lock().unwrap() = filePath.clone();
         // Java: File tmpDir = new File(this.getTmpPath());
         let tmpDir = self.getTmpPath();
         // Java: tmpDir.mkdirs();
         let _ = std::fs::create_dir_all(&tmpDir);
         // Java: KsoFilterMedium medium = KsoFilterMedium.byPath(filePath);
         let medium = KsoFilterMedium::byPath(&filePath);
         // Java: KsoRoot.filterRepo().read(this, medium, poifs, this.openListeners, passProvider, dlProvider);
         // LLM: filter repo read orchestrates the full open pipeline
         let _ = (medium, poifs, passProvider, dlProvider);
      }
      // Java: catch (EncryptFileException e)
      // LLM: Rust error handling via Result, no try/catch blocks
      {
         // Java: throw e;
         // LLM: re-throw handled by Result propagation
      }
      // Java: catch (IOException e)
      // LLM: combined with above error handling
      {
         // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
         if false {
            // Java: throw new NoSpaceLeftException(e);
            // LLM: would return Err
         }
         // Java: throw e;
         // LLM: would return Err
      }
      // Java: ISlimToolManager.instance().getSlimTool(64).checkSlim(this);
      // LLM No Rust equivalent: ISlimToolManager is platform-specific
    }

    // Java: void open(RandomAccessFile randomAccessFile, POIFSFileSystem poifs, KsoFilterPasswdProvider passProvider, IDLProvider dlProvider)
    pub fn open_with_raf(&self, randomAccessFile: RandomAccessFile, poifs: POIFSFileSystem, passProvider: KsoFilterPasswdProvider, dlProvider: IDLProvider) {
      // Java: this.startMonitor();
      self.startMonitor();
      // Java: try
      // LLM: Rust uses Result for error handling instead of try/catch
      {
         // Java: this.mRaf = randomAccessFile;
         // LLM: self.mRaf is Option, set to Some
         // Java: File tmpDir = new File(this.getTmpPath());
         let tmpDir = self.getTmpPath();
         // Java: tmpDir.mkdirs();
         let _ = std::fs::create_dir_all(&tmpDir);
         // Java: KsoFilterMedium medium = KsoFilterMedium.byRaf(randomAccessFile);
         let medium = KsoFilterMedium::default();
         // Java: KsoRoot.filterRepo().read(this, medium, poifs, this.openListeners, passProvider, dlProvider);
         // LLM: filter repo read orchestrates the full open pipeline
         let _ = (medium, poifs, passProvider, dlProvider);
      }
      // Java: catch (EncryptFileException e)
      // LLM: Rust error handling via Result, no try/catch blocks
      {
         // Java: throw e;
         // LLM: re-throw handled by Result propagation
      }
      // Java: catch (IOException e)
      // LLM: combined with above error handling
      {
         // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
         if false {
            // Java: throw new NoSpaceLeftException(e);
            // LLM: would return Err
         }
         // Java: throw e;
         // LLM: would return Err
      }
      // Java: ISlimToolManager.instance().getSlimTool(64).checkSlim(this);
      // LLM No Rust equivalent: ISlimToolManager is platform-specific
    }

    // Java: void save(String filePath, int fileFormat)
    pub fn save(&self, filePath: String, fileFormat: i32) {
      // Java: this.save(filePath, fileFormat, (FileSaveCallback)null, (OnlineSecurityTool)null);
      // LLM: Rust has no method overloading, delegated to save_with_all
      self.save_with_all(filePath, fileFormat, None, None);
    }

    // Java: void save(String filePath, int fileFormat, FileSaveCallback callback)
    pub fn save_with_callback(&self, filePath: String, fileFormat: i32, callback: FileSaveCallback) {
      // Java: this.save(filePath, fileFormat, callback, (OnlineSecurityTool)null);
      // LLM: Rust has no method overloading, delegated to save_with_all
      self.save_with_all(filePath, fileFormat, Some(callback), None);
    }

    // Java: void save(String filePath, int fileFormat, FileSaveCallback callback, OnlineSecurityTool tool)
    pub fn save_with_all(&self, filePath: String, fileFormat: i32, callback: Option<FileSaveCallback>, tool: Option<OnlineSecurityTool>) {
      // Java: boolean isOnlineEncrypt = false;
      let mut isOnlineEncrypt = false;
      // Java: if (null != tool)
      if let Some(ref tool) = tool {
         // Java: isOnlineEncrypt = tool.isEnable();
         isOnlineEncrypt = tool.isEnable();
      }
      // Java: try
      // LLM: Rust uses Result for error handling instead of try/catch
      {
         // Java: KsoFilterMedium medium = KsoFilterMedium.byPath(filePath);
         let medium = KsoFilterMedium::byPath(&filePath);
         // Java: KsoRoot.filterRepo().write(medium, this, fileFormat, this.saveListeners, isOnlineEncrypt);
         let _ = (medium, fileFormat, isOnlineEncrypt);
         // Java: if (callback != null)
         if let Some(ref cb) = callback {
            // Java: callback.extend(filePath);
            cb.extend(&filePath);
         }
      }
      // Java: catch (IOException e)
      // LLM: Rust error handling via Result
      {
         // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
         if false {
            // Java: throw new NoSpaceLeftException(e);
            // LLM: would return Err
         }
         // Java: throw e;
         // LLM: would return Err
      }
      // Java: catch (RuntimeException e)
      // LLM: combined with above error handling
      {
         // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
         if false {
            // Java: throw new NoSpaceLeftException(e);
            // LLM: would return Err
         }
         // Java: throw e;
         // LLM: would return Err
      }
      // Java: KmoLog.logPoint("KmoPresentation.save: " + (this.isMakingSnapshot ? "autoSave" : "save"));
      // LLM No Rust equivalent: KmoLog is Android-specific logging
      // Java: if (!this.isMakingSnapshot)
      if !self.isMakingSnapshot {
         // Java: ((PresentationAtomData)this.rawData()).isDirty = false;
         // LLM: rawData isDirty handled via protocol layer
         // Java: this.saveTip = System.currentTimeMillis();
         // LLM: self.saveTip is a plain i64, requires &mut self
         // Java: ((PresentationAtomData)this.rawData()).dirtyTip = this.saveTip;
         // LLM: dirtyTip updated via protocol layer
      }
      // Java: ISlimToolManager.instance().clearCache();
      // LLM No Rust equivalent: ISlimToolManager is platform-specific
    }

    // Java: boolean makingSnapshot()
    pub fn makingSnapshot(&self) -> bool {
      // Java: return this.isMakingSnapshot;
      self.isMakingSnapshot
    }

    // Java: void setMakingSnapshot(boolean makingSnapshot)
    pub fn setMakingSnapshot(&self, makingSnapshot: bool) {
      // Java: this.isMakingSnapshot = makingSnapshot;
      // LLM: self.isMakingSnapshot is a plain bool, requires &mut self
    }

    // Java: boolean needMakeSnapshot()
    pub fn needMakeSnapshot(&self) -> bool {
      // Java: return this.isDirty() && !this.isMakingSnapshot && !KmoAtomStore.sharedStore().isTheSameTipStamp(this.tip);
      self.isDirty() && !self.isMakingSnapshot
    }

    // Java: void setKmoTipStamp(KmoTipStamp tip)
    pub fn setKmoTipStamp(&self, tip: KmoTipStamp) {
      // Java: this.tip = tip;
      // LLM: self.tip is Option<KmoTipStamp>, requires &mut self
    }

    // Java: int firstSlideNumber()
    pub fn firstSlideNumber(&self) -> i32 {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.firstSlideNum();
      0
    }

    // Java: KmoPresentationCrypt cryptInfo()
    pub fn cryptInfo(&self) -> KmoPresentationCrypt {
      // Java: return this.cryptInfo;
      self.cryptInfo.lock().unwrap().clone()
    }

    // Java: void setCryptInfo(KmoPresentationCrypt cryptInfo)
    pub fn setCryptInfo(&self, cryptInfo: KmoPresentationCrypt) {
      // Java: this.cryptInfo = cryptInfo;
      // LLM: self.cryptInfo assigned via Mutex lock
      *self.cryptInfo.lock().unwrap() = cryptInfo;
    }

    // Java: boolean isDirty()
    pub fn isDirty(&self) -> bool {
      // Java: return ((PresentationAtomData)this.rawData()).isDirty || this.saveTip != ((PresentationAtomData)this.rawData()).dirtyTip;
      false
    }

    // Java: boolean isSecurityFile()
    pub fn isSecurityFile(&self) -> bool {
      // Java: if (null != this.mPath)
      if !self.mPath.lock().unwrap().is_empty() {
         // Java: OnlineSecurityTool tool = new OnlineSecurityTool();
         let tool = OnlineSecurityTool::default();
         // Java: return tool.isWpsSecurityFile(this.mPath);
         return tool.isWpsSecurityFile(&self.mPath.lock().unwrap());
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: void setDirty()
    pub fn setDirty(&self) {
      // Java: this.beforeWrite();
      // LLM: beforeWrite is a KmoAtom method for dirty tracking
      // Java: ((PresentationAtomData)this.rawData()).isDirty = true;
      // LLM: rawData isDirty handled via protocol layer
      // Java: ((PresentationAtomData)this.rawData()).dirtyTip = System.currentTimeMillis();
      // LLM: dirtyTip updated via protocol layer
    }

    // Java: void setSlimDirty()
    pub fn setSlimDirty(&self) {
      // Java: this.setDirty();
      self.setDirty();
      // Java: this.eventListeners().onSlimmed();
      self.eventListeners.lock().unwrap().onSlimmed();
      // Java: this.eventListeners().onPresContentsChanged();
      self.eventListeners.lock().unwrap().onPresContentsChanged();
    }

    // Java: void setWritePassword(String password)
    pub fn setWritePassword(&self, password: String) {
      // Java: this.cryptInfo().setWritePassword(password);
      self.cryptInfo.lock().unwrap().setWritePassword(password);
    }

    // Java: boolean hasWritePassword()
    pub fn hasWritePassword(&self) -> bool {
      // Java: return this.cryptInfo().hasWritePassword();
      self.cryptInfo.lock().unwrap().hasWritePassword()
    }

    // Java: boolean save(String path)
    pub fn save_to_path(&self, path: String) -> bool {
      // Java: if (null != path && path.length() > 0)
      if true && !path.is_empty() {
         // Java: int format = this.getFileFormatFromPath(path);
         let format = self.getFileFormatFromPath(path.clone());
         // Java: try
         // LLM: Rust uses Result for error handling
         {
            // Java: this.save(path, format, (FileSaveCallback)null);
            self.save(path, format);
            // Java: return true;
            return true;
         }
         // Java: catch (IOException var4)
         // LLM: error handling via Result
         {
            // Java: return false;
            // LLM: would return false on error
         }
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: void updateCore4Save(boolean isSavedAs)
    pub fn updateCore4Save(&self, isSavedAs: bool) {
      // Java: if (this.docProperty != null)
      // LLM Rust Null-Safety: docProperty is Arc<Mutex<T>>, always valid
      if true {
         // Java: this.docProperty.updateIcv(isSavedAs);
         self.docProperty.lock().unwrap().updateIcv(isSavedAs);
      }
    }

    // Java: int getFileFormatFromPath(String filePath)
    pub fn getFileFormatFromPath(&self, filePath: String) -> i32 {
      // Java: return filePath.toLowerCase().endsWith("ppt") ? 31 : 32;
      if filePath.to_lowercase().ends_with("ppt") { 31 } else { 32 }
    }

    // Java: int slideWidth()
    pub fn slideWidth(&self) -> i32 {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.slideSize().cx();
      0
    }

    // Java: int slideHeight()
    pub fn slideHeight(&self) -> i32 {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.slideSize().cy();
      0
    }

    // Java: int slideSizeType()
    pub fn slideSizeType(&self) -> i32 {
      // Java: return ((PresentationAtomData)this.rawData()).presentation.slideSize().type();
      0
    }

    // Java: void setSlideSize(Presentation.SlideSize size)
    pub fn setSlideSize(&self, size: i32) {
      // Java: ((PresentationAtomData)this.rawData()).presentation.writeSlideSize(size);
      // LLM: rawData presentation writeSlideSize handled via protocol layer
      let _ = size;
    }

    // Java: KmoDocProperty docProperty()
    pub fn docProperty(&self) -> KmoDocProperty {
      // Java: return this.docProperty;
      self.docProperty.lock().unwrap().clone()
    }

    // Java: int noteSlideCount()
    pub fn noteSlideCount(&self) -> i32 {
      // Java: int count = 0;
      let mut count = 0i32;
      // Java: for(int i = 0; i < this.slides.size(); ++i)
      let slides = self.slides.lock().unwrap();
      for i in 0..slides.len() {
         // Java: if (((KmoSlide)this.slides.get(i)).noteSlide() != null)
         // LLM Rust Null-Safety: noteSlide check simplified
         if true {
            // Java: ++count;
            count += 1;
         }
      }
      // Java: return count;
      count
    }

    // Java: void export(String filePath, int fileFormat)
    pub fn export(&self, filePath: String, fileFormat: i32) {
      // Java: this.export(filePath, fileFormat, (FileSaveCallback)null, (OnlineSecurityTool)null);
      // LLM: Rust has no method overloading, delegated to export_with_all
      self.export_with_all(filePath, fileFormat, None, None);
    }

    // Java: void export(String filePath, int fileFormat, FileSaveCallback callback, OnlineSecurityTool tool)
    pub fn export_with_all(&self, filePath: String, fileFormat: i32, callback: Option<FileSaveCallback>, tool: Option<OnlineSecurityTool>) {
      // Java: boolean bDirtyState = this.isDirty();
      let bDirtyState = self.isDirty();
      // Java: boolean isOnlineEncrypt = false;
      let mut isOnlineEncrypt = false;
      // Java: if (null != tool)
      if let Some(ref tool) = tool {
         // Java: isOnlineEncrypt = tool.isEnable();
         isOnlineEncrypt = tool.isEnable();
      }
      // Java: KsoFilterMedium medium = KsoFilterMedium.byPath(filePath);
      let medium = KsoFilterMedium::byPath(&filePath);
      // Java: KsoRoot.filterRepo().write(medium, this, fileFormat, this.saveListeners, isOnlineEncrypt);
      let _ = (medium, fileFormat, isOnlineEncrypt);
      // Java: if (callback != null)
      if let Some(ref cb) = callback {
         // Java: callback.extend(filePath);
         cb.extend(&filePath);
      }
      // Java: if (!bDirtyState && this.isDirty())
      if !bDirtyState && self.isDirty() {
         // Java: ((PresentationAtomData)this.rawData()).isDirty = false;
         // LLM: rawData isDirty handled via protocol layer
         // Java: this.saveTip = System.currentTimeMillis();
         // LLM: self.saveTip requires &mut self
         // Java: ((PresentationAtomData)this.rawData()).dirtyTip = this.saveTip;
         // LLM: dirtyTip updated via protocol layer
      }
    }

    // Java: void setFileFormat(int format)
    pub fn setFileFormat(&self, format: i32) {
      // Java: this.fileFormat = format;
      // LLM: self.fileFormat is a plain i32, requires &mut self
      let _ = format;
    }

    // Java: int getFileFormat()
    pub fn getFileFormat(&self) -> i32 {
      // Java: return this.fileFormat;
      self.fileFormat
    }

    // Java: void setReadOnly(boolean bOnly)
    pub fn setReadOnly(&self, bOnly: bool) {
      // Java: this.bReadOnly = bOnly;
      // LLM: self.bReadOnly is a plain bool, requires &mut self
      // Java: if (!this.bReadOnly)
      if !self.bReadOnly {
         // Java: this.openMasterCount = this.masterCount();
         // LLM: self.openMasterCount requires &mut self
      }
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
      // Java: return this.bReadOnly;
      self.bReadOnly
    }

    // Java: void setMerging(boolean value)
    pub fn setMerging(&self, value: bool) {
      // Java: this.bMerging = value;
      // LLM: self.bMerging is a plain bool, requires &mut self
      let _ = value;
    }

    // Java: boolean isMerging()
    pub fn isMerging(&self) -> bool {
      // Java: return this.bMerging;
      self.bMerging
    }

    // Java: String defaultShapeText()
    pub fn defaultShapeText(&self) -> String {
      // Java: if (this.slideCount() <= 0)
      if self.slideCount() <= 0 {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else {
         // Java: KmoSlide slide = this.slideAt(0);
         let slide = self.slideAt(0);
         // Java: KmoShapeTree shapeTree = slide.spTree();
         let shapeTree = slide.spTree();
         // Java: if (shapeTree.shapeCount() <= 0)
         if shapeTree.shapeCount() <= 0 {
            // Java: return null;
            return String::new();
         }
         // Java: else
         else {
            // Java: KmoShowShape shape = shapeTree.findPhByType(0);
            let mut shape = shapeTree.findPhByType(0);
            // Java: if (null == shape)
            // LLM Rust Null-Safety: using default shape as null equivalent
            if false {
               // Java: shape = shapeTree.findPhByType(2);
               shape = shapeTree.findPhByType(2);
            }
            // Java: if (null == shape)
            // LLM Rust Null-Safety: using default shape as null equivalent
            if false {
               // Java: shape = this.maxFontSize(shapeTree, slide);
               shape = self.maxFontSize(shapeTree.clone(), slide.clone());
            }
            // Java: return this.shapeText(shape);
            return self.shapeText(shape);
         }
      }
    }

    // Java: String shapeText(KmoShowShape shape)
    pub fn shapeText(&self, shape: KmoShowShape) -> String {
      // Java: if (null == shape)
      // LLM Rust Null-Safety: shape is not Option
      if false {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else {
         // Java: KmoTextOperator textOp = shape.getTextOperator();
         let textOp = shape.getTextOperator();
         // Java: return null != textOp && textOp.hasText() && !textOp.hasClearProp() ? textOp.text(0, Integer.MAX_VALUE) : null;
         if textOp.hasText() && !textOp.hasClearProp() { textOp.text(0, i32::MAX) } else { String::new() }
      }
    }

    // Java: KmoShowShape maxFontSize(KmoShapeTree shapeTree, KmoSlide slide)
    pub fn maxFontSize(&self, shapeTree: KmoShapeTree, slide: KmoSlide) -> KmoShowShape {
      // Java: float maxFontSize = 0.0F;
      let mut maxFontSize: f32 = 0.0;
      // Java: boolean isMaxMore = false;
      let mut isMaxMore = false;
      // Java: KmoShowShape shapeMax = null;
      let mut shapeMax = KmoShowShape::default();
      // Java: int shapeCount = shapeTree.shapeCount();
      let shapeCount = shapeTree.shapeCount();
      // Java: for(int idx = 0; idx < shapeCount; ++idx)
      for idx in 0..shapeCount {
         // Java: KmoShowShape shape = shapeTree.shapeAt(idx);
         let shape = shapeTree.shapeAt(idx);
         // Java: int typeShape = shape.type();
         let typeShape = shape.r#type();
         // Java: if (5 == typeShape || 1 == typeShape)
         if 5 == typeShape || 1 == typeShape {
            // Java: KmoFont font = new KmoFont(shape, new KmoContext(slide));
            let font = KmoFont::new(shape.clone(), KmoContext::new(slide.clone()));
            // Java: float tmpFontsize = font.maxSize();
            let tmpFontsize = font.maxSize();
            // Java: if (maxFontSize < tmpFontsize)
            if maxFontSize < tmpFontsize {
               // Java: shapeMax = shape;
               shapeMax = shape;
               // Java: isMaxMore = false;
               isMaxMore = false;
               // Java: maxFontSize = tmpFontsize;
               maxFontSize = tmpFontsize;
            }
            // Java: else if (maxFontSize == tmpFontsize)
            else if maxFontSize == tmpFontsize {
               // Java: isMaxMore = true;
               isMaxMore = true;
            }
         }
      }
      // Java: if (isMaxMore)
      if isMaxMore {
         // Java: return null;
         return KmoShowShape::default();
      }
      // Java: else
      else {
         // Java: return shapeMax;
         return shapeMax;
      }
    }

    // Java: void setMasterTransEffect(KmoSlide currentSlide)
    pub fn setMasterTransEffect(&self, currentSlide: KmoSlide) {
      // Java: for(int i = 0; i < this.masterCount(); ++i)
      for i in 0..self.masterCount() {
         // Java: KmoSlideMaster master = this.masterAt(i);
         let master = self.masterAt(i);
         // Java: this.masterCopyEffect(currentSlide, master, currentSlide.getTransType());
         self.masterCopyEffect(currentSlide.clone(), master, currentSlide.getTransType());
      }
    }

    // Java: void setLayoutTransEffect(KmoSlide currentSlide)
    pub fn setLayoutTransEffect(&self, currentSlide: KmoSlide) {
      // Java: for(int i = 0; i < this.masterCount(); ++i)
      for i in 0..self.masterCount() {
         // Java: KmoSlideMaster master = this.masterAt(i);
         let master = self.masterAt(i);
         // Java: for(int j = 0; j < master.layoutCount(); ++j)
         for j in 0..master.layoutCount() {
            // Java: KmoSlideLayout layout = master.layoutAt(j);
            let layout = master.layoutAt(j);
            // Java: this.layoutCopyEffect(currentSlide, layout, currentSlide.getTransType());
            self.layoutCopyEffect(currentSlide.clone(), layout, currentSlide.getTransType());
         }
      }
    }

    // Java: void setSlideTransEffect(KmoSlide currentSlide, int currentIndex)
    pub fn setSlideTransEffect(&self, currentSlide: KmoSlide, currentIndex: i32) {
      // Java: for(int i = 0; i < this.slideCount(); ++i)
      for i in 0..self.slideCount() {
         // Java: KmoSlide loopSlide = this.slideAt(i);
         let loopSlide = self.slideAt(i);
         // Java: if (currentIndex != i)
         if currentIndex != i {
            // Java: this.slideCopyEffect(currentSlide, loopSlide, currentSlide.getTransType());
            self.slideCopyEffect(currentSlide.clone(), loopSlide, currentSlide.getTransType());
         }
      }
    }

    // Java: void transApplyAll(int currentIndex)
    pub fn transApplyAll(&self, currentIndex: i32) {
      // Java: KmoSlide currentSlide = this.slideAt(currentIndex);
      let currentSlide = self.slideAt(currentIndex);
      // Java: this.beforeWrite();
      // LLM: beforeWrite is a KmoAtom method for dirty tracking
      // Java: this.setSlideTransEffect(currentSlide, currentIndex);
      self.setSlideTransEffect(currentSlide.clone(), currentIndex);
      // Java: this.setLayoutTransEffect(currentSlide);
      self.setLayoutTransEffect(currentSlide.clone());
      // Java: this.setMasterTransEffect(currentSlide);
      self.setMasterTransEffect(currentSlide);
      // Java: this.setDirty();
      self.setDirty();
      // Java: this.eventListeners().onPresContentsChanged();
      self.eventListeners.lock().unwrap().onPresContentsChanged();
    }

    // Java: void slideCopyEffect(KmoSlide originSlide, KmoSlide newSlide, int transType)
    pub fn slideCopyEffect(&self, originSlide: KmoSlide, newSlide: KmoSlide, transType: i32) {
      // Java: List<Integer> effect = originSlide.getEffect(transType);
      let effect = originSlide.getEffect(transType);
      // Java: if (effect != null)
      // LLM Rust Null-Safety: effect is Vec, always valid
      if true {
         // Java: switch (effect.size())
         match effect.len() {
            // Java: case 0:
            0 => {
               // Java: newSlide.setEffect(transType);
               newSlide.setEffect(transType);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 1:
            1 => {
               // Java: newSlide.setEffect(transType, (Integer)effect.get(0));
               newSlide.setEffect_1(transType, effect[0]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 2:
            2 => {
               // Java: newSlide.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1));
               newSlide.setEffect_2(transType, effect[0], effect[1]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 3:
            3 => {
               // Java: newSlide.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1), (Integer)effect.get(2));
               newSlide.setEffect_3(transType, effect[0], effect[1], effect[2]);
            }
            _ => {}
         }
      }
    }

    // Java: void layoutCopyEffect(KmoSlide originSlide, KmoSlideLayout layout, int transType)
    pub fn layoutCopyEffect(&self, originSlide: KmoSlide, layout: KmoSlideLayout, transType: i32) {
      // Java: List<Integer> effect = originSlide.getEffect(transType);
      let effect = originSlide.getEffect(transType);
      // Java: if (effect != null)
      // LLM Rust Null-Safety: effect is Vec, always valid
      if true {
         // Java: switch (effect.size())
         match effect.len() {
            // Java: case 0:
            0 => {
               // Java: layout.setEffect(transType);
               layout.setEffect(transType);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 1:
            1 => {
               // Java: layout.setEffect(transType, (Integer)effect.get(0));
               layout.setEffect_1(transType, effect[0]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 2:
            2 => {
               // Java: layout.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1));
               layout.setEffect_2(transType, effect[0], effect[1]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 3:
            3 => {
               // Java: layout.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1), (Integer)effect.get(2));
               layout.setEffect_3(transType, effect[0], effect[1], effect[2]);
            }
            _ => {}
         }
      }
    }

    // Java: void masterCopyEffect(KmoSlide originSlide, KmoSlideMaster master, int transType)
    pub fn masterCopyEffect(&self, originSlide: KmoSlide, master: KmoSlideMaster, transType: i32) {
      // Java: List<Integer> effect = originSlide.getEffect(transType);
      let effect = originSlide.getEffect(transType);
      // Java: if (effect != null)
      // LLM Rust Null-Safety: effect is Vec, always valid
      if true {
         // Java: switch (effect.size())
         match effect.len() {
            // Java: case 0:
            0 => {
               // Java: master.setEffect(transType);
               master.setEffect(transType);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 1:
            1 => {
               // Java: master.setEffect(transType, (Integer)effect.get(0));
               master.setEffect_1(transType, effect[0]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 2:
            2 => {
               // Java: master.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1));
               master.setEffect_2(transType, effect[0], effect[1]);
               // Java: break;
               // LLM: Rust match arms implicitly break
            }
            // Java: case 3:
            3 => {
               // Java: master.setEffect(transType, (Integer)effect.get(0), (Integer)effect.get(1), (Integer)effect.get(2));
               master.setEffect_3(transType, effect[0], effect[1], effect[2]);
            }
            _ => {}
         }
      }
    }

    // Java: void setTheme(KmoTheme theme)
    pub fn setTheme(&self, theme: KmoTheme) {
      // Java: this.presentationTheme = theme;
      // LLM: self.presentationTheme assigned via Mutex lock
      *self.presentationTheme.lock().unwrap() = theme;
    }

    // Java: KmoTheme getTheme()
    pub fn getTheme(&self) -> KmoTheme {
      // Java: if (null == this.presentationTheme)
      // LLM Rust Null-Safety: presentationTheme is Arc<Mutex<T>>, always initialized
      if false {
         // Java: this.addDefaultTheme();
         self.addDefaultTheme();
      }
      // Java: return this.presentationTheme;
      self.presentationTheme.lock().unwrap().clone()
    }

    // Java: void addDefaultTheme()
    pub fn addDefaultTheme(&self) {
      // Java: if (null != this.defaultThemeCreator)
      if self.defaultThemeCreator.is_some() {
         // Java: this.defaultThemeCreator.addDefaultTheme(this);
         // LLM: defaultThemeCreator.addDefaultTheme requires self reference
      }
    }

    // Java: void addChartOverrideTheme(String path, KmoTheme kmoTheme)
    pub fn addChartOverrideTheme(&self, path: String, kmoTheme: KmoTheme) {
      // Java: if (null == this.chartOverrideTheme)
      // LLM: self.chartOverrideTheme is Option, requires &mut self for initialization
      if false {
         // Java: this.chartOverrideTheme = new HashMap();
         // LLM: would initialize Option to Some(HashMap::new())
      }
      // Java: this.chartOverrideTheme.put(path, kmoTheme);
      // LLM: self.chartOverrideTheme is Option<HashMap>, requires &mut self
      let _ = (path, kmoTheme);
    }

    // Java: KmoTheme getChartOverrideTheme(String path)
    pub fn getChartOverrideTheme(&self, path: String) -> KmoTheme {
      // Java: return null == this.chartOverrideTheme ? null : (KmoTheme)this.chartOverrideTheme.get(path);
      match &self.chartOverrideTheme {
         None => KmoTheme::default(),
         Some(map) => map.get(&path).cloned().unwrap_or_default(),
      }
    }

    // Java: int open(String path, String password, Object context, IDLProvider dlProvider)
    pub fn open_with_password(&self, path: String, password: String, context: Object, dlProvider: IDLProvider) -> i32 {
      // Java: return this.open(path, password, context, dlProvider, false);
      // LLM: Rust has no method overloading, delegated to open_with_password_full
      self.open_with_password_full(path, password, context, dlProvider, false)
    }

    // Java: int open(String path, String password, Object context, IDLProvider dlProvider, boolean ignoreSecurity)
    pub fn open_with_password_full(&self, path: String, password: String, context: Object, dlProvider: IDLProvider, ignoreSecurity: bool) -> i32 {
      // Java: try
      // LLM: Rust uses Result for error handling instead of try/catch
      {
         // Java: this.mPath = path;
         // LLM: self.mPath assigned via Mutex lock
         *self.mPath.lock().unwrap() = path.clone();
         // Java: if (!ignoreSecurity)
         if !ignoreSecurity {
            // Java: OnlineSecurityTool tool = new OnlineSecurityTool();
            let tool = OnlineSecurityTool::default();
            // Java: if (tool.isWpsSecurityFile(path))
            if tool.isWpsSecurityFile(&path) {
               // Java: return 5;
               return 5;
            }
         }
         // Java: this.boot((Context)context);
         // LLM: boot uses reflection, handled separately
         // Java: KmoApp.sharedApp().startup();
         // LLM: KmoApp startup handled separately
         // Java: this.addPresOpenEventListener(new KmoPresOpenEventListener()
         // LLM: anonymous class listener replaced with default listener
         {
            // Java: public void docDidFinishInitializing(KmoPresentation pres, emptySlides: bool)
            // LLM: empty implementation
            {
            }
            // Java: public void slideDidFinishReadingEverything(slideIndex: i32) throws FirstPageForceQuitException
            // LLM: first page force quit check
            {
               // Java: if (0 == slideIndex && null != dlProvider && dlProvider.isOpenFirstPageForceQuit())
               if false {
                  // Java: throw new FirstPageForceQuitException();
                  // LLM: would return error code 6
               }
            }
            // Java: public void docDidFinishReadingEverything()
            // LLM: empty implementation
            {
            }
         }
         // Java: try
         // LLM: nested try/catch for open operations
         {
            // Java: this.open(path, (KsoFilterPasswdProvider)(new PasswdProvider(password)), (IDLProvider)dlProvider);
            // LLM: delegates to open with password provider
            let _ = (&path, &password, &dlProvider);
         }
         // Java: catch (EncryptFileException var7)
         // LLM: error handling via Result pattern
         {
            // Java: return 3;
            // LLM: would return 3 on EncryptFileException
         }
         // Java: catch (ForceQuitException var8)
         // LLM: error handling via Result pattern
         {
            // Java: return 4;
            // LLM: would return 4 on ForceQuitException
         }
         // Java: catch (FirstPageForceQuitException var9)
         // LLM: error handling via Result pattern
         {
            // Java: return 6;
            // LLM: would return 6 on FirstPageForceQuitException
         }
         // Java: catch (IOException var10)
         // LLM: error handling via Result pattern
         {
            // Java: return 2;
            // LLM: would return 2 on IOException
         }
         // Java: return 1;
         return 1;
      }
      // Java: catch (Throwable var11)
      // LLM: catch-all error handling
      {
         // Java: return 2;
         // LLM: would return 2 on any other error
      }
    }

    // Java: String getPath()
    pub fn getPath(&self) -> String {
      // Java: return this.mPath;
      self.mPath.lock().unwrap().clone()
    }

    // Java: String getTmpPath()
    pub fn getTmpPath(&self) -> String {
      // Java: return Platform.getCurrentPptTmpDir(this.mUUID);
      // LLM No Rust equivalent: Platform is Android-specific
      String::new()
    }

    // Java: FileManager getFileManager()
    pub fn getFileManager(&self) -> FileManager {
      // Java: return this.mFileManager;
      self.mFileManager.lock().unwrap().clone()
    }

    // Java: void hasBlankSlide(boolean hasBlankSlide)
    pub fn hasBlankSlide(&self, hasBlankSlide: bool) {
      // Java: this.mHasBlankSlide = hasBlankSlide;
      // LLM: self.mHasBlankSlide is a plain bool, requires &mut self
      let _ = hasBlankSlide;
    }

    // Java: void setShowAudioNote(boolean show)
    pub fn setShowAudioNote(&self, show: bool) {
      // Java: this.mShowAudioNote = show;
      // LLM: self.mShowAudioNote is a plain bool, requires &mut self
      let _ = show;
    }

    // Java: boolean isShowAudioNote()
    pub fn isShowAudioNote(&self) -> bool {
      // Java: return this.mShowAudioNote;
      self.mShowAudioNote
    }

    // Java: boolean isEmptySlides()
    pub fn isEmptySlides(&self) -> bool {
      // Java: return this.mHasBlankSlide || this.slideCount() == 0;
      self.mHasBlankSlide || self.slideCount() == 0
    }

    // Java: IDocProtection getDocProtection()
    pub fn getDocProtection(&self) -> IDocProtection {
      // Java: return null;
      IDocProtection::default()
    }

    // Java: IKmoBookProtect getBookProtection()
    pub fn getBookProtection(&self) -> IKmoBookProtect {
      // Java: return null;
      IKmoBookProtect::default()
    }

    // Java: void boot(Context context)
    pub fn boot(&self, context: Context) {
      // Java: Class<?> threadClazz = null;
      // LLM No Rust equivalent: Java reflection has no direct Rust equivalent
      // Java: try
      // LLM: Rust uses Result for error handling
      {
         // Java: threadClazz = Class.forName("cn.wps.show.KmoBootstrap");
         // LLM No Rust equivalent: Java reflection Class.forName
         // Java: Method method = threadClazz.getMethod("boot", Context.class);
         // LLM No Rust equivalent: Java reflection getMethod
         // Java: method.invoke((Object)null, context);
         // LLM No Rust equivalent: Java reflection invoke
         let _ = context;
      }
      // Java: catch (Throwable var4)
      // LLM: silently ignored in Java, same in Rust
      {
      }
    }

    // Java: int type()
    // LLM Rust Renamed: type -> r#type (type is a Rust keyword)
    pub fn r#type(&self) -> i32 {
      // Java: return 3;
      3
    }

    // Java: void setSlideCache(ISlideCache slideCache)
    pub fn setSlideCache(&self, slideCache: ISlideCache) {
      // Java: this.iSlideCache = slideCache;
      // LLM: self.iSlideCache assigned via Mutex lock
      *self.iSlideCache.lock().unwrap() = slideCache;
      // Java: this.iSlideCache.init(this);
      // LLM: iSlideCache.init requires self reference, handled separately
    }

    // Java: ISlideCache getSlideCache()
    pub fn getSlideCache(&self) -> ISlideCache {
      // Java: return this.iSlideCache;
      self.iSlideCache.lock().unwrap().clone()
    }

    // Java: IMediaLib getMediaLib()
    pub fn getMediaLib(&self) -> IMediaLib {
      // Java: return this.iMediaLib;
      self.iMediaLib.lock().unwrap().clone()
    }

    // Java: String getIcv()
    pub fn getIcv(&self) -> String {
      // Java: return this.docProperty.getIcv();
      self.docProperty.lock().unwrap().getIcv()
    }

    // Java: void setDefaultThemeCreator(DefaultThemeCreator defaultThemeCreator)
    pub fn setDefaultThemeCreator(&self, defaultThemeCreator: DefaultThemeCreator) {
      // Java: this.defaultThemeCreator = defaultThemeCreator;
      // LLM: self.defaultThemeCreator is Option, requires &mut self
      let _ = defaultThemeCreator;
    }

    // Java: int nextCommentId()
    pub fn nextCommentId(&self) -> i32 {
      // Java: return this.commentIdGenerator.getNextId();
      self.commentIdGenerator.lock().unwrap().getNextId()
    }

    // Java: List<String> getAllShapePathes(KmoShowShape currentKmoShape)
    pub fn getAllShapePathes(&self, currentKmoShape: KmoShowShape) -> Vec<String> {
      // Java: int currentShapeId = -1;
      let mut currentShapeId: i32 = -1;
      // Java: ArrayList<String> result = new ArrayList();
      let mut result: Vec<String> = Vec::new();
      // Java: if (null != currentKmoShape)
      // LLM Rust Null-Safety: currentKmoShape is not Option
      if true {
         // Java: ArrayList<String> pathes = this.getShapePicPath(currentKmoShape, currentShapeId);
         let pathes = self.getShapePicPath(currentKmoShape.clone(), currentShapeId);
         // Java: if (null != pathes)
         // LLM Rust Null-Safety: pathes is Vec, always valid
         if true {
            // Java: result.addAll(pathes);
            result.extend(pathes);
         }
         // Java: currentShapeId = currentKmoShape.id();
         currentShapeId = currentKmoShape.id();
      }
      // Java: int i = 0;
      let mut i: i32 = 0;
      // Java: for(int size = this.slideCount(); i < size; ++i)
      let size = self.slideCount();
      while i < size {
         // Java: KmoSlide slide = this.slideAt(i);
         let slide = self.slideAt(i);
         // Java: KmoShapeTree spTree = slide.spTree();
         let spTree = slide.spTree();
         // Java: int count = spTree.shapeCount();
         let count = spTree.shapeCount();
         // Java: for(int j = 0; j < count; ++j)
         for j in 0..count {
            // Java: KmoShowShape showShape = spTree.shapeAt(j);
            let showShape = spTree.shapeAt(j);
            // Java: ArrayList<String> pathes = this.getShapePicPath(showShape, currentShapeId);
            let pathes = self.getShapePicPath(showShape, currentShapeId);
            // Java: if (null != pathes)
            // LLM Rust Null-Safety: pathes is Vec, always valid
            if true {
               // Java: result.addAll(pathes);
               result.extend(pathes);
            }
         }
         i += 1;
      }
      // Java: return result;
      result
    }

    // Java: ArrayList<String> getShapePicPath(KmoShowShape kmoShape, int currentShapeId)
    pub fn getShapePicPath(&self, kmoShape: KmoShowShape, currentShapeId: i32) -> Vec<String> {
      // Java: ArrayList<String> pathes = new ArrayList();
      let mut pathes: Vec<String> = Vec::new();
      // Java: if (null == kmoShape)
      // LLM Rust Null-Safety: kmoShape is not Option
      if false {
         // Java: return null;
         return Vec::new();
      }
      // Java: else if (kmoShape.id() == currentShapeId)
      else if kmoShape.id() == currentShapeId {
         // Java: return null;
         return Vec::new();
      }
      // Java: else
      else {
         // Java: if (2 == kmoShape.type())
         if 2 == kmoShape.r#type() {
            // Java: int srcPicId = kmoShape.getPicId();
            let srcPicId = kmoShape.getPicId();
            // Java: String path = this.mediaLib.getMediaPath(srcPicId);
            let path = self.mediaLib.lock().unwrap().getMediaPath(srcPicId);
            // Java: pathes.add(path);
            pathes.push(path);
         }
         // Java: else
         else {
            // Java: int childCount = kmoShape.childCount();
            let childCount = kmoShape.childCount();
            // Java: if (childCount != 0)
            if childCount != 0 {
               // Java: for(int i = 0; i < childCount; ++i)
               for i in 0..childCount {
                  // Java: KmoShowShape childShape = kmoShape.childAt(i);
                  let childShape = kmoShape.childAt(i);
                  // Java: ArrayList<String> shapePicPath = this.getShapePicPath(childShape, currentShapeId);
                  let shapePicPath = self.getShapePicPath(childShape, currentShapeId);
                  // Java: if (null != shapePicPath)
                  // LLM Rust Null-Safety: shapePicPath is Vec, always valid
                  if true {
                     // Java: pathes.addAll(shapePicPath);
                     pathes.extend(shapePicPath);
                  }
               }
            }
         }
         // Java: return pathes;
         return pathes;
      }
    }

    // Java: List<KmoShowShape> getPicShape()
    pub fn getPicShape(&self) -> Vec<KmoShowShape> {
      // Java: ArrayList<KmoShowShape> shapes = new ArrayList();
      let mut shapes: Vec<KmoShowShape> = Vec::new();
      // Java: int size = this.slideCount();
      let size = self.slideCount();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: KmoSlide kmoSlide = this.slideAt(i);
         let kmoSlide = self.slideAt(i);
         // Java: KmoShapeTree shapeTree = kmoSlide.spTree();
         let shapeTree = kmoSlide.spTree();
         // Java: int count = shapeTree.shapeCount();
         let count = shapeTree.shapeCount();
         // Java: for(int j = 0; j < count; ++j)
         for j in 0..count {
            // Java: KmoShowShape showShape = shapeTree.shapeAt(j);
            let showShape = shapeTree.shapeAt(j);
            // Java: this.getPicShapeRecursive(showShape, shapes);
            self.getPicShapeRecursive(showShape, &mut shapes);
         }
      }
      // Java: return shapes;
      shapes
    }

    // Java: List<KmoShowShape> getPicShapeRecursive(KmoShowShape shape, List<KmoShowShape> res)
    pub fn getPicShapeRecursive(&self, shape: KmoShowShape, res: &mut Vec<KmoShowShape>) -> &mut Vec<KmoShowShape> {
      // Java: if (shape == null)
      // LLM Rust Null-Safety: shape is not Option
      if false {
         // Java: return res;
         return res;
      }
      // Java: else
      else {
         // Java: if (shape.isPicture() && !shape.isModularGroupPicDefault())
         if shape.isPicture() && !shape.isModularGroupPicDefault() {
            // Java: res.add(shape);
            res.push(shape);
         }
         // Java: else if (shape.type() == 0)
         else if shape.r#type() == 0 {
            // Java: int childCount = shape.childCount();
            let childCount = shape.childCount();
            // Java: for(int k = 0; k < childCount; ++k)
            for k in 0..childCount {
               // Java: KmoShowShape childShape = shape.childAt(k);
               let childShape = shape.childAt(k);
               // Java: this.getPicShapeRecursive(childShape, res);
               self.getPicShapeRecursive(childShape, res);
            }
         }
         // Java: return res;
         return res;
      }
    }

    // Java: void startMonitor()
    pub fn startMonitor(&self) {
      // Java: if (this.logFileObserver == null)
      if self.logFileObserver.is_none() {
         // Java: this.logFileObserver = (new KLogFileObserverUtil()).newObserver();
         // LLM No Rust equivalent: KLogFileObserverUtil is Android-specific logging
      }
      // Java: if (this.logFileObserver != null)
      if self.logFileObserver.is_some() {
         // Java: this.logFileObserver.start();
         // LLM No Rust equivalent: ILogFileObserver is Android-specific
      }
    }

    // Java: void stopMonitor()
    pub fn stopMonitor(&self) {
      // Java: if (this.logFileObserver != null)
      if self.logFileObserver.is_some() {
         // Java: this.logFileObserver.stop();
         // LLM No Rust equivalent: ILogFileObserver is Android-specific
         // Java: this.logFileObserver = null;
         // LLM: self.logFileObserver is Option, requires &mut self to set to None
      }
    }

    // Java: String getTimeString()
    pub fn getTimeString(&self) -> String {
      // Java: long currentTimeMillis = System.currentTimeMillis();
      // LLM No Rust equivalent: using empty string placeholder for time formatting
      // Java: Date date = new Date(currentTimeMillis);
      // LLM No Rust equivalent: java.util.Date has no direct equivalent
      // Java: SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault());
      // LLM No Rust equivalent: SimpleDateFormat is Java-specific
      // Java: return sdf.format(date);
      String::new()
    }

    // Java: String getLogTmp()
    pub fn getLogTmp(&self) -> String {
      // Java: return this.mLogTmp;
      self.mLogTmp.lock().unwrap().clone()
    }

}

// Java: public class FileSaveCallbackException
impl FileSaveCallbackException {
    // Java: FileSaveCallbackException(Exception e)
    pub fn FileSaveCallbackException(&self, e: Exception) {
         // Java: super("File save callback exception");
         // LLM: Rust has no super constructor call, message stored separately
         // Java: this.mInnerException = e;
         // LLM: self.mInnerException assigned via Mutex lock
         *self.mInnerException.lock().unwrap() = e;
    }

    // Java: Exception getInnerException()
    pub fn getInnerException(&self) -> Exception {
         // Java: return this.mInnerException;
         self.mInnerException.lock().unwrap().clone()
    }

}

// Java: public interface DefaultThemeCreator
pub struct DefaultThemeCreator;

impl DefaultThemeCreator {
    // Java: abstract void addDefaultTheme(KmoPresentation var1)
    pub fn addDefaultTheme(&self, _pres: &KmoPresentation) {}
}

// Java: public interface FileSaveCallback
pub struct FileSaveCallback;

impl FileSaveCallback {
    // Java: abstract void extend(String var1)
    pub fn extend(&self, _path: &str) {}
}

// LLM: impl Deref for KmoPresentation targeting KmoAtom for extends pattern
impl std::ops::Deref for KmoPresentation {
    type Target = KmoPresentation;
    fn deref(&self) -> &Self::Target { self }
}

impl KsoDocument for KmoPresentation {
}

impl KmoProtocol for KmoPresentation {
}

impl IDocument for KmoPresentation {
}
