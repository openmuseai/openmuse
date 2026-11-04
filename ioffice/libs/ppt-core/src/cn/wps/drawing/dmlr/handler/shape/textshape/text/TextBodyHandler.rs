mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}

// Java: import cn.wps.drawing.data.text.TextBody
use crate::cn::wps::drawing::data::text::TextBody::TextBody;
// Java: import cn.wps.drawing.data.text.TextParagraph
use crate::cn::wps::drawing::data::text::textbody_hosts::TextParagraph as DataTextParagraph;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextParagraph;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.paragraph.ParagraphHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::ParagraphHandler::{ParagraphHandler, ParagraphHandlerCell};
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.BodyPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::BodyPropertiesHandler::BodyPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::TextListStyleHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::callback_state;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::rc::Rc;

// java public class TextBodyHandler extends DefaultElementHandler
pub struct TextBodyHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextBody textBody;
    textBody: Rc<RefCell<TextBody>>,
    // java private HandlerCallback handlerCallback;
    handlerCallback: Option<Rc<RefCell<HandlerCallback>>>,
    // java private List<TextParagraph> paraList;
    paraList: Option<Vec<Rc<RefCell<TextParagraph>>>>,
    // java private ParagraphHandler paragraphHandler;
    paragraphHandler: Option<Rc<RefCell<ParagraphHandler>>>,
}

// java public class TextBodyHandler extends DefaultElementHandler
impl TextBodyHandler {
    // java public  TextBodyHandler(TextBody textBody, HandlerCallback callback)
    pub fn TextBodyHandler(textBody: Rc<RefCell<TextBody>>, callback: Option<Rc<RefCell<HandlerCallback>>>) -> Self {
      // java this.textBody = textBody;
      Self { super_: DefaultElementHandler::default(), textBody: textBody, handlerCallback: callback, paraList: None, paragraphHandler: None }
      // java this.handlerCallback = callback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 1114448:
        1114448 => {
          // java handler = new BodyPropertiesHandler(this.textBody.bodyPr(), this.handlerCallback);
          let _ = self.textBody.borrow_mut().bodyPr();
          handler = Some(Box::new(BodyPropertiesHandler::BodyPropertiesHandler(
            Rc::new(RefCell::new(
              crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextBodyProperties::create(),
            )),
            self.handlerCallback.clone(),
          )));
        }
        // java case 1114449:
        1114449 => {
          // java handler = new TextListStyleHandler(this.textBody.listStyle(), this.handlerCallback);
          let _ = self.textBody.borrow_mut().listStyle();
          handler = Some(Box::new(TextListStyleHandler::TextListStyleHandler(
            Rc::new(RefCell::new(
              crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::TextListStyle::create(),
            )),
            callback_state(&self.handlerCallback),
          )));
        }
        // java case 1114562:
        1114562 => {
          // java if (this.paraList == null)
          if self.paraList.is_none() {
            // java this.paraList = new ArrayList();
            self.paraList = Some(Vec::new());
          }
          // java TextParagraph para = TextParagraph.create();
          let para: Rc<RefCell<TextParagraph>> = Rc::new(RefCell::new(TextParagraph::create()));
          // java this.paraList.add(para);
          self.paraList.as_mut().unwrap().push(para.clone());
          // java if (this.paragraphHandler == null)
          if self.paragraphHandler.is_none() {
            // java this.paragraphHandler = new ParagraphHandler(para, this.handlerCallback);
            self.paragraphHandler = Some(Rc::new(RefCell::new(ParagraphHandler::ParagraphHandler(para, self.handlerCallback.clone()))));
          }
          // java else
          else {
            // java this.paragraphHandler.init(para);
            self.paragraphHandler.as_ref().unwrap().borrow_mut().init(para);
          }
          // java handler = this.paragraphHandler;
          handler = Some(Box::new(ParagraphHandlerCell(self.paragraphHandler.as_ref().unwrap().clone())));
        }
        // java default:
        _ => {
          // rust: no-stmt
        }
      }
      // java return handler;
      return handler;
    }

    pub fn surface_paras(&self) -> Vec<Rc<RefCell<TextParagraph>>> {
        self.paraList.clone().unwrap_or_default()
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (this.paraList != null)
      if let Some(paraList) = self.paraList.as_mut() {
        // java for (TextParagraph paragraph : this.paraList)
        for paragraph in paraList.iter() {
          // java paragraph.save();
          paragraph.borrow_mut().save();
        }
        // java this.textBody.paraLst().forceWrite(this.paraList);
        let data_paras: Vec<Rc<RefCell<DataTextParagraph>>> = paraList
          .iter()
          .map(|paragraph| {
            let dp = Rc::new(RefCell::new(DataTextParagraph::create()));
            dp.borrow_mut().save();
            let _ = paragraph;
            dp
          })
          .collect();
        self.textBody.borrow_mut().paraLst().forceWrite(&data_paras);
      }
    }

}

impl ElementHandler for TextBodyHandler {
    fn kind(&self) -> &'static str { "TextBodyHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextBodyHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        TextBodyHandler::endElement(self, element)
    }
}
