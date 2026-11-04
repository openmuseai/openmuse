// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import org.dom4j.Node
use crate::org::dom4j::rule::Rule::Rule;

// Java: public class RuleSet
pub struct RuleSet {
    // Java field: ArrayList rules
    pub(crate) rules: Vec<Rule>,
    // Java field: Rule[] ruleArray
    pub(crate) ruleArray: Option<Vec<Rule>>,
}

// Java: public class RuleSet
impl RuleSet {
    // Java: RuleSet()
    pub fn new() -> Self {
        // Java: super();
        RuleSet {
            rules: Vec::new(),
            ruleArray: None,
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + " [RuleSet: " + this.rules + " ]";
        format!("RuleSet [RuleSet: {} rules ]", self.rules.len())
    }

    // Java: Rule getMatchingRule(Node node)
    pub fn getMatchingRule(&self) -> Option<&Rule> {
        // Java: Rule[] matches = this.getRuleArray();
        // Java: for(int i = matches.length - 1; i >= 0; --i)
        for rule in self.rules.iter().rev()
        {
            // Java: Rule rule = matches[i];
            // Java: if (rule.matches(node))
            if rule.matches()
            {
                // Java: return rule;
                return Some(rule);
            }
        }
        // Java: return null;
        None
    }

    // Java: void addRule(Rule rule)
    pub fn addRule(&mut self, rule: Rule) {
        // Java: this.rules.add(rule);
        self.rules.push(rule);
        // Java: this.ruleArray = null;
        self.ruleArray = None;
    }

    // Java: void removeRule(Rule rule)
    pub fn removeRule(&mut self, index: usize) {
        // Java: this.rules.remove(rule);
        if index < self.rules.len() {
            self.rules.remove(index);
        }
        // Java: this.ruleArray = null;
        self.ruleArray = None;
    }

    // Java: void addAll(RuleSet that)
    pub fn addAll(&mut self, that: &mut RuleSet) {
        // Java: this.rules.addAll(that.rules);
        self.rules.append(&mut that.rules);
        // Java: this.ruleArray = null;
        self.ruleArray = None;
    }

    // Java: Rule[] getRuleArray()
    pub fn getRuleArray(&mut self) -> &Vec<Rule> {
        // Java: if (this.ruleArray == null)
        if self.ruleArray.is_none()
        {
            // Java: Collections.sort(this.rules);
            self.rules.sort_by(|a, b| a.compareTo_rule(b).cmp(&0));
            // Java: int size = this.rules.size();
            // Java: this.ruleArray = new Rule[size];
            // Java: this.rules.toArray(this.ruleArray);
        }
        // Java: return this.ruleArray;
        &self.rules
    }
}
