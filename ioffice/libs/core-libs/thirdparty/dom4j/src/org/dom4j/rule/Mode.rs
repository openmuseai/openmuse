// Java: import java.util.EnumMap
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.dom4j.Attribute
// Java: import org.dom4j.Document
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeType
use std::collections::HashMap;
use crate::org::dom4j::NodeType::NodeType;
use crate::org::dom4j::rule::Rule::Rule;
use crate::org::dom4j::rule::RuleSet::RuleSet;

// Java: public class Mode
pub struct Mode {
    // Java field: EnumMap<NodeType, RuleSet> ruleSets
    pub(crate) ruleSets: HashMap<NodeType, RuleSet>,
    // Java field: Map<String, RuleSet> elementNameRuleSets
    pub(crate) elementNameRuleSets: Option<HashMap<String, RuleSet>>,
    // Java field: Map<String, RuleSet> attributeNameRuleSets
    pub(crate) attributeNameRuleSets: Option<HashMap<String, RuleSet>>,
}

// Java: public class Mode
impl Mode {
    // Java: Mode()
    pub fn new() -> Self {
        // Java: super();
        Mode {
            ruleSets: HashMap::new(),
            elementNameRuleSets: None,
            attributeNameRuleSets: None,
        }
    }

    // Java: void fireRule(Node node)
    pub fn fireRule(&self) {
        // Java: if (node != null)
        {
            // Java: Rule rule = this.getMatchingRule(node);
            // Java: if (rule != null)
            {
                // Java: Action action = rule.getAction();
                // Java: if (action != null)
                {
                    // Java: action.run(node);
                }
            }
        }
    }

    // Java: void applyTemplates(Element element)
    pub fn applyTemplates_element(&self) {
        // Java: int i = 0;
        // Java: for(int size = element.attributeCount(); i < size; ++i)
        {
            // Java: Attribute attribute = element.attribute(i);
            // Java: this.fireRule(attribute);
        }
        // Java: i = 0;
        // Java: for(int size = element.nodeCount(); i < size; ++i)
        {
            // Java: Node node = element.node(i);
            // Java: this.fireRule(node);
        }
    }

    // Java: void applyTemplates(Document document)
    pub fn applyTemplates_document(&self) {
        // Java: int i = 0;
        // Java: for(int size = document.nodeCount(); i < size; ++i)
        {
            // Java: Node node = document.node(i);
            // Java: this.fireRule(node);
        }
    }

    // Java: void addRule(Rule rule)
    pub fn addRule(&mut self, rule: Rule) {
        // Java: NodeType matchType = rule.getMatchType();
        let match_type = rule.getMatchType();
        // Java: String name = rule.getMatchesNodeName();
        let name = rule.getMatchesNodeName().map(|s| s.to_string());
        // Java: if (name != null)
        if let Some(ref name_str) = name
        {
            // Java: switch (matchType)
            match match_type
            {
                // Java: case ELEMENT_NODE:
                NodeType::ElementNode => {
                    // Java: this.elementNameRuleSets = this.addToNameMap(this.elementNameRuleSets, name, rule);
                    let mut map = self.elementNameRuleSets.take().unwrap_or_default();
                    self.addToNameMap_impl(&mut map, name_str, &rule);
                    self.elementNameRuleSets = Some(map);
                    // Java: break;
                }
                // Java: case ATTRIBUTE_NODE:
                NodeType::AttributeNode => {
                    // Java: this.attributeNameRuleSets = this.addToNameMap(this.attributeNameRuleSets, name, rule);
                    let mut map = self.attributeNameRuleSets.take().unwrap_or_default();
                    self.addToNameMap_impl(&mut map, name_str, &rule);
                    self.attributeNameRuleSets = Some(map);
                }
                _ => {}
            }
        }
        // Java: if (matchType == NodeType.ANY_NODE)
        if match_type == NodeType::AnyNode
        {
            // Java: for(RuleSet ruleSet : this.ruleSets.values())
            // Java: if (ruleSet != null)
            // Java: ruleSet.addRule(rule);
        }
        // Java: this.getRuleSet(matchType).addRule(rule);
        self.getRuleSet(match_type).addRule(rule);
    }

    // Java: void removeRule(Rule rule)
    pub fn removeRule(&mut self, rule: &Rule, index: usize) {
        // Java: NodeType matchType = rule.getMatchType();
        let match_type = rule.getMatchType();
        // Java: String name = rule.getMatchesNodeName();
        let name = rule.getMatchesNodeName().map(|s| s.to_string());
        // Java: if (name != null)
        if let Some(ref name_str) = name
        {
            // Java: switch (matchType)
            match match_type
            {
                // Java: case ELEMENT_NODE:
                NodeType::ElementNode => {
                    // Java: this.removeFromNameMap(this.elementNameRuleSets, name, rule);
                    self.removeFromNameMap_impl(&name_str, index);
                    // Java: break;
                }
                // Java: case ATTRIBUTE_NODE:
                NodeType::AttributeNode => {
                    // Java: this.removeFromNameMap(this.attributeNameRuleSets, name, rule);
                    self.removeFromNameMap_attr(&name_str, index);
                }
                _ => {}
            }
        }
        // Java: this.getRuleSet(matchType).removeRule(rule);
        self.getRuleSet(match_type).removeRule(index);
        // Java: if (matchType == NodeType.ANY_NODE)
        if match_type == NodeType::AnyNode
        {
            // Java: this.getRuleSet(NodeType.ANY_NODE).removeRule(rule);
            self.getRuleSet(NodeType::AnyNode).removeRule(index);
        }
    }

