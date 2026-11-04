// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.ElementHandler
// use crate::org::dom4j::ElementHandler;
// Java: import org.dom4j.ElementPath
// use crate::org::dom4j::ElementPath;

// Java: public class SAXModifyElementHandler implements ElementHandler
pub struct SAXModifyElementHandler {
    // Java field: ElementModifier elemModifier
    elemModifier: i32,
    // Java field: Element modifiedElement
    modifiedElement: i32,
}

// Java: public class SAXModifyElementHandler implements ElementHandler
impl SAXModifyElementHandler {
    // Java: SAXModifyElementHandler(ElementModifier elemModifier)
    pub fn new(_elemModifier: i32) -> Self {
      // Java: super();
      // Java: this.elemModifier = elemModifier;
      Self { elemModifier: _elemModifier, modifiedElement: 0 }
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart(&self, _elementPath: i32) {
      // Java: this.modifiedElement = elementPath.getCurrent();
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters(&self, _chars: &str) {
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd(&self, _elementPath: i32) {
      // Java: try
      {
         // Java: Element origElement = elementPath.getCurrent();
         // Java: Element currentParent = origElement.getParent();
         // Java: if (currentParent != null)
         {
            // Java: Element clonedElem = (Element)origElement.clone();
            // Java: this.modifiedElement = this.elemModifier.modifyElement(clonedElem);
            // Java: if (this.modifiedElement != null)
            {
               // Java: this.modifiedElement.setParent(origElement.getParent());
               // Java: this.modifiedElement.setDocument(origElement.getDocument());
               // Java: int contentIndex = currentParent.indexOf(origElement);
               // Java: currentParent.content().set(contentIndex, this.modifiedElement);
            }
            // Java: origElement.detach();
         }
         // Java: else if (origElement.isRootElement())
         {
            // Java: Element clonedElem = (Element)origElement.clone();
            // Java: this.modifiedElement = this.elemModifier.modifyElement(clonedElem);
            // Java: if (this.modifiedElement != null)
            {
               // Java: this.modifiedElement.setDocument(origElement.getDocument());
               // Java: Document doc = origElement.getDocument();
               // Java: doc.setRootElement(this.modifiedElement);
            }
            // Java: origElement.detach();
         }
         // Java: if (elementPath instanceof ElementStack)
         {
            // Java: ElementStack elementStack = (ElementStack)elementPath;
            // Java: elementStack.popElement();
            // Java: elementStack.pushElement(this.modifiedElement);
         }
      }
      // Java: catch (Exception ex)
      {
         // Java: throw new SAXModifyException(ex);
      }
    }

    // Java: boolean shouldNotifyChild()
    pub fn shouldNotifyChild(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: void onStartChild(ElementPath elementPath)
    pub fn onStartChild(&self, _elementPath: i32) {
    }

    // Java: void onEndChild(ElementPath elementPath)
    pub fn onEndChild(&self, _elementPath: i32) {
    }

    // Java: Element getModifiedElement()
    pub fn getModifiedElement(&self) -> i32 {
      // Java: return this.modifiedElement;
      0
    }

}

// impl ElementHandler for SAXModifyElementHandler {
// }
