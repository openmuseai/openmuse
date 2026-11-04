// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument as WriterTextDocument;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.TableTool
use crate::cn::wps::moffice::writer::data::TableTool::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.BuildinStyleSheet
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::*;
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KExchange
use crate::cn::wps::moffice::writer::core::footnoteendnote::KExchange::*;
// Java: import cn.wps.moffice.writer.core.footnoteendnote.KFootEndnoteSetting
use crate::cn::wps::moffice::writer::core::footnoteendnote::KFootEndnoteSetting::*;
// Java: import cn.wps.moffice.writer.core.footnoteendnote.FootnoteSettingItem
use crate::cn::wps::moffice::writer::core::footnoteendnote::FootnoteSettingItem::*;

// java public class KFootnoteEndnote
#[derive(Default)]
pub struct KFootnoteEndnote {
    // java TextDocument mTextDocument;
    pub(crate) mTextDocument: *mut WriterTextDocument,
    // java KExchange mFootnoteEndnoteExchange;
    pub(crate) mFootnoteEndnoteExchange: Option<KExchange>,
    // java KFootEndnoteSetting mFootnoteEndnoteSetting;
    pub(crate) mFootnoteEndnoteSetting: Option<KFootEndnoteSetting>,
}

// java public class KFootnoteEndnote
impl KFootnoteEndnote {
    fn text_document(&self) -> &WriterTextDocument {
      unsafe { &*self.mTextDocument }
    }

    fn text_document_mut(&mut self) -> &mut WriterTextDocument {
      unsafe { &mut *self.mTextDocument }
    }

    // java public  KFootnoteEndnote(TextDocument textDocument)
    pub fn KFootnoteEndnote(textDocument: &WriterTextDocument) -> Self {
      // java this.mTextDocument = textDocument;
      Self { mTextDocument: textDocument as *const WriterTextDocument as *mut WriterTextDocument, mFootnoteEndnoteExchange: Default::default(), mFootnoteEndnoteSetting: Default::default() }
    }