    // Java: Rule getMatchingRule(Node node)
    pub fn getMatchingRule(&self) -> Option<&Rule> {
        // Java: NodeType matchType = node.getNodeTypeEnum();
        // Java: switch (matchType)
        {
            // Java: case ELEMENT_NODE:
            // Java: if (this.elementNameRuleSets != null)
            {
                // Java: String name = node.getName();
                // Java: RuleSet ruleSet = (RuleSet)this.elementNameRuleSets.get(name);
                // Java: if (ruleSet != null)
                {
                    // Java: Rule answer = ruleSet.getMatchingRule(node);
                    // Java: if (answer != null)
                    {
                        // Java: return answer;
                    }
                }
            }
            // Java: break;
            // Java: case ATTRIBUTE_NODE:
            // Java: if (this.attributeNameRuleSets != null)
            {
                // Java: String name = node.getName();
                // Java: RuleSet ruleSet = (RuleSet)this.attributeNameRuleSets.get(name);
                // Java: if (ruleSet != null)
                {
                    // Java: Rule answer = ruleSet.getMatchingRule(node);
                    // Java: if (answer != null)
                    {
                        // Java: return answer;
                    }
                }
            }
        }
        // Java: Rule answer = null;
        // Java: RuleSet ruleSet = (RuleSet)this.ruleSets.get(matchType);
        // Java: if (ruleSet != null)
        {
            // Java: answer = ruleSet.getMatchingRule(node);
        }
        // Java: if (answer == null && matchType != NodeType.ANY_NODE)
        {
            // Java: ruleSet = (RuleSet)this.ruleSets.get(matchType);
            // Java: if (ruleSet != null)
            {
                // Java: answer = ruleSet.getMatchingRule(node);
            }
        }
        // Java: return answer;
        None
    }

    // Java: RuleSet getRuleSet(NodeType matchType)
    pub fn getRuleSet(&mut self, match_type: NodeType) -> &mut RuleSet {
        // Java: RuleSet ruleSet = (RuleSet)this.ruleSets.get(matchType);
        // Java: if (ruleSet == null)
        if !self.ruleSets.contains_key(&match_type)
        {
            // Java: ruleSet = new RuleSet();
            // Java: this.ruleSets.put(matchType, ruleSet);
            self.ruleSets.insert(match_type, RuleSet::new());
            // Java: if (matchType != NodeType.ANY_NODE)
            if match_type != NodeType::AnyNode
            {
                // Java: RuleSet allRules = (RuleSet)this.ruleSets.get(matchType);
                // Java: if (allRules != null)
                {
                    // Java: ruleSet.addAll(allRules);
                }
            }
        }
        // Java: return ruleSet;
        self.ruleSets.get_mut(&match_type).unwrap()
    }

    // Java: protected Map<String, RuleSet> addToNameMap(Map<String, RuleSet> map, String name, Rule rule)
    fn addToNameMap_impl(&self, map: &mut HashMap<String, RuleSet>, name: &str, rule: &Rule) {
        // Java: if (map == null)
        {
            // Java: map = new HashMap();
        }
        // Java: RuleSet ruleSet = (RuleSet)map.get(name);
        // Java: if (ruleSet == null)
        if !map.contains_key(name)
        {
            // Java: ruleSet = new RuleSet();
            // Java: map.put(name, ruleSet);
            map.insert(name.to_string(), RuleSet::new());
        }
        // Java: ruleSet.addRule(rule);
        // Java: return map;
        let _ = rule;
    }

    // Java: void removeFromNameMap(Map<String, RuleSet> map, String name, Rule rule)
    fn removeFromNameMap_impl(&mut self, name: &str, index: usize) {
        // Java: if (map != null)
        if let Some(ref mut map) = self.elementNameRuleSets
        {
            // Java: RuleSet ruleSet = (RuleSet)map.get(name);
            // Java: if (ruleSet != null)
            if let Some(rule_set) = map.get_mut(name)
            {
                // Java: ruleSet.removeRule(rule);
                rule_set.removeRule(index);
            }
        }
    }

    fn removeFromNameMap_attr(&mut self, name: &str, index: usize) {
        if let Some(ref mut map) = self.attributeNameRuleSets {
            if let Some(rule_set) = map.get_mut(name) {
                rule_set.removeRule(index);
            }
        }
    }
}
