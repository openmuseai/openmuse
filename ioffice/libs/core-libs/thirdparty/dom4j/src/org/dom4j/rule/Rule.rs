// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeType
use crate::org::dom4j::rule::Action::Action;
use crate::org::dom4j::rule::Pattern::Pattern;
use crate::org::dom4j::NodeType::NodeType;

// Java: public class Rule implements Comparable
pub struct Rule {
    // Java field: String mode
    pub(crate) mode: Option<String>,
    // Java field: int importPrecedence
    pub(crate) importPrecedence: i32,
    // Java field: double priority
    pub(crate) priority: f64,
    // Java field: int appearenceCount
    pub(crate) appearenceCount: i32,
    // Java field: Pattern pattern
    pub(crate) pattern: Option<Box<dyn Pattern>>,
    // Java field: Action action
    pub(crate) action: Option<Box<dyn Action>>,
}

// Java: public class Rule implements Comparable
impl Rule {
    // Java: Rule()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.priority = (double)0.5F;
        Rule {
            mode: None,
            importPrecedence: 0,
            priority: 0.5,
            appearenceCount: 0,
            pattern: None,
            action: None,
        }
    }

    // Java: Rule(Pattern pattern)
    pub fn new_with_pattern(pattern: Box<dyn Pattern>) -> Self {
        // Java: super();
        // Java: this.pattern = pattern;
        // Java: this.priority = pattern.getPriority();
        let priority = pattern.getPriority();
        Rule {
            mode: None,
            importPrecedence: 0,
            priority,
            appearenceCount: 0,
            pattern: Some(pattern),
            action: None,
        }
    }

    // Java: Rule(Pattern pattern, Action action)
    pub fn new_with_pattern_and_action(pattern: Box<dyn Pattern>, action: Box<dyn Action>) -> Self {
        // Java: this(pattern);
        // Java: this.action = action;
        let mut rule = Self::new_with_pattern(pattern);
        rule.action = Some(action);
        rule
    }

    // Java: Rule(Rule that, Pattern pattern)
    pub fn new_from_rule(that: &Rule, pattern: Box<dyn Pattern>) -> Self {
        // Java: super();
        // Java: this.mode = that.mode;
        // Java: this.importPrecedence = that.importPrecedence;
        // Java: this.priority = that.priority;
        // Java: this.appearenceCount = that.appearenceCount;
        // Java: this.action = that.action;
        // Java: this.pattern = pattern;
        Rule {
            mode: that.mode.clone(),
            importPrecedence: that.importPrecedence,
            priority: that.priority,
            appearenceCount: that.appearenceCount,
            pattern: Some(pattern),
            action: None,
        }
    }

    // Java: boolean equals(Object that)
    pub fn equals(&self, that: &Rule) -> bool {
        // Java: if (that instanceof Rule)
        // Java: return this.compareTo((Rule)that) == 0;
        // Java: else
        // Java: return false;
        self.compareTo_rule(that) == 0
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.importPrecedence + this.appearenceCount;
        self.importPrecedence + self.appearenceCount
    }

    // Java: int compareTo(Object that)
    pub fn compareTo(&self, that: &Rule) -> i32 {
        // Java: return that instanceof Rule ? this.compareTo((Rule)that) : this.getClass().getName().compareTo(that.getClass().getName());
        self.compareTo_rule(that)
    }

    // Java: int compareTo(Rule that)
    pub fn compareTo_rule(&self, that: &Rule) -> i32 {
        // Java: int answer = this.importPrecedence - that.importPrecedence;
        let mut answer = self.importPrecedence - that.importPrecedence;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = (int)Math.round(this.priority - that.priority);
            answer = (self.priority - that.priority).round() as i32;
            // Java: if (answer == 0)
            if answer == 0
            {
                // Java: answer = this.appearenceCount - that.appearenceCount;
                answer = self.appearenceCount - that.appearenceCount;
            }
        }
        // Java: return answer;
        answer
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "[ pattern: " + this.getPattern() + " action: " + this.getAction() + " ]";
        format!("Rule[ importPrecedence: {} priority: {} ]", self.importPrecedence, self.priority)
    }

    // Java: boolean matches(Node node)
    pub fn matches(&self) -> bool {
        // Java: return this.pattern.matches(node);
        false
    }

    // Java: Rule[] getUnionRules()
    pub fn getUnionRules(&self) -> Option<Vec<Rule>> {
        // Java: Pattern[] patterns = this.pattern.getUnionPatterns();
        // Java: if (patterns == null)
        {
            // Java: return null;
        }
        // Java: else
        {
            // Java: int size = patterns.length;
            // Java: Rule[] answer = new Rule[size];
            // Java: for(int i = 0; i < size; ++i)
            {
                // Java: answer[i] = new Rule(this, patterns[i]);
            }
            // Java: return answer;
        }
        None
    }

    // Java: NodeType getMatchType()
    pub fn getMatchType(&self) -> NodeType {
        // Java: return this.pattern.getMatchType();
        NodeType::AnyNode
    }

    // Java: String getMatchesNodeName()
    pub fn getMatchesNodeName(&self) -> Option<&str> {
        // Java: return this.pattern.getMatchesNodeName();
        None
    }

    // Java: String getMode()
    pub fn getMode(&self) -> Option<&str> {
        // Java: return this.mode;
        self.mode.as_deref()
    }

    // Java: void setMode(String mode)
    pub fn setMode(&mut self, mode: String) {
        // Java: this.mode = mode;
        self.mode = Some(mode);
    }

    // Java: int getImportPrecedence()
    pub fn getImportPrecedence(&self) -> i32 {
        // Java: return this.importPrecedence;
        self.importPrecedence
    }

    // Java: void setImportPrecedence(int importPrecedence)
    pub fn setImportPrecedence(&mut self, import_precedence: i32) {
        // Java: this.importPrecedence = importPrecedence;
        self.importPrecedence = import_precedence;
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
        // Java: return this.priority;
        self.priority
    }

    // Java: void setPriority(double priority)
    pub fn setPriority(&mut self, priority: f64) {
        // Java: this.priority = priority;
        self.priority = priority;
    }

    // Java: int getAppearenceCount()
    pub fn getAppearenceCount(&self) -> i32 {
        // Java: return this.appearenceCount;
        self.appearenceCount
    }

    // Java: void setAppearenceCount(int appearenceCount)
    pub fn setAppearenceCount(&mut self, appearence_count: i32) {
        // Java: this.appearenceCount = appearenceCount;
        self.appearenceCount = appearence_count;
    }

    // Java: Pattern getPattern()
    pub fn getPattern(&self) -> &Option<Box<dyn Pattern>> {
        // Java: return this.pattern;
        &self.pattern
    }

    // Java: void setPattern(Pattern pattern)
    pub fn setPattern(&mut self, pattern: Box<dyn Pattern>) {
        // Java: this.pattern = pattern;
        self.pattern = Some(pattern);
    }

    // Java: Action getAction()
    pub fn getAction(&self) -> &Option<Box<dyn Action>> {
        // Java: return this.action;
        &self.action
    }

    // Java: void setAction(Action action)
    pub fn setAction(&mut self, action: Box<dyn Action>) {
        // Java: this.action = action;
        self.action = Some(action);
    }
}
