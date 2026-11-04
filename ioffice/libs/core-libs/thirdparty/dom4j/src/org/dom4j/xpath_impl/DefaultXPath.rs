// Java: import java.io.Serializable
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.HashSet
use std::collections::HashSet;
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
use std::any::Any;
use std::sync::Arc;
// Java: import org.dom4j.InvalidXPathException
use crate::org::dom4j::InvalidXPathException::InvalidXPathException;
// Java: import org.dom4j.Node
use crate::org::dom4j::Node::Node;
// Java: import org.dom4j.NodeFilter
use crate::org::dom4j::NodeFilter::NodeFilter;
// Java: import org.dom4j.XPath
use crate::org::dom4j::xpath::XPath as Dom4jXPathTrait;
// Java: import org.dom4j.XPathException
use crate::org::dom4j::XPathException::XPathException;
// Java: import org.jaxen.FunctionContext
use core_libs_jaxen::org::jaxen::FunctionContext::FunctionContext;
// Java: import org.jaxen.JaxenException
use core_libs_jaxen::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.NamespaceContext
use core_libs_jaxen::org::jaxen::NamespaceContext::NamespaceContext;
// Java: import org.jaxen.SimpleNamespaceContext
use core_libs_jaxen::org::jaxen::SimpleNamespaceContext::SimpleNamespaceContext;
// Java: import org.jaxen.VariableContext
use core_libs_jaxen::org::jaxen::VariableContext::VariableContext;
// Java: import org.jaxen.dom4j.Dom4jXPath
use crate::org::jaxen::dom4j::Dom4jXPath::Dom4jXPath;
use core_libs_jaxen::org::jaxen::BaseXPath::BaseXPath;

type Object = Arc<dyn Any + Send + Sync>;
type List = Vec<Object>;

// Java: public class DefaultXPath implements XPath, NodeFilter, Serializable
pub struct DefaultXPath {
    // Java field: String text
    pub(crate) text: String,
    // Java field: org.jaxen.XPath xpath
    pub(crate) xpath: BaseXPath,
    // Java field: NamespaceContext namespaceContext
    pub(crate) namespaceContext: Option<Box<dyn NamespaceContext>>,
}

// Java: public class DefaultXPath implements XPath, NodeFilter, Serializable
impl DefaultXPath {
    // Java: DefaultXPath(String text)
    pub fn new(text: String) -> Result<Self, InvalidXPathException> {
        // Java: super();
        // Java: this.text = text;
        // Java: this.xpath = parse(text);
        let xpath = Self::parse(&text);
        Ok(DefaultXPath {
            text,
            xpath,
            namespaceContext: None,
        })
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "[XPath: " + this.xpath + "]";
        format!("[XPath: {}]", self.text)
    }

    // Java: String getText()
    pub fn getText(&self) -> &str {
        // Java: return this.text;
        &self.text
    }

    // Java: FunctionContext getFunctionContext()
    pub fn getFunctionContext(&self) -> Option<&dyn FunctionContext> {
        // Java: return this.xpath.getFunctionContext();
        self.xpath.getFunctionContext()
    }

    // Java: void setFunctionContext(FunctionContext functionContext)
    pub fn setFunctionContext(&mut self, functionContext: Box<dyn FunctionContext>) {
        // Java: this.xpath.setFunctionContext(functionContext);
        self.xpath.setFunctionContext(functionContext);
    }

    // Java: NamespaceContext getNamespaceContext()
    pub fn getNamespaceContext(&self) -> Option<&dyn NamespaceContext> {
        // Java: return this.namespaceContext;
        self.namespaceContext.as_deref()
    }

    // Java: void setNamespaceURIs(Map map)
    pub fn setNamespaceURIs(&mut self, map: HashMap<String, String>) {
        // Java: this.setNamespaceContext(new SimpleNamespaceContext(map));
        self.setNamespaceContext_ns(Box::new(SimpleNamespaceContext::new_with_map(map)));
    }

    // Java: void setNamespaceContext(NamespaceContext namespaceContext)
    pub fn setNamespaceContext_ns(&mut self, namespaceContext: Box<dyn NamespaceContext>) {
        // Java: this.namespaceContext = namespaceContext;
        self.namespaceContext = Some(namespaceContext);
        // Java: this.xpath.setNamespaceContext(namespaceContext);
    }

