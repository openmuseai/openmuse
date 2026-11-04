// java package cn.wps.moss.xlsxr.util;

// java import cn.wps.base.log.IOLog;
// java import cn.wps.kfc.xpxml.hlxml.HLXmlReader;
// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.moffice.io.ForceQuitException;
use std::io::Read;
use std::panic::{catch_unwind, AssertUnwindSafe};

use kfc_kfc_alpha::kfc::xpxml::hlxml::HLXmlReader::HLXmlReader;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlName2Id::XmlName2Id;

use super::MapManager::MapManager;

struct BookIds;

impl XmlName2Id for BookIds {
    fn get_id(&self, prefix: i32, name: i32) -> i32 {
        MapManager::getInstance().getId(prefix, name)
    }
}

// java public final class UnifiedXmlReader
pub struct UnifiedXmlReader;

// java private static final String TAG = null;
const TAG: &str = "";

// java public final class UnifiedXmlReader
impl UnifiedXmlReader {
    // java public static void read(InputStream stream, ElementHandler handler)
    pub fn read(
        stream: &mut dyn Read,
        handler: Box<dyn ElementHandler>,
    ) -> std::io::Result<()> {
        // java read(stream, handler, -1);
        Self::read__2(stream, handler, -1)
    }

    // java public static void read(InputStream stream, ElementHandler handler, int labelPrefix)
    pub fn read__2(
        stream: &mut dyn Read,
        handler: Box<dyn ElementHandler>,
        label_prefix: i32,
    ) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(stream, true, labelPrefix);
        reader.parser_stream_begin_prefix(stream, true, label_prefix)
    }

    // java public static void read(InputStream stream, ElementHandler handler, String encoding)
    pub fn read__3(
        stream: &mut dyn Read,
        handler: Box<dyn ElementHandler>,
        encoding: &str,
    ) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(stream, encoding);
        reader.parser_stream_encoding(stream, encoding)
    }

    // java public static void read(String srcPath, ElementHandler handler)
    pub fn read__4(src_path: &str, handler: Box<dyn ElementHandler>) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(srcPath);
        reader.parser_path(src_path)
    }

    // java public static void read(String srcPath, int beginPos, int endPos, ElementHandler handler, boolean isBeginAtRoot)
    pub fn read__5(
        src_path: &str,
        begin_pos: i32,
        end_pos: i32,
        handler: Box<dyn ElementHandler>,
        is_begin_at_root: bool,
    ) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(srcPath, beginPos, endPos, isBeginAtRoot);
        reader.parser_path_offsets_root(src_path, begin_pos, end_pos, is_begin_at_root)
    }

    // java public static void readSheetXml(InputStream stream, ElementHandler handler)
    pub fn readSheetXml(stream: &mut dyn Read, handler: Box<dyn ElementHandler>) {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java try {
        let caught = catch_unwind(AssertUnwindSafe(|| {
            // java reader.parser(stream, true);
            reader.parser_stream_begin(stream, true)
        }));
        match caught {
            Ok(Ok(())) => {}
            Ok(Err(_)) => {
                // java } catch (Throwable e) {
                // java IOLog.e(TAG, "Throwable", e);
                let _ = TAG;
            }
            Err(payload) => {
                let force = payload
                    .downcast_ref::<&str>()
                    .is_some_and(|s| s.starts_with("ForceQuitException"))
                    || payload
                        .downcast_ref::<String>()
                        .is_some_and(|s| s.starts_with("ForceQuitException"));
                if force {
                    // java } catch (ForceQuitException e) {
                    // java IOLog.e(TAG, "ForceQuitException", e);
                    let _ = TAG;
                    // java throw new ForceQuitException();
                    panic!("ForceQuitException");
                }
                // java } catch (Throwable e) {
                // java IOLog.e(TAG, "Throwable", e);
                let _ = TAG;
            }
        }
    }

    // java public static void readSheetXml(String srcPath, ElementHandler handler)
    pub fn readSheetXml__2(src_path: &str, handler: Box<dyn ElementHandler>) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(srcPath);
        reader.parser_path(src_path)
    }

    // java public static void readSheetXml(String srcPath, int beginPos, int endPos, ElementHandler handler, boolean isBeginAtRoot)
    pub fn readSheetXml__3(
        src_path: &str,
        begin_pos: i32,
        end_pos: i32,
        handler: Box<dyn ElementHandler>,
        is_begin_at_root: bool,
    ) -> std::io::Result<()> {
        // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
        let mut reader = HLXmlReader::new(Box::new(BookIds), handler);
        // java reader.parser(srcPath, beginPos, endPos, isBeginAtRoot);
        reader.parser_path_offsets_root(src_path, begin_pos, end_pos, is_begin_at_root)
    }
}
