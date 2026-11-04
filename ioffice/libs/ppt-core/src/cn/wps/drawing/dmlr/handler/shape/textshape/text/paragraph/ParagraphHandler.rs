mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}

// Java: import cn.wps.drawing.data.text.TextParagraph
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextParagraph;
// Java: import cn.wps.drawing.data.text.TextRun
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextRun;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.run.TextCharacterPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::run::TextCharacterPropertiesHandler::TextCharacterPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.paragraph.TextParagraphPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphPropertiesHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::callback_state;
use std::cell::RefCell;
use std::rc::Rc;

// java public class ParagraphHandler extends DefaultElementHandler
pub struct ParagraphHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private HandlerCallback handlerCallback;
    handlerCallback: Option<Rc<RefCell<HandlerCallback>>>,
    // java private TextParagraph paragraph;
    paragraph: Rc<RefCell<TextParagraph>>,
    // java private List<TextRun> runList;
    runList: Vec<Rc<RefCell<TextRun>>>,
    // java private TextRunsHandler textRunsHandler;
    textRunsHandler: Option<Rc<RefCell<TextRunsHandler>>>,
}

#[derive(Clone)]
pub struct ParagraphHandlerCell(pub Rc<RefCell<ParagraphHandler>>);

// java static class TextRunsHandler extends DefaultElementHandler
pub struct TextRunsHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextRun txRun;
    txRun: Rc<RefCell<TextRun>>,
    // java private HandlerCallback handlerCallback;
    handlerCallback: Option<Rc<RefCell<HandlerCallback>>>,
}

#[derive(Clone)]
pub struct TextRunsHandlerCell(pub Rc<RefCell<TextRunsHandler>>);

// java static class RegularTextHandler extends DefaultElementHandler
pub struct RegularTextHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextRun txRun;
    txRun: Rc<RefCell<TextRun>>,
}

// java static class FieldTextHandler extends DefaultElementHandler
pub struct FieldTextHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextRun txRun;
    txRun: Rc<RefCell<TextRun>>,
}

// java public class ParagraphHandler extends DefaultElementHandler
impl ParagraphHandler {
    // java public  ParagraphHandler(TextParagraph paragraph, HandlerCallback handlerCallback)
    pub fn ParagraphHandler(paragraph: Rc<RefCell<TextParagraph>>, handlerCallback: Option<Rc<RefCell<HandlerCallback>>>) -> Self {
      // java this.init(paragraph);
      let mut this = Self { super_: DefaultElementHandler::default(), handlerCallback: handlerCallback, paragraph: paragraph.clone(), runList: Vec::new(), textRunsHandler: None };
      this.init(paragraph);
      // java this.handlerCallback = handlerCallback;
      this
    }

    // java public void init(TextParagraph paragraph)
    pub fn init(&mut self, paragraph: Rc<RefCell<TextParagraph>>) {
      // java this.paragraph = paragraph;
      self.paragraph = paragraph;
      // java this.runList = new ArrayList();
      self.runList = Vec::new();
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 1114193: case 1114618: case 1114619:
        1114193 | 1114618 | 1114619 => {
          // java TextRun txRun = TextRun.create();
          let txRun: Rc<RefCell<TextRun>> = Rc::new(RefCell::new(TextRun::create()));
          // java this.runList.add(txRun);
          self.runList.push(txRun.clone());
          // java if (this.textRunsHandler == null)
          if self.textRunsHandler.is_none() {
            // java this.textRunsHandler = new TextRunsHandler(txRun, this.handlerCallback);
            self.textRunsHandler = Some(Rc::new(RefCell::new(TextRunsHandler::TextRunsHandler(txRun, self.handlerCallback.clone()))));
          }
          // java else
          else {
            // java this.textRunsHandler.init(txRun);
            self.textRunsHandler.as_ref().unwrap().borrow_mut().init(txRun);
          }
          // java handler = this.textRunsHandler;
          handler = Some(Box::new(TextRunsHandlerCell(self.textRunsHandler.as_ref().unwrap().clone())));
        }
        // java case 1114531:
        1114531 => {
          // java handler = new TextParagraphPropertiesHandler(this.paragraph.paraPr(), this.handlerCallback);
          handler = Some(Box::new(TextParagraphPropertiesHandler::TextParagraphPropertiesHandler(
            self.paragraph.borrow().paraPr(),
            callback_state(&self.handlerCallback),
          )));
        }
        // java case 1114532:
        1114532 => {
          // java handler = new TextCharacterPropertiesHandler(this.paragraph.endParaRPr(), this.handlerCallback);
          handler = Some(Box::new(TextCharacterPropertiesHandler::TextCharacterPropertiesHandler(
            self.paragraph.borrow().endParaRPr(),
            self.handlerCallback.clone(),
          )));
        }
        // java default:
        _ => {
          // rust: no-stmt
        }
      }
      // java return handler;
      return handler;
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (this.runList != null)
      if true {
        // java for (TextRun run : this.runList)
        for run in self.runList.iter() {
          // java if (null != this.handlerCallback && this.handlerCallback.isGVml())
          if let Some(handlerCallback) = self.handlerCallback.as_ref() {
            if handlerCallback.borrow().isGVml() {
              // java this.handlerCallback.onMergeThemeTypeface(run);
              handlerCallback.borrow_mut().onMergeThemeTypeface(&run.borrow());
            }
          }
          // java run.save();
          run.borrow_mut().save();
        }
        // java this.paragraph.textruns().write(this.runList);
        self.paragraph.borrow_mut().textruns().write(&self.runList);
      }
    }

}

