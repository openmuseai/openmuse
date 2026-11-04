#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.MathArgmentType
// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.element_handler.ParagraphSimple
// Java: import cn.wps.io.xwpf.element_handler.RubyContent
// Java: import cn.wps.io.xwpf.element_handler.RubyPr
// Java: import cn.wps.io.xwpf.element_handler.SectionHeadFooterProps
// Java: import cn.wps.io.xwpf.element_handler.numbering.NumLvlType
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
// Java: import cn.wps.io.xwpf.element_handler.styles.StylePrModel
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
// Java: import cn.wps.io.xwpf.usermodel.XWPFHeaderFooter
// Java: import cn.wps.io.xwpf.usermodel.XWPFSettings
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public interface IDocumentImporter extends IImporterBase
pub trait IDocumentImporter: crate::io::xwpf::IImporterBase::IImporterBase {
    // Java: abstract void dispose()
    fn dispose(&self) {}

    // Java: abstract void dispose4StyleXml()
    fn dispose4StyleXml(&self) {}

    // Java: abstract void onHeaderDocumentEnd(XWPFHeaderFooter.HdrFtrType var1)
    fn onHeaderDocumentEnd(&self, _var1: i32) {}

    // Java: abstract void onFooterDocumentEnd(XWPFHeaderFooter.HdrFtrType var1)
    fn onFooterDocumentEnd(&self, _var1: i32) {}

    // Java: abstract void onMainDocumentStart(boolean var1)
    fn onMainDocumentStart(&self, _var1: bool) {}

    // Java: abstract void onMainDocumentEnd()
    fn onMainDocumentEnd(&self) {}

    // Java: abstract void onImportCommentStart(Attributes var1)
    fn onImportCommentStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportCommentEnd()
    fn onImportCommentEnd(&self) {}

    // Java: abstract void importCustomXmls()
    fn importCustomXmls(&self) {}

