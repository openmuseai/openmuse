// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnresolvableException
use crate::org::jaxen::UnresolvableException::UnresolvableException;
// Java: import org.jaxen.expr.iter.IterableAxis
use crate::org::jaxen::expr::iter::IterableAxis::IterableAxis;
use crate::org::jaxen::{Object, List};
use super::DefaultStep::DefaultStep;
use super::PredicateSet::PredicateSet;

// Java field: long serialVersionUID

// Java: public class DefaultNameStep extends DefaultStep implements NameStep
pub struct DefaultNameStep {
    pub(crate) base: DefaultStep,
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: String localName
    pub(crate) localName: String,
    // Java field: boolean matchesAnyName
    pub(crate) matchesAnyName: bool,
    // Java field: boolean hasPrefix
    hasPrefix: bool,
}

// Java: public class DefaultNameStep extends DefaultStep implements NameStep
impl DefaultNameStep {
    // Java: DefaultNameStep(IterableAxis axis, String prefix, String localName, PredicateSet predicateSet)
    pub fn new(axis: IterableAxis, prefix: String, localName: String, predicateSet: PredicateSet) -> Self {
      // Java: super(axis, predicateSet);
      // Java: this.prefix = prefix;
      // Java: this.localName = localName;
      // Java: this.matchesAnyName = "*".equals(localName);
      let matchesAnyName = localName == "*";
      // Java: this.hasPrefix = this.prefix != null && this.prefix.length() > 0;
      let hasPrefix = !prefix.is_empty();
      DefaultNameStep {
         base: DefaultStep::new(axis, predicateSet),
         prefix,
         localName,
         matchesAnyName,
         hasPrefix,
      }
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
      // Java: return this.prefix;
      &self.prefix
    }

    // Java: String getLocalName()
    pub fn getLocalName(&self) -> &str {
      // Java: return this.localName;
      &self.localName
    }

    // Java: boolean isMatchesAnyName()
    pub fn isMatchesAnyName(&self) -> bool {
      // Java: return this.matchesAnyName;
      self.matchesAnyName
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer(64);
      let mut buf = String::with_capacity(64);
      // Java: buf.append(this.getAxisName()).append("::");
      buf.push_str(&self.base.getAxisName());
      buf.push_str("::");
      // Java: if (this.getPrefix() != null && this.getPrefix().length() > 0)
      if !self.getPrefix().is_empty() {
         // Java: buf.append(this.getPrefix()).append(':');
         buf.push_str(self.getPrefix());
         buf.push(':');
      }
      // Java: return buf.append(this.getLocalName()).append(super.getText()).toString();
      buf.push_str(self.getLocalName());
      buf.push_str(&self.base.getText());
      buf
    }