impl ElementHandler for ParagraphHandler {
    fn kind(&self) -> &'static str { "ParagraphHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        ParagraphHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        ParagraphHandler::endElement(self, element)
    }
}

impl ElementHandler for ParagraphHandlerCell {
    fn kind(&self) -> &'static str { "ParagraphHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.borrow_mut().enterSubElement(subElement)
    }
    fn endElement(&mut self, element: i32) {
        self.0.borrow_mut().endElement(element)
    }
}

// java static class TextRunsHandler extends DefaultElementHandler
impl TextRunsHandler {
    // java public  TextRunsHandler(TextRun txRun, HandlerCallback handlerCallback)
    pub fn TextRunsHandler(txRun: Rc<RefCell<TextRun>>, handlerCallback: Option<Rc<RefCell<HandlerCallback>>>) -> Self {
      // java this.init(txRun);
      let mut this = Self { super_: DefaultElementHandler::default(), txRun: txRun.clone(), handlerCallback: handlerCallback };
      this.init(txRun);
      // java this.handlerCallback = handlerCallback;
      this
    }

    // java public void init(TextRun txRun)
    pub fn init(&mut self, txRun: Rc<RefCell<TextRun>>) {
      // java this.txRun = txRun;
      self.txRun = txRun;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 1114193:
        1114193 => {
          // java this.txRun.putMode(0);
          self.txRun.borrow_mut().putMode(0);
        }
        // java case 1114618:
        1114618 => {
          // java this.txRun.putMode(1);
          self.txRun.borrow_mut().putMode(1);
          // java this.txRun.putLength(1);
          self.txRun.borrow_mut().putLength(1);
        }
        // java case 1114619:
        1114619 => {
          // java XmlRoAttr tmp = attrsNotCopy.getByName(1114245);
          let mut tmp = attrsNotCopy.get_by_name(1114245);
          // java if (tmp != null)
          if tmp.is_some() {
            // java this.txRun.fieldRun().putId(tmp.toString());
            self.txRun.borrow_mut().field_id = Some(tmp.unwrap().to_string());
            // java tmp = attrsNotCopy.getByName(1114388);
            tmp = attrsNotCopy.get_by_name(1114388);
            // java if (tmp != null)
            if tmp.is_some() {
              // java this.txRun.fieldRun().putType(tmp.toString());
              self.txRun.borrow_mut().field_type = Some(tmp.unwrap().to_string());
            }
            // java this.txRun.putMode(2);
            self.txRun.borrow_mut().putMode(2);
          }
        }
        // java default:
        _ => {
          // rust: no-stmt
        }
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java switch (subElement)
      match subElement {
        // java case 1114214:
        1114214 => {
          // java switch (this.txRun.mode())
          match self.txRun.borrow().mode() {
            // java case 0:
            0 => {
              // java return new RegularTextHandler(this.txRun);
              return Some(Box::new(RegularTextHandler::RegularTextHandler(self.txRun.clone())));
            }
            // java case 2:
            2 => {
              // java return new FieldTextHandler(this.txRun);
              return Some(Box::new(FieldTextHandler::FieldTextHandler(self.txRun.clone())));
            }
            // java default:
            _ => {
              // java return null;
              return None;
            }
          }
        }
        // java case 1114531:
        1114531 => {
          // java if (this.txRun.mode() != 2)
          if self.txRun.borrow().mode() != 2 {
            // java return null;
            return None;
          }
          // java return new TextParagraphPropertiesHandler(this.txRun.fieldRun().pPr(), this.handlerCallback);
          return Some(Box::new(TextParagraphPropertiesHandler::TextParagraphPropertiesHandler(
            TextParagraphPropertiesHandler_ppr(),
            callback_state(&self.handlerCallback),
          )));
        }
        // java case 1114603:
        1114603 => {
          // java switch (this.txRun.mode())
          match self.txRun.borrow().mode() {
            // java case 0:
            0 => {
              // java return new TextCharacterPropertiesHandler(this.txRun.regularRun().rPr(), this.handlerCallback);
              return Some(Box::new(TextCharacterPropertiesHandler::TextCharacterPropertiesHandler(
                self.txRun.borrow().regular_rpr(),
                self.handlerCallback.clone(),
              )));
            }
            // java case 1:
            1 => {
              // java return new TextCharacterPropertiesHandler(this.txRun.linebreakRun().rPr(), this.handlerCallback);
              return Some(Box::new(TextCharacterPropertiesHandler::TextCharacterPropertiesHandler(
                self.txRun.borrow().linebreak_rpr(),
                self.handlerCallback.clone(),
              )));
            }
            // java case 2:
            2 => {
              // java return new TextCharacterPropertiesHandler(this.txRun.fieldRun().rPr(), this.handlerCallback);
              return Some(Box::new(TextCharacterPropertiesHandler::TextCharacterPropertiesHandler(
                self.txRun.borrow().field_rpr(),
                self.handlerCallback.clone(),
              )));
            }
            // java default:
            _ => {
              // java return null;
              return None;
            }
          }
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
    }

}

fn TextParagraphPropertiesHandler_ppr() -> crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphProperties {
    crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphProperties { slot: 1114531 }
}

impl ElementHandler for TextRunsHandler {
    fn kind(&self) -> &'static str { "TextRunsHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        TextRunsHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextRunsHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

impl ElementHandler for TextRunsHandlerCell {
    fn kind(&self) -> &'static str { "TextRunsHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        self.0.borrow_mut().startElement(element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.borrow_mut().enterSubElement(subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

// java static class RegularTextHandler extends DefaultElementHandler
impl RegularTextHandler {
    // java public  RegularTextHandler(TextRun txRun)
    pub fn RegularTextHandler(txRun: Rc<RefCell<TextRun>>) -> Self {
      // java this.txRun = txRun;
      Self { super_: DefaultElementHandler::default(), txRun: txRun }
    }

    // java public void characters(String chars)
    pub fn characters(&mut self, chars: &str) {
      // java this.txRun.regularRun().putText(chars);
      self.txRun.borrow_mut().regular_text = chars.to_string();
      // java this.txRun.putLength(chars.length());
      self.txRun.borrow_mut().putLength(chars.len() as i32);
    }

}

impl ElementHandler for RegularTextHandler {
    fn kind(&self) -> &'static str { "RegularTextHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
    fn characters(&mut self, chars: &str) {
        RegularTextHandler::characters(self, chars)
    }
}

// java static class FieldTextHandler extends DefaultElementHandler
impl FieldTextHandler {
    // java public  FieldTextHandler(TextRun txRun)
    pub fn FieldTextHandler(txRun: Rc<RefCell<TextRun>>) -> Self {
      // java this.txRun = txRun;
      Self { super_: DefaultElementHandler::default(), txRun: txRun }
    }

    // java public void characters(String chars)
    pub fn characters(&mut self, chars: &str) {
      // java this.txRun.fieldRun().putText(chars);
      self.txRun.borrow_mut().field_text = chars.to_string();
      // java this.txRun.putLength(chars.length());
      self.txRun.borrow_mut().putLength(chars.len() as i32);
    }

}

impl ElementHandler for FieldTextHandler {
    fn kind(&self) -> &'static str { "FieldTextHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
    fn characters(&mut self, chars: &str) {
        FieldTextHandler::characters(self, chars)
    }
}


