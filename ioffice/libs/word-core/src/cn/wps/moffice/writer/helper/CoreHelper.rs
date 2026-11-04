#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{
    Entry as IChpxEntry, IChpxTable,
};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxEntry, IPapxTable,
};

// java public final class CoreHelper
#[derive(Default)]
pub struct CoreHelper {}

// java public final class CoreHelper
impl CoreHelper {
    // java public static String getClsid(String path)
    pub fn getClsid(_path: Option<&str>) -> Option<String> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static String pickVersion(String name)
    pub fn pickVersion(_name: Option<&str>) -> Option<String> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isVersionMcdPrefix(String name)
    pub fn isVersionMcdPrefix(_name: Option<&str>) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isCmtRefMcdPrefix(String name)
    pub fn isCmtRefMcdPrefix(_name: Option<&str>) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isShapeMcdPrefix(String name)
    pub fn isShapeMcdPrefix(_name: Option<&str>) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isChart(Shape shape)
    pub fn isChart(_shape: *mut u8) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isInlineChart(Shape shape)
    pub fn isInlineChart(_shape: *mut u8) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isChartShape0x08(Shape shape)
    pub fn isChartShape0x08(_shape: *mut u8) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isChartShape(Shape shape)
    pub fn isChartShape(_shape: *mut u8) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void dumpPicture4Chart(MediaLib mediaLib, Shape shape)
    pub fn dumpPicture4Chart(_mediaLib: *mut u8, _shape: *mut u8) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private static RectF getChartRectF(Shape shape, RectF rectF)
    fn getChartRectF(_shape: *mut u8, _rectF: *mut u8) -> *mut u8 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void setCellMark(char cellMark, Document document, IPapxTable.Entry entry)
    pub fn setCellMark(
        cellMark: char,
        document: Option<&mut dyn Document>,
        entry: Option<&dyn IPapxEntry>,
    ) {
      let cp: i32 = entry.map(|e| e.endCp()).unwrap_or(0) - 1;
      let textTope: Option<*mut dyn ITextRope> = document.and_then(|d| d.getTextRope());
      Assert::assertNotNull_msg("textTope should not be null", &textTope);
      let fc: i32 = match textTope {
            Some(r) if !r.is_null() => unsafe { (*r).cp2fc(cp) },
            _ => 0,
        };
      let textPool: Option<*mut dyn ITextPool> = match textTope {
            Some(r) if !r.is_null() => unsafe { (*r).getTextPool() },
            _ => None,
        };
      Assert::assertNotNull_msg("textPool should not be null", &textPool);
      if let Some(tp) = textPool {
            if !tp.is_null() {
                unsafe { (*tp).set(fc, cellMark) };
            }
        }
    }

    // java public static int convert2vmlPertcent(float floatValue)
    pub fn convert2vmlPertcent(_floatValue: f32) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static IPapxTable.Entry appendParagraph(SubDocument document, char ch, PropertySet chpx, PropertySet papx)
    pub fn appendParagraph__char_ps_ps(
        document: Option<&mut dyn Document>,
        ch: char,
        chpx: Option<&PropertySet>,
        papx: Option<&PropertySet>,
    ) -> Option<Box<dyn IPapxEntry>> {
      let Some(document) = document else {
            return None;
        };
      let positionInserted: i32 = Self::appendText__char_ps_doc(document, ch, chpx);
      Self::insertToPapxTable(Some(document), positionInserted, papx)
    }

    // java public static IPapxTable.Entry appendParagraph(SubDocument document, char ch, int cp, PropertySet chpx, PropertySet papx)
    pub fn appendParagraph__char_int_ps_ps(
        document: Option<&mut dyn Document>,
        ch: char,
        cp: i32,
        chpx: Option<&PropertySet>,
        papx: Option<&PropertySet>,
    ) -> Option<Box<dyn IPapxEntry>> {
      let Some(document) = document else {
            return None;
        };
      Self::appendText__char_ps_doc(document, ch, chpx);
      Self::insertToPapxTable(Some(document), cp, papx)
    }

    // java public static IPapxTable.Entry insertToPapxTable(SubDocument document, int cp, PropertySet papx)
    pub fn insertToPapxTable(
        document: Option<&mut dyn Document>,
        cp: i32,
        papx: Option<&PropertySet>,
    ) -> Option<Box<dyn IPapxEntry>> {
      let papxTable: Option<*mut dyn IPapxTable> = document.and_then(|d| d.getPapxTable());
      Assert::assertNotNull_msg("papxTable should not be null!", &papxTable);
      match papxTable {
            Some(t) if !t.is_null() => unsafe { IPapxTable::insert(&mut *t, cp, papx) },
            _ => None,
        }
    }

    // java public static int appendText(SubDocument document, char ch, PropertySet props)
    pub fn appendText__char_ps(
        document: Option<&mut dyn Document>,
        ch: char,
        props: Option<&PropertySet>,
    ) -> i32 {
      match document {
            Some(d) => Self::appendText__char_ps_doc(d, ch, props),
            None => 0,
        }
    }

    fn appendText__char_ps_doc(
        document: &mut dyn Document,
        ch: char,
        props: Option<&PropertySet>,
    ) -> i32 {
      let s = ch.to_string();
      Self::appendText__string_ps_doc(document, Some(&s), props)
    }

    // java public static int appendText(SubDocument document, String text, PropertySet props)
    pub fn appendText__string_ps(
        document: Option<&mut dyn Document>,
        text: Option<&str>,
        props: Option<&PropertySet>,
    ) -> i32 {
      match document {
            Some(d) => Self::appendText__string_ps_doc(d, text, props),
            None => 0,
        }
    }

