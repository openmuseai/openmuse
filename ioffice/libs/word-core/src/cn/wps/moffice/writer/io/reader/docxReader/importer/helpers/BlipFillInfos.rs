// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.BlipFillInfos
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::core::shape::DumpInfo::{DumpInfo, FillHandle};
use crate::cn::wps::moffice::writer::core::shape::PictureDumpMap::PictureDumpMap;
use crate::cn::wps::moffice::writer::core::shape::PictureLoader::{
    BlipHandlerRef, FinishHandlerRef, IDumpBlipHandler, PictureLoader,
};
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::media::IMediaLib::IMediaLib;
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::io::xwpf::usermodel::XWPFPictureData::XWPFPictureData;
use io_wps_io::io::xwpf::util::ZipEntrySource::ZipEntrySource;
use io_wps_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use std::cell::RefCell;
use std::error::Error;
use std::io::{self, Read, Write};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use util::base::assertion::IOAssert::IOAssert;
use util::moffice::crash::NoSpaceLeftException::NoSpaceLeftException;
use util::moffice::writer::io::reader::helper::DumpPicBuffer::DumpPicBuffer;
use util::util::manager::FileManager::FileManager;
use util::util::zip::model::ZipBaseModel::ZipBaseModel;

// java private static BlipFillInfos sInstance;
thread_local! {
    static sInstance: RefCell<Option<BlipFillInfos>> = RefCell::new(None);
}

// java public class BlipFillInfos
pub struct BlipFillInfos {
    // java private ArrayList<BlipDocxInfo> mInfos = new ArrayList(30);
    mInfos: Vec<BlipDocxInfo>,
}

// java public static class BlipInfo
struct BlipInfo {
    // java Shape shape;
    shape: Arc<Mutex<Shape>>,
    // java int type;
    r#type: i32,
}