    // java public boolean addFootnote(SelectionType selType, KRange range)
    pub fn addFootnote(&mut self, selType: &SelectionType, range: &mut KRange) -> bool {
      // java Document mainDocument = range.getSubDocument();
      let mut mainDocument: Box<dyn Document> = range.getSubDocument();
      // java if (mainDocument.getType() != 0)
      if mainDocument.getType() != 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.mTextDocument.start();
        self.text_document_mut().start();
            // java this.fixRange(selType, range);
            self.fixRange(&selType, &mut range);
            // java int cp = range.getEnd();
            let mut cp: i32 = range.getEnd();
            // java Document footnote = this.getFootnoteDocument();
            let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
            // java PLCFootnoteEndnoteRef footRef = mainDocument.getFootnoteRef();
            let mut footRef: PLCFootnoteEndnoteRef = mainDocument.getFootnoteRef();
            // java PLCFootnoteEndnoteTxt footText = footnote.getFootnoteTxt();
            let mut footText: PLCFootnoteEndnoteTxt = footnote.getFootnoteTxt();
            // java this.addEmptyDocumentContent(footnote);
            self.addEmptyDocumentContent(&footnote);
            // java PLC.Node node = footRef.lowerBound(cp);
            let mut node: Box<dyn Node> = footRef.lowerBound(cp);
            // java int contentCp = footnote.getLength();
            let mut contentCp: i32 = footnote.getLength();
            // java if (!node.isHead())
            if !node.isHead() {
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode refNode = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)node;
              let mut refNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (node).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
              // java contentCp = refNode.getFootnoteEndnoteTxtNode().position();
              contentCp = refNode.getFootnoteEndnoteTxtNode().position();
            }
            // java mainDocument.getTextStream().insertPlaceHolder(cp, '\u0002', this.getFootnoteReferenceProp());
            mainDocument.getTextStream().insertPlaceHolder(cp, '\u{0002}', self.getFootnoteReferenceProp());
            // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = footRef.addFootnoteEndnoteRef(cp);
            let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = footRef.addFootnoteEndnoteRef(cp);
            // java newRefNode.setNumberStart(1);
            newRefNode.setNumberStart(1);
            // java footnote.getTextStream().insertParagraph(contentCp, '\r', PropertySet.empty, this.getFootnoteTextProp());
            footnote.getTextStream().insertParagraph(contentCp, '\r', PropertySet::empty(), self.getFootnoteTextProp());
            // java PLC.Node oldNode = footText.find(contentCp);
            let mut oldNode: Box<dyn Node> = footText.find(contentCp);
            // java if (null != oldNode && !oldNode.isHead())
            if oldNode.is_some() && !oldNode.isHead() {
              // java footText.moveRight(oldNode, 1);
              footText.moveRight(oldNode, 1);
            }
            // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = footText.addFootnoteEndnoteTxt(contentCp);
            let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = footText.addFootnoteEndnoteTxt(contentCp);
            // java footnote.getTextStream().insertPlaceHolder(contentCp++, '\u0002', this.getFootnoteReferenceProp());
            footnote.getTextStream().insertPlaceHolder({ let _tmp = contentCp; contentCp += 1; _tmp }, '\u{0002}', self.getFootnoteReferenceProp());
            // java footnote.getTextStream().insertText(contentCp++, new char[]{' '}, 0, 1, PropertySet.empty);
            footnote.getTextStream().insertText({ let _tmp = contentCp; contentCp += 1; _tmp }, vec![' '], 0, 1, PropertySet::empty());
            // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
            newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
            // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
            newTxtNode.setFootnoteEndnoteRef(newRefNode);
            // java range.setSubDocument(footnote);
            range.setSubDocument(footnote);
            // java range.setRange(contentCp, contentCp);
            range.setRange(contentCp, contentCp);
              // java this.mTextDocument.commit("add footnote!");
              self.text_document_mut().commit__string(&"add footnote!");
        // java return true;
        return true;
      }
    }

    // java public boolean addEndnote(SelectionType selType, KRange range)
    pub fn addEndnote(&mut self, selType: &SelectionType, range: &mut KRange) -> bool {
      // java Document mainDocument = range.getSubDocument();
      let mut mainDocument: Box<dyn Document> = range.getSubDocument();
      // java if (mainDocument.getType() != 0)
      if mainDocument.getType() != 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.mTextDocument.start();
        self.text_document_mut().start();
            // java this.fixRange(selType, range);
            self.fixRange(&selType, &mut range);
            // java int cp = range.getEnd();
            let mut cp: i32 = range.getEnd();
            // java Document endnote = this.getEndnoteDocument();
            let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
            // java PLCFootnoteEndnoteRef endRef = mainDocument.getEndnoteRef();
            let mut endRef: PLCFootnoteEndnoteRef = mainDocument.getEndnoteRef();
            // java PLCFootnoteEndnoteTxt endText = endnote.getEndnoteTxt();
            let mut endText: PLCFootnoteEndnoteTxt = endnote.getEndnoteTxt();
            // java this.addEmptyDocumentContent(endnote);
            self.addEmptyDocumentContent(&endnote);
            // java int contentCp = endnote.getLength();
            let mut contentCp: i32 = endnote.getLength();
            // java PLC.Node node = endRef.lowerBound(cp);
            let mut node: Box<dyn Node> = endRef.lowerBound(cp);
            // java if (!node.isHead())
            if !node.isHead() {
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode refNode = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)node;
              let mut refNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (node).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
              // java contentCp = refNode.getFootnoteEndnoteTxtNode().position();
              contentCp = refNode.getFootnoteEndnoteTxtNode().position();
            }
            // java mainDocument.getTextStream().insertPlaceHolder(cp, '\u0002', this.getEndnoteReferenceProp());
            mainDocument.getTextStream().insertPlaceHolder(cp, '\u{0002}', self.getEndnoteReferenceProp());
            // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = endRef.addFootnoteEndnoteRef(cp);
            let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = endRef.addFootnoteEndnoteRef(cp);
            // java newRefNode.setNumberStart(1);
            newRefNode.setNumberStart(1);
            // java endnote.getTextStream().insertParagraph(contentCp, '\r', PropertySet.empty, this.getEndnoteTextProp());
            endnote.getTextStream().insertParagraph(contentCp, '\r', PropertySet::empty(), self.getEndnoteTextProp());
            // java PLC.Node oldNode = endText.find(contentCp);
            let mut oldNode: Box<dyn Node> = endText.find(contentCp);
            // java if (null != oldNode && !oldNode.isHead())
            if oldNode.is_some() && !oldNode.isHead() {
              // java endText.moveRight(oldNode, 1);
              endText.moveRight(oldNode, 1);
            }
            // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = endText.addFootnoteEndnoteTxt(contentCp);
            let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = endText.addFootnoteEndnoteTxt(contentCp);
            // java endnote.getTextStream().insertPlaceHolder(contentCp++, '\u0002', this.getEndnoteReferenceProp());
            endnote.getTextStream().insertPlaceHolder({ let _tmp = contentCp; contentCp += 1; _tmp }, '\u{0002}', self.getEndnoteReferenceProp());
            // java endnote.getTextStream().insertText(contentCp++, new char[]{' '}, 0, 1, PropertySet.empty);
            endnote.getTextStream().insertText({ let _tmp = contentCp; contentCp += 1; _tmp }, vec![' '], 0, 1, PropertySet::empty());
            // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
            newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
            // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
            newTxtNode.setFootnoteEndnoteRef(newRefNode);
            // java range.setSubDocument(endnote);
            range.setSubDocument(endnote);
            // java range.setRange(contentCp, contentCp);
            range.setRange(contentCp, contentCp);
              // java this.mTextDocument.commit("add endnote!");
              self.text_document_mut().commit__string(&"add endnote!");
        // java return true;
        return true;
      }
    }

    // java private void fixRange(SelectionType selType, KRange range)
    pub fn fixRange(&mut self, selType: &SelectionType, range: &mut KRange) {
      // java int start = range.getStart();
      let mut start: i32 = range.getStart();
      // java int end = range.getEnd();
      let mut end: i32 = range.getEnd();
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java int documentLen = document.getLength();
      let mut documentLen: i32 = document.getLength();
      // java if (end >= documentLen)
      if end >= documentLen {
        // java range.setRange(start, documentLen - 1);
        range.setRange(start, documentLen - 1);
      }
      // java if (SelectionType.isTableSelect(selType))
      if SelectionType::isTableSelect(selType) {
        // java if (selType == SelectionType.TABLEROW)
        if selType == SelectionType::TABLEROW {
          // java range.setRange(range.getStart(), range.getEnd() - 2);
          range.setRange(range.getStart(), range.getEnd() - 2);
        }
        // java else if (selType == SelectionType.TABLECOLUMN)
        else if selType == SelectionType::TABLECOLUMN {
          /* DEFERRED_NON_FUNCTION —— `TableTool.getLastCell` 返回 `CellInfo`，
           *   属延后的 `data/table` 子树。本分支只处理**交互式整列选择**
           *   （`SelectionType.TABLECOLUMN`，供鼠标拖选表格列用），
           *   不在 docx 解析→排版→渲染管线上。
           *   Java 原逻辑：
           *     CellInfo cellInfo = TableTool.getLastCell(range);
           *     if (cellInfo != null) range.setRange(cellInfo.getStartCP(), cellInfo.getEndCP() - 1); */
          // java CellInfo cellInfo = TableTool.getLastCell(range);
          // java if (cellInfo != null) { range.setRange(cellInfo.getStartCP(), cellInfo.getEndCP() - 1); }
        }
      }
      // java if (SelectionType.isShapeSelect(selType))
      if SelectionType::isShapeSelect(selType) {
        // java PLCField.FieldTriple fieldTriple = document.getPLCField().getContainingField(start);
        let mut fieldTriple: PLCField::FieldTriple = document.getPLCField().getContainingField(start);
        // java if (fieldTriple != null)
        if fieldTriple.is_some() {
          // java range.setRange(fieldTriple.getStartCP(), fieldTriple.getEnd());
          range.setRange(fieldTriple.getStartCP(), fieldTriple.getEnd());
        }
        // java else
        else {
          // java range.setRange(start, start + 1);
          range.setRange(start, start + 1);
        }
      }
    }

    // java public PropertySet getFootnoteReferenceProp()
    pub fn getFootnoteReferenceProp(&self) -> PropertySet {
      // java int id = 38;
      let mut id: i32 = 38;
      // java KStyle style = this.mTextDocument.getStyles().getStyleById(id, false);
      let mut style: KStyle = self.text_document().getStyles().getStyleById(id, false);
      // java if (null == style)
      if style.is_none() {
        // java style = new KStyle();
        style = KStyle::new();
        // java style.setId(id);
        style.setId(id);
        // java style.setName(BuildinStyleSheet.getName(id));
        style.setName(BuildinStyleSheet::getName(id));
        // java style.setBase(65);
        style.setBase(65);
        // java style.setType(2);
        style.setType(2);
        // java style.setQFormat(false);
        style.setQFormat(false);
        // java style.setUiPriority(99L);
        style.setUiPriority(99);
        // java style.setProperty(PropertySet.valueOf(13, 1));
        style.setProperty(PropertySet::valueOf__int_object(13, &1));
        // java this.mTextDocument.getStyles().add(style);
        self.text_document_mut().getStyles().add(style);
      }
      // java MutablePropertySet prop = new MutablePropertySet();
      let mut prop: MutablePropertySet = MutablePropertySet::new();
      // java prop.setInt(2, id);
      prop.setInt(2, id);
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java return prop.freeze();
      return prop.freeze();
    }

    // java public PropertySet getEndnoteReferenceProp()
    pub fn getEndnoteReferenceProp(&self) -> PropertySet {
      // java int id = 42;
      let mut id: i32 = 42;
      // java KStyle style = this.mTextDocument.getStyles().getStyleById(id, false);
      let mut style: KStyle = self.text_document().getStyles().getStyleById(id, false);
      // java if (null == style)
      if style.is_none() {
        // java style = new KStyle();
        style = KStyle::new();
        // java style.setId(id);
        style.setId(id);
        // java style.setName(BuildinStyleSheet.getName(id));
        style.setName(BuildinStyleSheet::getName(id));
        // java style.setBase(65);
        style.setBase(65);
        // java style.setType(2);
        style.setType(2);
        // java style.setQFormat(false);
        style.setQFormat(false);
        // java style.setUiPriority(99L);
        style.setUiPriority(99);
        // java style.setProperty(PropertySet.valueOf(13, 1));
        style.setProperty(PropertySet::valueOf__int_object(13, &1));
        // java this.mTextDocument.getStyles().add(style);
        self.text_document_mut().getStyles().add(style);
      }
      // java MutablePropertySet prop = new MutablePropertySet();
      let mut prop: MutablePropertySet = MutablePropertySet::new();
      // java prop.setInt(2, id);
      prop.setInt(2, id);
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java return prop.freeze();
      return prop.freeze();
    }

    // java public PropertySet getFootnoteTextProp()
    pub fn getFootnoteTextProp(&self) -> PropertySet {
      // java int id = 29;
      let mut id: i32 = 29;
      // java KStyle style = this.mTextDocument.getStyles().getStyleById(id, false);
      let mut style: KStyle = self.text_document().getStyles().getStyleById(id, false);
      // java if (null == style)
      if style.is_none() {
        // java style = new KStyle();
        style = KStyle::new();
        // java style.setId(id);
        style.setId(id);
        // java style.setName(BuildinStyleSheet.getName(id));
        style.setName(BuildinStyleSheet::getName(id));
        // java style.setBase(0);
        style.setBase(0);
        // java style.setType(1);
        style.setType(1);
        // java style.setLink(38);
        style.setLink(38);
        // java style.setQFormat(false);
        style.setQFormat(false);
        // java style.setUiPriority(99L);
        style.setUiPriority(99);
        // java MutablePropertySet prop = new MutablePropertySet();
        let mut prop: MutablePropertySet = MutablePropertySet::new();
        // java prop.setInt(193, 0);
        prop.setInt(193, 0);
        // java prop.setBoolean(196, false);
        prop.setBoolean(196, false);
        // java prop.setFloat(10, 9.0F);
        prop.setFloat(10, 9);
        // java prop.setFloat(37, 9.0F);
        prop.setFloat(37, 9);
        // java style.setProperty(prop.freeze());
        style.setProperty(prop.freeze());
        // java this.mTextDocument.getStyles().add(style);
        self.text_document_mut().getStyles().add(style);
      }
      // java return PropertySet.valueOf(191, id);
      return PropertySet::valueOf__int_object(191, &id);
    }

    // java public PropertySet getEndnoteTextProp()
    pub fn getEndnoteTextProp(&self) -> PropertySet {
      // java int id = 43;
      let mut id: i32 = 43;
      // java KStyle style = this.mTextDocument.getStyles().getStyleById(id, false);
      let mut style: KStyle = self.text_document().getStyles().getStyleById(id, false);
      // java if (null == style)
      if style.is_none() {
        // java style = new KStyle();
        style = KStyle::new();
        // java style.setId(id);
        style.setId(id);
        // java style.setName(BuildinStyleSheet.getName(id));
        style.setName(BuildinStyleSheet::getName(id));
        // java style.setType(1);
        style.setType(1);
        // java style.setLink(42);
        style.setLink(42);
        // java style.setQFormat(false);
        style.setQFormat(false);
        // java style.setUiPriority(99L);
        style.setUiPriority(99);
        // java MutablePropertySet prop = new MutablePropertySet();
        let mut prop: MutablePropertySet = MutablePropertySet::new();
        // java prop.setInt(193, 0);
        prop.setInt(193, 0);
        // java prop.setBoolean(196, false);
        prop.setBoolean(196, false);
        // java style.setProperty(prop.freeze());
        style.setProperty(prop.freeze());
        // java this.mTextDocument.getStyles().add(style);
        self.text_document_mut().getStyles().add(style);
      }
      // java return PropertySet.valueOf(191, id);
      return PropertySet::valueOf__int_object(191, &id);
    }

    // java public boolean footnote2Endnote(KRange range)
    pub fn footnote2Endnote(&self, range: &KRange) -> bool {
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java KExchange footnoteEndnoteExchange = this.mTextDocument.getFootnoteEndnoteExchange();
      let mut footnoteEndnoteExchange: KExchange = self.text_document().getFootnoteEndnoteExchange();
      // java if (document.getType() == 1)
      if document.getType() == 1 {
        // java footnoteEndnoteExchange.exchange(ChangeRule.FOOTNOTE_TO_ENDNOTE, range);
        footnoteEndnoteExchange.exchange(ChangeRule::FOOTNOTE_TO_ENDNOTE, range);
      }
      // java else
      else {
        // java footnoteEndnoteExchange.exchange(ChangeRule.FOOTNOTE_TO_ENDNOTE);
        footnoteEndnoteExchange.exchange(ChangeRule::FOOTNOTE_TO_ENDNOTE);
      }
      // java return true;
      return true;
    }

    // java public boolean endnote2footnote(KRange range)
    pub fn endnote2footnote(&self, range: &KRange) -> bool {
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java KExchange footnoteEndnoteExchange = this.mTextDocument.getFootnoteEndnoteExchange();
      let mut footnoteEndnoteExchange: KExchange = self.text_document().getFootnoteEndnoteExchange();
      // java if (document.getType() == 4)
      if document.getType() == 4 {
        // java footnoteEndnoteExchange.exchange(ChangeRule.ENDNOTE_TO_FOOTNOTE, range);
        footnoteEndnoteExchange.exchange(ChangeRule::ENDNOTE_TO_FOOTNOTE, range);
      }
      // java else
      else {
        // java footnoteEndnoteExchange.exchange(ChangeRule.ENDNOTE_TO_FOOTNOTE);
        footnoteEndnoteExchange.exchange(ChangeRule::ENDNOTE_TO_FOOTNOTE);
      }
      // java return true;
      return true;
    }

    // java private boolean isEmpty(Document document)
    pub fn isEmpty(&self, document: &dyn Document) -> bool {
      // java return document.getLength() == 0;
      return document.getLength() == 0;
    }

    // java public boolean hasEndnote()
    pub fn hasEndnote(&self) -> bool {
      // java return !this.getEndnoteDocument().getEndnoteTxt().isEmpty();
      return !self.getEndnoteDocument().getEndnoteTxt().is_empty();
    }

    // java public boolean hasFootnote()
    pub fn hasFootnote(&self) -> bool {
      // java return !this.getFootnoteDocument().getFootnoteTxt().isEmpty();
      return !self.getFootnoteDocument().getFootnoteTxt().is_empty();
    }

    // java public Document getFootnoteDocument()
    pub fn getFootnoteDocument(&self) -> Box<dyn Document> {
      // java return this.mTextDocument.getSubdocument(1);
      return self.text_document().getSubdocument(1);
    }

    // java public Document getEndnoteDocument()
    pub fn getEndnoteDocument(&self) -> Box<dyn Document> {
      // java return this.mTextDocument.getSubdocument(4);
      return self.text_document().getSubdocument(4);
    }

    // java public void addEmptyDocumentContent(Document document)
    pub fn addEmptyDocumentContent(&self, document: &dyn Document) {
      // java if (this.isEmpty(document))
      if self.isEmpty(&document) {
        // java document.getTextStream().disableRevisionFileter();
        document.getTextStream().disableRevisionFileter();
        // java document.getTextStream().insertParagraph(0, '\r', PropertySet.empty, PropertySet.empty);
        document.getTextStream().insertParagraph(0, '\r', PropertySet::empty(), PropertySet::empty());
        // java document.getTextStream().insertParagraph(1, '\r', PropertySet.empty, PropertySet.empty);
        document.getTextStream().insertParagraph(1, '\r', PropertySet::empty(), PropertySet::empty());
        // java document.getTextStream().enableRevisionFileter();
        document.getTextStream().enableRevisionFileter();
      }
    }

    // java public KFootEndnoteSetting getFootEndnoteSetting()
    pub fn getFootEndnoteSetting(&mut self) -> KFootEndnoteSetting {
      // java if (null == this.mFootnoteEndnoteExchange)
      if self.mFootnoteEndnoteExchange.is_none() {
        // java this.mFootnoteEndnoteSetting = new KFootEndnoteSetting(this.mTextDocument);
        self.mFootnoteEndnoteSetting = KFootEndnoteSetting::new(self.text_document());
      }
      // java return this.mFootnoteEndnoteSetting;
      return self.mFootnoteEndnoteSetting;
    }

    // java public boolean apply(KRange range, FootnoteSettingItem settingItem)
    pub fn apply(&self, range: &KRange, settingItem: &FootnoteSettingItem) -> bool {
      // java return this.getFootEndnoteSetting().apply(range, settingItem);
      return self.getFootEndnoteSetting().apply(range, settingItem);
    }

    // java public FootnoteSettingItem getDefaultSettingItem(KRange range)
    pub fn getDefaultSettingItem(&self, range: &KRange) -> FootnoteSettingItem {
      // java return this.getFootEndnoteSetting().getDefaultSettingItem(range);
      return self.getFootEndnoteSetting().getDefaultSettingItem(range);
    }

}