    fn appendText__string_ps_doc(
        document: &mut dyn Document,
        text: Option<&str>,
        props: Option<&PropertySet>,
    ) -> i32 {
      Assert::assertNotNull_msg("text should not be null", &text);
      let nonempty = text.map(|t| !t.is_empty()).unwrap_or(false);
      Assert::assertTrue_msg("text.length() > 0 should be true!", nonempty);
      if !nonempty {
        panic!("text.length() > 0 should be true!");
      }
      Assert::assertNotNull_msg("props should not be null", &props);
      let positionInserted: i32 = document.getLength();
      let textRope: Option<*mut dyn ITextRope> = document.getTextRope();
      Assert::assertNotNull_msg("textRope should not be null", &textRope);
      if let (Some(r), Some(t)) = (textRope, text) {
            if !r.is_null() {
                // Java String.length() is code-unit count; Rust `str::len` is UTF-8 bytes.
                unsafe { (*r).append__string_int_int(t, 0, t.chars().count() as i32) };
            }
        }
      let chpxTable: Option<*mut dyn IChpxTable> = document.getChpxTable();
      Assert::assertNotNull_msg("chpxTable should not be null", &chpxTable);
      if let Some(t) = chpxTable {
            if !t.is_null() {
                unsafe { IChpxTable::insert(&mut *t, positionInserted, props) };
            }
        }
      positionInserted
    }

    // java public static int appendText(SubDocument document, char[] buffer, int start, int end, PropertySet props)
    pub fn appendText__chararr_int_int_ps(
        document: Option<&mut dyn Document>,
        buffer: Option<&[char]>,
        start: i32,
        end: i32,
        props: Option<&PropertySet>,
    ) -> i32 {
      let Some(document) = document else {
            return 0;
        };
      Assert::assertNotNull_msg("buffer should not be null", &buffer);
      Assert::assertTrue_msg("text.length() > 0 should be true!", end > 0);
      if end <= 0 {
        panic!("text.length() > 0 should be true!");
      }
      Assert::assertNotNull_msg("props should not be null", &props);
      let positionInserted: i32 = document.getLength();
      let textRope: Option<*mut dyn ITextRope> = document.getTextRope();
      Assert::assertNotNull_msg("textRope should not be null", &textRope);
      if let (Some(r), Some(buf)) = (textRope, buffer) {
            if !r.is_null() {
                let v: Vec<char> = buf.to_vec();
                unsafe { (*r).append__chararr_int_int(&v, start, end) };
            }
        }
      let chpxTable: Option<*mut dyn IChpxTable> = document.getChpxTable();
      Assert::assertNotNull_msg("chpxTable should not be null", &chpxTable);
      if let Some(t) = chpxTable {
            if !t.is_null() {
                unsafe { IChpxTable::insert(&mut *t, positionInserted, props) };
            }
        }
      positionInserted
    }

    // java public static float vmlRotation2DmlRot(float rot, boolean flipH, boolean flipV)
    pub fn vmlRotation2DmlRot(_rot: f32, _flipH: bool, _flipV: bool) -> f32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static float dmlRot2VmlRotation(float rotation, boolean flipH, boolean flipV)
    pub fn dmlRot2VmlRotation(_rotation: f32, _flipH: bool, _flipV: bool) -> f32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void dumpAllChartShapePictures(TextDocument doc)
    pub fn dumpAllChartShapePictures(_doc: *mut u8) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void dumpAllDiagram(TextDocument doc)
    pub fn dumpAllDiagram(_doc: *mut u8) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void collectAllOLE(TextDocument doc, List<OLE> OLEList)
    pub fn collectAllOLE(_doc: *mut u8, _OLEList: *mut u8) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static void supplyEndPara(SubDocument doc)
    pub fn supplyEndPara(doc: Option<&mut dyn Document>) {
      let lastPara: Option<Box<dyn IPapxEntry>> = match doc.as_ref().and_then(|d| d.getPapxTable()) {
            Some(t) if !t.is_null() => unsafe {
                IPapxTable::end(&*t).and_then(|e| IPapxEntry::getPrev(e.as_ref()))
            },
            _ => None,
        };
      if lastPara.as_ref().map(|e| e.isHead()).unwrap_or(true) {
        Self::appendParagraph__char_int_ps_ps(
                doc,
                '\r',
                0,
                Some(&*PROPERTYSET_EMPTY),
                Some(&*PROPERTYSET_EMPTY),
            );
      } else if lastPara
            .as_ref()
            .and_then(|e| IPapxEntry::getProperty(e.as_ref()))
            .map(|p| p.getBoolean__int_boolean(225, false))
            .unwrap_or(false)
      {
        Self::appendParagraph__char_ps_ps(
                doc,
                '\r',
                Some(&*PROPERTYSET_EMPTY),
                Some(&*PROPERTYSET_EMPTY),
            );
      }
    }

    // java public static Geometry getNewGeometry(Shape shape, Segment[] oldSegments, int[] oldVertices)
    pub fn getNewGeometry(_shape: *mut u8, _oldSegments: *mut u8, _oldVertices: Option<&[i32]>) -> *mut u8 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static boolean isValidChar(char ch)
    pub fn isValidChar(ch: char) -> bool {
      '\u{ffff}' != ch && '\u{fffd}' != ch && '\u{fffe}' != ch
    }

    // java public static HashMap<Integer, ArrayList<String>> writeCommentCustDatas(File itemPath, File propsPath, Document mainDoc, Object owner)
    pub fn writeCommentCustDatas(
        _itemPath: *mut u8,
        _propsPath: *mut u8,
        _mainDoc: Option<&mut dyn Document>,
        _owner: *mut u8,
    ) -> *mut u8 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }
}
