// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.copy.CopyDataBase
// CARGO_BLOCKER: copy module parked — minimal base for KExchange inheritance surface
#[derive(Default)]
pub struct CopyDataBase;

impl CopyDataBase {
    pub fn new() -> Self {
        Self
    }
}
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.footnoteendnote.ChangeRule
use crate::cn::wps::moffice::writer::core::footnoteendnote::ChangeRule::*;

// java public class KExchange extends CopyDataBase
#[derive(Default)]
pub struct KExchange {
    // java CopyDataBase super_
    pub(crate) super_: CopyDataBase,
    // java TextDocument textDocument;
    // NOTE: Java 引用语义；*const 打破 TextDocument↔KExchange 递归（E0072），生命周期由 TextDocument 持有者保证
    pub(crate) textDocument: Option<*const TextDocument>,
}

// java public class KExchange extends CopyDataBase
impl KExchange {
    // java public  KExchange(TextDocument textDocument)
    pub fn KExchange(textDocument: &TextDocument) -> Self {
      // java this.textDocument = textDocument;
      Self { super_: CopyDataBase::new(), textDocument: Some(textDocument as *const TextDocument) }
    }

    // java public boolean exchange(ChangeRule rule)
    pub fn exchange__changerule(&mut self, rule: &ChangeRule) -> bool {
      // java switch (rule)
      match rule {
        // java case FOOTNOTE_TO_ENDNOTE:
        FOOTNOTE_TO_ENDNOTE => {
          // java return this.footnote2endnote();
          return self.footnote2endnote();
        }
        // java case ENDNOTE_TO_FOOTNOTE:
        ENDNOTE_TO_FOOTNOTE => {
          // java return this.endnote2footnote();
          return self.endnote2footnote();
        }
        // java case EXCHANGE:
        EXCHANGE => {
          // java return this.exchange();
          return self.exchange();
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java public boolean exchange(ChangeRule rule, KRange range)
    pub fn exchange__changerule_krange(&mut self, rule: &ChangeRule, range: &KRange) -> bool {
      // java switch (rule)
      match rule {
        // java case FOOTNOTE_TO_ENDNOTE:
        FOOTNOTE_TO_ENDNOTE => {
          // java return this.footnote2endnote(range);
          return self.footnote2endnote__krange(&mut range);
        }
        // java case ENDNOTE_TO_FOOTNOTE:
        ENDNOTE_TO_FOOTNOTE => {
          // java return this.endnote2footnote(range);
          return self.endnote2footnote__krange(&mut range);
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java private boolean footnote2endnote(KRange range)
    pub fn footnote2endnote__krange(&mut self, range: &mut KRange) -> bool {
      // java Assert.assertEquals(1, range.getSubDocument().getType());
      Assert::assertEquals__int_int(1, range.getSubDocument().getType());
      // java if (1 != range.getSubDocument().getType())
      if 1 != range.getSubDocument().getType() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Document footnote = this.getFootnoteDocument();
        let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
        // java Document endnote = this.getEndnoteDocument();
        let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
        // java if (this.isEmpty(footnote))
        if self.isEmpty(&footnote) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java this.textDocument.start();
          if let Some(td) = self.textDocument {
            unsafe { (*td).start() };
          }
          // java boolean var30;
          let mut var30: bool;
              // java footnote.getTextStream().disableRevisionFileter();
              footnote.getTextStream().disableRevisionFileter();
              // java endnote.getTextStream().disableRevisionFileter();
              endnote.getTextStream().disableRevisionFileter();
              // java int newCurorCP = -1;
              let mut newCurorCP: i32 = -1;
              // java if (this.isEmpty(endnote))
              if self.isEmpty(&endnote) {
                // java this.addEmptyDocumentContent(endnote);
                self.addEmptyDocumentContent(&endnote);
                // java newCurorCP = 2;
                newCurorCP = 2;
              }
              // java Document mainDocument = this.getMainDocument();
              let mut mainDocument: Box<dyn Document> = self.getMainDocument();
              // java PLCFootnoteEndnoteRef footRef = mainDocument.getFootnoteRef();
              let mut footRef: PLCFootnoteEndnoteRef = mainDocument.getFootnoteRef();
              // java PLCFootnoteEndnoteRef endRef = mainDocument.getEndnoteRef();
              let mut endRef: PLCFootnoteEndnoteRef = mainDocument.getEndnoteRef();
              // java PLCFootnoteEndnoteTxt footText = footnote.getFootnoteTxt();
              let mut footText: PLCFootnoteEndnoteTxt = footnote.getFootnoteTxt();
              // java PLCFootnoteEndnoteTxt endText = endnote.getEndnoteTxt();
              let mut endText: PLCFootnoteEndnoteTxt = endnote.getEndnoteTxt();
              // java int startCP = range.getStart();
              let mut startCP: i32 = range.getStart();
              // java int endCP = range.getEnd();
              let mut endCP: i32 = range.getEnd();
              // java PLC.Node node = footText.findFootnoteEndnoteTxtContainsCP(startCP);
              let mut node: Box<dyn Node> = footText.findFootnoteEndnoteTxtContainsCP(startCP);
              // java ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> footRefNodeList;
              let mut footRefNodeList: Vec<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = vec![];
              footRefNodeList = Vec::new();
              // java for(footRefNodeList = new ArrayList() null != node && !node.isEnd() && node.getCP() <= endCP node = node.getNext())
              while node.is_some() && !node.isEnd() && node.getCP() <= endCP {
                // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode footTextNode = (PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode)node;
                let mut footTextNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = (node).downcast_ref::<PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode>().unwrap();
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footRefNode = footTextNode.getFootnoteEndnoteRef();
                let mut footRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = footTextNode.getFootnoteEndnoteRef();
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = endRef.addFootnoteEndnoteRef(footRefNode.getCP());
                let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = endRef.addFootnoteEndnoteRef(footRefNode.getCP());
                // java newRefNode.setNumberStart(footRefNode.getNumStart());
                newRefNode.setNumberStart(footRefNode.getNumStart());
                // java int destCP = endnote.getLength();
                let mut destCP: i32 = endnote.getLength();
                // java PLC.Node next = newRefNode.getNext();
                let mut next: Box<dyn Node> = newRefNode.getNext();
                // java if (next instanceof PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)
                if next.downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().is_some() {
                  // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode nextRef = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)next;
                  let mut nextRef: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (next).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
                  // java destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                  destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                }
                // java if (newCurorCP == -1)
                if newCurorCP == -1 {
                  // java newCurorCP = destCP;
                  newCurorCP = destCP;
                }
                // java long rg = footText.getFootnoteEndnoteRange(footTextNode);
                let mut rg: i64 = footText.getFootnoteEndnoteRange(footTextNode);
                // java this.copyData(footnote, endnote, destCP, rg);
                self.copyData(&footnote, &endnote, destCP, rg);
                // java int len = RANGEFactory.getLength(rg);
                let mut len: i32 = RANGEFactory::getLength(rg);
                // java PLC.Node destTxtNode = endText.find(destCP);
                let mut destTxtNode: Box<dyn Node> = endText.find(destCP);
                // java if (!destTxtNode.isHead() && destTxtNode.position() == destCP)
                if !destTxtNode.isHead() && destTxtNode.position() == destCP {
                  // java endText.moveRight(destTxtNode, len);
                  endText.moveRight(destTxtNode, len);
                }
                // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = endText.addFootnoteEndnoteTxt(destCP);
                let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = endText.addFootnoteEndnoteTxt(destCP);
                // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
                newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
                // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
                newTxtNode.setFootnoteEndnoteRef(newRefNode);
                // java footRefNodeList.add(footRefNode);
                footRefNodeList.add(footRefNode);
              }
              // java for(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footRefNode : footRefNodeList)
              for footRefNode in footRefNodeList.iter() {
                // java footRef.remove(footRefNode);
                footRef.remove(footRefNode);
              }
              // java footRefNodeList.clear();
              footRefNodeList.clear();
              // java footnote.getTextStream().enableRevisionFileter();
              footnote.getTextStream().enableRevisionFileter();
              // java endnote.getTextStream().enableRevisionFileter();
              endnote.getTextStream().enableRevisionFileter();
              // java boolean result = newCurorCP != -1;
              let mut result: bool = newCurorCP != -1;
              // java if (result)
              if result {
                // java range.setSubDocument(endnote);
                range.setSubDocument(endnote);
                // java range.setRange(newCurorCP, newCurorCP);
                range.setRange(newCurorCP, newCurorCP);
              }
              // java var30 = result;
              var30 = result;
                // java this.textDocument.commit("chang footnote to endnote!");
                self.textDocument.commit__string(&"chang footnote to endnote!");
          // java return var30;
          return var30;
        }
      }
    }

    // java private boolean endnote2footnote(KRange range)
    pub fn endnote2footnote__krange(&mut self, range: &mut KRange) -> bool {
      // java Assert.assertEquals(4, range.getSubDocument().getType());
      Assert::assertEquals__int_int(4, range.getSubDocument().getType());
      // java if (4 != range.getSubDocument().getType())
      if 4 != range.getSubDocument().getType() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Document footnote = this.getFootnoteDocument();
        let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
        // java Document endnote = this.getEndnoteDocument();
        let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
        // java if (this.isEmpty(endnote))
        if self.isEmpty(&endnote) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java this.textDocument.start();
          if let Some(td) = self.textDocument {
            unsafe { (*td).start() };
          }
          // java boolean var30;
          let mut var30: bool;
              // java footnote.getTextStream().disableRevisionFileter();
              footnote.getTextStream().disableRevisionFileter();
              // java endnote.getTextStream().disableRevisionFileter();
              endnote.getTextStream().disableRevisionFileter();
              // java int newCurorCP = -1;
              let mut newCurorCP: i32 = -1;
              // java if (this.isEmpty(footnote))
              if self.isEmpty(&footnote) {
                // java this.addEmptyDocumentContent(footnote);
                self.addEmptyDocumentContent(&footnote);
                // java newCurorCP = 2;
                newCurorCP = 2;
              }
              // java Document mainDocument = this.getMainDocument();
              let mut mainDocument: Box<dyn Document> = self.getMainDocument();
              // java PLCFootnoteEndnoteRef footRef = mainDocument.getFootnoteRef();
              let mut footRef: PLCFootnoteEndnoteRef = mainDocument.getFootnoteRef();
              // java PLCFootnoteEndnoteRef endRef = mainDocument.getEndnoteRef();
              let mut endRef: PLCFootnoteEndnoteRef = mainDocument.getEndnoteRef();
              // java PLCFootnoteEndnoteTxt footText = footnote.getFootnoteTxt();
              let mut footText: PLCFootnoteEndnoteTxt = footnote.getFootnoteTxt();
              // java PLCFootnoteEndnoteTxt endText = endnote.getEndnoteTxt();
              let mut endText: PLCFootnoteEndnoteTxt = endnote.getEndnoteTxt();
              // java int startCP = range.getStart();
              let mut startCP: i32 = range.getStart();
              // java int endCP = range.getEnd();
              let mut endCP: i32 = range.getEnd();
              // java PLC.Node node = endText.findFootnoteEndnoteTxtContainsCP(startCP);
              let mut node: Box<dyn Node> = endText.findFootnoteEndnoteTxtContainsCP(startCP);
              // java ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> endRefNodeList;
              let mut endRefNodeList: Vec<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = vec![];
              endRefNodeList = Vec::new();
              // java for(endRefNodeList = new ArrayList() null != node && !node.isEnd() && node.getCP() <= endCP node = node.getNext())
              while node.is_some() && !node.isEnd() && node.getCP() <= endCP {
                // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode endTextNode = (PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode)node;
                let mut endTextNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = (node).downcast_ref::<PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode>().unwrap();
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endRefNode = endTextNode.getFootnoteEndnoteRef();
                let mut endRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = endTextNode.getFootnoteEndnoteRef();
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = footRef.addFootnoteEndnoteRef(endRefNode.getCP());
                let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = footRef.addFootnoteEndnoteRef(endRefNode.getCP());
                // java newRefNode.setNumberStart(endRefNode.getNumStart());
                newRefNode.setNumberStart(endRefNode.getNumStart());
                // java int destCP = footnote.getLength();
                let mut destCP: i32 = footnote.getLength();
                // java PLC.Node next = newRefNode.getNext();
                let mut next: Box<dyn Node> = newRefNode.getNext();
                // java if (next instanceof PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)
                if next.downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().is_some() {
                  // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode nextRef = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)next;
                  let mut nextRef: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (next).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
                  // java destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                  destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                }
                // java if (newCurorCP == -1)
                if newCurorCP == -1 {
                  // java newCurorCP = destCP;
                  newCurorCP = destCP;
                }
                // java long rg = endText.getFootnoteEndnoteRange(endTextNode);
                let mut rg: i64 = endText.getFootnoteEndnoteRange(endTextNode);
                // java this.copyData(endnote, footnote, destCP, rg);
                self.copyData(&endnote, &footnote, destCP, rg);
                // java int len = RANGEFactory.getLength(rg);
                let mut len: i32 = RANGEFactory::getLength(rg);
                // java PLC.Node destTxtNode = footText.find(destCP);
                let mut destTxtNode: Box<dyn Node> = footText.find(destCP);
                // java if (!destTxtNode.isHead() && destTxtNode.position() == destCP)
                if !destTxtNode.isHead() && destTxtNode.position() == destCP {
                  // java footText.moveRight(destTxtNode, len);
                  footText.moveRight(destTxtNode, len);
                }
                // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = footText.addFootnoteEndnoteTxt(destCP);
                let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = footText.addFootnoteEndnoteTxt(destCP);
                // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
                newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
                // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
                newTxtNode.setFootnoteEndnoteRef(newRefNode);
                // java endRefNodeList.add(endRefNode);
                endRefNodeList.add(endRefNode);
              }
              // java for(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endRefNode : endRefNodeList)
              for endRefNode in endRefNodeList.iter() {
                // java endRef.remove(endRefNode);
                endRef.remove(endRefNode);
              }
              // java endRefNodeList.clear();
              endRefNodeList.clear();
              // java footnote.getTextStream().enableRevisionFileter();
              footnote.getTextStream().enableRevisionFileter();
              // java endnote.getTextStream().enableRevisionFileter();
              endnote.getTextStream().enableRevisionFileter();
              // java boolean result = newCurorCP != -1;
              let mut result: bool = newCurorCP != -1;
              // java if (result)
              if result {
                // java range.setSubDocument(footnote);
                range.setSubDocument(footnote);
                // java range.setRange(newCurorCP, newCurorCP);
                range.setRange(newCurorCP, newCurorCP);
              }
              // java var30 = result;
              var30 = result;
                // java this.textDocument.commit("chang endnote to footnote!");
                self.textDocument.commit__string(&"chang endnote to footnote!");
          // java return var30;
          return var30;
        }
      }
    }

    // java private boolean footnote2endnote()
    pub fn footnote2endnote(&mut self) -> bool {
      // java Document footnote = this.getFootnoteDocument();
      let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
      // java Document endnote = this.getEndnoteDocument();
      let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
      // java if (this.isEmpty(endnote))
      if self.isEmpty(&endnote) {
        // java this.exchange();
        self.exchange();
        // java return true;
        return true;
      }
      // java else
      else {
        // java this.textDocument.start();
        self.textDocument.start();
        // java boolean var23;
        let mut var23: bool;
            // java footnote.getTextStream().disableRevisionFileter();
            footnote.getTextStream().disableRevisionFileter();
            // java endnote.getTextStream().disableRevisionFileter();
            endnote.getTextStream().disableRevisionFileter();
            // java Document mainDocument = this.getMainDocument();
            let mut mainDocument: Box<dyn Document> = self.getMainDocument();
            // java PLCFootnoteEndnoteRef footRef = mainDocument.getFootnoteRef();
            let mut footRef: PLCFootnoteEndnoteRef = mainDocument.getFootnoteRef();
            // java PLCFootnoteEndnoteRef endRef = mainDocument.getEndnoteRef();
            let mut endRef: PLCFootnoteEndnoteRef = mainDocument.getEndnoteRef();
            // java PLCFootnoteEndnoteTxt footText = footnote.getFootnoteTxt();
            let mut footText: PLCFootnoteEndnoteTxt = footnote.getFootnoteTxt();
            // java PLCFootnoteEndnoteTxt endText = endnote.getEndnoteTxt();
            let mut endText: PLCFootnoteEndnoteTxt = endnote.getEndnoteTxt();
            // java PLC.Iterator iter = footRef.createIterator();
            let mut iter: Vec<()> = footRef.createIterator();
            // java while (iter.hasNext())
            while iter.hasNext() {
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footRefNode = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)iter.next();
              let mut footRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (iter.next()).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = endRef.addFootnoteEndnoteRef(footRefNode.getCP());
              let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = endRef.addFootnoteEndnoteRef(footRefNode.getCP());
              // java newRefNode.setNumberStart(footRefNode.getNumStart());
              newRefNode.setNumberStart(footRefNode.getNumStart());
              // java int destCP = endnote.getLength();
              let mut destCP: i32 = endnote.getLength();
              // java PLC.Node next = newRefNode.getNext();
              let mut next: Box<dyn Node> = newRefNode.getNext();
              // java if (next instanceof PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)
              if next.downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().is_some() {
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode nextRef = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)next;
                let mut nextRef: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (next).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
                // java destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
              }
              // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode = footRefNode.getFootnoteEndnoteTxtNode();
              let mut txtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = footRefNode.getFootnoteEndnoteTxtNode();
              // java long rg = footText.getFootnoteEndnoteRange(txtNode);
              let mut rg: i64 = footText.getFootnoteEndnoteRange(txtNode);
              // java this.copyData(footnote, endnote, destCP, rg);
              self.copyData(&footnote, &endnote, destCP, rg);
              // java int len = RANGEFactory.getLength(rg);
              let mut len: i32 = RANGEFactory::getLength(rg);
              // java PLC.Node destTxtNode = endText.find(destCP);
              let mut destTxtNode: Box<dyn Node> = endText.find(destCP);
              // java if (!destTxtNode.isHead() && destTxtNode.position() == destCP)
              if !destTxtNode.isHead() && destTxtNode.position() == destCP {
                // java endText.moveRight(destTxtNode, len);
                endText.moveRight(destTxtNode, len);
              }
              // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = endText.addFootnoteEndnoteTxt(destCP);
              let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = endText.addFootnoteEndnoteTxt(destCP);
              // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
              newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
              // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
              newTxtNode.setFootnoteEndnoteRef(newRefNode);
            }
            let mut footnoteRefNode: Box<dyn Node> = footRef.begin();
            // java for(PLC.Node footnoteRefNode = footRef.begin(); !footnoteRefNode.isEnd() footnoteRefNode = footnoteRefNode.getNext())
            while !footnoteRefNode.isEnd() {
              // java footRef.remove(footnoteRefNode);
              footRef.remove(footnoteRefNode);
            }
            // java footnote.getTextStream().enableRevisionFileter();
            footnote.getTextStream().enableRevisionFileter();
            // java endnote.getTextStream().enableRevisionFileter();
            endnote.getTextStream().enableRevisionFileter();
            // java var23 = true;
            var23 = true;
              // java this.textDocument.commit("chang footnote to endnote!");
              self.textDocument.commit__string(&"chang footnote to endnote!");
        // java return var23;
        return var23;
      }
    }

    // java private boolean endnote2footnote()
    pub fn endnote2footnote(&mut self) -> bool {
      // java Document footnote = this.getFootnoteDocument();
      let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
      // java Document endnote = this.getEndnoteDocument();
      let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
      // java if (this.isEmpty(footnote))
      if self.isEmpty(&footnote) {
        // java this.exchange();
        self.exchange();
        // java return true;
        return true;
      }
      // java else
      else {
        // java this.textDocument.start();
        self.textDocument.start();
            // java footnote.getTextStream().disableRevisionFileter();
            footnote.getTextStream().disableRevisionFileter();
            // java endnote.getTextStream().disableRevisionFileter();
            endnote.getTextStream().disableRevisionFileter();
            // java Document mainDocument = this.getMainDocument();
            let mut mainDocument: Box<dyn Document> = self.getMainDocument();
            // java PLCFootnoteEndnoteRef footRef = mainDocument.getFootnoteRef();
            let mut footRef: PLCFootnoteEndnoteRef = mainDocument.getFootnoteRef();
            // java PLCFootnoteEndnoteRef endRef = mainDocument.getEndnoteRef();
            let mut endRef: PLCFootnoteEndnoteRef = mainDocument.getEndnoteRef();
            // java PLCFootnoteEndnoteTxt footText = footnote.getFootnoteTxt();
            let mut footText: PLCFootnoteEndnoteTxt = footnote.getFootnoteTxt();
            // java PLCFootnoteEndnoteTxt endText = endnote.getEndnoteTxt();
            let mut endText: PLCFootnoteEndnoteTxt = endnote.getEndnoteTxt();
            // java PLC.Iterator iter = endRef.createIterator();
            let mut iter: Vec<()> = endRef.createIterator();
            // java while (iter.hasNext())
            while iter.hasNext() {
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endRefNode = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)iter.next();
              let mut endRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (iter.next()).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
              // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = footRef.addFootnoteEndnoteRef(endRefNode.getCP());
              let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = footRef.addFootnoteEndnoteRef(endRefNode.getCP());
              // java newRefNode.setNumberStart(endRefNode.getNumStart());
              newRefNode.setNumberStart(endRefNode.getNumStart());
              // java int destCP = footnote.getLength();
              let mut destCP: i32 = footnote.getLength();
              // java PLC.Node next = newRefNode.getNext();
              let mut next: Box<dyn Node> = newRefNode.getNext();
              // java if (next instanceof PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)
              if next.downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().is_some() {
                // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode nextRef = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)next;
                let mut nextRef: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (next).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
                // java destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
                destCP = nextRef.getFootnoteEndnoteTxtNode().getCP();
              }
              // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode = endRefNode.getFootnoteEndnoteTxtNode();
              let mut txtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = endRefNode.getFootnoteEndnoteTxtNode();
              // java long rg = endText.getFootnoteEndnoteRange(txtNode);
              let mut rg: i64 = endText.getFootnoteEndnoteRange(txtNode);
              // java this.copyData(endnote, footnote, destCP, rg);
              self.copyData(&endnote, &footnote, destCP, rg);
              // java int len = RANGEFactory.getLength(rg);
              let mut len: i32 = RANGEFactory::getLength(rg);
              // java PLC.Node destTxtNode = footText.find(destCP);
              let mut destTxtNode: Box<dyn Node> = footText.find(destCP);
              // java if (!destTxtNode.isHead() && destTxtNode.position() == destCP)
              if !destTxtNode.isHead() && destTxtNode.position() == destCP {
                // java footText.moveRight(destTxtNode, len);
                footText.moveRight(destTxtNode, len);
              }
              // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = footText.addFootnoteEndnoteTxt(destCP);
              let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = footText.addFootnoteEndnoteTxt(destCP);
              // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
              newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
              // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
              newTxtNode.setFootnoteEndnoteRef(newRefNode);
            }
            let mut endnoteRefNode: Box<dyn Node> = endRef.begin();
            // java for(PLC.Node endnoteRefNode = endRef.begin(); !endnoteRefNode.isEnd() endnoteRefNode = endnoteRefNode.getNext())
            while !endnoteRefNode.isEnd() {
              // java endRef.remove(endnoteRefNode);
              endRef.remove(endnoteRefNode);
            }
            // java footnote.getTextStream().enableRevisionFileter();
            footnote.getTextStream().enableRevisionFileter();
            // java endnote.getTextStream().enableRevisionFileter();
            endnote.getTextStream().enableRevisionFileter();
              // java this.textDocument.commit("chang endnote to footnote!");
              self.textDocument.commit__string(&"chang endnote to footnote!");
        // java return false;
        return false;
      }
    }

    // java private boolean exchange()
    pub fn exchange(&mut self) -> bool {
      // java Document footnote = this.getFootnoteDocument();
      let mut footnote: Box<dyn Document> = self.getFootnoteDocument();
      // java Document endnote = this.getEndnoteDocument();
      let mut endnote: Box<dyn Document> = self.getEndnoteDocument();
      // java this.textDocument.start();
      self.textDocument.start();
          // java footnote.getTextStream().disableRevisionFileter();
          footnote.getTextStream().disableRevisionFileter();
          // java endnote.getTextStream().disableRevisionFileter();
          endnote.getTextStream().disableRevisionFileter();
          // java this.getTextDocument().liteExchangeDocument(footnote, endnote);
          self.getTextDocument().liteExchangeDocument(footnote, endnote);
          // java footnote.exchangFootEndnoteTxt();
          footnote.exchangFootEndnoteTxt();
          // java endnote.exchangFootEndnoteTxt();
          endnote.exchangFootEndnoteTxt();
          // java this.getMainDocument().exchangFootEndnoteRef();
          self.getMainDocument().exchangFootEndnoteRef();
          // java footnote.getTextStream().enableRevisionFileter();
          footnote.getTextStream().enableRevisionFileter();
          // java endnote.getTextStream().enableRevisionFileter();
          endnote.getTextStream().enableRevisionFileter();
            // java this.textDocument.commit("exchang footnote and endnote!");
            self.textDocument.commit__string(&"exchang footnote and endnote!");
      // java return true;
      return true;
    }

    // java public void addEmptyDocumentContent(Document document)
    pub fn addEmptyDocumentContent(&self, document: &dyn Document) {
      // java if (this.isEmpty(document))
      if self.isEmpty(&document) {
        // java document.getTextStream().insertParagraph(0, '\r', PropertySet.empty, PropertySet.empty);
        document.getTextStream().insertParagraph(0, '\r', PropertySet::empty(), PropertySet::empty());
        // java document.getTextStream().insertParagraph(1, '\r', PropertySet.empty, PropertySet.empty);
        document.getTextStream().insertParagraph(1, '\r', PropertySet::empty(), PropertySet::empty());
      }
    }

    // java public void copyData(Document srcDocument, Document desDocument, int destCP, long rg)
    pub fn copyData(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, rg: i64) {
      // java this.copyTextRope(srcDocument, desDocument, destCP, rg);
      self.super_.copyTextRope(&srcDocument, &desDocument, destCP, rg);
      // java this.copyCPTable(srcDocument, desDocument, destCP, rg);
      self.copyCPTable(&srcDocument, &desDocument, destCP, rg);
      // java this.copyField(srcDocument, desDocument, destCP, rg);
      self.super_.copyField(&srcDocument, &desDocument, destCP, rg);
      // java this.copyBookmarks(srcDocument, desDocument, destCP, rg);
      self.super_.copyBookmarks(&srcDocument, &desDocument, destCP, rg);
      // java this.copyShapes(srcDocument, desDocument, destCP, rg);
      self.super_.copyShapes(&srcDocument, &desDocument, destCP, rg);
      // java this.copyMath(srcDocument, desDocument, destCP, rg);
      self.super_.copyMath(&srcDocument, &mut desDocument, destCP, rg);
    }

    // java public void copyCPTable(Document srcDocument, Document desDocument, int destCP, long srcRange)
    pub fn copyCPTable(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java IChpxTable srcChpx = srcDocument.getChpxTable();
      let mut srcChpx: Box<dyn IChpxTable> = srcDocument.getChpxTable();
      // java IChpxTable destChpx = desDocument.getChpxTable();
      let mut destChpx: Box<dyn IChpxTable> = desDocument.getChpxTable();
      // java int len = endCP - startCP;
      let mut len: i32 = endCP - startCP;
      // java IChpxTable.Entry chEntry = destChpx.seek(destCP);
      let mut chEntry: Box<dyn ChpxEntry> = destChpx.seek(destCP);
      // java if (chEntry.position() == destCP)
      if chEntry.position() == destCP {
        // java PropertySet property = chEntry.getProperty();
        let mut property: PropertySet = chEntry.getProperty();
        // java destChpx.remove(chEntry);
        destChpx.remove(chEntry);
        // java destChpx.insert(destCP + len, property);
        destChpx.insert(destCP + len, property);
      }
      // java IChpxTable.Entry runEntry = srcChpx.seek(startCP);
      let mut runEntry: Box<dyn ChpxEntry> = srcChpx.seek(startCP);
      // java IChpxTable.Entry runEnd = srcChpx.seek(endCP);
      let mut runEnd: Box<dyn ChpxEntry> = srcChpx.seek(endCP);
      let mut insertCP: i32 = destCP;
      // java for(int insertCP = destCP; runEntry != runEnd runEntry = runEntry.getNext())
      while runEntry != runEnd {
        // java PropertySet property = this.correctRunStyle(runEntry.getProperty());
        let mut property: PropertySet = self.correctRunStyle(&runEntry.getProperty());
        // java destChpx.insert(insertCP, property);
        destChpx.insert(insertCP, property);
        // java insertCP += runEntry.length();
        insertCP = insertCP + (runEntry.len() as i32);
      }
      // java IPapxTable srcPapx = srcDocument.getPapxTable();
      let mut srcPapx: Box<dyn IPapxTable> = srcDocument.getPapxTable();
      // java IPapxTable destPapx = desDocument.getPapxTable();
      let mut destPapx: Box<dyn IPapxTable> = desDocument.getPapxTable();
      // java IPapxTable.Entry paEntry = destPapx.seek(destCP);
      let mut paEntry: Box<dyn PapxEntry> = destPapx.seek(destCP);
      // java if (paEntry.position() == destCP)
      if paEntry.position() == destCP {
        // java PropertySet property = paEntry.getProperty();
        let mut property: PropertySet = paEntry.getProperty();
        // java destPapx.remove(paEntry);
        destPapx.remove(paEntry);
        // java destPapx.insert(destCP + len, property);
        destPapx.insert(destCP + len, property);
      }
      // java IPapxTable.Entry paraEntry = srcPapx.seek(startCP);
      let mut paraEntry: Box<dyn PapxEntry> = srcPapx.seek(startCP);
      // java IPapxTable.Entry paraEnd = srcPapx.seek(endCP);
      let mut paraEnd: Box<dyn PapxEntry> = srcPapx.seek(endCP);
      let mut var24: i32 = destCP;
      // java for(int var24 = destCP; paraEntry != paraEnd paraEntry = paraEntry.getNext())
      while paraEntry != paraEnd {
        // java PropertySet property = this.correctParagraphStyle(paraEntry.getProperty());
        let mut property: PropertySet = self.correctParagraphStyle(&paraEntry.getProperty());
        // java destPapx.insert(var24, property);
        destPapx.insert(var24, property);
        // java var24 += paraEntry.length();
        var24 = var24 + (paraEntry.len() as i32);
      }
    }

    // java public PropertySet correctRunStyle(PropertySet runProp)
    pub fn correctRunStyle(&mut self, runProp: &PropertySet) -> PropertySet {
      // java if (runProp.getBoolean(28, false))
      if runProp.getBoolean__int_boolean(28, false) {
        // java int style = runProp.getInt(2, 0);
        let mut style: i32 = runProp.getInt__int_int(2, 0);
        // java if (style == 42)
        if style == 42 {
          // java style = 38;
          style = 38;
        }
        // java else if (style == 38)
        else if style == 38 {
          // java style = 42;
          style = 42;
        }
        // java MutablePropertySet newProperty = new MutablePropertySet(runProp);
        let mut newProperty: MutablePropertySet = MutablePropertySet::new(runProp);
        // java newProperty.setInt(2, style);
        newProperty.setInt(2, style);
        // java return newProperty.freeze();
        return newProperty.freeze();
      }
      // java else
      else {
        // java return runProp;
        return runProp;
      }
    }

    // java public PropertySet correctParagraphStyle(PropertySet paraProp)
    pub fn correctParagraphStyle(&mut self, paraProp: &PropertySet) -> PropertySet {
      // java if (paraProp.contains(191))
      if paraProp.contains(191) {
        // java int style = paraProp.getInt(191, 0);
        let mut style: i32 = paraProp.getInt__int_int(191, 0);
        // java if (style == 43)
        if style == 43 {
          // java style = 29;
          style = 29;
        }
        // java else if (style == 29)
        else if style == 29 {
          // java style = 43;
          style = 43;
        }
        // java MutablePropertySet newProperty = new MutablePropertySet(paraProp);
        let mut newProperty: MutablePropertySet = MutablePropertySet::new(paraProp);
        // java newProperty.setInt(191, style);
        newProperty.setInt(191, style);
        // java return newProperty.freeze();
        return newProperty.freeze();
      }
      // java else
      else {
        // java return paraProp;
        return paraProp;
      }
    }

    // java private boolean isEmpty(Document document)
    pub fn isEmpty(&self, document: &dyn Document) -> bool {
      // java return document.getLength() == 0;
      return document.getLength() == 0;
    }

    // java private TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> TextDocument {
      // java return this.textDocument;
      return self.textDocument;
    }

    // java private Document getFootnoteDocument()
    pub fn getFootnoteDocument(&self) -> Box<dyn Document> {
      // java return this.textDocument.getSubdocument(1);
      return self.textDocument.getSubdocument(1);
    }

    // java private Document getEndnoteDocument()
    pub fn getEndnoteDocument(&self) -> Box<dyn Document> {
      // java return this.textDocument.getSubdocument(4);
      return self.textDocument.getSubdocument(4);
    }

    // java private Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.textDocument.getMainDocument();
      return self.textDocument.getMainDocument();
    }

}

impl std::ops::Deref for KExchange {
    type Target = CopyDataBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
