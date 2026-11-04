// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.drawing.GroupShape
use drawing::cn::wps::moffice::drawing::GroupShape::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KParagraphsID
use crate::cn::wps::moffice::writer::core::KParagraphsID::*;
// Java: import cn.wps.moffice.writer.core.KTableRevision
use crate::cn::wps::moffice::writer::core::KTableRevision::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.command.CTableResizeCmd
use crate::cn::wps::moffice::writer::core::command::CTableResizeCmd::*;
// Java: import cn.wps.moffice.writer.core.command.CTableRowModifyCmd
use crate::cn::wps::moffice::writer::core::command::CTableRowModifyCmd::*;
// Java: import cn.wps.moffice.writer.core.command.CTableRowModifyCmd_Section
use crate::cn::wps::moffice::writer::core::command::CTableRowModifyCmd_Section::*;
// Java: import cn.wps.moffice.writer.core.shape.ShapeMetrics
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::*;
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
use crate::cn::wps::moffice::writer::core::shape::ShapeTool::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentTxt
use crate::cn::wps::moffice::writer::data::PLCCommentTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
use crate::cn::wps::moffice::writer::data::PLCShapeAnchor::*;
// Java: import cn.wps.moffice.writer.data.PLCTextboxText
use crate::cn::wps::moffice::writer::data::PLCTextboxText::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.DxaColumn
use crate::cn::wps::moffice::writer::data::value::DxaColumn::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::*;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::*;
// Java: import cn.wps.moffice.writer.data.value.TableLookBuilder
use crate::cn::wps::moffice::writer::data::value::TableLookBuilder::*;
// Java: import cn.wps.moffice.writer.drawing.TextFrame
use crate::cn::wps::moffice::writer::drawing::TextFrame::*;
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.table.KTablePropertyRange
// CARGO_BLOCKER: KTablePropertyRange not generated
// use crate::cn::wps::moffice::writer::core::table::KTablePropertyRange::*;

// java public class Tools
#[derive(Default)]
pub struct Tools {
}

// java public class Tools
impl Tools {
    // java public static PropertySet getDefaultCellPropertySet(Document document, int tableDeep, PropertySet paraProp)
    pub fn getDefaultCellPropertySet__document_int_propertyset(document: &dyn Document, tableDeep: i32, paraProp: &PropertySet) -> PropertySet {
      // java return getDefaultCellPropertySet(document, tableDeep, paraProp, false);
      return Self::getDefaultCellPropertySet__document_int_propertyset_boolean(&document, tableDeep, &paraProp, false);
    }

    // java public static PropertySet getDefaultCellPropertySet(Document document, int tableDeep, PropertySet paraProp, boolean fRemoveProperty)
    pub fn getDefaultCellPropertySet__document_int_propertyset_boolean(document: &dyn Document, tableDeep: i32, paraProp: &PropertySet, fRemoveProperty: bool) -> PropertySet {
      // java MutablePropertySet propertySet = new MutablePropertySet();
      let mut propertySet: MutablePropertySet = MutablePropertySet::new();
      // java propertySet.add(PropertyUtil.getFinalNormalParageraphFormat(paraProp));
      propertySet.add(PropertyUtil::getFinalNormalParageraphFormat(&paraProp));
      // java propertySet.setValue(223, true);
      propertySet.setValue(223, true);
      // java propertySet.setValue(224, tableDeep);
      propertySet.setValue(224, tableDeep);
      // java propertySet.setValue(226, true);
      propertySet.setValue(226, true);
      // java if (fRemoveProperty)
      if fRemoveProperty {
        // java PropertyTool propertyTool = new PropertyTool(document);
        let mut propertyTool: PropertyTool = PropertyTool::new(document);
        // java MutablePropertySet finalProp = new MutablePropertySet();
        let mut finalProp: MutablePropertySet = MutablePropertySet::new();
        // java propertyTool.getMergedParaProperty(finalProp, paraProp, true, true);
        propertyTool.getMergedParaProperty(finalProp, paraProp, true, true);
        // java if (finalProp.contains(208))
        if finalProp.contains(208) {
          // java propertySet.setValue(208, 0);
          propertySet.setValue(208, 0);
        }
        // java if (finalProp.contains(209))
        if finalProp.contains(209) {
          // java propertySet.setValue(209, 0);
          propertySet.setValue(209, 0);
        }
        // java if (finalProp.contains(210))
        if finalProp.contains(210) {
          // java propertySet.setValue(210, 0);
          propertySet.setValue(210, 0);
        }
        // java if (finalProp.contains(212))
        if finalProp.contains(212) {
          // java propertySet.setValue(212, 0);
          propertySet.setValue(212, 0);
        }
        // java if (finalProp.contains(213))
        if finalProp.contains(213) {
          // java propertySet.setValue(213, 0);
          propertySet.setValue(213, 0);
        }
        // java if (finalProp.contains(231))
        if finalProp.contains(231) {
          // java propertySet.setValue(231, DefaultValue.paragraph_borderBottom);
          propertySet.setValue(231, DefaultValue::paragraph_borderBottom);
        }
        // java if (finalProp.contains(228))
        if finalProp.contains(228) {
          // java propertySet.setValue(228, DefaultValue.paragraph_shd);
          propertySet.setValue(228, DefaultValue::paragraph_shd);
        }
      }
      // java return propertySet.freeze();
      return propertySet.freeze();
    }

    // java public static PropertySet getFinalChpx(PropertySet chpx)
    pub fn getFinalChpx(chpx: &PropertySet) -> PropertySet {
      // java MutablePropertySet propertySet = new MutablePropertySet();
      let mut propertySet: MutablePropertySet = MutablePropertySet::new();
      // java propertySet.add(PropertyUtil.getFinalNormalRunFormat(chpx));
      propertySet.add(PropertyUtil::getFinalNormalRunFormat(&chpx));
      // java if (propertySet.getInt(2, 0) == 4095)
      if propertySet.getInt(2, 0) == 4095 {
        // java propertySet.remove(2);
        propertySet.remove(2);
      }
      // java return propertySet.freeze();
      return propertySet.freeze();
    }

