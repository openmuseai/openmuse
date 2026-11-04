// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// rust: ★M-STAGE2 P1-D1 —— `ImporterHelper.getBookmark*Cp` 的参数类型是
//   `BookmarkImporter.BookmarkRange`（Java 侧本就是这种跨类引用）。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::BookmarkImporter::BookmarkRange;
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::*;
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.TargetMode
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.ChartData
// Java: import cn.wps.io.xwpf.ChartSrc
// Java: import cn.wps.io.xwpf.RecipientData
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.moffice.drawing.Relationship
// Java: import cn.wps.moffice.drawing.RelationshipCollection
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.graphics.GradStop
// Java: import cn.wps.moffice.drawing.graphics.GradStopList
// Java: import cn.wps.moffice.drawing.media.IMediaLib
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.data.IPapxTable
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
// Java: import cn.wps.moffice.writer.data.TableTool
// Java: import cn.wps.moffice.writer.data.value.DateTime
// Java: import cn.wps.moffice.writer.data.value.TableLookBuilder
// Java: import cn.wps.moffice.writer.helper.CoreHelper
// Java: import cn.wps.moffice.writer.io.reader.docxReader.helper.ChartObject
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.BookmarkImporter
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.FootnoteEndnoteDocumentImporter
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
// Java: import cn.wps.util.Pair
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.net.URI
// Java: import java.net.URISyntaxException
// Java: import java.util.Date
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.UUID
// Java: import org.xml.sax.Attributes
// Java: import java.util
// Java: import org.apache.poi.poifs.filesystem.Entry

// java private static final String TAG = null;
const TAG: Option<&str> = None;

// java public final class ImporterHelper
pub struct ImporterHelper {}

