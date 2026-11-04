// Java: import java.util.HashMap
// Java: import org.dom4j.Document
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node
// Java: import org.dom4j.rule.pattern.NodeTypePattern
use std::collections::HashMap;
use crate::org::dom4j::rule::Action::Action;
use crate::org::dom4j::rule::Rule::Rule;
use crate::org::dom4j::rule::Mode::Mode;
use crate::org::dom4j::rule::Pattern::Pattern;

// Java: public class RuleManager
pub struct RuleManager {
    // Java field: HashMap modes
    pub(crate) modes: HashMap<String, Mode>,
    // Java field: int appearenceCount
    pub(crate) appearenceCount: i32,
    // Java field: Action valueOfAction
    pub(crate) valueOfAction: Option<Box<dyn Action>>,
}

// Java: public class RuleManager
impl RuleManager {
    // Java: RuleManager()
    pub fn new() -> Self {
        // Java: super();
        RuleManager {
            modes: HashMap::new(),
            appearenceCount: 0,
            valueOfAction: None,
        }
    }

    // Java: Mode getMode(String modeName)
    pub fn getMode(&mut self, mode_name: &str) -> &mut Mode {
        // Java: Mode mode = (Mode)this.modes.get(modeName);
        // Java: if (mode == null)
        if !self.modes.contains_key(mode_name)
        {
            // Java: mode = this.createMode();
            let mode = Self::createMode_static();
            // Java: this.modes.put(modeName, mode);
            self.modes.insert(mode_name.to_string(), mode);
        }
        // Java: return mode;
        self.modes.get_mut(mode_name).unwrap()
    }

    // Java: void addRule(Rule rule)
    pub fn addRule(&mut self, mut rule: Rule) {
        // Java: rule.setAppearenceCount(++this.appearenceCount);
        self.appearenceCount += 1;
        rule.setAppearenceCount(self.appearenceCount);
        // Java: Mode mode = this.getMode(rule.getMode());
        let mode_name = rule.getMode().unwrap_or("").to_string();
        // Java: Rule[] childRules = rule.getUnionRules();
        let child_rules = rule.getUnionRules();
        // Java: if (childRules != null)
        if let Some(children) = child_rules
        {
            // Java: int i = 0;
            // Java: for(int size = childRules.length; i < size; ++i)
            for child in children
            {
                // Java: mode.addRule(childRules[i]);
                self.getMode(&mode_name).addRule(child);
            }
        }
        // Java: else
        else
        {
            // Java: mode.addRule(rule);
            self.getMode(&mode_name).addRule(rule);
        }
    }

    // Java: void removeRule(Rule rule)
    pub fn removeRule(&mut self, rule: &Rule, index: usize) {
        // Java: Mode mode = this.getMode(rule.getMode());
        let mode_name = rule.getMode().unwrap_or("").to_string();
        // Java: Rule[] childRules = rule.getUnionRules();
        // Java: if (childRules != null)
        {
            // Java: int i = 0;
            // Java: for(int size = childRules.length; i < size; ++i)
            {
                // Java: mode.removeRule(childRules[i]);
            }
        }
        // Java: else
        {
            // Java: mode.removeRule(rule);
            self.getMode(&mode_name).removeRule(rule, index);
        }
    }

    // Java: Rule getMatchingRule(String modeName, Node node)
    pub fn getMatchingRule(&self, mode_name: &str) -> Option<&Rule> {
        // Java: Mode mode = (Mode)this.modes.get(modeName);
        let mode = self.modes.get(mode_name);
        // Java: if (mode != null)
        if let Some(mode) = mode
        {
            // Java: return mode.getMatchingRule(node);
            return mode.getMatchingRule();
        }
        // Java: else
        {
            // Java: System.out.println("Warning: No Mode for mode: " + mode);
            eprintln!("Warning: No Mode for mode: {}", mode_name);
            // Java: return null;
            None
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.modes.clear();
        self.modes.clear();
        // Java: this.appearenceCount = 0;
        self.appearenceCount = 0;
    }

    // Java: Action getValueOfAction()
    pub fn getValueOfAction(&self) -> &Option<Box<dyn Action>> {
        // Java: return this.valueOfAction;
        &self.valueOfAction
    }

    // Java: void setValueOfAction(Action valueOfAction)
    pub fn setValueOfAction(&mut self, value_of_action: Box<dyn Action>) {
        // Java: this.valueOfAction = valueOfAction;
        self.valueOfAction = Some(value_of_action);
    }

    // Java: Mode createMode()
    fn createMode_static() -> Mode {
        // Java: Mode mode = new Mode();
        let mode = Mode::new();
        // Java: this.addDefaultRules(mode);
        // Java: return mode;
        mode
    }

    // Java: void addDefaultRules(final Mode mode)
    pub fn addDefaultRules(&self, _mode: &mut Mode) {
        // Java: Action applyTemplates = new Action()
        {
            // Java: public void run(Node node) throws Exception
            {
                // Java: if (node instanceof Element)
                {
                    // Java: mode.applyTemplates((Element)node);
                }
                // Java: else if (node instanceof Document)
                {
                    // Java: mode.applyTemplates((Document)node);
                }
            }
        }
        // Java: Action valueOf = this.getValueOfAction();
        // Java: this.addDefaultRule(mode, NodeTypePattern.ANY_DOCUMENT, applyTemplates);
        // Java: this.addDefaultRule(mode, NodeTypePattern.ANY_ELEMENT, applyTemplates);
        // Java: if (valueOf != null)
        {
            // Java: this.addDefaultRule(mode, NodeTypePattern.ANY_ATTRIBUTE, valueOf);
            // Java: this.addDefaultRule(mode, NodeTypePattern.ANY_TEXT, valueOf);
        }
    }

    // Java: void addDefaultRule(Mode mode, Pattern pattern, Action action)
    pub fn addDefaultRule(&self, _mode: &mut Mode, _pattern: &dyn Pattern, _action: &dyn Action) {
        // Java: Rule rule = this.createDefaultRule(pattern, action);
        // Java: mode.addRule(rule);
    }

    // Java: Rule createDefaultRule(Pattern pattern, Action action)
    pub fn createDefaultRule(&self) -> Rule {
        // Java: Rule rule = new Rule(pattern, action);
        let mut rule = Rule::new();
        // Java: rule.setImportPrecedence(-1);
        rule.setImportPrecedence(-1);
        // Java: return rule;
        rule
    }
}