    // java public static PropertySet getDefaultRowPropertySet(int tableDeep, DefTable defTable, int styleID, Boolean bidi)
    pub fn getDefaultRowPropertySet(tableDeep: i32, defTable: &DefTable, styleID: i32, bidi: bool) -> PropertySet {
      // java MutablePropertySet mPropertySet = new MutablePropertySet();
      let mut mPropertySet: MutablePropertySet = MutablePropertySet::new();
      // java mPropertySet.setValue(223, true);
      mPropertySet.setValue(223, true);
      // java if (1 != tableDeep)
      if 1 != tableDeep {
        // java mPropertySet.setValue(226, true);
        mPropertySet.setValue(226, true);
      }
      // java mPropertySet.setValue(225, true);
      mPropertySet.setValue(225, true);
      // java mPropertySet.setValue(224, tableDeep);
      mPropertySet.setValue(224, tableDeep);
      // java mPropertySet.setValue(301, DefaultValue.table_defCellMargin);
      mPropertySet.setValue(301, DefaultValue::table_defCellMargin);
      // java mPropertySet.setValue(310, DefaultValue.table_borders);
      mPropertySet.setValue(310, DefaultValue::table_borders);
      // java mPropertySet.setValue(302, DefaultValue.table_fit);
      mPropertySet.setValue(302, DefaultValue::table_fit);
      // java mPropertySet.setValue(292, styleID);
      mPropertySet.setValue(292, styleID);
      // java mPropertySet.setValue(342, new TableLookBuilder());
      mPropertySet.setValue(342, TableLookBuilder::new());
      // java mPropertySet.setValue(306, defTable);
      mPropertySet.setValue(306, defTable);
      // java int cols = defTable.getCellCount();
      let mut cols: i32 = defTable.getCellCount();
      // java int[] colorRefs = new int[cols];
      let mut colorRefs: Vec<i32> = vec![Default::default(); (cols) as usize];
      let mut i: i32 = 0;
      // java for(int i = 0; i < cols; ++i)
      while i < cols {
        // java colorRefs[i] = -16777216;
        colorRefs[(i) as usize] = -16777216;
      }
      // java mPropertySet.setValue(323, colorRefs);
      mPropertySet.setValue(323, colorRefs);
      // java mPropertySet.setValue(324, colorRefs);
      mPropertySet.setValue(324, colorRefs);
      // java mPropertySet.setValue(325, colorRefs);
      mPropertySet.setValue(325, colorRefs);
      // java mPropertySet.setValue(326, colorRefs);
      mPropertySet.setValue(326, colorRefs);
      // java mPropertySet.setValue(343, bidi);
      mPropertySet.setValue(343, bidi);
      // java return mPropertySet.freeze();
      return mPropertySet.freeze();
    }

    // java public static void createCell(Document document, int insertPos, PropertySet chpx, PropertySet papx)
    pub fn createCell(document: &dyn Document, insertPos: i32, chpx: &PropertySet, papx: &PropertySet) {
      // java char c = (char)(1 == papx.getInt(224) ? 7 : 13);
      let mut c: char = todo!("cast → char");
      // java document.getTextStream().insertParagraph(insertPos, c, chpx, papx);
      document.getTextStream().insertParagraph(insertPos, c, chpx, papx);
    }

    // java public static void addParaId2Map(Document document, int cp, PropertySet papxPropertySet)
    pub fn addParaId2Map(document: &dyn Document, cp: i32, papxPropertySet: &PropertySet) {
      // java if (papxPropertySet.contains(282))
      if papxPropertySet.contains(282) {
        // java String paraId = papxPropertySet.getString(282);
        let mut paraId: String = papxPropertySet.getString__int(282);
        // java KParagraphsID kParagraphsID = document.getTextDocument().getKParagraphsID();
        let mut kParagraphsID: KParagraphsID = document.getTextDocument().getKParagraphsID();
        // java IPapxTable.Entry paraEntry = document.getTextStream().seekPapxTable(cp);
        let mut paraEntry: Box<dyn Entry> = document.getTextStream().seekPapxTable(cp);
        // java kParagraphsID.putId(paraId, document.getType(), paraEntry);
        kParagraphsID.putId(paraId, document.getType(), paraEntry);
      }
    }

    // java public static PropertySet createParaId(PropertySet papxPropertySet, Document document)
    pub fn createParaId(papxPropertySet: &PropertySet, document: &dyn Document) -> PropertySet {
      // java KParagraphsID kParagraphsID = document.getTextDocument().getKParagraphsID();
      let mut kParagraphsID: KParagraphsID = document.getTextDocument().getKParagraphsID();
      // java String id = kParagraphsID.createId();
      let mut id: String = kParagraphsID.createId();
      // java MutablePropertySet mutablePropertySet = new MutablePropertySet(papxPropertySet);
      let mut mutablePropertySet: MutablePropertySet = MutablePropertySet::new(papxPropertySet);
      // java mutablePropertySet.setValue(282, id);
      mutablePropertySet.setValue(282, id);
      // java return mutablePropertySet.freeze();
      return mutablePropertySet.freeze();
    }

    // java public static void insertParaBefore(Document document, int cp, char c, PropertySet prop)
    pub fn insertParaBefore__document_int_char_propertyset(document: &dyn Document, cp: i32, c: char, prop: &PropertySet) {
      // java IPapxTable.Entry pNextPara = document.getPapxTable().seek(cp);
      let mut pNextPara: Box<dyn Entry> = document.getPapxTable().seek(cp);
      // java document.getTextStream().insertParagraph(cp, c, PropertySet.empty, pNextPara.getProperty());
      document.getTextStream().insertParagraph(cp, c, PropertySet::empty(), pNextPara.getProperty());
      // java IPapxTable.Entry pPrevPara = document.getPapxTable().seek(cp);
      let mut pPrevPara: Box<dyn Entry> = document.getPapxTable().seek(cp);
      // java pPrevPara.setProperty(prop);
      pPrevPara.setProperty(prop);
    }

    // java public static void insertParaBefore(Document document, int cp, char c, PropertySet runProp, PropertySet paraProp)
    pub fn insertParaBefore__document_int_char_propertyset_propertyset(document: &dyn Document, cp: i32, c: char, runProp: &PropertySet, paraProp: &PropertySet) {
      // java IPapxTable.Entry pNextPara = document.getPapxTable().seek(cp);
      let mut pNextPara: Box<dyn Entry> = document.getPapxTable().seek(cp);
      // java document.getTextStream().insertParagraph(cp, c, runProp, pNextPara.getProperty());
      document.getTextStream().insertParagraph(cp, c, runProp, pNextPara.getProperty());
      // java IPapxTable.Entry pPrevPara = document.getPapxTable().seek(cp);
      let mut pPrevPara: Box<dyn Entry> = document.getPapxTable().seek(cp);
      // java pPrevPara.setProperty(paraProp);
      pPrevPara.setProperty(paraProp);
    }

