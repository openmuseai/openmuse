use crate::kfc::xpxml::hlxml::HLElementType::HLElementType;
use crate::kfc::xpxml::hlxml::HLLabelParser::HLLabelParser;
use crate::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;

// Java: public class HLStrHandlerBuilder implements HLInnerAccepter
pub struct HLStrHandlerBuilder {
    // Java field: Stack<ElementStrHandler> mHandlers
    mHandlers: Vec<Box<dyn ElementStrHandler>>,
    // Java field: boolean bFirstElement
    bFirstElement: bool,
    // Java field: int mSkipLevel
    mSkipLevel: i32,
    // Java field: int mLastElemType
    mLastElemType: i32,
}

// Java: public class HLStrHandlerBuilder implements HLInnerAccepter
impl HLStrHandlerBuilder {
    // Java: HLStrHandlerBuilder(ElementStrHandler handler)
    pub fn new(handler: Box<dyn ElementStrHandler>) -> Self {
        // Java: super();
        let mut handlers: Vec<Box<dyn ElementStrHandler>> = Vec::new();
        // Java: this.mHandlers.add(handler);
        handlers.push(handler);
        HLStrHandlerBuilder {
            mHandlers: handlers,
            bFirstElement: true,
            mSkipLevel: 0,
            mLastElemType: -1,
        }
    }

    // Java: void startElement(String label)
    pub fn start_element(&mut self, label: &str) {
        // Java: if (this.bFirstElement)
        if self.bFirstElement {
            // Java: this.bFirstElement = false;
            self.bFirstElement = false;
            // Java: ((ElementStrHandler)this.mHandlers.peek()).startElement(label);
            self.mHandlers.last_mut().expect("EmptyStackException").start_element(label);
        }
        // Java: else
        else {
            // Java: ElementStrHandler handler = ((ElementStrHandler)this.mHandlers.peek()).enterSubElement(label);
            let sub = self.mHandlers.last_mut().expect("EmptyStackException").enter_sub_element(label);
            // Java: if (handler != null)
            if let Some(mut sub_handler) = sub {
                // Java: handler.startElement(label);
                sub_handler.start_element(label);
                // Java: this.mHandlers.add(handler);
                self.mHandlers.push(sub_handler);
            }
            // Java: else
            else {
                // Java: this.mSkipLevel = 1;
                self.mSkipLevel = 1;
            }
        }
    }

    // Java: void endElement(String label)
    pub fn end_element(&mut self, label: &str) {
        // Java: if (this.mSkipLevel > 0)
        if self.mSkipLevel > 0 {
            // Java: --this.mSkipLevel;
            self.mSkipLevel -= 1;
        }
        // Java: else
        else {
            // Java: ((ElementStrHandler)this.mHandlers.peek()).endElement(label);
            self.mHandlers.last_mut().expect("EmptyStackException").end_element(label);
            // Java: this.mHandlers.pop();
            self.mHandlers.pop();
        }
    }

    // Java: void addAttr(String name, String value)
    pub fn add_attr(&mut self, name: &str, value: &str) {
        // Java: if (this.mSkipLevel <= 0)
        if self.mSkipLevel <= 0 {
            // Java: ((ElementStrHandler)this.mHandlers.peek()).addAttr(name, value);
            self.mHandlers.last_mut().expect("EmptyStackException").add_attr(name, value);
        }
    }

    // Java: void label(char[] chars, int start, int end)
    pub fn label(&mut self, chars: &[char], start: i32, end: i32) {
        // Java: int type = HLElementType.getType(chars, start, end);
        let label_type = HLElementType::get_type_chars(chars, start as usize, end as usize);
        // Java: this.mLastElemType = type;
        self.mLastElemType = label_type;
        // Java: if (type != 0 && type != 1 && type != 2 && type != 3 && type != 4)
        if label_type != 0 && label_type != 1 && label_type != 2 && label_type != 3 && label_type != 4 {
            // Java: String tmp = new String(chars, start, end - start + 1);
            let tmp: String = chars[start as usize..=(end as usize)].iter().collect();
            // Java: throw new IllegalArgumentException("意外错误:" + tmp);
            panic!("意外错误:{}", tmp);
        }
        // Java: else if (this.mSkipLevel <= 0)
        else if self.mSkipLevel <= 0 {
            // Java: HLStrLabelParser.parseLabel(chars, start, end, type, this);
            super::HLStrLabelParser::HLStrLabelParser::parseLabel(chars,start as usize,end as usize,label_type,self);
        }
        // Java: else
        else {
            // Java: if (type != 0 && type != 1)
            if label_type != 0 && label_type != 1 {
                // Java: if (type == 4)
                if label_type == 4 {
                    // Java: --this.mSkipLevel;
                    self.mSkipLevel -= 1;
                }
            }
            // Java: else
            else {
                // Java: ++this.mSkipLevel;
                self.mSkipLevel += 1;
            }
        }
    }

    // Java: void characters(char[] chars, int start, int end, boolean bEscape)
    pub fn characters(&mut self, chars: &[char], start: i32, end: i32, b_escape: bool) {
        // Java: if (this.mSkipLevel == 0 && (this.mLastElemType == 0 || this.mLastElemType == 1))
        if self.mSkipLevel == 0 && (self.mLastElemType == 0 || self.mLastElemType == 1) {
            // Java: String tmp = HLLabelParser.buildValue(chars, start, end, bEscape);
            let tmp = HLLabelParser::build_value(chars, start as usize, end as usize, b_escape);
            // Java: ((ElementStrHandler)this.mHandlers.peek()).characters(tmp);
            self.mHandlers.last_mut().expect("EmptyStackException").characters(&tmp);
        }
    }
}

impl super::super::HLInnerAccepter::HLInnerAccepter for HLStrHandlerBuilder{fn label(&mut self,c:&[char],s:usize,e:usize){HLStrHandlerBuilder::label(self,c,s as i32,e as i32)}fn characters(&mut self,c:&[char],s:usize,e:usize,b:bool){HLStrHandlerBuilder::characters(self,c,s as i32,e as i32,b)}}
