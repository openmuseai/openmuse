use crate::kfc::xpxml::hlxml::HLElementType::HLElementType;
use crate::kfc::xpxml::hlxml::HLInnerAccepter::HLInnerAccepter;
use crate::kfc::xpxml::hlxml::HLLabelParser::HLLabelParser;
use crate::kfc::xpxml::hlxml::Mode::Mode;
use crate::kfc::xpxml::reader::ElementHandler::ElementHandler;
use crate::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
use crate::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

struct PtrIds(*const dyn XmlName2Id);

impl XmlName2Id for PtrIds {
    fn get_id(&self, prefix: i32, name: i32) -> i32 {
        unsafe { (*self.0).get_id(prefix, name) }
    }
}

// Java: public final class HLHandlerBuilder implements HLInnerAccepter
pub struct HLHandlerBuilder {
    // Java field: Stack<ElementHandler> mHandlers
    mHandlers: Vec<Box<dyn ElementHandler>>,
    // Java field: HLLabelParser mLabelParser
    mLabelParser: HLLabelParser,
    // Java field: boolean bFirstElement
    bFirstElement: bool,
    // Java field: int mSkipLevel
    mSkipLevel: i32,
    // Java field: int mLastElemType
    mLastElemType: i32,
}

// Java: public final class HLHandlerBuilder implements HLInnerAccepter
impl HLHandlerBuilder {
    // Java: HLHandlerBuilder(XmlName2Id xmlIds, ElementHandler handler, boolean isBeginAtRoot, Mode mode)
    pub fn new(
        xml_ids: *const dyn XmlName2Id,
        handler: Box<dyn ElementHandler>,
        is_begin_at_root: bool,
        mode: Mode,
    ) -> Self {
        // Java: this.mSkipLevel = 0;
        // Java: this.mLastElemType = -1;
        // Java: this.mHandlers = new Stack();
        // Java: this.mHandlers.push(handler);
        // Java: this.mLabelParser = new HLLabelParser(xmlIds, mode);
        // Java: this.mRoAttrs = new XmlRoAttrList();
        // Java: this.mLabelParser.setRoAttr(this.mRoAttrs);
        // Java: this.bFirstElement = isBeginAtRoot;
        let mut parser = HLLabelParser::new(Box::new(PtrIds(xml_ids)), mode);
        parser.set_ro_attr(XmlRoAttrList::new());
        HLHandlerBuilder {
            mHandlers: vec![handler],
            mLabelParser: parser,
            bFirstElement: is_begin_at_root,
            mSkipLevel: 0,
            mLastElemType: -1,
        }
    }

    // Java: void setLabelParserPrefix(int labelParserPrefix)
    pub fn set_label_parser_prefix(&mut self, label_parser_prefix: i32) {
        // Java: this.mLabelParser.setOuterLabelPrefix(labelParserPrefix);
        self.mLabelParser.set_outer_label_prefix(label_parser_prefix);
    }

    pub fn into_root(mut self) -> Box<dyn ElementHandler> {
        while self.mHandlers.len() > 1 {
            self.mHandlers.pop();
        }
        self.mHandlers.pop().expect("HLHandlerBuilder root")
    }
}

impl HLInnerAccepter for HLHandlerBuilder {
    // Java: void label(char[] chars, int start, int end)
    fn label(&mut self, chars: &[char], start: usize, end: usize) {
        // Java: int type = HLElementType.getType(chars, start, end);
        let label_type = HLElementType::get_type_chars(chars, start, end);
        // Java: this.mLastElemType = type;
        self.mLastElemType = label_type;
        // Java: if (type != 0 && type != 1 && type != 2 && type != 3 && type != 4)
        if label_type != 0 && label_type != 1 && label_type != 2 && label_type != 3 && label_type != 4 {
            // Java: throw new IllegalArgumentException("意外错误:" + tmp);
            let tmp: String = chars[start..=end].iter().collect();
            panic!("意外错误:{tmp}");
        }
        // Java: else if (this.mSkipLevel > 0)
        if self.mSkipLevel > 0 {
            // Java: if (type != 0 && type != 1)
            if label_type != 0 && label_type != 1 {
                // Java: if (type == 4) --this.mSkipLevel;
                if label_type == 4 {
                    self.mSkipLevel -= 1;
                }
            } else {
                // Java: ++this.mSkipLevel;
                self.mSkipLevel += 1;
            }
            return;
        }
        // Java: this.mRoAttrs.reset();
        self.mLabelParser.reset_attrs();
        // Java: this.mLabelParser.parseLabel(chars, start, end, type);
        self.mLabelParser.parse_label(chars, start as i32, end as i32, label_type);
        // Java: int element = this.mLabelParser.getLabel();
        let element = self.mLabelParser.get_label();
        let attrs = self.mLabelParser.take_attrs();
        // Java: if (element == -1)
        if element == -1 {
            // Java: if (type == 0 || type == 1) this.mSkipLevel = 1;
            if label_type == 0 || label_type == 1 {
                self.mSkipLevel = 1;
            }
        }
        // Java: else if (this.bFirstElement)
        else if self.bFirstElement {
            // Java: this.topHandler().startElement(element, this.mRoAttrs);
            if let Some(handler) = self.mHandlers.last_mut() {
                handler.startElement(element, &attrs);
            }
            // Java: if (type == 2 || type == 3) this.topHandler().endElement(element);
            if label_type == 2 || label_type == 3 {
                if let Some(handler) = self.mHandlers.last_mut() {
                    handler.endElement(element);
                }
            }
            // Java: this.bFirstElement = false;
            self.bFirstElement = false;
        } else if label_type != 0 && label_type != 1 {
            // Java: if (type == 4)
            if label_type == 4 {
                // Java: this.topHandler().endElement(element);
                if let Some(handler) = self.mHandlers.last_mut() {
                    handler.endElement(element);
                }
                // Java: if (this.mHandlers.size() > 1) this.mHandlers.pop();
                if self.mHandlers.len() > 1 {
                    self.mHandlers.pop();
                }
            } else {
                // Java: ElementHandler subHandler = this.topHandler().enterSubElement(element);
                let sub = self.mHandlers.last_mut().and_then(|h| h.enterSubElement(element));
                // Java: if (subHandler != null) { startElement; endElement; }
                if let Some(mut sub) = sub {
                    sub.startElement(element, &attrs);
                    sub.endElement(element);
                }
            }
        } else {
            // Java: ElementHandler subHandler = this.topHandler().enterSubElement(element);
            let sub = self.mHandlers.last_mut().and_then(|h| h.enterSubElement(element));
            // Java: if (subHandler != null) { push; startElement; } else { mSkipLevel = 1; }
            if let Some(mut sub) = sub {
                sub.startElement(element, &attrs);
                self.mHandlers.push(sub);
            } else {
                self.mSkipLevel = 1;
            }
        }
        self.mLabelParser.set_ro_attr(attrs);
    }

    // Java: void characters(char[] chars, int start, int end, boolean bEscape)
    fn characters(&mut self, chars: &[char], start: usize, end: usize, b_escape: bool) {
        // Java: if (this.mSkipLevel == 0 && (this.mLastElemType == 0 || this.mLastElemType == 1))
        if self.mSkipLevel == 0 && (self.mLastElemType == 0 || self.mLastElemType == 1) {
            // Java: String tmp = HLLabelParser.buildValue(chars, start, end, bEscape);
            let tmp = HLLabelParser::build_value(chars, start, end, b_escape);
            // Java: this.topHandler().characters(tmp);
            if let Some(handler) = self.mHandlers.last_mut() {
                handler.characters(&tmp);
            }
        }
    }
}