    // java public static int getMainCp(Document document, int cp)
    pub fn getMainCp(document: &dyn Document, cp: i32) -> i32 {
      // java switch (document.getType())
      match document.getType() {
        // java case 0:
        0 => {
          // java return cp;
          return cp;
        }
        // java case 1:
        1 => {
          // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode = (PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode)document.getFootnoteTxt().findFloor(cp);
          let mut txtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = (document.getFootnoteTxt().findFloor(cp)).downcast_ref::<PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode>().unwrap();
          // java if (null != txtNode)
          if txtNode.is_some() {
            // java return txtNode.getFootnoteEndnoteRef().getCP();
            return txtNode.getFootnoteEndnoteRef().getCP();
          }
        }
        // java case 2:
        2 => {
          // java PLCSection.SectionNode sectionNode = document.getPLCHeader().getSection(cp);
          let mut sectionNode: PLCSection::SectionNode = document.getPLCHeader().getSection(cp);
          // java return document.getMainDocument().getPLCSection().getSectionStart(sectionNode);
          return document.getMainDocument().getPLCSection().getSectionStart(sectionNode);
        }
        // java case 3:
        3 => {
          // java PLCCommentTxt.CommentTxtNode commentTxt = (PLCCommentTxt.CommentTxtNode)document.getCommentTxt().findFloor(cp);
          let mut commentTxt: PLCCommentTxt::CommentTxtNode = (document.getCommentTxt().findFloor(cp)).downcast_ref::<PLCCommentTxt::CommentTxtNode>().unwrap();
          // java if (null != commentTxt)
          if commentTxt.is_some() {
            // java return commentTxt.getCommentRef().getCommentStart().getCP();
            return commentTxt.getCommentRef().getCommentStart().getCP();
          }
        }
        // java case 4:
        4 => {
          // java txtNode = (PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode)document.getEndnoteTxt().findFloor(cp);
          txtNode = (document.getEndnoteTxt().findFloor(cp)).downcast_ref::<PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode>().unwrap();
          // java if (null != txtNode)
          if txtNode.is_some() {
            // java return txtNode.getFootnoteEndnoteRef().getCP();
            return txtNode.getFootnoteEndnoteRef().getCP();
          }
        }
        // java case 5:
        5 => {
          // java PLCTextboxText.TextboxTextNode textNode = (PLCTextboxText.TextboxTextNode)document.getTextboxText().findFloor(cp);
          let mut textNode: PLCTextboxText::TextboxTextNode = (document.getTextboxText().findFloor(cp)).downcast_ref::<PLCTextboxText::TextboxTextNode>().unwrap();
          // java if (null != textNode)
          if textNode.is_some() {
            // java PLCShapeAnchor.ShapeAnchor shapeAnchor = document.getMainDocument().getShapeAnchors().findById(textNode.getShapeId());
            let mut shapeAnchor: PLCShapeAnchor::ShapeAnchor = document.getMainDocument().getShapeAnchors().findById(textNode.getShapeId());
            // java if (null != shapeAnchor)
            if shapeAnchor.is_some() {
              // java return shapeAnchor.position();
              return shapeAnchor.position();
            }
          }
        }
        // java case 6:
        6 => {
          // java textNode = (PLCTextboxText.TextboxTextNode)document.getTextboxText().findFloor(cp);
          textNode = (document.getTextboxText().findFloor(cp)).downcast_ref::<PLCTextboxText::TextboxTextNode>().unwrap();
          // java if (null != textNode)
          if textNode.is_some() {
            // java Document headerDoc = document.getTextDocument().getSubdocument(2);
            let mut headerDoc: Box<dyn Document> = document.getTextDocument().getSubdocument(2);
            // java PLCShapeAnchor.ShapeAnchor shapeAnchor = headerDoc.getShapeAnchors().findById(textNode.getShapeId());
            let mut shapeAnchor: PLCShapeAnchor::ShapeAnchor = headerDoc.getShapeAnchors().findById(textNode.getShapeId());
            // java if (null != shapeAnchor)
            if shapeAnchor.is_some() {
              // java return getMainCp(headerDoc, shapeAnchor.position());
              return Self::getMainCp(&headerDoc, shapeAnchor.position());
            }
          }
        }
        // java default:
        _ => {
        }
      }
      // java return 0;
      return 0;
    }

    // java public static PageSetup getPageSetup(Document document, int cp)
    pub fn getPageSetup(document: &dyn Document, cp: i32) -> PageSetup {
      // java return document.getTextDocument().getSectionProperty().getPageSetup(getMainCp(document, cp));
      return document.getTextDocument().getSectionProperty().getPageSetup(Self::getMainCp(&document, cp));
    }

    // java public static int getMainDocumentCoreWidth(Document document, int cp)
    pub fn getMainDocumentCoreWidth(document: &dyn Document, cp: i32) -> i32 {
      // java cp = getMainCp(document, cp);
      cp = Self::getMainCp(&document, cp);
      // java PageSetup pageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      let mut pageSetup: PageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      // java int coreWidth = pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight();
      let mut coreWidth: i32 = pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight();
      // java if (0 == document.getType())
      if 0 == document.getType() {
        // java PLCSection sections = document.getPLCSection();
        let mut sections: PLCSection = document.getPLCSection();
        // java if (null != sections)
        if sections.is_some() {
          // java PLCSection.SectionNode sectionNode = sections.getSection(cp);
          let mut sectionNode: PLCSection::SectionNode = sections.getSection(cp);
          // java if (null != sectionNode)
          if sectionNode.is_some() {
            // java PropertySet secPropSet = sectionNode.sepx;
            let mut secPropSet: PropertySet = sectionNode.sepx;
            // java Revision revision = (Revision)secPropSet.getValue(727);
            let mut revision: Revision = (secPropSet.getValue(727)).downcast_ref::<Revision>().unwrap();
            // java if (null != revision)
            if revision.is_some() {
              // java secPropSet = PropertySet.merge(secPropSet, revision.getProperty());
              secPropSet = PropertySet::merge(&secPropSet, &revision.getProperty());
            }
            // java if (secPropSet.getInt(680, 0) > 0)
            if secPropSet.getInt(680, 0) > 0 {
              // java coreWidth = getMinColumnWidth(coreWidth, secPropSet);
              coreWidth = Self::getMinColumnWidth(coreWidth, &secPropSet);
            }
          }
        }
      }
      // java return coreWidth;
      return coreWidth;
    }

