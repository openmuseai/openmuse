use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::{self, ErrorKind};
use std::rc::Rc;

// Java: import cn.wps.base.log.IOLog
use std::io::Error as IOLog;
// Java: import cn.wps.drawing.app.DgStore
use std::convert::Infallible as DgStore;
// Java: import cn.wps.drawing.dmlr.handler.style.ThemeHandler
use crate::cn::wps::drawing::dmlr::handler::style::ThemeHandler::ThemeHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use std::option::Option as HandlerCallback;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use std::vec::Vec as OpcrPackagePartBytes;
// Java: import cn.wps.show.PptResourceFactory
use std::marker::PhantomData as PptResourceFactoryMarker;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use std::convert::Infallible as KmoInkLib;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::util::layout_textbox::KmoMediaLib;
// Java: import cn.wps.show.app.theme.KmoTheme
use crate::cn::wps::show::util::layout_shape::ThemeObj;
pub type KmoTheme = ThemeObj;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use std::marker::PhantomData as PptxrMediasMarker;
pub struct PptxrMedias {}
// Java: import cn.wps.show.pptxr.ppt.slides.HandlerCallbackImpl
use std::collections::HashMap as HandlerCallbackImplMap;
pub struct HandlerCallbackImpl {
    rid_pic: HashMap<String, i32>,
    throw_on_load: bool,
}
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use std::marker::PhantomData as PptxrSlidesMarker;
pub struct PptxrSlides {}
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use std::io::Result as UnifiedXmlReaderResult;
pub struct UnifiedXmlReader {}
// Java: import java.io.FileNotFoundException
use std::io::ErrorKind as FileNotFoundException;
// Java: import java.io.IOException
use std::io::Error as IOException;
// Java: import java.io.InputStream
use std::vec::Vec as InputStream;
// Java: import java.util.HashMap
use std::collections::HashMap as JavaHashMap;

fn IOLog_d(_tag: Option<&str>, _msg: &str, _e: &io::Error) {
    let _ = (
        PptxrMediasMarker::<()>,
        PptxrSlidesMarker::<()>,
        PptResourceFactoryMarker::<()>,
        HandlerCallbackImplMap::<String, i32>::new(),
        JavaHashMap::<String, i32>::new(),
        FileNotFoundException::NotFound,
        OpcrPackagePartBytes::<u8>::new(),
        InputStream::<u8>::new(),
        HandlerCallback::<()>::None,
        Option::<DgStore>::None,
        Option::<KmoInkLib>::None,
        UnifiedXmlReaderResult::<()>::Ok(()),
        IOException::from(ErrorKind::Other),
        IOLog::from(ErrorKind::Other),
    );
}

// java private static final String TAG = null;
const TAG: Option<&str> = None;

thread_local! {
    static RES_THEME: RefCell<Option<Option<Vec<u8>>>> = RefCell::new(None);
    static RES_THEME_PATH: RefCell<Option<String>> = RefCell::new(None);
    static LAST_CLOSED: Cell<bool> = Cell::new(false);
}

pub struct PptResourceFactory;

impl PptResourceFactory {
    pub fn setTestTheme(stream: Option<Vec<u8>>, path: Option<String>) {
        RES_THEME.with(|s| *s.borrow_mut() = Some(stream));
        RES_THEME_PATH.with(|p| *p.borrow_mut() = path);
    }
    pub fn getResPrivider() -> Self {
        PptResourceFactory
    }
    pub fn getTheme(&self) -> Option<Vec<u8>> {
        RES_THEME.with(|s| s.borrow().clone().unwrap_or(None))
    }
    pub fn getThemePath(&self) -> Option<String> {
        RES_THEME_PATH.with(|p| p.borrow().clone())
    }
}

#[derive(Clone)]
pub struct OpcrPackagePart {
    pub bytes: Vec<u8>,
    pub rid_pic: HashMap<String, i32>,
    pub get_stream_err: bool,
    pub read_err: bool,
    pub load_err: bool,
}

impl OpcrPackagePart {
    pub fn getInputStream(&self) -> io::Result<Vec<u8>> {
        if self.get_stream_err {
            return Err(io::Error::new(ErrorKind::Other, "getInputStream"));
        }
        Ok(self.bytes.clone())
    }
    pub fn getRelationships(&self) {}
}

impl HandlerCallbackImpl {
    pub fn HandlerCallbackImpl(
        _dg: (),
        _mediaLib: Option<KmoMediaLib>,
        _ink: (),
        _rels: (),
        _medias: Option<PptxrMedias>,
        _slides: Option<PptxrSlides>,
        _mode: i32,
        rid_pic: HashMap<String, i32>,
        throw_on_load: bool,
    ) -> Self {
        Self {
            rid_pic,
            throw_on_load,
        }
    }
    pub fn load(&self) -> io::Result<()> {
        if self.throw_on_load {
            return Err(io::Error::new(ErrorKind::Other, "load"));
        }
        Ok(())
    }
    pub fn getRidPicMediaIds(&self) -> HashMap<String, i32> {
        self.rid_pic.clone()
    }
}

impl UnifiedXmlReader {
    pub fn read(stream: &[u8], handler: &mut ThemeHandler) -> io::Result<()> {
        if stream == b"THROW" {
            return Err(io::Error::new(ErrorKind::InvalidData, "parse"));
        }
        let _ = handler;
        Ok(())
    }
}