    // Java: List evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<List, JaxenException> {
      // Java: List contextNodeSet = context.getNodeSet();
      // Java: int contextSize = contextNodeSet.size();
      // Java: if (contextSize == 0)
      // Java: return Collections.EMPTY_LIST;
      // Java: else
      // Java: ContextSupport support = context.getContextSupport();
      // Java: IterableAxis iterableAxis = this.getIterableAxis();
      // Java: boolean namedAccess = !this.matchesAnyName && iterableAxis.supportsNamedAccess(support);
      // Java: if (contextSize == 1)
      // Java: Object contextNode = contextNodeSet.get(0);
      // Java: if (namedAccess)
      // Java: String uri = null;
      // Java: if (this.hasPrefix)
      // Java: uri = support.translateNamespacePrefixToUri(this.prefix);
      // Java: if (uri == null)
      // Java: throw new UnresolvableException("XPath expression uses unbound namespace prefix " + this.prefix);
      // Java: Iterator axisNodeIter = iterableAxis.namedAccessIterator(contextNode, support, this.localName, this.prefix, uri);
      // Java: if (axisNodeIter != null && axisNodeIter.hasNext())
      // Java: List newNodeSet = new ArrayList();
      // Java: while(axisNodeIter.hasNext())
      // Java: newNodeSet.add(axisNodeIter.next());
      // Java: return this.getPredicateSet().evaluatePredicates(newNodeSet, support);
      // Java: else
      // Java: return Collections.EMPTY_LIST;
      // Java: else
      // Java: Iterator axisNodeIter = iterableAxis.iterator(contextNode, support);
      // Java: if (axisNodeIter != null && axisNodeIter.hasNext())
      // Java: List newNodeSet = new ArrayList(contextSize);
      // Java: while(axisNodeIter.hasNext())
      // Java: Object eachAxisNode = axisNodeIter.next();
      // Java: if (this.matches(eachAxisNode, support))
      // Java: newNodeSet.add(eachAxisNode);
      // Java: return this.getPredicateSet().evaluatePredicates(newNodeSet, support);
      // Java: else
      // Java: return Collections.EMPTY_LIST;
      // Java: else
      // Java: IdentitySet unique = new IdentitySet();
      // Java: List interimSet = new ArrayList(contextSize);
      // Java: List newNodeSet = new ArrayList(contextSize);
      // Java: if (namedAccess)
      // Java: String uri = null;
      // Java: if (this.hasPrefix)
      // Java: uri = support.translateNamespacePrefixToUri(this.prefix);
      // Java: if (uri == null)
      // Java: throw new UnresolvableException("XPath expression uses unbound namespace prefix " + this.prefix);
      // Java: for(int i = 0; i < contextSize; ++i)
      // Java: Object eachContextNode = contextNodeSet.get(i);
      // Java: Iterator axisNodeIter = iterableAxis.namedAccessIterator(eachContextNode, support, this.localName, this.prefix, uri);
      // Java: if (axisNodeIter != null && axisNodeIter.hasNext())
      // Java: while(axisNodeIter.hasNext())
      // Java: Object eachAxisNode = axisNodeIter.next();
      // Java: interimSet.add(eachAxisNode);
      // Java: for(Object eachPredicateNode : this.getPredicateSet().evaluatePredicates(interimSet, support))
      // Java: if (!unique.contains(eachPredicateNode))
      // Java: unique.add(eachPredicateNode);
      // Java: newNodeSet.add(eachPredicateNode);
      // Java: interimSet.clear();
      // Java: else
      // Java: for(int i = 0; i < contextSize; ++i)
      // Java: Object eachContextNode = contextNodeSet.get(i);
      // Java: Iterator axisNodeIter = this.axisIterator(eachContextNode, support);
      // Java: if (axisNodeIter != null && axisNodeIter.hasNext())
      // Java: while(axisNodeIter.hasNext())
      // Java: Object eachAxisNode = axisNodeIter.next();
      // Java: if (this.matches(eachAxisNode, support))
      // Java: interimSet.add(eachAxisNode);
      // Java: for(Object eachPredicateNode : this.getPredicateSet().evaluatePredicates(interimSet, support))
      // Java: if (!unique.contains(eachPredicateNode))
      // Java: unique.add(eachPredicateNode);
      // Java: newNodeSet.add(eachPredicateNode);
      // Java: interimSet.clear();
      // Java: return newNodeSet;
      todo!("DefaultNameStep::evaluate")
    }

    // Java: boolean matches(Object node, ContextSupport contextSupport)
    pub fn matches(&self, _node: &Object, _contextSupport: &ContextSupport) -> Result<bool, JaxenException> {
      // Java: Navigator nav = contextSupport.getNavigator();
      // Java: String myUri = null;
      // Java: String nodeName = null;
      // Java: String nodeUri = null;
      // Java: if (nav.isElement(node))
      // Java: nodeName = nav.getElementName(node);
      // Java: nodeUri = nav.getElementNamespaceUri(node);
      // Java: else
      // Java: if (nav.isText(node))
      // Java: return false;
      // Java: if (nav.isAttribute(node))
      // Java: if (this.getAxis() != 9)
      // Java: return false;
      // Java: nodeName = nav.getAttributeName(node);
      // Java: nodeUri = nav.getAttributeNamespaceUri(node);
      // Java: else
      // Java: if (nav.isDocument(node))
      // Java: return false;
      // Java: if (!nav.isNamespace(node))
      // Java: return false;
      // Java: if (this.getAxis() != 10)
      // Java: return false;
      // Java: nodeName = nav.getNamespacePrefix(node);
      // Java: if (this.hasPrefix)
      // Java: myUri = contextSupport.translateNamespacePrefixToUri(this.prefix);
      // Java: if (myUri == null)
      // Java: throw new UnresolvableException("Cannot resolve namespace prefix '" + this.prefix + "'");
      // Java: else if (this.matchesAnyName)
      // Java: return true;
      // Java: if (this.hasNamespace(myUri) != this.hasNamespace(nodeUri))
      // Java: return false;
      // Java: else
      // Java: return !this.matchesAnyName && !nodeName.equals(this.getLocalName()) ? false : this.matchesNamespaceURIs(myUri, nodeUri);
      todo!("DefaultNameStep::matches")
    }

    // Java: boolean hasNamespace(String uri)
    pub fn hasNamespace(&self, uri: &Option<String>) -> bool {
      // Java: return uri != null && uri.length() > 0;
      match uri {
         Some(s) => !s.is_empty(),
         None => false,
      }
    }

    // Java: boolean matchesNamespaceURIs(String uri1, String uri2)
    pub fn matchesNamespaceURIs(&self, uri1: &Option<String>, uri2: &Option<String>) -> bool {
      // Java: if (uri1 == uri2)
      if uri1 == uri2 {
         // Java: return true;
         return true;
      }
      // Java: else if (uri1 == null)
      else if uri1.is_none() {
         // Java: return uri2.length() == 0;
         return uri2.as_ref().map_or(true, |s| s.is_empty());
      }
      // Java: else if (uri2 == null)
      else if uri2.is_none() {
         // Java: return uri1.length() == 0;
         return uri1.as_ref().map_or(true, |s| s.is_empty());
      }
      // Java: else
      else {
         // Java: return uri1.equals(uri2);
         uri1 == uri2
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: String prefix = this.getPrefix();
      let prefix = self.getPrefix();
      // Java: String qName = "".equals(prefix) ? this.getLocalName() : this.getPrefix() + ":" + this.getLocalName();
      let qName = if prefix.is_empty() {
         self.getLocalName().to_string()
      } else {
         format!("{}:{}", self.getPrefix(), self.getLocalName())
      };
      // Java: return "[(DefaultNameStep): " + qName + "]";
      format!("[(DefaultNameStep): {}]", qName)
    }
}
