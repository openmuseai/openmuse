// Java: import java.util.List
// Java: import org.dom4j.Document
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node
// Java: import org.dom4j.XPath
use crate::org::dom4j::rule::Action::Action;
use crate::org::dom4j::rule::Rule::Rule;
use crate::org::dom4j::rule::RuleManager::RuleManager;

// Java: public class Stylesheet
pub struct Stylesheet {
    // Java field: RuleManager ruleManager
    pub(crate) ruleManager: RuleManager,
    // Java field: String modeName
    pub(crate) modeName: Option<String>,
}

// Java: public class Stylesheet
impl Stylesheet {
    // Java: Stylesheet()
    pub fn new() -> Self {
        // Java: super();
        Stylesheet {
            ruleManager: RuleManager::new(),
            modeName: None,
        }
    }

    // Java: void addRule(Rule rule)
    pub fn addRule(&mut self, rule: Rule) {
        // Java: this.ruleManager.addRule(rule);
        self.ruleManager.addRule(rule);
    }

    // Java: void removeRule(Rule rule)
    pub fn removeRule(&mut self, rule: &Rule, index: usize) {
        // Java: this.ruleManager.removeRule(rule);
        self.ruleManager.removeRule(rule, index);
    }

    // Java: void run(Object input)
    pub fn run_obj(&self) {
        // Java: this.run(input, this.modeName);
    }

    // Java: void run(Object input, String mode)
    pub fn run_obj_mode(&self) {
        // Java: if (input instanceof Node)
        {
            // Java: this.run((Node)input, mode);
        }
        // Java: else if (input instanceof List)
        {
            // Java: this.run((List)input, mode);
        }
    }

    // Java: void run(List list)
    pub fn run_list(&self) {
        // Java: this.run(list, this.modeName);
    }

    // Java: void run(List list, String mode)
    pub fn run_list_mode(&self) {
        // Java: int i = 0;
        // Java: for(int size = list.size(); i < size; ++i)
        {
            // Java: Object object = list.get(i);
            // Java: if (object instanceof Node)
            {
                // Java: this.run((Node)object, mode);
            }
        }
    }

    // Java: void run(Node node)
    pub fn run_node(&self) {
        // Java: this.run(node, this.modeName);
    }

    // Java: void run(Node node, String mode)
    pub fn run_node_mode(&mut self, mode: &str) {
        // Java: Mode mod = this.ruleManager.getMode(mode);
        let _mode = self.ruleManager.getMode(mode);
        // Java: mod.fireRule(node);
        _mode.fireRule();
    }

    // Java: void applyTemplates(Object input, XPath xpath)
    pub fn applyTemplates_xpath(&self) {
        // Java: this.applyTemplates(input, xpath, this.modeName);
    }

    // Java: void applyTemplates(Object input, XPath xpath, String mode)
    pub fn applyTemplates_xpath_mode(&mut self, mode: &str) {
        // Java: Mode mod = this.ruleManager.getMode(mode);
        let _mode = self.ruleManager.getMode(mode);
        // Java: for(Node current : xpath.selectNodes(input))
        {
            // Java: mod.fireRule(current);
        }
    }

    // Java: void applyTemplates(Object input, org.jaxen.XPath xpath)
    pub fn applyTemplates_jaxen(&self) {
        // Java: this.applyTemplates(input, xpath, this.modeName);
    }

    // Java: void applyTemplates(Object input, org.jaxen.XPath xpath, String mode)
    pub fn applyTemplates_jaxen_mode(&mut self, mode: &str) {
        // Java: Mode mod = this.ruleManager.getMode(mode);
        let _mode = self.ruleManager.getMode(mode);
        // Java: for(Object current : xpath.selectNodes(input))
        {
            // Java: mod.fireRule((Node)current);
        }
    }

    // Java: void applyTemplates(Object input)
    pub fn applyTemplates_obj(&self) {
        // Java: this.applyTemplates(input, this.modeName);
    }

    // Java: void applyTemplates(Object input, String mode)
    pub fn applyTemplates_obj_mode(&mut self, mode: &str) {
        // Java: Mode mod = this.ruleManager.getMode(mode);
        let _mode = self.ruleManager.getMode(mode);
        // Java: if (input instanceof Element)
        {
            // Java: Element element = (Element)input;
            // Java: int i = 0;
            // Java: for(int size = element.nodeCount(); i < size; ++i)
            {
                // Java: Node node = element.node(i);
                // Java: mod.fireRule(node);
            }
        }
        // Java: else if (input instanceof Document)
        {
            // Java: Document document = (Document)input;
            // Java: int i = 0;
            // Java: for(int size = document.nodeCount(); i < size; ++i)
            {
                // Java: Node node = document.node(i);
                // Java: mod.fireRule(node);
            }
        }
        // Java: else if (input instanceof List)
        {
            // Java: List list = (List)input;
            // Java: int i = 0;
            // Java: for(int size = list.size(); i < size; ++i)
            {
                // Java: Object object = list.get(i);
                // Java: if (object instanceof Element)
                {
                    // Java: this.applyTemplates((Element)object, (String)mode);
                }
                // Java: else if (object instanceof Document)
                {
                    // Java: this.applyTemplates((Document)object, (String)mode);
                }
            }
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.ruleManager.clear();
        self.ruleManager.clear();
    }

    // Java: String getModeName()
    pub fn getModeName(&self) -> Option<&str> {
        // Java: return this.modeName;
        self.modeName.as_deref()
    }

    // Java: void setModeName(String modeName)
    pub fn setModeName(&mut self, mode_name: String) {
        // Java: this.modeName = modeName;
        self.modeName = Some(mode_name);
    }

    // Java: Action getValueOfAction()
    pub fn getValueOfAction(&self) -> &Option<Box<dyn Action>> {
        // Java: return this.ruleManager.getValueOfAction();
        self.ruleManager.getValueOfAction()
    }

    // Java: void setValueOfAction(Action valueOfAction)
    pub fn setValueOfAction(&mut self, value_of_action: Box<dyn Action>) {
        // Java: this.ruleManager.setValueOfAction(valueOfAction);
        self.ruleManager.setValueOfAction(value_of_action);
    }
}