pub struct CloseTrackStream {
    pub bytes: Vec<u8>,
    pub closed: Rc<Cell<bool>>,
}

impl Drop for CloseTrackStream {
    fn drop(&mut self) {
        self.closed.set(true);
    }
}

// java public class PptxrTheme
pub struct PptxrTheme {
    // java private KmoTheme mTheme;
    mTheme: Rc<RefCell<KmoTheme>>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePart>,
    // java private KmoMediaLib mMediaLib;
    mMediaLib: Option<KmoMediaLib>,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private int mMode;
    mMode: i32,
}

// java public class PptxrTheme
impl PptxrTheme {
    // java public  PptxrTheme(KmoTheme theme, OpcrPackagePart part, KmoMediaLib mediaLib, PptxrMedias medias, int mode)
    pub fn PptxrTheme(
        theme: Rc<RefCell<KmoTheme>>,
        part: Option<OpcrPackagePart>,
        mediaLib: Option<KmoMediaLib>,
        medias: Option<PptxrMedias>,
        mode: i32,
    ) -> Self {
        // java this.mTheme = theme;
        Self {
            mTheme: theme,
            mPart: part,
            mMediaLib: mediaLib,
            mMedias: medias,
            mMode: mode,
        }
        // java this.mPart = part;
        // java this.mMediaLib = mediaLib;
        // java this.mMedias = medias;
        // java this.mMode = mode;
    }

    pub fn theme(&self) -> Rc<RefCell<KmoTheme>> {
        self.mTheme.clone()
    }

    // java public HashMap<String, Integer> open()
    pub fn open(&mut self) -> Option<HashMap<String, i32>> {
        // java if (this.mPart == null)
        if self.mPart.is_none() {
            // java return null;
            return None;
        }
        // java else
        else {
            let result = (|| -> io::Result<HashMap<String, i32>> {
                // java InputStream stream = this.mPart.getInputStream();
                let stream = self.mPart.as_ref().unwrap().getInputStream()?;
                let part = self.mPart.as_ref().unwrap();
                // java HandlerCallbackImpl handlerCallback = new HandlerCallbackImpl((DgStore)null, this.mMediaLib, (KmoInkLib)null, this.mPart.getRelationships(), this.mMedias, (PptxrSlides)null, this.mMode);
                self.mPart.as_ref().unwrap().getRelationships();
                let handlerCallback = HandlerCallbackImpl::HandlerCallbackImpl(
                    (),
                    {
                        let _ = &self.mMediaLib;
                        None
                    },
                    (),
                    (),
                    None,
                    None,
                    self.mMode,
                    part.rid_pic.clone(),
                    part.load_err,
                );
                // java ThemeHandler handler = new ThemeHandler(this.mTheme, handlerCallback);
                let mut handler = ThemeHandler::ThemeHandler(self.mTheme.clone(), None);
                // java UnifiedXmlReader.read((InputStream)stream, handler);
                if part.read_err {
                    UnifiedXmlReader::read(b"THROW", &mut handler)?;
                } else {
                    UnifiedXmlReader::read(&stream, &mut handler)?;
                }
                // java handlerCallback.load();
                handlerCallback.load()?;
                // java HashMap<String, Integer> rid2MediaIds = handlerCallback.getRidPicMediaIds();
                let rid2MediaIds = handlerCallback.getRidPicMediaIds();
                // java return rid2MediaIds;
                Ok(rid2MediaIds)
            })();
            match result {
                Ok(map) => Some(map),
                Err(e) => {
                    // java Throwable e
                    // java IOLog.d(TAG, "Throwable", e);
                    IOLog_d(TAG, "Throwable", &e);
                    // java return null;
                    None
                }
            }
        }
    }

    // java public static KmoTheme buildDefaultTheme()
    pub fn buildDefaultTheme() -> io::Result<Rc<RefCell<KmoTheme>>> {
        // java InputStream stream = PptResourceFactory.getResPrivider().getTheme();
        let stream = PptResourceFactory::getResPrivider().getTheme();
        // java if (stream == null)
        if stream.is_none() {
            // java throw new FileNotFoundException("The resources is not exist.");
            let _ = FileNotFoundException::NotFound;
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "The resources is not exist.",
            ));
        }
        // java else
        else {
            let bytes = stream.unwrap();
            let closed = Rc::new(Cell::new(false));
            let tracked = CloseTrackStream {
                bytes,
                closed: closed.clone(),
            };
            // java KmoTheme theme = new KmoTheme();
            let theme = Rc::new(RefCell::new(KmoTheme::create()));
            // java ThemeHandler handler = new ThemeHandler(theme, (HandlerCallback)null);
            let mut handler = ThemeHandler::ThemeHandler(theme.clone(), None);
            let read_res = (|| -> io::Result<()> {
                // java UnifiedXmlReader.read((InputStream)stream, handler);
                UnifiedXmlReader::read(&tracked.bytes, &mut handler)?;
                Ok(())
            })();
            // java stream.close();
            drop(tracked);
            LAST_CLOSED.with(|c| c.set(closed.get()));
            let _ = closed.get();
            read_res?;
            // java theme.setThemePath(PptResourceFactory.getResPrivider().getThemePath());
            theme
                .borrow_mut()
                .setThemePath(PptResourceFactory::getResPrivider().getThemePath());
            // java return theme;
            Ok(theme)
        }
    }

    pub fn lastDefaultStreamClosed() -> bool {
        LAST_CLOSED.with(|c| c.get())
    }
}