    // Java: VariableContext getVariableContext()
    pub fn getVariableContext(&self) -> Option<&dyn VariableContext> {
        // Java: return this.xpath.getVariableContext();
        self.xpath.getVariableContext()
    }

    // Java: void setVariableContext(VariableContext variableContext)
    pub fn setVariableContext(&mut self, variableContext: Box<dyn VariableContext>) {
        // Java: this.xpath.setVariableContext(variableContext);
        self.xpath.setVariableContext(variableContext);
    }

    // Java: Object evaluate(Object context)
    pub fn evaluate(&self, context: Object) -> Option<Object> {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: List answer = this.xpath.selectNodes(context);
            match self.xpath.selectNodes(context) {
                Ok(answer) => {
                    // Java: return answer != null && answer.size() == 1 ? answer.get(0) : answer;
                    if answer.len() == 1 {
                        Some(answer.into_iter().next().unwrap())
                    } else {
                        Some(Arc::new(answer))
                    }
                }
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return null;
                    None
                }
            }
        }
    }

    // Java: Object selectObject(Object context)
    pub fn selectObject(&self, context: Object) -> Option<Object> {
        // Java: return this.evaluate(context);
        self.evaluate(context)
    }

    // Java: List selectNodes(Object context)
    pub fn selectNodes(&self, context: Object) -> List {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: return this.xpath.selectNodes(context);
            match self.xpath.selectNodes(context) {
                Ok(result) => result,
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return Collections.EMPTY_LIST;
                    Vec::new()
                }
            }
        }
    }

    // Java: List selectNodes(Object context, XPath sortXPath)
    pub fn selectNodes_sorted(&self, context: Object, sort_xpath: &DefaultXPath) -> List {
        // Java: List answer = this.selectNodes(context);
        let mut answer = self.selectNodes(context);
        // Java: sortXPath.sort(answer);
        sort_xpath.sort(&mut answer);
        // Java: return answer;
        answer
    }

    // Java: List selectNodes(Object context, XPath sortXPath, boolean distinct)
    pub fn selectNodes_sorted_distinct(&self, context: Object, sort_xpath: &DefaultXPath, distinct: bool) -> List {
        // Java: List answer = this.selectNodes(context);
        let mut answer = self.selectNodes(context);
        // Java: sortXPath.sort(answer, distinct);
        sort_xpath.sort_distinct(&mut answer, distinct);
        // Java: return answer;
        answer
    }

    // Java: Node selectSingleNode(Object context)
    pub fn selectSingleNode(&self, context: Object) -> Option<Object> {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: Object answer = this.xpath.selectSingleNode(context);
            match self.xpath.selectSingleNode(context) {
                Ok(opt_answer) => {
                    // Java: if (answer instanceof Node)
                    if let Some(answer) = opt_answer {
                        // Java: return (Node)answer;
                        return Some(answer);
                    }
                    // Java: else if (answer == null)
                    // Java: return null;
                    None
                    // Java: else
                    // Java: throw new XPathException("The result of the XPath expression is not a Node. It was: " + answer + " of type: " + answer.getClass().getName());
                }
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return null;
                    None
                }
            }
        }
    }

    // Java: String valueOf(Object context)
    pub fn valueOf(&self, context: Object) -> String {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: return this.xpath.stringValueOf(context);
            match self.xpath.stringValueOf(context) {
                Ok(result) => result,
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return "";
                    String::new()
                }
            }
        }
    }

    // Java: Number numberValueOf(Object context)
    pub fn numberValueOf(&self, context: Object) -> Option<f64> {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: return this.xpath.numberValueOf(context);
            match self.xpath.numberValueOf(context) {
                Ok(result) => Some(result),
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return null;
                    None
                }
            }
        }
    }

    // Java: boolean booleanValueOf(Object context)
    pub fn booleanValueOf(&self, context: Object) -> bool {
        // Java: try
        {
            // Java: this.setNSContext(context);
            // Java: return this.xpath.booleanValueOf(context);
            match self.xpath.booleanValueOf(context) {
                Ok(result) => result,
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return false;
                    false
                }
            }
        }
    }

    // Java: void sort(List list)
    pub fn sort(&self, list: &mut List) {
        // Java: this.sort(list, false);
        self.sort_distinct(list, false);
    }

    // Java: void sort(List list, boolean distinct)
    pub fn sort_distinct(&self, list: &mut List, distinct: bool) {
        // Java: if (list != null && !list.isEmpty())
        if !list.is_empty()
        {
            // Java: int size = list.size();
            let size = list.len();
            // Java: HashMap sortValues = new HashMap(size);
            let mut sort_values: HashMap<usize, String> = HashMap::with_capacity(size);
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size
            {
                // Java: Object object = list.get(i);
                // Java: if (object instanceof Node)
                {
                    // Java: Node node = (Node)object;
                    // Java: Object expression = this.getCompareValue(node);
                    let expression = self.getCompareValue_obj(list[i].clone());
                    // Java: sortValues.put(node, expression);
                    sort_values.insert(i, expression);
                }
            }
            // Java: this.sort(list, sortValues);
            self.sort_with_values(list, &sort_values);
            // Java: if (distinct)
            if distinct
            {
                // Java: this.removeDuplicates(list, sortValues);
                self.removeDuplicates(list, &sort_values);
            }
        }
    }

    // Java: boolean matches(Node node)
    pub fn matches_node(&self, node: Object) -> bool {
        // Java: try
        {
            // Java: this.setNSContext(node);
            // Java: List answer = this.xpath.selectNodes(node);
            match self.xpath.selectNodes(node) {
                Ok(answer) => {
                    // Java: if (answer != null && answer.size() > 0)
                    if !answer.is_empty()
                    {
                        // Java: Object item = answer.get(0);
                        // Java: return item instanceof Boolean ? (Boolean)item : answer.contains(node);
                        false
                    }
                    // Java: else
                    else
                    {
                        // Java: return false;
                        false
                    }
                }
                // Java: catch (JaxenException e)
                Err(_e) => {
                    // Java: this.handleJaxenException(e);
                    // Java: return false;
                    false
                }
            }
        }
    }

    // Java: void sort(List list, final Map sortValues)
    fn sort_with_values(&self, _list: &mut List, _sort_values: &HashMap<usize, String>) {
        // Java: Collections.sort(list, new Comparator()
        {
            // Java: public int compare(Object o1, Object o2)
            {
                // Java: o1 = sortValues.get(o1);
                // Java: o2 = sortValues.get(o2);
                // Java: if (o1 == o2)
                {
                    // Java: return 0;
                }
                // Java: else if (o1 instanceof Comparable)
                {
                    // Java: Comparable c1 = (Comparable)o1;
                    // Java: return c1.compareTo(o2);
                }
                // Java: else if (o1 == null)
                {
                    // Java: return 1;
                }
                // Java: else if (o2 == null)
                {
                    // Java: return -1;
                }
                // Java: else
                {
                    // Java: return o1.equals(o2) ? 0 : -1;
                }
            }
        }
    }

    // Java: void removeDuplicates(List list, Map sortValues)
    fn removeDuplicates(&self, list: &mut List, _sort_values: &HashMap<usize, String>) {
        // Java: HashSet distinctValues = new HashSet();
        let mut distinct_values: HashSet<String> = HashSet::new();
        // Java: Iterator iter = list.iterator();
        // Java: while(iter.hasNext())
        list.retain(|_node| {
            // Java: Object node = iter.next();
            // Java: Object value = sortValues.get(node);
            let value = String::new();
            // Java: if (distinctValues.contains(value))
            if distinct_values.contains(&value)
            {
                // Java: iter.remove();
                false
            }
            // Java: else
            else
            {
                // Java: distinctValues.add(value);
                distinct_values.insert(value);
                true
            }
        });
    }

    // Java: Object getCompareValue(Node node)
    fn getCompareValue_obj(&self, node: Object) -> String {
        // Java: return this.valueOf(node);
        self.valueOf(node)
    }

    // Java: void setNSContext(Object context)
    fn setNSContext(&self, _context: &Object) {
        // Java: if (this.namespaceContext == null)
        if self.namespaceContext.is_none()
        {
            // Java: this.xpath.setNamespaceContext(DefaultNamespaceContext.create(context));
        }
    }

    // Java: void handleJaxenException(JaxenException exception)
    fn handleJaxenException(&self, _exception: JaxenException) {
        // Java: throw new XPathException(this.text, exception);
    }

    fn parse(text: &str) -> BaseXPath {
        match Dom4jXPath::new(text) {
            Ok(xpath) => xpath.base,
            Err(_e) => {
                panic!("Invalid XPath expression: {}", text);
            }
        }
    }
}