    // java private static int getMinColumnWidth(int coreWidth, PropertySet secPropSet)
    pub fn getMinColumnWidth(coreWidth: i32, secPropSet: &PropertySet) -> i32 {
      // java DxaColumn[] dxaColumns = (DxaColumn[])secPropSet.getValue(721);
      let mut dxaColumns: Vec<DxaColumn> = todo!("cast → Vec<DxaColumn>");
      // java int columns = secPropSet.getInt(680, 0) + 1;
      let mut columns: i32 = secPropSet.getInt__int_int(680, 0) + 1;
      // java if (null != dxaColumns)
      if dxaColumns.is_some() {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = (dxaColumns.len() as i32);
        // java for(int size = dxaColumns.length; i < size; ++i)
        while i < size {
          // java coreWidth = Math.min(coreWidth, dxaColumns[i].getWidth());
          coreWidth = Math::min(coreWidth, dxaColumns[(i) as usize].getWidth());
        }
      }
      // java else
      else {
        // java int spaceBeforeColumns = secPropSet.getInt(692, 720);
        let mut spaceBeforeColumns: i32 = secPropSet.getInt__int_int(692, 720);
        // java coreWidth = (coreWidth - spaceBeforeColumns * (columns - 1)) / columns;
        coreWidth = coreWidth - spaceBeforeColumns * columns - 1 / columns;
      }
      // java return coreWidth;
      return coreWidth;
    }

    // java public static int getTextBoxCoreWidth(Document document, int cp)
    pub fn getTextBoxCoreWidth(document: &dyn Document, cp: i32) -> Option<i32> {
      // java PLCTextboxText.TextboxTextNode textboxTextNode = document.getTextboxText().findTextboxByCP(cp);
      let mut textboxTextNode: PLCTextboxText::TextboxTextNode = document.getTextboxText().findTextboxByCP(cp);
      // java if (textboxTextNode == null)
      if textboxTextNode.is_none() {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int shapeID = textboxTextNode.getShapeId();
        let mut shapeID: i32 = textboxTextNode.getShapeId();
        // java Document anchorDocument;
        let mut anchorDocument: Box<dyn Document> = Box::new(()) as Box<dyn Document>;
        // java if (document.getType() == 5)
        if document.getType() == 5 {
          // java anchorDocument = document.getMainDocument();
          anchorDocument = document.getMainDocument();
        }
        // java else
        else {
          // java anchorDocument = document.getTextDocument().getSubdocument(2);
          anchorDocument = document.getTextDocument().getSubdocument(2);
        }
        // java Shape shape = anchorDocument.getDrawingContainer().findShapeByID(shapeID);
        let mut shape: Shape = anchorDocument.getDrawingContainer().findShapeByID(shapeID);
        // java TextFrame textbox = (TextFrame)shape.getTextObj();
        let mut textbox: TextFrame = (shape.getTextObj()).downcast_ref::<TextFrame>().unwrap();
        // java float textMargin = textbox != null ? textbox.getMarginLeft() + textbox.getMarginRight() : 0.0F;
        let mut textMargin: f32 = (if textbox.is_some() { textbox.getMarginLeft() + textbox.getMarginRight() } else { 0 });
        // java return ShapeMetrics.transToLayout(getShapeWidth(shape) - textMargin);
        return ShapeMetrics::transToLayout__float(Self::getShapeWidth(&shape) - textMargin);
      }
    }

    // java public static int getMainDocumentMinLeft(Document document, int cp)
    pub fn getMainDocumentMinLeft(document: &dyn Document, cp: i32) -> i32 {
      // java PageSetup pageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      let mut pageSetup: PageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      // java return -pageSetup.getMarginLeft();
      return -pageSetup.getMarginLeft();
    }

    // java public static int getMainDocumentMaxRight(Document document, int cp)
    pub fn getMainDocumentMaxRight(document: &dyn Document, cp: i32) -> i32 {
      // java PageSetup pageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      let mut pageSetup: PageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      // java return pageSetup.getWidth() - pageSetup.getMarginLeft();
      return pageSetup.getWidth() - pageSetup.getMarginLeft();
    }

    // java static boolean checkTrackChange(Document document)
    pub fn checkTrackChange(document: &dyn Document) -> bool {
      // java return document.getTextDocument().getTrackChange();
      return document.getTextDocument().getTrackChange();
    }

    // java static boolean checkTableFormatChange(Document document, CellsItem cellItem)
    pub fn checkTableFormatChange(document: &dyn Document, cellItem: &CellsItem) -> bool {
      // java PropertySet originalRowProp = cellItem.getRowProperty();
      let mut originalRowProp: PropertySet = cellItem.getRowProperty();
      // java return originalRowProp.contains(363) ? true : document.getTextDocument().getTrackChange();
      return (if originalRowProp.contains(363) { true } else { document.getTextDocument().getTrackChange() });
    }

    // java static void updateRevision(Document document, PropertySet originalRowProp, PropertySet rowRevisionProp, long rowRange)
    pub fn updateRevision(document: &dyn Document, originalRowProp: &PropertySet, rowRevisionProp: &PropertySet, rowRange: i64) {
      // java int[] ids = new int[rowRevisionProp.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (rowRevisionProp.size()) as usize];
      // java rowRevisionProp.getAll(ids, (Object[])null);
      rowRevisionProp.getAll(Some(&mut ids), None);
      // java for(int id : ids)
      for id in ids.iter() {
        // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(document, rowRange, id, originalRowProp.getValue(id), rowRevisionProp.getValue(id));
        let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(document, rowRange, id, originalRowProp.getValue(id), rowRevisionProp.getValue(id));
        // java cmd.setModifyRevision(false);
        cmd.setModifyRevision(false);
        // java cmd.execute();
        cmd.execute();
        // java document.getTextDocument().submit(cmd);
        document.getTextDocument().submit(cmd);
      }
    }

