// Java: import org.dom4j.Element
// Java: import org.dom4j.ElementHandler
use crate::org::dom4j::io::ElementStack::ElementStack;

// Java: public class PruningElementStack extends ElementStack
pub struct PruningElementStack {
    // Java field: ElementHandler elementHandler
    pub(crate) base: ElementStack,
    // Java field: String[] path
    pub(crate) path: Vec<String>,
    // Java field: int matchingElementIndex
    pub(crate) matchingElementIndex: i32,
}

// Java: public class PruningElementStack extends ElementStack
impl PruningElementStack {
    // Java: PruningElementStack(String[] path, ElementHandler elementHandler)
    pub fn new(path: Vec<String>) -> Self {
        // Java: super(false);
        // Java: this.path = path;
        // Java: this.elementHandler = elementHandler;
        // Java: this.checkPath();
        let matching = if path.len() >= 2 { path.len() as i32 - 2 } else { 0 };
        PruningElementStack {
            base: ElementStack::new(false),
            path,
            matchingElementIndex: matching,
        }
    }

    // Java: PruningElementStack(String[] path, ElementHandler elementHandler, int defaultCapacity)
    pub fn new_with_capacity(path: Vec<String>, default_capacity: usize) -> Self {
        // Java: super(defaultCapacity, false);
        // Java: this.path = path;
        // Java: this.elementHandler = elementHandler;
        // Java: this.checkPath();
        let matching = if path.len() >= 2 { path.len() as i32 - 2 } else { 0 };
        PruningElementStack {
            base: ElementStack::new_with_capacity(default_capacity, false),
            path,
            matchingElementIndex: matching,
        }
    }

    // Java: Element popElement()
    pub fn popElement(&mut self) -> Option<String> {
        // Java: Element answer = super.popElement();
        let answer = self.base.popElement();
        // Java: if (this.lastElementIndex == this.matchingElementIndex && this.lastElementIndex >= 0 && this.validElement(answer, this.lastElementIndex + 1))
        {
            // Java: Element parent = null;
            // Java: for(int i = 0; i <= this.lastElementIndex; ++i)
            {
                // Java: parent = this.stack[i];
                // Java: if (!this.validElement(parent, i))
                {
                    // Java: parent = null;
                    // Java: break;
                }
            }
            // Java: if (parent != null)
            {
                // Java: this.pathMatches(parent, answer);
            }
        }
        // Java: return answer;
        answer
    }

    // Java: void pathMatches(Element parent, Element selectedNode)
    pub fn pathMatches(&self) {
        // Java: this.elementHandler.onEnd(this);
        // Java: parent.remove((Element)selectedNode);
    }

    // Java: boolean validElement(Element element, int index)
    pub fn validElement(&self, name: &str, index: usize) -> bool {
        // Java: String requiredName = this.path[index];
        let required = self.path.get(index);
        // Java: String name = element.getName();
        // Java: if (requiredName == name)
        if let Some(req) = required {
            if req == name
            {
                // Java: return true;
                return true;
            }
        }
        // Java: else
        {
            // Java: return requiredName != null && name != null ? requiredName.equals(name) : false;
            false
        }
    }

    // Java: void checkPath()
    fn checkPath(&self) {
        // Java: if (this.path.length < 2)
        if self.path.len() < 2
        {
            // Java: throw new RuntimeException("Invalid path of length: " + this.path.length + " it must be greater than 2");
            panic!("Invalid path of length: {} it must be greater than 2", self.path.len());
        }
        // Java: else
        {
            // Java: this.matchingElementIndex = this.path.length - 2;
        }
    }
}