    // Java: abstract void onImportParagraphStart(SubDocType var1, Attributes var2)
    fn onImportParagraphStart(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportParagraphEnd(ParagraphSimple var1, SubDocType var2, int var3)
    fn onImportParagraphEnd(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: i32) {}

    // Java: abstract void onImportTableStart(SubDocType var1, int var2)
    fn onImportTableStart(&self, _var1: i32, _var2: i32) {}

    // Java: abstract void onImportHyperlinkBegin(String var1, SubDocType var2)
    fn onImportHyperlinkBegin(&self, _var1: &str, _var2: i32) {}

    // Java: abstract void onImportHyperlinkEnd(SubDocType var1)
    fn onImportHyperlinkEnd(&self, _var1: i32) {}

    // Java: abstract void onImportFldSimpleEnd(SubDocType var1)
    fn onImportFldSimpleEnd(&self, _var1: i32) {}

    // Java: abstract void convertProp(PropHandlerHelper.PropType var1, MutablePropertySet var2, String var3, int var4, Attributes var5)
    fn convertProp(&self, _var1: i32, _var2: &mut dyn std::any::Any, _var3: &str, _var4: i32, _var5: &dyn std::any::Any) {}

    // Java: abstract void convertRunProp(MutablePropertySet var1, String var2, int var3, Object var4)
    fn convertRunProp(&self, _var1: &mut dyn std::any::Any, _var2: &str, _var3: i32, _var4: &dyn std::any::Any) {}

    // Java: abstract void converTableCellProp(MutablePropertySet var1, int var2, Attributes var3, int var4)
    fn converTableCellProp(&self, _var1: &mut dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any, _var4: i32) {}

    // Java: abstract MutablePropertySet correctProps(int var1, MutablePropertySet var2, MutablePropertySet var3)
    fn correctProps(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: Option<&dyn std::any::Any>) -> Box<dyn std::any::Any + Send + Sync> { Box::new(()) }

    // Java: abstract void onImportRun(SubDocType var1, MutablePropertySet var2, char[] var3, int var4, int var5)
    fn onImportRun_chars(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &[char], _var4: i32, _var5: i32) {}

    // Java: abstract void onImportRun(SubDocType var1, MutablePropertySet var2, String var3)
    fn onImportRun_str(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &str) {}

    // Java: abstract void onImportRunContent(SubDocType var1, int var2, MutablePropertySet var3, Attributes var4)
    fn onImportRunContent(&self, _var1: i32, _var2: i32, _var3: &dyn std::any::Any, _var4: &dyn std::any::Any) {}

    // Java: abstract void onImportTableRowStart(SubDocType var1, String var2, MutablePropertySet var3)
    fn onImportTableRowStart(&self, _var1: i32, _var2: &str, _var3: &dyn std::any::Any) {}

    // Java: abstract void onImportTableRowEnd(SubDocType var1, int var2, int var3, MutablePropertySet var4, MutablePropertySet var5)
    fn onImportTableRowEnd(&self, _var1: i32, _var2: i32, _var3: i32, _var4: &dyn std::any::Any, _var5: &dyn std::any::Any) {}

    // Java: abstract void onImportTableCelStart(SubDocType var1)
    fn onImportTableCelStart(&self, _var1: i32) {}

    // Java: abstract void onImportTableCellEnd(SubDocType var1, int var2)
    fn onImportTableCellEnd(&self, _var1: i32, _var2: i32) {}

    // Java: abstract void onImportFldSimpleBegin(Attributes var1, SubDocType var2)
    fn onImportFldSimpleBegin(&self, _var1: &dyn std::any::Any, _var2: i32) {}

    // Java: abstract void onImportFldChar(Attributes var1, MutablePropertySet var2, SubDocType var3)
    fn onImportFldChar(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: abstract void onImportSectProp(MutablePropertySet var1, SectionHeadFooterProps var2)
    fn onImportSectProp(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportBookmarkStartEnd(int var1, Attributes var2, SubDocType var3)
    fn onImportBookmarkStartEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: abstract void onImportCommentRange(int var1, Attributes var2, SubDocType var3)
    fn onImportCommentRange(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: abstract void onImportCommentRef(Attributes var1, MutablePropertySet var2, SubDocType var3)
    fn onImportCommentRef(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: abstract void onImportFootnoteEndnoteReference(int var1, Attributes var2, MutablePropertySet var3, SubDocType var4)
    fn onImportFootnoteEndnoteReference(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any, _var4: i32) {}

    // Java: abstract void onImportFootnoteEndnoteStart(int var1, Attributes var2)
    fn onImportFootnoteEndnoteStart(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportBdoDir(int var1, Attributes var2, SubDocType var3)
    fn onImportBdoDir(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: abstract void onImportPdf(SubDocType var1)
    fn onImportPdf(&self, _var1: i32) {}

    // Java: abstract void dumpAllPictures()
    fn dumpAllPictures(&self) {}

    // Java: abstract void dumpVbaMacro(XWPFDocument var1)
    fn dumpVbaMacro(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportCommentExStart(Attributes var1)
    fn onImportCommentExStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportFontStart(Attributes var1)
    fn onImportFontStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportFontChild(int var1, Attributes var2)
    fn onImportFontChild(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportFontEnd()
    fn onImportFontEnd(&self) {}

    // Java: abstract void onImportSettingsStart(XWPFSettings var1)
    fn onImportSettingsStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportSettingsChild(int var1, Attributes var2)
    fn onImportSettingsChild(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportSettingsEnd()
    fn onImportSettingsEnd(&self) {}

    // Java: abstract void onImportStyleDocDefaults(MutablePropertySet var1, MutablePropertySet var2)
    fn onImportStyleDocDefaults(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportStyleStart(Attributes var1)
    fn onImportStyleStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportStyleChild(int var1, Attributes var2)
    fn onImportStyleChild(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportStyleTblStylePrEnd(StylePrModel var1)
    fn onImportStyleTblStylePrEnd(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void convertStyleTcStylePr(int var1, Attributes var2, MutablePropertySet var3)
    fn convertStyleTcStylePr(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any) {}

    // Java: abstract void onImportStyleEnd(StylePrModel var1)
    fn onImportStyleEnd(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportStylesEnd()
    fn onImportStylesEnd(&self) {}

    // Java: abstract void onImportNumberingNumStart(int var1, Attributes var2)
    fn onImportNumberingNumStart(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingNumChild(int var1, Attributes var2)
    fn onImportNumberingNumChild(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingNumEnd()
    fn onImportNumberingNumEnd(&self) {}

    // Java: abstract void onImportNumberingLevelStart(NumLvlType var1, Attributes var2)
    fn onImportNumberingLevelStart(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingLevelChild(NumLvlType var1, int var2, Attributes var3)
    fn onImportNumberingLevelChild(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingLevelEnd(NumLvlType var1, MutablePropertySet var2, MutablePropertySet var3)
    fn onImportNumberingLevelEnd(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingAbstractNumStart(Attributes var1)
    fn onImportNumberingAbstractNumStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingAbstractNumChild(int var1, Attributes var2)
    fn onImportNumberingAbstractNumChild(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportNumberingAbstractNumEnd()
    fn onImportNumberingAbstractNumEnd(&self) {}

    // Java: abstract void onImportOMathStart(SubDocType var1, boolean var2, MutablePropertySet var3)
    fn onImportOMathStart(&self, _var1: i32, _var2: bool, _var3: &dyn std::any::Any) {}

    // Java: abstract void onImportOMathEnd(SubDocType var1)
    fn onImportOMathEnd(&self, _var1: i32) {}

    // Java: abstract void importMathElementsProp(SubDocType var1, MathIOType var2, int var3, Attributes var4)
    fn importMathElementsProp(&self, _var1: i32, _var2: i32, _var3: i32, _var4: &dyn std::any::Any) {}

    // Java: abstract void importMathArgmentEnd(SubDocType var1, MutablePropertySet var2, MathIOType var3, MathArgmentType var4)
    fn importMathArgmentEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32, _var4: i32) {}

    // Java: abstract void importMathObjectEnd(MutablePropertySet var1, MathIOType var2, SubDocType var3)
    fn importMathObjectEnd(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: i32) {}

    // Java: abstract void onImportMathObjectBegin(SubDocType var1, MathIOType var2)
    fn onImportMathObjectBegin(&self, _var1: i32, _var2: i32) {}

    // Java: abstract void onImportMathArgmentStart(SubDocType var1, MathIOType var2, MathArgmentType var3)
    fn onImportMathArgmentStart(&self, _var1: i32, _var2: i32, _var3: i32) {}

    // Java: abstract void onImportMathMatrixRowEnd(SubDocType var1)
    fn onImportMathMatrixRowEnd(&self, _var1: i32) {}

    // Java: abstract IVmlImporter getVmlImporter(SubDocType var1)
    fn getVmlImporter(&self, _var1: i32) -> Option<std::rc::Rc<std::cell::RefCell<dyn crate::io::xwpf::IVmlImporter::IVmlImporter>>> { None }

    // Java: abstract IDmlImporter getDmlImporter(SubDocType var1)
    fn getDmlImporter(&self, _var1: i32) -> Option<crate::io::xwpf::IDmlImporter::DmlImporterRef> { None }

    // Java: abstract boolean hasContent()
    fn hasContent(&self) -> bool { false }

    // Java: abstract void onImportFontSchemeStart(Attributes var1)
    fn onImportFontSchemeStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportMajorFontStart()
    fn onImportMajorFontStart(&self) {}

    // Java: abstract void onImportMinorFontStart()
    fn onImportMinorFontStart(&self) {}

    // Java: abstract void onImportThemeCSFontStart(Attributes var1)
    fn onImportThemeCSFontStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportThemeEAFontStart(Attributes var1)
    fn onImportThemeEAFontStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportThemeLatinFontStart(Attributes var1)
    fn onImportThemeLatinFontStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportThemeFontStart(Attributes var1)
    fn onImportThemeFontStart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportMoveRangeStart(int var1, Attributes var2)
    fn onImportMoveRangeStart(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportMoveRangeEnd(int var1, Attributes var2)
    fn onImportMoveRangeEnd(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract IDmlTextImporter getDmlTextImporter(MutablePropertySet var1)
    fn getDmlTextImporter(&self, _var1: &dyn std::any::Any) -> Box<dyn std::any::Any + Send + Sync> { Box::new(()) }

    // Java: abstract void onImportRuby(SubDocType var1, RubyPr var2, RubyContent var3, RubyContent var4)
    fn onImportRuby(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any, _var4: &dyn std::any::Any) {}

    // Java: abstract void setThemePackagePart(POIXMLDocumentPart var1)
    fn setThemePackagePart(&self, _var1: &dyn std::any::Any) {}

    // Java: abstract void onImportMedia(SubDocType var1, Attributes var2)
    fn onImportMedia(&self, _var1: i32, _var2: &dyn std::any::Any) {}

    // Java: abstract void onImportDocFieldStartEnd(int var1, Attributes var2, SubDocType var3)
    fn onImportDocFieldStartEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) {}

    // Java: boolean isLock()
    fn isLock(&self) -> bool {
        false
    }
}
