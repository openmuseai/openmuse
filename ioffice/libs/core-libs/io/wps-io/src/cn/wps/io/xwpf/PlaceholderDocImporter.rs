use crate::io::xwpf::IImporterBase::IImporterBase;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

/// Null stand-in when a real IDocumentImporter is not available.
pub struct PlaceholderDocImporter;

impl IImporterBase for PlaceholderDocImporter {
    fn mode(&self) -> i32 { -1 }
}

impl IDocumentImporter for PlaceholderDocImporter {
    fn dispose(&self) { }
    fn dispose4StyleXml(&self) { }
    fn onHeaderDocumentEnd(&self, _var1: i32) { }
    fn onFooterDocumentEnd(&self, _var1: i32) { }
    fn onMainDocumentStart(&self, _var1: bool) { }
    fn onMainDocumentEnd(&self) { }
    fn onImportCommentStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportCommentEnd(&self) { }
    fn importCustomXmls(&self) { }
    fn onImportParagraphStart(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportParagraphEnd(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: i32) { }
    fn onImportTableStart(&self, _var1: i32, _var2: i32) { }
    fn onImportHyperlinkBegin(&self, _var1: &str, _var2: i32) { }
    fn onImportHyperlinkEnd(&self, _var1: i32) { }
    fn onImportFldSimpleEnd(&self, _var1: i32) { }
    fn convertProp(&self, _var1: i32, _var2: &mut dyn std::any::Any, _var3: &str, _var4: i32, _var5: &dyn std::any::Any) { }
    fn convertRunProp(&self, _var1: &mut dyn std::any::Any, _var2: &str, _var3: i32, _var4: &dyn std::any::Any) { }
    fn converTableCellProp(&self, _var1: &mut dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any, _var4: i32) { }
    fn correctProps(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: Option<&dyn std::any::Any>) -> Box<dyn std::any::Any + Send + Sync> { Box::new(()) }
    fn onImportRun_chars(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &[char], _var4: i32, _var5: i32) { }
    fn onImportRun_str(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &str) { }
    fn onImportRunContent(&self, _var1: i32, _var2: i32, _var3: &dyn std::any::Any, _var4: &dyn std::any::Any) { }
    fn onImportTableRowStart(&self, _var1: i32, _var2: &str, _var3: &dyn std::any::Any) { }
    fn onImportTableRowEnd(&self, _var1: i32, _var2: i32, _var3: i32, _var4: &dyn std::any::Any, _var5: &dyn std::any::Any) { }
    fn onImportTableCelStart(&self, _var1: i32) { }
    fn onImportTableCellEnd(&self, _var1: i32, _var2: i32) { }
    fn onImportFldSimpleBegin(&self, _var1: &dyn std::any::Any, _var2: i32) { }
    fn onImportFldChar(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: i32) { }
    fn onImportSectProp(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) { }
    fn onImportBookmarkStartEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) { }
    fn onImportCommentRange(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) { }
    fn onImportCommentRef(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: i32) { }
    fn onImportFootnoteEndnoteReference(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any, _var4: i32) { }
    fn onImportFootnoteEndnoteStart(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportBdoDir(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) { }
    fn onImportPdf(&self, _var1: i32) { }
    fn dumpAllPictures(&self) { }
    fn dumpVbaMacro(&self, _var1: &dyn std::any::Any) { }
    fn onImportCommentExStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportFontStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportFontChild(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportFontEnd(&self) { }
    fn onImportSettingsStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportSettingsChild(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportSettingsEnd(&self) { }
    fn onImportStyleDocDefaults(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) { }
    fn onImportStyleStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportStyleChild(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportStyleTblStylePrEnd(&self, _var1: &dyn std::any::Any) { }
    fn convertStyleTcStylePr(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any) { }
    fn onImportStyleEnd(&self, _var1: &dyn std::any::Any) { }
    fn onImportStylesEnd(&self) { }
    fn onImportNumberingNumStart(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportNumberingNumChild(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportNumberingNumEnd(&self) { }
    fn onImportNumberingLevelStart(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) { }
    fn onImportNumberingLevelChild(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any) { }
    fn onImportNumberingLevelEnd(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any) { }
    fn onImportNumberingAbstractNumStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportNumberingAbstractNumChild(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportNumberingAbstractNumEnd(&self) { }
    fn onImportOMathStart(&self, _var1: i32, _var2: bool, _var3: &dyn std::any::Any) { }
    fn onImportOMathEnd(&self, _var1: i32) { }
    fn importMathElementsProp(&self, _var1: i32, _var2: i32, _var3: i32, _var4: &dyn std::any::Any) { }
    fn importMathArgmentEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32, _var4: i32) { }
    fn importMathObjectEnd(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: i32) { }
    fn onImportMathObjectBegin(&self, _var1: i32, _var2: i32) { }
    fn onImportMathArgmentStart(&self, _var1: i32, _var2: i32, _var3: i32) { }
    fn onImportMathMatrixRowEnd(&self, _var1: i32) { }
    fn getVmlImporter(&self, _var1: i32) -> Option<std::rc::Rc<std::cell::RefCell<dyn crate::io::xwpf::IVmlImporter::IVmlImporter>>> { None }
    fn getDmlImporter(&self, _var1: i32) -> Option<crate::io::xwpf::IDmlImporter::DmlImporterRef> { None }
    fn hasContent(&self) -> bool { false }
    fn onImportFontSchemeStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportMajorFontStart(&self) { }
    fn onImportMinorFontStart(&self) { }
    fn onImportThemeCSFontStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportThemeEAFontStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportThemeLatinFontStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportThemeFontStart(&self, _var1: &dyn std::any::Any) { }
    fn onImportMoveRangeStart(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportMoveRangeEnd(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn getDmlTextImporter(&self, _var1: &dyn std::any::Any) -> Box<dyn std::any::Any + Send + Sync> { Box::new(()) }
    fn onImportRuby(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any, _var4: &dyn std::any::Any) { }
    fn setThemePackagePart(&self, _var1: &dyn std::any::Any) { }
    fn onImportMedia(&self, _var1: i32, _var2: &dyn std::any::Any) { }
    fn onImportDocFieldStartEnd(&self, _var1: i32, _var2: &dyn std::any::Any, _var3: i32) { }
}