    // java public static PropertySet getFinalProperty(PropertySet originalRowProp)
    pub fn getFinalProperty(originalRowProp: &PropertySet) -> PropertySet {
      // java Revision revision = (Revision)originalRowProp.getValue(363);
      let mut revision: Revision = (originalRowProp.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java if (null == revision)
      if revision.is_none() {
        // java return originalRowProp;
        return originalRowProp;
      }
      // java else
      else {
        // java PropertySet revisionProp = revision.getProperty();
        let mut revisionProp: PropertySet = revision.getProperty();
        // java return PropertySet.merge(originalRowProp, revisionProp);
        return PropertySet::merge(&originalRowProp, &revisionProp);
      }
    }

    // java static CTableRowModifyCmd submitCMD(Document document, long range, int id, Object propOld, Object propNew)
    pub fn submitCMD(document: &dyn Document, range: i64, id: i32, propOld: &dyn Object, propNew: &dyn Object) -> CTableRowModifyCmd {
      // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(document, range, id, propOld, propNew);
      let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(document, range, id, propOld, propNew);
      // java cmd.setModifyRevision(true);
      cmd.setModifyRevision(true);
      // java cmd.execute();
      cmd.execute();
      // java document.getTextDocument().submit(cmd);
      document.getTextDocument().submit(cmd);
      // java return cmd;
      return cmd;
    }

    // java static CTableRowModifyCmd submitCMD_section(Document document, long range, int id, Object propOld, Object propNew)
    pub fn submitCMD_section(document: &dyn Document, range: i64, id: i32, propOld: &dyn Object, propNew: &dyn Object) -> CTableRowModifyCmd {
      // java CTableRowModifyCmd cmd = new CTableRowModifyCmd_Section(document, range, id, propOld, propNew);
      let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd_Section::new(document, range, id, propOld, propNew);
      // java cmd.setModifyRevision(true);
      cmd.setModifyRevision(true);
      // java cmd.execute();
      cmd.execute();
      // java document.getTextDocument().submit(cmd);
      document.getTextDocument().submit(cmd);
      // java return cmd;
      return cmd;
    }

    // java public static void requestTableResize(Document document, long range)
    pub fn requestTableResize(document: &dyn Document, range: i64) {
      // java CTableResizeCmd cmd = new CTableResizeCmd(document, range);
      let mut cmd: CTableResizeCmd = CTableResizeCmd::new(document, range);
      // java cmd.execute();
      cmd.execute();
      // java document.getTextDocument().submit(cmd);
      document.getTextDocument().submit(cmd);
    }

    // java public static void setDefTable(Document document, CellsItem cellItem, DefTable oldDefTable, DefTable newDefTable)
    pub fn setDefTable(document: &dyn Document, cellItem: &CellsItem, oldDefTable: &DefTable, newDefTable: &DefTable) {
      // java if (checkTrackChange(document))
      if Self::checkTrackChange(&document) {
        // java setDefTableInRevision(document, cellItem, newDefTable);
        Self::setDefTableInRevision(&document, &cellItem, &newDefTable);
      }
      // java else
      else {
        // java submitCMD(document, cellItem.rowRange, 306, oldDefTable, newDefTable);
        Self::submitCMD(&document, cellItem.rowRange, 306, &oldDefTable, &newDefTable);
      }
    }

    // java private static void setDefTableInRevision(Document document, CellsItem cellItem, DefTable newDefTable)
    pub fn setDefTableInRevision(document: &dyn Document, cellItem: &CellsItem, newDefTable: &DefTable) {
      // java KTableRevision kTableRevision = new KTableRevision(document);
      let mut kTableRevision: KTableRevision = KTableRevision::new(document);
      // java PropertySet originalRowProp = cellItem.getRowProperty();
      let mut originalRowProp: PropertySet = cellItem.getRowProperty();
      // java PropertySet rowRevisionProp = kTableRevision.setTableRevision(originalRowProp);
      let mut rowRevisionProp: PropertySet = kTableRevision.setTableRevision(originalRowProp);
      // java updateRevision(document, originalRowProp, rowRevisionProp, cellItem.rowRange);
      Self::updateRevision(&document, &originalRowProp, &rowRevisionProp, cellItem.rowRange);
      // java Revision revision = (Revision)rowRevisionProp.getValue(363);
      let mut revision: Revision = (rowRevisionProp.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java submitCMD(document, cellItem.rowRange, 306, revision.getProperty().getValue(306), newDefTable);
      Self::submitCMD(&document, cellItem.rowRange, 306, &revision.getProperty().getValue(306), &newDefTable);
    }

    // java static void setDefSHD(TextDocument textDocument, Document document, CellsItem cellItem, TableDefSHD newDefShd, TableDefSHD oldDefShd)
    pub fn setDefSHD(textDocument: &TextDocument, document: &dyn Document, cellItem: &CellsItem, newDefShd: &TableDefSHD, oldDefShd: &TableDefSHD) {
      // java if (checkTrackChange(document))
      if Self::checkTrackChange(&document) {
        // java setDefSHDInRevision(document, cellItem, newDefShd);
        Self::setDefSHDInRevision(&document, &cellItem, &newDefShd);
      }
      // java else
      else {
        // java setDefSHDInNormal(textDocument, document, cellItem.rowRange, newDefShd, oldDefShd);
        Self::setDefSHDInNormal(&mut textDocument, &document, cellItem.rowRange, &newDefShd, &oldDefShd);
      }
    }

    // java private static void setDefSHDInNormal(TextDocument textDocument, Document document, long rowRange, TableDefSHD newDefShd, TableDefSHD oldDefShd)
    pub fn setDefSHDInNormal(textDocument: &mut TextDocument, document: &dyn Document, rowRange: i64, newDefShd: &TableDefSHD, oldDefShd: &TableDefSHD) {
      // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(document, rowRange, 308, oldDefShd, newDefShd);
      let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(document, rowRange, 308, oldDefShd, newDefShd);
      // java cmd.setModifyRevision(true);
      cmd.setModifyRevision(true);
      // java cmd.execute();
      cmd.execute();
      // java textDocument.submit(cmd);
      textDocument.submit(&cmd);
    }

    // java private static void setDefSHDInRevision(Document document, CellsItem cellItem, TableDefSHD newDefShd)
    pub fn setDefSHDInRevision(document: &dyn Document, cellItem: &CellsItem, newDefShd: &TableDefSHD) {
      // java KTableRevision kTableRevision = new KTableRevision(document);
      let mut kTableRevision: KTableRevision = KTableRevision::new(document);
      // java PropertySet originalRowProp = cellItem.getRowProperty();
      let mut originalRowProp: PropertySet = cellItem.getRowProperty();
      // java PropertySet rowRevisionProp = kTableRevision.setTableRevision(originalRowProp);
      let mut rowRevisionProp: PropertySet = kTableRevision.setTableRevision(originalRowProp);
      // java MutablePropertySet mutablePropertySet = new MutablePropertySet(rowRevisionProp);
      let mut mutablePropertySet: MutablePropertySet = MutablePropertySet::new(rowRevisionProp);
      // java if (!rowRevisionProp.contains(306))
      if !rowRevisionProp.contains(306) {
        // java mutablePropertySet.setValue(306, cellItem.getRowProperty().getValue(306));
        mutablePropertySet.setValue(306, cellItem.getRowProperty().getValue(306));
      }
      // java updateRevision(document, originalRowProp, mutablePropertySet.get(), cellItem.rowRange);
      Self::updateRevision(&document, &originalRowProp, &mutablePropertySet.get(), cellItem.rowRange);
      // java Revision revision = (Revision)rowRevisionProp.getValue(363);
      let mut revision: Revision = (rowRevisionProp.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java submitCMD(document, cellItem.rowRange, 308, revision.getProperty().getValue(308), newDefShd);
      Self::submitCMD(&document, cellItem.rowRange, 308, &revision.getProperty().getValue(308), &newDefShd);
    }

    // java public static Object getPropFromRowProp(PropertySet rowProp, int id)
    pub fn getPropFromRowProp(rowProp: &PropertySet, id: i32) -> Option<Box<dyn Object>> {
      // java Assert.assertNotNull("rowProp should not be null!", rowProp);
      Assert::assertNotNull__string_object(&"rowProp should not be null!", Some(&rowProp));
      // java Revision revision = (Revision)rowProp.getValue(363);
      let mut revision: Revision = (rowProp.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java if (null == revision)
      if revision.is_none() {
        // java return rowProp.getValue(id);
        return rowProp.getValue(id);
      }
      // java else
      else {
        // java PropertySet revisionProp = revision.getProperty();
        let mut revisionProp: PropertySet = revision.getProperty();
        // java Object revisionValue = revisionProp.getValue(id);
        let mut revisionValue: Box<dyn Object> = revisionProp.getValue(id);
        // java return null == revisionValue ? rowProp.getValue(id) : revisionValue;
        return (if revisionValue.is_none() { Some(rowProp.getValue(id)) } else { Some(revisionValue) });
      }
    }

    // java public static void fixTableWidth(Document document, TableInfo tableInfo, boolean checkOverflowWidth)
    pub fn fixTableWidth(document: &dyn Document, tableInfo: &dyn TableInfo, checkOverflowWidth: bool) {
      // java PropertySet finalProp = tableInfo.getTableProperty();
      let mut finalProp: PropertySet = tableInfo.getTableProperty();
      // java TableLength newTableWidth = null;
      let mut newTableWidth: TableLength = TableLength::default();
      // java TableJCData jcData = null;
      let mut jcData: TableJCData = TableJCData::default();
      // java float maxRgdxa = 0.0F;
      let mut maxRgdxa: f32 = 0;
      // java if (tableInfo.getLevel() == 1)
      if tableInfo.getLevel() == 1 {
        // java int docType = document.getType();
        let mut docType: i32 = document.getType();
        // java if (docType != 0 && docType != 2 && docType != 1 && docType != 4)
        if docType != 0 && docType != 2 && docType != 1 && docType != 4 {
          // java if (document.getType() == 5)
          if document.getType() == 5 {
            // java float coreWidth = (float)getTextBoxCoreWidth(document, tableInfo.getStartCP());
            let mut coreWidth: f32 = ((Self::getTextBoxCoreWidth(&document, tableInfo.getStartCP())) as f32);
            // java maxRgdxa = coreWidth;
            maxRgdxa = coreWidth;
          }
        }
        // java else
        else {
          // java float coreWidth = (float)getMainDocumentCoreWidth(document, tableInfo.getStartCP());
          let mut coreWidth: f32 = ((Self::getMainDocumentCoreWidth(&document, tableInfo.getStartCP())) as f32);
          // java if (document.getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle())
          if document.getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle() {
            // java float leftMargin = (float)TableTool.getDefaultCellMarginLeft(tableInfo.getRowByIndex(0));
            let mut leftMargin: f32 = ((TableTool::getDefaultCellMarginLeft(&tableInfo.getRowByIndex(0))) as f32);
            // java float rightMargin = (float)TableTool.getDefaultCellMarginRight(tableInfo.getRowByIndex(0));
            let mut rightMargin: f32 = ((TableTool::getDefaultCellMarginRight(&tableInfo.getRowByIndex(0))) as f32);
            // java maxRgdxa = coreWidth + leftMargin + rightMargin;
            maxRgdxa = coreWidth + leftMargin + rightMargin;
          }
        }
      }
      // java else
      else {
        // java CellInfo cellInfo = tableInfo.getParentCell();
        let mut cellInfo: CellInfo = tableInfo.getParentCell();
        // java maxRgdxa = (float)(cellInfo.getRgdxaRight() - cellInfo.getRgdxaLeft() - 5 - cellInfo.getMarginLeft() - cellInfo.getMarginRight());
        maxRgdxa = ((cellInfo.getRgdxaRight() - cellInfo.getRgdxaLeft() - 5 - cellInfo.getMarginLeft() - cellInfo.getMarginRight()) as f32);
      }
      // java if (finalProp.contains(295))
      if finalProp.contains(295) {
        // java TableLength tableWidth = (TableLength)finalProp.getValue(295);
        let mut tableWidth: TableLength = (finalProp.getValue(295)).downcast_ref::<TableLength>().unwrap();
        // java switch (tableWidth.getType())
        match tableWidth.getType() {
          // java case 1:
          1 => {
            // java newTableWidth = tableWidth;
            newTableWidth = tableWidth;
            // java if (checkOverflowWidth && tableInfo.getLevel() == 1)
            if checkOverflowWidth && tableInfo.getLevel() == 1 {
              // java jcData = tableInfo.getJC();
              jcData = tableInfo.getJC();
              // java int rgdxaWidth = jcData.jcRight - jcData.jcLeft;
              let mut rgdxaWidth: i32 = jcData.jcRight - jcData.jcLeft;
              // java if ((float)rgdxaWidth - maxRgdxa > 40.0F)
              if ((rgdxaWidth) as f32) - maxRgdxa > 40 {
                // java newTableWidth = new TableLength(3, jcData.jcRight - jcData.jcLeft);
                newTableWidth = TableLength::new(3, jcData.jcRight - jcData.jcLeft);
              }
            }
          }
          // java case 2:
          2 => {
            // java jcData = tableInfo.getJC();
            jcData = tableInfo.getJC();
            // java maxRgdxa = Math.max(maxRgdxa, (float)jcData.jcRight);
            maxRgdxa = Math::max(maxRgdxa, ((jcData.jcRight) as f32));
            // java newTableWidth = new TableLength(2, (int)((float)(jcData.jcRight - jcData.jcLeft) * 1.0F / (maxRgdxa - (float)jcData.jcLeft) * 5000.0F));
            newTableWidth = TableLength::new(2, ((((jcData.jcRight - jcData.jcLeft) as f32) * 1 / maxRgdxa - ((jcData.jcLeft) as f32) * 5000) as i32));
          }
          // java default:
          _ => {
            // java jcData = tableInfo.getJC();
            jcData = tableInfo.getJC();
            // java newTableWidth = new TableLength(tableWidth.getType(), jcData.jcRight - jcData.jcLeft);
            newTableWidth = TableLength::new(tableWidth.getType(), jcData.jcRight - jcData.jcLeft);
          }
        }
        // java if (newTableWidth != tableWidth)
        if newTableWidth != tableWidth {
          // java KTablePropertyRange tablePropertyRange = KTablePropertyRange.createTablePropertyRange(document, tableInfo.getStartCP(), tableInfo.getEndCP());
          let mut tablePropertyRange: KTablePropertyRange = KTablePropertyRange::createTablePropertyRange(&document, tableInfo.getStartCP(), tableInfo.getEndCP());
          // java tablePropertyRange.setPropertyNormalAndRevision(295, newTableWidth);
          tablePropertyRange.setPropertyNormalAndRevision(295, newTableWidth);
        }
      }
      // java else if (checkOverflowWidth && tableInfo.getLevel() == 1)
      else if checkOverflowWidth && tableInfo.getLevel() == 1 {
        // java jcData = tableInfo.getJC();
        jcData = tableInfo.getJC();
        // java int rgdxaWidth = jcData.jcRight - jcData.jcLeft;
        let mut rgdxaWidth: i32 = jcData.jcRight - jcData.jcLeft;
        // java if ((float)rgdxaWidth - maxRgdxa > 40.0F)
        if ((rgdxaWidth) as f32) - maxRgdxa > 40 {
          // java newTableWidth = new TableLength(3, jcData.jcRight - jcData.jcLeft);
          newTableWidth = TableLength::new(3, jcData.jcRight - jcData.jcLeft);
          // java KTablePropertyRange tablePropertyRange = KTablePropertyRange.createTablePropertyRange(document, tableInfo.getStartCP(), tableInfo.getEndCP());
          let mut tablePropertyRange: KTablePropertyRange = KTablePropertyRange::createTablePropertyRange(&document, tableInfo.getStartCP(), tableInfo.getEndCP());
          // java tablePropertyRange.setPropertyNormalAndRevision(295, newTableWidth);
          tablePropertyRange.setPropertyNormalAndRevision(295, newTableWidth);
        }
      }
    }

    // java public static void fixColumnWidths(Document document, TableInfo tableInfo)
    pub fn fixColumnWidths__document_tableinfo(document: &dyn Document, tableInfo: &dyn TableInfo) {
      // java fixColumnWidths(document, tableInfo, false);
      Self::fixColumnWidths__document_tableinfo_boolean(&document, &tableInfo, false);
    }

    // java public static void fixColumnWidths_section(Document document, TableInfo tableInfo)
    pub fn fixColumnWidths_section(document: &dyn Document, tableInfo: &dyn TableInfo) {
      // java fixColumnWidths(document, tableInfo, true);
      Self::fixColumnWidths__document_tableinfo_boolean(&document, &tableInfo, true);
    }

    // java public static void fixColumnWidths(Document document, TableInfo tableInfo, boolean isInSection)
    pub fn fixColumnWidths__document_tableinfo_boolean(document: &dyn Document, tableInfo: &dyn TableInfo, isInSection: bool) {
      // java tableInfo.getData().mTableJCData = null;
      tableInfo.getData().mTableJCData = None;
      // java TableJCData jcdata = tableInfo.getJC();
      let mut jcdata: TableJCData = tableInfo.getJC();
      // java float tableRgdxa = (float)Math.abs(jcdata.jcRight - jcdata.jcLeft);
      let mut tableRgdxa: f32 = ((Math::abs(jcdata.jcRight - jcdata.jcLeft)) as f32);
      // java if (tableRgdxa != 0.0F)
      if tableRgdxa != 0 {
        let mut row: RowInfo = tableInfo.getRowByIndex(0);
        // java for(RowInfo row = tableInfo.getRowByIndex(0); row != null row = row.getNextRow())
        while row.is_some() {
          // java PropertySet rowProperty = row.getFinalProperty();
          let mut rowProperty: PropertySet = row.getFinalProperty();
          // java DefTable defTable = (DefTable)rowProperty.getValue(306);
          let mut defTable: DefTable = (rowProperty.getValue(306)).downcast_ref::<DefTable>().unwrap();
          // java if (null != defTable)
          if defTable.is_some() {
            // java DefTableBuilder newDefTable = DefTableBuilder.obtain(defTable);
            let mut newDefTable: DefTableBuilder = DefTableBuilder::obtain__deftable(&defTable);
            // java int cellCount = defTable.getCellCount();
            let mut cellCount: i32 = defTable.getCellCount();
            // java int rgdxaMax = defTable.getItMax();
            let mut rgdxaMax: i32 = defTable.getItMax();
            // java Assert.assertTrue("cellCount != rgdxMax - 1", cellCount == rgdxaMax - 1);
            Assert::assertTrue__string_boolean(&"cellCount != rgdxMax - 1", cellCount == rgdxaMax - 1);
            let mut cellIndex: i32 = 0;
            // java for(int cellIndex = 0; cellIndex < cellCount ++cellIndex)
            while cellIndex < cellCount {
              // java TableCellBuilder tableCell = newDefTable.getTableCellBuilder(cellIndex);
              let mut tableCell: TableCellBuilder = newDefTable.getTableCellBuilder(cellIndex);
              // java if (null == tableCell)
              if tableCell.is_none() {
                // java return;
                return;
              }
              // java int widthType = tableCell.getWidthType();
              let mut widthType: i32 = tableCell.getWidthType();
              // java int cellRgdxa = defTable.getRgdxa(cellIndex + 1) - defTable.getRgdxa(cellIndex);
              let mut cellRgdxa: i32 = defTable.getRgdxa(cellIndex + 1) - defTable.getRgdxa(cellIndex);
              // java if (2 == widthType)
              if 2 == widthType {
                // java tableCell.setWidth((int)((float)cellRgdxa / tableRgdxa * 5000.0F));
                tableCell.setWidth(((((cellRgdxa) as f32) / tableRgdxa * 5000) as i32));
              }
              // java else
              else {
                // java tableCell.setWidth(cellRgdxa);
                tableCell.setWidth(cellRgdxa);
              }
            }
            // java if (isInSection)
            if isInSection {
              // java submitCMD_section(document, row.getRange(), 306, defTable, newDefTable.freeze());
              Self::submitCMD_section(&document, row.getRange(), 306, &defTable, &newDefTable.freeze());
            }
            // java else
            else {
              // java submitCMD(document, row.getRange(), 306, defTable, newDefTable.freeze());
              Self::submitCMD(&document, row.getRange(), 306, &defTable, &newDefTable.freeze());
            }
          }
        }
      }
    }

    // java public static void fixTableWidthBefore(Document document, TableInfo tableInfo)
    pub fn fixTableWidthBefore(document: &dyn Document, tableInfo: &dyn TableInfo) {
      // java TableJCData jcdata = tableInfo.getJC();
      let mut jcdata: TableJCData = tableInfo.getJC();
      let mut row: RowInfo = tableInfo.getRowByIndex(0);
      // java for(RowInfo row = tableInfo.getRowByIndex(0); row != null row = row.getNextRow())
      while row.is_some() {
        // java PropertySet rowProperty = row.getFinalProperty();
        let mut rowProperty: PropertySet = row.getFinalProperty();
        // java DefTable defTable = (DefTable)rowProperty.getValue(306);
        let mut defTable: DefTable = (rowProperty.getValue(306)).downcast_ref::<DefTable>().unwrap();
        // java if (defTable != null && defTable.getItMax() > 0)
        if defTable.is_some() && defTable.getItMax() > 0 {
          // java int newWidthBefore = defTable.getRgdxa(0) - jcdata.jcLeft;
          let mut newWidthBefore: i32 = defTable.getRgdxa(0) - jcdata.jcLeft;
          // java if (rowProperty.contains(296))
          if rowProperty.contains(296) {
            // java TableLength tableLength = (TableLength)rowProperty.getValue(296);
            let mut tableLength: TableLength = (rowProperty.getValue(296)).downcast_ref::<TableLength>().unwrap();
            // java int widthBefore = tableLength.getValue();
            let mut widthBefore: i32 = tableLength.getValue();
            // java if (newWidthBefore != widthBefore)
            if newWidthBefore != widthBefore {
              // java TableLength newTableLenth = TableLength.obtain(3, newWidthBefore);
              let mut newTableLenth: TableLength = TableLength::obtain(3, newWidthBefore);
              // java submitCMD(document, row.getRange(), 296, tableLength, newTableLenth);
              Self::submitCMD(&document, row.getRange(), 296, &tableLength, &newTableLenth);
            }
          }
          // java else if (newWidthBefore > 6)
          else if newWidthBefore > 6 {
            // java TableLength newTableLenth = TableLength.obtain(3, newWidthBefore);
            let mut newTableLenth: TableLength = TableLength::obtain(3, newWidthBefore);
            // java submitCMD(document, row.getRange(), 296, (Object)null, newTableLenth);
            Self::submitCMD(&document, row.getRange(), 296, Default::default(), &newTableLenth);
          }
        }
      }
    }

    // java public static void changeTableFit(Document document, TableInfo tableInfo, boolean isTableFit)
    pub fn changeTableFit(document: &dyn Document, tableInfo: &mut TableInfo, isTableFit: bool) {
      // java while (tableInfo.getLevel() > 1)
      while tableInfo.getLevel() > 1 {
        // java CellInfo parentCell = tableInfo.getParentCell();
        let mut parentCell: CellInfo = tableInfo.getParentCell();
        // java RowInfo parentRow = parentCell.getParent();
        let mut parentRow: RowInfo = parentCell.getParent();
        // java tableInfo = parentRow.getParent();
        tableInfo = parentRow.getParent();
      }
      // java Boolean newTableFit = isTableFit ? Boolean.TRUE : Boolean.FALSE;
      let mut newTableFit: bool = (if isTableFit { Boolean::TRUE } else { Boolean::FALSE });
      let mut row: RowInfo = tableInfo.getRowByIndex(0);
      // java for(RowInfo row = tableInfo.getRowByIndex(0); row != null row = row.getNextRow())
      while row.is_some() {
        // java PropertySet rowProperty = row.getFinalProperty();
        let mut rowProperty: PropertySet = row.getFinalProperty();
        // java Boolean oldTableFit = (Boolean)rowProperty.getValue(302);
        let mut oldTableFit: bool = todo!("cast → bool");
        // java submitCMD(document, row.getRange(), 302, oldTableFit, newTableFit);
        Self::submitCMD(&document, row.getRange(), 302, &oldTableFit, &newTableFit);
      }
    }

    // java public static float getShapeWidth(Shape shape)
    pub fn getShapeWidth(shape: &Shape) -> f32 {
      // java return shape.isChildShape() ? shape.getShapePos().getShapeRect().width() / ShapeTool.getShapeScale((GroupShape)shape.getParent()) : shape.getShapePos().getShapeRect().width();
      return (if shape.isChildShape() { shape.getShapePos().getShapeRect().width() / ShapeTool::getShapeScale(Some(&(shape.getParent()).downcast_ref::<GroupShape>().unwrap())) } else { shape.getShapePos().getShapeRect().width() });
    }

}