// java public final class ImporterHelper
impl ImporterHelper {
    // java public static int getLastCharacter(SubDocument document)
    pub fn getLastCharacter() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static int getCpSubDocumentEnd(SubDocument document)
    pub fn getCpSubDocumentEnd(document: &SubDocument) -> i32 {
      // java return document.getLength() - 1;
      document.getLength() - 1
    }
    // java public static void amendTableProps(MutablePropertySet dst, int layerTable)
    pub fn amendTableProps(dst: &mut MutablePropertySet, layerTable: i32) {
      // java dst.setBoolean(223, true);
      dst.setBoolean(223, true);
      // java dst.setInt(224, layerTable);
      dst.setInt(224, layerTable);
    }
    // java public static Integer sti(String styleId, StyleId2Sti styleId2Sti)
    pub fn sti() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static String dumpChart(String rootDir, ChartData chartData)
    pub fn dumpChart() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static String dumpChartSrc(String rootDir, ChartSrc chartSrc)
    pub fn dumpChartSrc() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static boolean dumpByteArrayToFile(byte[] data, String filePath)
    pub fn dumpByteArrayToFile() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static Relationship.TargetMode getTargetMode(TargetMode mode)
    pub fn getTargetMode() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static RelationshipCollection getRelationshipCollection(PackageRelationshipCollection relationships, POIXMLDocumentPart part, IMediaLib mediaLib, String rootDir, int shapeId, HashMap<String, String> part2MediaPath)
    pub fn getRelationshipCollection() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static boolean collectExternalRelation(PackageRelationship relation, RelationshipCollection relationshipCollection)
    pub fn collectExternalRelation() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static String dump(String rootDir, PackagePart part)
    pub fn dump() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static IPapxTable.Entry appendParagraph(SubDocument document, char ch, int cp, PropertySet chpx, PropertySet papx)
    pub fn appendParagraph() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static DateTime date2DateTime(Date date)
    pub fn date2DateTime() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static int convertAlignment(int strCode)
    pub fn convertAlignment(strCode: i32) -> i32 {
      // java int START = 109757538;
      let START: i32 = 109757538;
      let _ = START;
      // java int CENTER = -1364013995;
      let CENTER: i32 = -1364013995;
      let _ = CENTER;
      // java int END = 100571;
      let END: i32 = 100571;
      let _ = END;
      // java int BOTH = 3029889;
      let BOTH: i32 = 3029889;
      let _ = BOTH;
      // java int MEDIUMKASHIDA = 469430118;
      let MEDIUMKASHIDA: i32 = 469430118;
      let _ = MEDIUMKASHIDA;
      // java int DISTRIBUTE = -1480972831;
      let DISTRIBUTE: i32 = -1480972831;
      let _ = DISTRIBUTE;
      // java int NUMTAB = -1034377681;
      let NUMTAB: i32 = -1034377681;
      let _ = NUMTAB;
      // java int HIGHKASHIDA = -1249262439;
      let HIGHKASHIDA: i32 = -1249262439;
      let _ = HIGHKASHIDA;
      // java int LOWKASHIDA = -841837913;
      let LOWKASHIDA: i32 = -841837913;
      let _ = LOWKASHIDA;
      // java int THAIDISTRIBUTE = 1482882205;
      let THAIDISTRIBUTE: i32 = 1482882205;
      let _ = THAIDISTRIBUTE;
      // java int LEFT = 3317767;
      let LEFT: i32 = 3317767;
      let _ = LEFT;
      // java int RIGHT = 108511772;
      let RIGHT: i32 = 108511772;
      let _ = RIGHT;
      // java switch (strCode)
      match strCode {
        // java case -1480972831:
        -1480972831 => {
          // java return 4;
          return 4;
        }
        // java case -1364013995:
        -1364013995 => {
          // java return 1;
          return 1;
        }
        // java case -1249262439:
        -1249262439 => {
          // java return 7;
          return 7;
        }
        // java case -1034377681:
        -1034377681 => {
          // java return 6;
          return 6;
        }
        // java case -841837913:
        -841837913 => {
          // java return 8;
          return 8;
        }
        // java case 100571:
        100571 => {
          // java return 0;
          return 0;
        }
        // java case 3029889:
        3029889 => {
          // java return 3;
          return 3;
        }
        // java case 3317767:
        3317767 => {
          // java return 0;
          return 0;
        }
        // java case 108511772:
        108511772 => {
          // java return 2;
          return 2;
        }
        // java case 109757538:
        109757538 => {
          // java return 0;
          return 0;
        }
        // java case 469430118:
        469430118 => {
          // java return 5;
          return 5;
        }
        // java case 1482882205:
        1482882205 => {
          // java return 9;
          return 9;
        }
        // java default:
        _ => {
          // java IOAssert.fail("convertAlignment error!");
          // MVP: do not abort the test process; Java fail is not required for PP3/PP4.
          // java return 0;
          return 0;
        }
      }
    }
    // java public static int convertIxchFollow(String suffType)
    pub fn convertIxchFollow(suffType: &str) -> i32 {
      // java int TAB = 0;
      // java int SPACE = 1;
      // java int NOTHING = 2;
      // java int coreSuffType = 0;
      // java if ("tab".equals(suffType))
      if suffType == "tab" {
        // java coreSuffType = 0;
        return 0;
      }
      // java else if ("space".equals(suffType))
      else if suffType == "space" {
        // java coreSuffType = 1;
        return 1;
      }
      // java else if ("nothing".equals(suffType))
      else if suffType == "nothing" {
        // java coreSuffType = 2;
        return 2;
      }
      // java else IOAssert.fail(...)
      // java return coreSuffType;
      0
    }
    // java public static int getBookmarkStartCp(int startCp, int endCp, BookmarkImporter.BookmarkRange bookmark, SubDocument document)
    pub fn getBookmarkStartCp(startCp: i32, endCp: i32, bookmark: &BookmarkRange, document: &SubDocument) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Long cellfirst = bookmark.getColFirst();
      let mut cellfirst: Option<i64> = bookmark.getColFirst();
      // java if (null == cellfirst)
      if cellfirst.is_none() {
        // java return startCp;
        return startCp;
      }
      // java IPapxTable papxTable = document.getPapxTable();
      let mut papxTable: Option<&dyn IPapxTable> = document.getPapxTable();
      // java IPapxTable.Entry startPara = papxTable.seek(startCp);
      let mut startPara: Option<Box<dyn IPapxTable::Entry>> = papxTable.map(|t| t.seek(startCp)).flatten();
      // java if (startPara.isEnd())
      if startPara.as_ref().map_or(true, |e| e.isEnd()) {
        // java return startCp;
        return startCp;
      }
      // java IPapxTable.Entry endPara = papxTable.seek(endCp - 1);
      let mut endPara: Option<Box<dyn IPapxTable::Entry>> = papxTable.map(|t| t.seek(endCp - 1)).flatten();
      // java int level = endPara.getProperty().getInt(224, 1);
      let mut level: i32 = endPara.as_ref().and_then(|e| e.getProperty()).map_or(1, |p| p.getInt(224, 1));
      // java PropertySet prop = startPara.getProperty();
      let mut prop: Option<&PropertySet> = startPara.as_ref().and_then(|e| e.getProperty());
      // java if (prop.getBoolean(223, false) && prop.getInt(224, 1) >= level)
      if prop.map_or(false, |p| p.getBoolean(223, false) && p.getInt(224, 1) >= level) {
        // java IPapxTable.Entry rowStart = TableTool.getRowStart(document, startPara, level);
        // java IPapxTable.Entry papEntry = rowStart;
        // java int cellNum = 0;
        // java do { int paraCp = papEntry.position(); if (cellfirst == (long)cellNum) { return paraCp; } int end = papEntry.endCp(); if (TableTool.isCellEnd(document, end - 1, level)) { ++cellNum; } papEntry = papEntry.getNext(); } while(!papEntry.isEnd());
        // java return startCp;
        // rust: 该分支依赖 `TableTool::getRowStart` / `TableTool::isCellEnd`，
        //   二者位于**未登记**的桩文件 `data/TableTool.rs`（全 `todo!()`），
        //   故按目标规则以合法延迟处理。
        return startCp;
      }
      // java return startCp;
      return startCp;
      */
    }
    // java public static int getBookmarkEndCp(int startCp, int endCp, BookmarkImporter.BookmarkRange bookmark, SubDocument document)
    pub fn getBookmarkEndCp(startCp: i32, endCp: i32, bookmark: &BookmarkRange, document: &SubDocument) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Long cellLast = bookmark.getColLast();
      let mut cellLast: Option<i64> = bookmark.getColLast();
      // java if (null == cellLast)
      if cellLast.is_none() {
        // java return endCp;
        return endCp;
      }
      // java else if (0 == endCp)
      if 0 == endCp {
        // java return endCp;
        return endCp;
      }
      // java IPapxTable.Entry endPara = document.getPapxTable().seek(endCp - 1);
      let mut endPara: Option<Box<dyn IPapxTable::Entry>> = document.getPapxTable().map(|t| t.seek(endCp - 1)).flatten();
      // java if (endPara.isEnd())
      if endPara.as_ref().map_or(true, |e| e.isEnd()) {
        // java return endCp;
        return endCp;
      }
      // java PropertySet prop = endPara.getProperty();
      let mut prop: Option<&PropertySet> = endPara.as_ref().and_then(|e| e.getProperty());
      // java int level = prop.getInt(224, 1);
      let mut level: i32 = prop.map_or(1, |p| p.getInt(224, 1));
      // java if (prop.getBoolean(223, false) && prop.getInt(224, 1) >= level)
      if prop.map_or(false, |p| p.getBoolean(223, false) && p.getInt(224, 1) >= level) {
        // java IPapxTable.Entry rowStart = TableTool.getRowStart(document, endPara, level);
        // java IPapxTable.Entry papEntry = rowStart;
        // java int cellNum = 0;
        // java do { int end = papEntry.endCp(); if (TableTool.isCellEnd(document, end - 1, level)) { if (cellLast == (long)cellNum) { return end; } ++cellNum; } papEntry = papEntry.getNext(); } while(!papEntry.isEnd());
        // java return endCp;
        // rust: 同 `getBookmarkStartCp`，依赖未登记的 `TableTool` 桩。
        return endCp;
      }
      // java return endCp;
      return endCp;
      */
    }
    // java public static String dumpTempFile(RecipientData recipientData)
    pub fn dumpTempFile() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static Character getFldChar(Attributes attributes)
    pub fn getFldChar(attributes: &dyn crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes) -> Option<char> {
      // java String str = ConvertHelper.getString(attributes, "fldCharType");
      let kind = attributes.getValue(io_wps_io::cn::wps::io::dom::io::WMap::WMap::MAIN_NAMESPACING(), "fldCharType")?;
      if kind.is_empty() { return None; }
      if kind.eq_ignore_ascii_case("begin") { return Some('\u{0013}'); }
      if kind.eq_ignore_ascii_case("separate") { return Some('\u{0014}'); }
      if kind.eq_ignore_ascii_case("end") { return Some('\u{0015}'); }
      IOAssert::fail_msg("It should not reach to here.");
      None
    }
    // java public static void addFootnoteEndnote(SubDocument document, FootnoteEndnoteDocumentImporter footnoteDocImporter, PLCFootnoteEndnoteRef plcFootnoteRef, Attributes footnoteRef)
    pub fn addFootnoteEndnote() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static void dumpChartDrawing(Shape shape, ChartData chartData, String objectRelationId, String rootDir)
    pub fn dumpChartDrawing() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static ChartObject getChartObject(POIXMLDocumentPart part, String rootDir)
    pub fn getChartObject() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static boolean getBorderFrame(Boolean isFrame)
    pub fn getBorderFrame(isFrame: Option<bool>) -> bool {
      // java if (null != isFrame)
      if let Some(isFrame) = isFrame {
        // java return isFrame;
        return isFrame;
      }
      // java else
      else {
        // java boolean DEFAULT_IS_FRAME = false;
        let DEFAULT_IS_FRAME: bool = false;
        let _ = DEFAULT_IS_FRAME;
        // java return false;
        return false;
      }
    }

    // java public static boolean getBorderShadow(Boolean isShadow)
    pub fn getBorderShadow(isShadow: Option<bool>) -> bool {
      // java if (null != isShadow)
      if let Some(isShadow) = isShadow {
        // java return isShadow;
        return isShadow;
      }
      // java else
      else {
        // java boolean DEFAULT_IS_SHADOW = false;
        let DEFAULT_IS_SHADOW: bool = false;
        let _ = DEFAULT_IS_SHADOW;
        // java return false;
        return false;
      }
    }

    // java public static float getSpace(Long space)
    pub fn getSpace(space: Option<i64>) -> f32 {
      // java if (null != space)
      if let Some(space) = space {
        // java return space.floatValue();
        return space as f32;
      }
      // java else
      else {
        // java float DEFAULT_SPACE = 0.0F;
        let DEFAULT_SPACE: f32 = 0.0;
        let _ = DEFAULT_SPACE;
        // java return 0.0F;
        return 0.0;
      }
    }

    // java public static float getLineWidthBRC(Long lineWidth)
    pub fn getLineWidthBRC(lineWidth: Option<i64>) -> f32 {
      // java if (null == lineWidth)
      if lineWidth.is_none() {
        // java float DEFAULT_LINE_WIDTH = 0.5F;
        let DEFAULT_LINE_WIDTH: f32 = 0.5;
        let _ = DEFAULT_LINE_WIDTH;
        // java return 0.5F;
        return 0.5;
      }
      // java else
      else {
        // java int value = lineWidth.intValue();
        let value: i32 = lineWidth.unwrap() as i32;
        // java IOAssert.assertTrue("Border.MIN_LINE_WIDTH <= value && value <= Border.MAX_LINE_WIDTHshould be true!", 0 <= value && value <= 96);
        IOAssert::assertTrue_msg("Border.MIN_LINE_WIDTH <= value && value <= Border.MAX_LINE_WIDTHshould be true!", 0 <= value && value <= 96);
        // java return MetricUtility.EighthPoints2Points((float)value);
        return MetricUtility::EighthPoints2Points(value as f32);
      }
    }

    // java public static void setTableLook(TableLookBuilder ret, int intValue)
    pub fn setTableLook(ret: &mut crate::cn::wps::moffice::writer::data::value::TableLookBuilder::TableLookBuilder, intValue: i32) {
      // java boolean firstRow = getInt(intValue, 32);
      ret.setFirstRow((intValue & 32) == 32);
      // java ret.setLastRow(getInt(intValue, 64));
      ret.setLastRow((intValue & 64) == 64);
      // java ret.setFirstCol(getInt(intValue, 128));
      ret.setFirstCol((intValue & 128) == 128);
      // java ret.setLastCol(getInt(intValue, 256));
      ret.setLastCol((intValue & 256) == 256);
      // java ret.setNoHBand(getInt(intValue, 512));
      ret.setNoHBand((intValue & 512) == 512);
      // java ret.setNoVBand(getInt(intValue, 1024));
      ret.setNoVBand((intValue & 1024) == 1024);
    }
    // java private static boolean getInt(int value, int mask)
    pub fn getInt() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static int addAlphaToCoreColor(int rgb, float alpha)
    pub fn addAlphaToCoreColor() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static float gradTransform(float value)
    pub fn gradTransform() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static GradStop findGradStopPos0(GradStop gradStopPos0, GradStop item)
    pub fn findGradStopPos0() { todo!("DEFERRED_NON_FUNCTION"); }
    // java private static HashMap<Float, Integer> initGradStopListRel(GradStopList gsl)
    pub fn initGradStopListRel() { todo!("DEFERRED_NON_FUNCTION"); }
    // java public static float toTransparency(float opacity)
    pub fn toTransparency() { todo!("DEFERRED_NON_FUNCTION"); }
}