impl BlipInfo {
    // java public BlipInfo(Shape shape, int type)
    fn new(shape: Arc<Mutex<Shape>>, r#type: i32) -> Self {
        Self { shape, r#type }
    }

    // java public BlipFill getFill()
    fn getFill(&self) -> Option<FillHandle> {
        // java return DumpInfo.getFill(this.shape, this.type);
        let mut shape = self.shape.lock().expect("shape");
        DumpInfo::getFill(&mut shape, self.r#type)
    }
}

// java public static class BlipDocxInfo extends PictureLoader.DefaultDumpBlipHandler.BlipBaseInfo
struct BlipDocxInfo {
    // java private List<BlipInfo> mDest;
    mDest: Vec<BlipInfo>,
    // java private PictureData mSource; identity is the shared OPC part.
    part: Rc<RefCell<POIXMLDocumentPart>>,
    part_key: usize,
    // java private TextDocument mDoc;
    mDoc: *mut TextDocument,
    handler: BlipHandlerRef,
}

// java public static final class DumpBlipDocxHandler extends PictureLoader.DefaultDumpBlipHandler
struct DumpBlipDocxHandler {
    part_key: usize,
    // java private boolean mHandled;
    mHandled: AtomicBool,
    dumped: AtomicBool,
    no_space: AtomicBool,
}

impl BlipFillInfos {
    fn new() -> Self {
        Self { mInfos: Vec::with_capacity(30) }
    }

    // java public static BlipFillInfos getInstance()
    pub fn getInstance() {
        // java if (null == sInstance) sInstance = new BlipFillInfos();
        sInstance.with(|slot| {
            let mut slot = slot.borrow_mut();
            if slot.is_none() {
                *slot = Some(BlipFillInfos::new());
            }
        });
    }

    // java public static void dispose()
    pub fn dispose() {
        sInstance.with(|slot| {
            let mut slot = slot.borrow_mut();
            // java if (null != sInstance) { mInfos.clear(); mInfos = null; sInstance = null; }
            if let Some(infos) = slot.as_mut() {
                infos.mInfos.clear();
            }
            *slot = None;
        });
    }

    fn picture_key(picture: &XWPFPictureData) -> usize {
        // Java compares PictureData with ==. from_relation shares one OPC part Rc.
        Rc::as_ptr(&picture.document_part()) as usize
    }

    // Java body of IOHelper.addBlipDocxInfo
    pub fn register(shape: &Arc<Mutex<Shape>>, picture: &XWPFPictureData, doc: &mut TextDocument, r#type: i32) {
        Self::getInstance();
        let shape = shape.clone();
        let doc_ptr = doc as *mut TextDocument;
        sInstance.with(|slot| {
            let mut slot = slot.borrow_mut();
            let infos = slot.as_mut().expect("BlipFillInfos");
            let key = Self::picture_key(picture);
            if let Some(info) = infos.mInfos.iter_mut().find(|info| info.part_key == key) {
                // java info.addBlipFill(shape, type);
                info.addBlipFill(&shape, r#type);
            } else {
                // java info = new BlipDocxInfo(shape, type, pictureData, doc); add(info);
                let info = BlipDocxInfo::new(shape.clone(), r#type, picture, doc_ptr);
                let shape_id = shape.lock().expect("shape").getShapeID();
                let handler = info.handler.clone();
                // java this.mInfos.add(info);
                infos.mInfos.push(info);
                // java PictureDumpMap.add(shape.getShapeID(), type, new DumpBlipDocxHandler(info));
                PictureDumpMap::add(shape_id, r#type, Some(handler));
            }
        });
    }

    // java public void dumpAllPictures()
    pub fn dumpAllPictures() {
        Self::getInstance();
        let (jobs, waits): (Vec<(Arc<Mutex<Shape>>, i32)>, Vec<(i32, i32)>) = sInstance.with(|slot| {
            let slot = slot.borrow();
            let infos = slot.as_ref().expect("mBlipFillInfos should not be null");
            // java IOAssert.assertNotNull("mBlipFillInfos should not be null", this.mInfos);
            IOAssert::assertNotNull_msg("mBlipFillInfos should not be null", &Some(infos.mInfos.len()));
            let mut jobs = Vec::new();
            let mut waits = Vec::new();
            // java for each info, for each blipInfo: pictureLoader.dump(shape, type)
            for info in &infos.mInfos {
                IOAssert::assertNotNull_msg("info should not be null", &Some(info.part_key));
                IOAssert::assertNotNull_msg("blipFills should not be null", &Some(info.mDest.len()));
                let first = info.mDest.first().expect("blipInfo");
                let shape_id = first.shape.lock().expect("shape").getShapeID();
                // java wait uses PictureDumpMap.get(first.shapeId, first.type)
                waits.push((shape_id, first.r#type));
                for blip in &info.mDest {
                    IOAssert::assertNotNull_msg("blipInfo should not be null", &Some(blip.r#type));
                    jobs.push((blip.shape.clone(), blip.r#type));
                }
            }
            (jobs, waits)
        });
        if jobs.is_empty() {
            return;
        }
        // java PictureLoader pictureLoader = new PictureLoader();
        let mut pictureLoader = PictureLoader::new();
        for (shape, r#type) in &jobs {
            pictureLoader.dump(shape, *r#type);
        }
        // java this.waitDumpBlipThreadFinish();
        for (shape_id, r#type) in waits {
            let handler = PictureDumpMap::get(shape_id, r#type).expect("handler");
            // java defaultHandler.notifyAndWaitDumpThreadFinish();
            handler.notifyAndWaitDumpThreadFinish();
            // java if (defaultHandler.hasNoSpaceLeftException()) { dispose(); throw new RuntimeException("No space left"); }
            if handler.hasNoSpaceLeftException() {
                DumpBlipDocxHandler::dispose();
                panic!("No space left");
            }
        }
        // java BlipFillInfos.DumpBlipDocxHandler.dispose();
        DumpBlipDocxHandler::dispose();
    }
}

impl BlipDocxInfo {
    // java public BlipDocxInfo(Shape shape, int type, PictureData source, TextDocument doc)
    fn new(shape: Arc<Mutex<Shape>>, r#type: i32, picture: &XWPFPictureData, doc: *mut TextDocument) -> Self {
        let part_key = BlipFillInfos::picture_key(picture);
        let handler: BlipHandlerRef = Arc::new(DumpBlipDocxHandler {
            part_key,
            mHandled: AtomicBool::new(false),
            dumped: AtomicBool::new(false),
            no_space: AtomicBool::new(false),
        });
        // java this.mDest = new ArrayList(5); this.mDest.add(new BlipInfo(shape, type));
        Self {
            mDest: vec![BlipInfo::new(shape, r#type)],
            part: picture.document_part(),
            part_key,
            mDoc: doc,
            handler,
        }
    }

    // java public void addBlipFill(Shape shape, int type)
    fn addBlipFill(&mut self, shape: &Arc<Mutex<Shape>>, r#type: i32) {
        let shape_id = shape.lock().expect("shape").getShapeID();
        // java this.mDest.add(new BlipInfo(shape, type));
        self.mDest.push(BlipInfo::new(shape.clone(), r#type));
        let first = &self.mDest[0];
        let first_id = first.shape.lock().expect("shape").getShapeID();
        // java PictureDumpMap.add(shape.getShapeID(), type, PictureDumpMap.get(first.shapeId, first.type));
        let handler = PictureDumpMap::get(first_id, first.r#type);
        PictureDumpMap::add(shape_id, r#type, handler);
        let last = self.mDest.len() - 1;
        // java this.setMediaID(last, first);
        Self::setMediaID(&self.mDest[last], &self.mDest[0]);
    }

    // java public boolean isEndNode()
    fn isEndNode(&self) -> bool {
        // java return null == mDumpFinishHandlers && null == mDest && null == mSource && null == mDoc;
        self.mDest.is_empty() && self.mDoc.is_null()
    }

    // java public void setMediaID(BlipInfo info, BlipInfo src)
    fn setMediaID(info: &BlipInfo, src: &BlipInfo) {
        // java BlipFill fill = info.getFill();
        let Some(fill) = info.getFill() else { return };
        // java BlipFill srcFill = src.getFill();
        let Some(src_fill) = src.getFill() else { return };
        // java fill.setMediaID(srcFill.getMediaID());
        fill.setMediaID(src_fill.getMediaID());
    }
}

impl DumpBlipDocxHandler {
    // java public static void dispose()
    fn dispose() {
        // java sDumpPicThread = null;
        // java DumpPicBuffer.clearPool();
        DumpPicBuffer::clearPool();
    }

    fn info_mut<R>(part_key: usize, f: impl FnOnce(&mut BlipDocxInfo) -> R) -> Option<R> {
        sInstance.with(|slot| {
            let mut slot = slot.borrow_mut();
            let infos = slot.as_mut()?;
            let info = infos.mInfos.iter_mut().find(|info| info.part_key == part_key)?;
            Some(f(info))
        })
    }

    // java protected int dumpBlip(BlipBaseInfo baseInfo)
    fn dumpBlip(&self) -> i32 {
        let Some(media_id) = Self::info_mut(self.part_key, |info| Self::dump_info(info, self)) else {
            return -1;
        };
        media_id
    }

    fn dump_info(info: &mut BlipDocxInfo, handler: &DumpBlipDocxHandler) -> i32 {
        // java PictureData pictureData = blipInfo.mSource;
        let picture = XWPFPictureData::from_relation(info.part.clone());
        // java XWPFPictureData.PictureDataSource source = pictureData.getDataSource();
        let filename = {
            let source = picture.getDataSource().expect("source");
            // java String filename = source.getFilename();
            source.getFilename().to_string()
        };
        // java IMediaLib mediaLib = blipInfo.mDoc.getMediaLib();
        let doc = info.mDoc;
        if doc.is_null() {
            return -1;
        }
        // java int mediaId = -1; scan dest fills for an existing id
        let mut media_id = -1i32;
        let size = info.mDest.len();
        if size > 0 {
            for dest in &info.mDest {
                // java mediaId = dest.getFill().getMediaID();
                let fill = dest.getFill().unwrap_or_else(|| panic!(
                    "BlipDocxInfo missing fill: shape_id={} type={} part={}",
                    dest.shape.lock().expect("shape").getShapeID(), dest.r#type,
                    info.part_key,
                ));
                media_id = fill.getMediaID();
                // java if (-1 != mediaId) break;
                if media_id != -1 {
                    break;
                }
            }
            if media_id != -1 {
                // java every dest fill.setMediaID(mediaId); return mediaId;
                for dest in &info.mDest {
                    dest.getFill().expect("fill").setMediaID(media_id);
                }
                return media_id;
            }
        }
        // java mediaId = this.addZipModelToMediaLib(...)
        media_id = Self::addZipModelToMediaLib(&picture, doc);
        // java if (mediaId == -1 && (filepath = doDumpBlip(...)) != null) mediaId = mediaLib.addMedia(...)
        if media_id == -1 {
            let root = unsafe { (*doc).getTmpRootDir() };
            let source = picture.getDataSource().expect("source");
            let entry = source.getEntry().clone();
            // java zipEntrySource.getInputStream(source.getEntry())
            if let Ok(mut input) = source.getSource().getInputStream(&entry) {
                if let Some(filepath) = Self::doDumpBlip(&root, &filename, &mut input, handler) {
                    let lib = unsafe { (*doc).getMediaLib() };
                    // java mediaLib.addMedia(filename, filepath, MediaTypeEnum.PICTURE);
                    media_id = lib.addMedia(&filename, &filepath, MediaTypeEnum::PICTURE);
                }
            }
        }
        // java if (-1 == mediaId) return mediaId;
        if media_id == -1 {
            return media_id;
        }
        // java else set every dest and return mediaId
        for dest in &info.mDest {
            dest.getFill().expect("fill").setMediaID(media_id);
        }
        media_id
    }

    // java private int addZipModelToMediaLib(...)
    fn addZipModelToMediaLib(picture: &XWPFPictureData, doc: *mut TextDocument) -> i32 {
        let Some(source) = picture.getDataSource() else { return -1 };
        // java if (!(source.getSource() instanceof ZipFileZipEntrySource)) return -1;
        if source.getSource().as_any().downcast_ref::<ZipFileZipEntrySource>().is_none() {
            return -1;
        }
        // java ZipEntry zipEntry = source.getEntry(); if (zipEntry == null) return -1;
        let entry = source.getEntry().clone();
        let filename = source.getFilename().to_string();
        // java fileName.indexOf(".") — first dot, not the last
        let suffix = match filename.find('.') {
            Some(index) => filename[index + 1..].to_string(),
            None => "temp".to_string(),
        };
        // java RAFStream rafStream = (RAFStream)zipEntrySource.getInputStream(zipEntry, true);
        let Ok(mut input) = source.getSource().getInputStream_with_flag(&entry, true) else { return -1 };
        let root = unsafe { (*doc).getTmpRootDir() };
        // java new ZipBaseModel(rafStream, decompressSuffix, zipEntry, fileManager)
        let model = ZipBaseModel::new_default(&mut *input, &suffix, FileManager::new(&root));
        let lib = unsafe { (*doc).getMediaLib() };
        // java return iMediaLib.addMedia(fileName, null, PICTURE, zipBaseModel);
        lib.addMedia_with_zip(&filename, "", MediaTypeEnum::PICTURE, Arc::new(Mutex::new(model)))
    }

    // java private String doDumpBlip(String rootDir, String fileName, InputStream is)
    fn doDumpBlip(root: &str, file_name: &str, input: &mut dyn Read, handler: &DumpBlipDocxHandler) -> Option<String> {
        // java String filepath = IOHelper.createPicturePath(rootDir, fileName);
        let filepath = IOHelper::createPicturePath(root, file_name)?;
        // java os = new FileOutputStream(filepath); catch FileNotFoundException return null
        let mut output = match std::fs::File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return None,
        };
        // java DumpPicBuffer dumpBuf = DumpPicBuffer.obtain(); byte[] buf = dumpBuf.getBuf();
        let dump_buf = DumpPicBuffer::obtain();
        IOAssert::assertNotNull_msg("dumpBuf should not be null.", &Some(0));
        let buf_len = dump_buf.getBuf().len();
        // java IOAssert.assertNotNull("buf should not be null.", buf);
        IOAssert::assertNotNull_msg("buf should not be null.", &Some(buf_len));
        let mut buf = vec![0u8; buf_len];
        // java IOUtils.copy(is, os, buf);
        let copy_result = io::copy(input, &mut output);
        let _ = output.flush();
        let _ = buf;
        // java dumpBuf.recycle();
        dump_buf.recycle();
        match copy_result {
            Ok(_) => Some(filepath),
            Err(error) => {
                // java if (NoSpaceLeftException.isNoSpaceLeftException(e)) throw new RuntimeException(e);
                let error: &dyn Error = &error;
                if NoSpaceLeftException::isNoSpaceLeftException(Some(error)) {
                    handler.no_space.store(true, Ordering::SeqCst);
                }
                // java return null;
                None
            }
        }
    }
}

impl IDumpBlipHandler for DumpBlipDocxHandler {
    // Java DefaultDumpBlipHandler.onHandle queues dumpBlip. The OPC part is an Rc,
    // so dumpBlip runs on the import thread that owns that part, then wait observes the id.
    fn onHandle(&self, _fill: FillHandle, _handler: Option<FinishHandlerRef>) -> bool {
        // java if (this.isHandled() && info instanceof BlipDocInfo) return true;
        // Queue only. The OPC part is an Rc, so dumpBlip runs on this import thread
        // when waitDumpBlipThreadFinish calls notifyAndWaitDumpThreadFinish.
        // java if (bAdd) this.setHandled();
        self.mHandled.store(true, Ordering::SeqCst);
        true
    }

    fn notifyAndWaitDumpThreadFinish(&self) -> bool {
        if !self.dumped.swap(true, Ordering::SeqCst) {
            let _media_id = self.dumpBlip();
        }
        true
    }

    fn hasNoSpaceLeftException(&self) -> bool {
        self.no_space.load(Ordering::SeqCst)
    }
}
