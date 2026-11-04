mod text_list_style_compile_surface {
    use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphProperties;

    #[derive(Default)]
    pub struct TextListStyle {}

    impl TextListStyle {
        pub fn create() -> Self {
            Self {}
        }
        pub fn defpPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114533 }
        }
        pub fn lvl1pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114534 }
        }
        pub fn lvl2pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114535 }
        }
        pub fn lvl3pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114536 }
        }
        pub fn lvl4pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114537 }
        }
        pub fn lvl5pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114538 }
        }
        pub fn lvl6pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114539 }
        }
        pub fn lvl7pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114540 }
        }
        pub fn lvl8pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114541 }
        }
        pub fn lvl9pPr(&mut self) -> TextParagraphProperties {
            TextParagraphProperties { slot: 1114542 }
        }
    }
}

// Java: import cn.wps.drawing.data.text.TextListStyle
use text_list_style_compile_surface::TextListStyle as _TextListStyleImport;
pub use text_list_style_compile_surface::TextListStyle;
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphProperties;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.paragraph.TextParagraphPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::paragraph::TextParagraphPropertiesHandler::TextParagraphPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallbackState;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use std::cell::RefCell;
use std::rc::Rc;

// java public class TextListStyleHandler extends DefaultElementHandler
pub struct TextListStyleHandler {
    // java private TextListStyle listStyle;
    listStyle: Rc<RefCell<TextListStyle>>,
    // java private HandlerCallback handlerCallback;
    handlerCallback: HandlerCallbackState,
    // java private TextParagraphPropertiesHandler textParagraphPropertiesHandler;
    textParagraphPropertiesHandler: Option<TextParagraphPropertiesHandler>,
}

// java public class TextListStyleHandler extends DefaultElementHandler
impl TextListStyleHandler {
    // java public  TextListStyleHandler(TextListStyle listStyle, HandlerCallback handlerCallback)
    pub fn TextListStyleHandler(
        listStyle: Rc<RefCell<TextListStyle>>,
        handlerCallback: HandlerCallbackState,
    ) -> Self {
      // java this.listStyle = listStyle;
      Self {
        listStyle,
        handlerCallback,
        textParagraphPropertiesHandler: None,
      }
      // java this.handlerCallback = handlerCallback;
    }

    pub fn reused_handler_init_count(&self) -> i32 {
        self.textParagraphPropertiesHandler
            .as_ref()
            .map(|h| h.init_count)
            .unwrap_or(0)
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java TextParagraphProperties textParagraphProperties = null;
      let mut textParagraphProperties: Option<TextParagraphProperties> = None;
      // java switch (subElement)
      match subElement {
        // java case 1114533:
        1114533 => {
          // java textParagraphProperties = this.listStyle.defpPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().defpPr());
        }
        // java case 1114534:
        1114534 => {
          // java textParagraphProperties = this.listStyle.lvl1pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl1pPr());
        }
        // java case 1114535:
        1114535 => {
          // java textParagraphProperties = this.listStyle.lvl2pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl2pPr());
        }
        // java case 1114536:
        1114536 => {
          // java textParagraphProperties = this.listStyle.lvl3pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl3pPr());
        }
        // java case 1114537:
        1114537 => {
          // java textParagraphProperties = this.listStyle.lvl4pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl4pPr());
        }
        // java case 1114538:
        1114538 => {
          // java textParagraphProperties = this.listStyle.lvl5pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl5pPr());
        }
        // java case 1114539:
        1114539 => {
          // java textParagraphProperties = this.listStyle.lvl6pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl6pPr());
        }
        // java case 1114540:
        1114540 => {
          // java textParagraphProperties = this.listStyle.lvl7pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl7pPr());
        }
        // java case 1114541:
        1114541 => {
          // java textParagraphProperties = this.listStyle.lvl8pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl8pPr());
        }
        // java case 1114542:
        1114542 => {
          // java textParagraphProperties = this.listStyle.lvl9pPr();
          textParagraphProperties = Some(self.listStyle.borrow_mut().lvl9pPr());
        }
        // java default:
        _ => {
          // rust: no-stmt
        }
      }
      // java if (textParagraphProperties != null)
      if let Some(textParagraphProperties) = textParagraphProperties {
        // java if (this.textParagraphPropertiesHandler == null)
        if self.textParagraphPropertiesHandler.is_none() {
          // java this.textParagraphPropertiesHandler = new TextParagraphPropertiesHandler(textParagraphProperties, this.handlerCallback) { };
          self.textParagraphPropertiesHandler = Some(TextParagraphPropertiesHandler::TextParagraphPropertiesHandler(
            textParagraphProperties,
            self.handlerCallback,
          ));
        }
        // java else
        else {
          // java this.textParagraphPropertiesHandler.init(textParagraphProperties);
          self.textParagraphPropertiesHandler.as_mut().unwrap().init(textParagraphProperties);
        }
        // java handler = this.textParagraphPropertiesHandler;
        handler = Some(Box::new(
          self.textParagraphPropertiesHandler.as_ref().unwrap().clone(),
        ));
      }
      // java return handler;
      return handler;
    }
}

impl ElementHandler for TextListStyleHandler {
    fn kind(&self) -> &'static str {
        "TextListStyleHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextListStyleHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}
