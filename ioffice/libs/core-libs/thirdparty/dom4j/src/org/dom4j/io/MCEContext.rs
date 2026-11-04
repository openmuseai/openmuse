// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.util.MutableInt
use util::cn::wps::moffice::util::MutableInt::MutableInt;
// Java: import java.util.ArrayList
// Java: import java.util.Stack
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace::Namespace;
// Java: import org.dom4j.tree.NamespaceStack
use crate::org::dom4j::tree::NamespaceStack::NamespaceStack;
// Java: import org.xml.sax.Attributes

// Java field: String NS_MCE
pub(crate) const NS_MCE: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
// Java field: String MCE_ATTR_IGNORABLE
pub(crate) const MCE_ATTR_IGNORABLE: &str = "Ignorable";
// Java field: String MCE_ATTR_PROCESS_CONTENT
pub(crate) const MCE_ATTR_PROCESS_CONTENT: &str = "ProcessContent";
// Java field: String MCE_ATTR_MUST_UNDERSTAND
pub(crate) const MCE_ATTR_MUST_UNDERSTAND: &str = "MustUnderstand";
// Java field: String MCE_ELEMENT_ALTERNATE_CONTENT
pub(crate) const MCE_ELEMENT_ALTERNATE_CONTENT: &str = "AlternateContent";
// Java field: String MCE_ELEMENT_CHOICE
pub(crate) const MCE_ELEMENT_CHOICE: &str = "Choice";
// Java field: String MCE_ATTR_REQUIRES
pub(crate) const MCE_ATTR_REQUIRES: &str = "Requires";
// Java field: String MCE_ELEMENT_FALLBACK
pub(crate) const MCE_ELEMENT_FALLBACK: &str = "Fallback";
// Java: public class MCEContext
pub struct MCEContext {
    // Java field: MceQNameLevelSet mUnderstandsSet
    mUnderstandsSet: MceQNameLevelSet,
    // Java field: MceQNameLevelSet mIgnoredSet
    mIgnoredSet: MceQNameLevelSet,
    // Java field: MceQNameLevelSet mProcessContentSet
    mProcessContentSet: MceQNameLevelSet,
    // Java field: MceSkipStack mMceSkipStack
    mMceSkipStack: MceSkipStack,
    // Java field: boolean mMceDisabled
    pub(crate) mMceDisabled: bool,
    // Java field: String SPACE
    // Java field: String COLON
}

const SPACE: &str = " ";
const COLON: &str = ":";

// Java: public class MceQNameLevel
pub struct MceQNameLevel {
    // Java field: String ns
    pub(crate) ns: Option<String>,
    // Java field: String ln
    pub(crate) ln: Option<String>,
    // Java field: int level
    pub(crate) level: i32,
}

// Java: public class MceQNameLevelSet
pub struct MceQNameLevelSet {
    // Java field: ArrayList<MceQNameLevel> mList
    pub(crate) mList: Vec<MceQNameLevel>,
    // Java field: int mMaxLevel
    pub(crate) mMaxLevel: i32,
    // Java field: MutableInt mPos
    pub(crate) mPos: MutableInt,
}

// Java: public enum MceSkipState
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MceSkipState {
    MCE_SKIP_STATE_IGNORE,
    MCE_SKIP_STATE_ALTERNATE_CONTENT,
    MCE_SKIP_STATE_CHOICE_MATCHED,
}

// Java: public class MceSkipItem
pub struct MceSkipItem {
    // Java field: int level_start
    pub(crate) level_start: i32,
    // Java field: int level_end
    pub(crate) level_end: i32,
    // Java field: MceSkipState state
    pub(crate) state: MceSkipState,
}

// Java: public class MceSkipStack
pub struct MceSkipStack {
    // Java field: Stack<MceSkipItem> mStack
    pub(crate) mStack: Vec<MceSkipItem>,
}

pub struct Attributes {
    values: Vec<(String, String, String, String)>,
}

impl Attributes {
    pub fn getValue_ns(&self, uri: &str, local_name: &str) -> Option<&str> {
        for (ns, ln, _qn, val) in &self.values {
            if ns == uri && ln == local_name {
                return Some(val.as_str());
            }
        }
        None
    }

    pub fn getLength(&self) -> i32 {
        self.values.len() as i32
    }

    pub fn getLocalName(&self, index: i32) -> &str {
        &self.values[index as usize].1
    }

    pub fn getValue_index(&self, index: i32) -> &str {
        &self.values[index as usize].3
    }
}

// Java: public class MCEContext
impl MCEContext {
    // Java: MCEContext()
    pub fn new() -> Self {
      // Java: super();
      MCEContext {
          mUnderstandsSet: MceQNameLevelSet::new(),
          mIgnoredSet: MceQNameLevelSet::new(),
          mProcessContentSet: MceQNameLevelSet::new(),
          mMceSkipStack: MceSkipStack::new(),
          mMceDisabled: true,
      }
    }

    // Java: void setDisableMCE(boolean disable)
    pub fn setDisableMCE(&mut self, disable: bool) {
      // Java: this.mMceDisabled = disable;
      self.mMceDisabled = disable;
    }

    // Java: boolean getDisableMCE()
    pub fn getDisableMCE(&self) -> bool {
      // Java: return this.mMceDisabled;
      self.mMceDisabled
    }

    // Java: MceQNameLevel lookupUnderstandsSet(String ns, String ln, boolean ignore_ln)
    pub fn lookupUnderstandsSet(&mut self, ns: Option<&str>, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
      // Java: return this.mUnderstandsSet.lookup(ns, ln, ignore_ln);
      self.mUnderstandsSet.lookup(ns, ln, ignore_ln)
    }

    // Java: MceQNameLevel lookupIgnoredSet(String ns, String ln, boolean ignore_ln)
    pub fn lookupIgnoredSet(&mut self, ns: Option<&str>, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
      // Java: return this.mIgnoredSet.lookup(ns, ln, ignore_ln);
      self.mIgnoredSet.lookup(ns, ln, ignore_ln)
    }

    // Java: MceQNameLevel lookupProcesscontentSet(String ns, String ln, boolean ignore_ln)
    pub fn lookupProcesscontentSet(&mut self, ns: Option<&str>, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
      // Java: return this.mProcessContentSet.lookup(ns, ln, ignore_ln);
      self.mProcessContentSet.lookup(ns, ln, ignore_ln)
    }

    // Java: void addIntoUnderstandsSet(String ns)
    pub fn addIntoUnderstandsSet(&mut self, ns: &str) {
      // Java: this.mUnderstandsSet.add(ns, (String)null, 0);
      self.mUnderstandsSet.add(Some(ns), None, 0);
    }

    // Java: void addIntoIgnoredSet(String ns, int level)
    pub fn addIntoIgnoredSet(&mut self, ns: &str, level: i32) {
      // Java: this.mIgnoredSet.add(ns, (String)null, level);
      self.mIgnoredSet.add(Some(ns), None, level);
    }

    // Java: void addIntoProcesscontentSet(String ns, String ln, int level)
    pub fn addIntoProcesscontentSet(&mut self, ns: &str, ln: &str, level: i32) {
      // Java: this.mProcessContentSet.add(ns, ln, level);
      self.mProcessContentSet.add(Some(ns), Some(ln), level);
    }

    // Java: void processIgnorableAttr(NamespaceStack namespaceStack, String attrValue, int level)
    pub fn processIgnorableAttr(&mut self, _namespaceStack: &NamespaceStack, attr_value: &str, level: i32) {
      // Java: String[] prefixes = attrValue.split(" ");
      let prefixes: Vec<&str> = attr_value.split(' ').collect();
      // Java: for(String prefix : prefixes)
      for _prefix in &prefixes
      {
         // Java: Namespace ns = namespaceStack.getNamespaceForPrefix(prefix);
         // Java: String nsUri = ns != null ? ns.getURI() : null;
         let ns_uri: Option<&str> = None; // TODO: NamespaceStack.getNamespaceForPrefix not fully implemented
         // Java: if (null != nsUri && null == this.mUnderstandsSet.lookup(nsUri, (String)null, true))
         if let Some(uri) = ns_uri {
            if self.mUnderstandsSet.lookup(Some(uri), None, true).is_none()
            {
               // Java: this.addIntoIgnoredSet(nsUri, level);
               self.mIgnoredSet.add(Some(uri), None, level);
            }
         }
      }
    }

    // Java: void processProcessContentAttr(NamespaceStack namespaceStack, String attrValue, int level)
    pub fn processProcessContentAttr(&mut self, _namespaceStack: &NamespaceStack, attr_value: &str, level: i32) {
      // Java: String[] qnames = attrValue.split(" ");
      let qnames: Vec<&str> = attr_value.split(' ').collect();
      // Java: for(String qname : qnames)
      for qname in &qnames
      {
         // Java: String[] prefixAndLocalName = qname.split(":");
         let prefix_and_local_name: Vec<&str> = qname.split(':').collect();
         // Java: if (prefixAndLocalName.length >= 2)
         if prefix_and_local_name.len() >= 2
         {
            // Java: String prefix = prefixAndLocalName[0];
            let _prefix = prefix_and_local_name[0];
            // Java: String localName = prefixAndLocalName[1];
            let local_name = prefix_and_local_name[1];
            // Java: Namespace ns = namespaceStack.getNamespaceForPrefix(prefix);
            // Java: String nsUri = ns != null ? ns.getURI() : null;
            let ns_uri: Option<&str> = None; // TODO: NamespaceStack.getNamespaceForPrefix not fully implemented
            // Java: if (null != nsUri && null == this.mUnderstandsSet.lookup(nsUri, (String)null, true))
            if let Some(uri) = ns_uri {
               if self.mUnderstandsSet.lookup(Some(uri), None, true).is_none()
               {
                  // Java: this.addIntoProcesscontentSet(nsUri, localName, level);
                  self.mProcessContentSet.add(Some(uri), Some(local_name), level);
               }
            }
         }
      }
    }

    // Java: void processMustUnderstandAttr(NamespaceStack namespaceStack, String attrValue, int level)
    pub fn processMustUnderstandAttr(&mut self, _namespaceStack: &NamespaceStack, attr_value: &str, level: i32) {
      // Java: String[] prefixes = attrValue.split(" ");
      let prefixes: Vec<&str> = attr_value.split(' ').collect();
      // Java: for(String prefix : prefixes)
      for _prefix in &prefixes
      {
         // Java: Namespace ns = namespaceStack.getNamespaceForPrefix(prefix);
         // Java: String nsUri = ns != null ? ns.getURI() : null;
         let ns_uri: Option<&str> = None; // TODO: NamespaceStack.getNamespaceForPrefix not fully implemented
         // Java: if (null != nsUri && null == this.mUnderstandsSet.lookup(nsUri, (String)null, true))
         if let Some(uri) = ns_uri {
            if self.mUnderstandsSet.lookup(Some(uri), None, true).is_none()
            {
               // Java: Assert.logFail("We meet a must-understood namespace which we do NOT understand.");
               Assert::logFail_msg("We meet a must-understood namespace which we do NOT understand.");
               // Java: this.addIntoIgnoredSet(nsUri, level);
               self.mIgnoredSet.add(Some(uri), None, level);
            }
         }
      }
    }

    // Java: void processMCEAttributes(NamespaceStack namespaceStack, Attributes attributes, int level)
    pub fn processMCEAttributes(&mut self, namespaceStack: &NamespaceStack, attributes: &Attributes, level: i32) {
      // Java: String ignorable = attributes.getValue("http://schemas.openxmlformats.org/markup-compatibility/2006", "Ignorable");
      let ignorable = attributes.getValue_ns(NS_MCE, "Ignorable");
      // Java: if (ignorable != null)
      if let Some(ignorable) = ignorable
      {
         // Java: this.processIgnorableAttr(namespaceStack, ignorable, level);
         let ignorable_owned = ignorable.to_string();
         self.processIgnorableAttr(namespaceStack, &ignorable_owned, level);
      }
      // Java: String processContent = attributes.getValue("http://schemas.openxmlformats.org/markup-compatibility/2006", "ProcessContent");
      let process_content = attributes.getValue_ns(NS_MCE, "ProcessContent");
      // Java: if (processContent != null)
      if let Some(process_content) = process_content
      {
         // Java: this.processProcessContentAttr(namespaceStack, processContent, level);
         let pc_owned = process_content.to_string();
         self.processProcessContentAttr(namespaceStack, &pc_owned, level);
      }
      // Java: String mustUnderstand = attributes.getValue("http://schemas.openxmlformats.org/markup-compatibility/2006", "MustUnderstand");
      let must_understand = attributes.getValue_ns(NS_MCE, "MustUnderstand");
      // Java: if (mustUnderstand != null)
      if let Some(must_understand) = must_understand
      {
         // Java: this.processMustUnderstandAttr(namespaceStack, mustUnderstand, level);
         let mu_owned = must_understand.to_string();
         self.processMustUnderstandAttr(namespaceStack, &mu_owned, level);
      }
    }

    // Java: String getRequiresAttrValue(Attributes attributes)
    pub fn getRequiresAttrValue(&self, attributes: &Attributes) -> Option<String> {
      // Java: int len = attributes.getLength();
      let len = attributes.getLength();
      // Java: for(int i = 0; i < len; ++i)
      for i in 0..len
      {
         // Java: String attrLocalName = attributes.getLocalName(i);
         let attr_local_name = attributes.getLocalName(i);
         // Java: if ("Requires".equals(attrLocalName))
         if "Requires" == attr_local_name
         {
            // Java: String attrValue = attributes.getValue(i);
            let attr_value = attributes.getValue_index(i);
            // Java: return attrValue;
            return Some(attr_value.to_string());
         }
      }
      // Java: return null;
      None
    }

    // Java: void processRequiresAttr(NamespaceStack namespaceStack, Attributes attributes, int level)
    pub fn processRequiresAttr(&mut self, _namespaceStack: &NamespaceStack, attributes: &Attributes, level: i32) {
      // Java: String attrRequires = this.getRequiresAttrValue(attributes);
      let attr_requires = self.getRequiresAttrValue(attributes);
      // Java: if (null != attrRequires)
      if let Some(_attr_requires) = attr_requires
      {
         // Java: Namespace ns = namespaceStack.getNamespaceForPrefix(attrRequires);
         // Java: String nsUri = ns != null ? ns.getURI() : null;
         let _ns_uri: Option<&str> = None; // TODO: NamespaceStack.getNamespaceForPrefix not fully implemented
         // Java: MceSkipItem top = this.mMceSkipStack.mStack.size() <= 0 ? null : (MceSkipItem)this.mMceSkipStack.mStack.peek();
         let understands_found = false; // placeholder since getNamespaceForPrefix is stubbed
         // Java: if (null != this.mUnderstandsSet.lookup(nsUri, (String)null, true) && MCEContext.MceSkipState.MCE_SKIP_STATE_CHOICE_MATCHED != top.state)
         if understands_found
         {
            if let Some(top) = self.mMceSkipStack.mStack.last_mut() {
                if top.state != MceSkipState::MCE_SKIP_STATE_CHOICE_MATCHED {
                    // Java: top.state = MCEContext.MceSkipState.MCE_SKIP_STATE_CHOICE_MATCHED;
                    top.state = MceSkipState::MCE_SKIP_STATE_CHOICE_MATCHED;
                    // Java: this.mMceSkipStack.push(level, level + 1, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
                    self.mMceSkipStack.push(level, level + 1, MceSkipState::MCE_SKIP_STATE_IGNORE);
                }
            }
         }
         // Java: else
         else
         {
            // Java: this.mMceSkipStack.push(level, Integer.MAX_VALUE, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
            self.mMceSkipStack.push(level, i32::MAX, MceSkipState::MCE_SKIP_STATE_IGNORE);
         }
      }
    }

    // Java: boolean isInMCEOrIgnoredNamespace(String uri)
    pub fn isInMCEOrIgnoredNamespace(&mut self, uri: &str) -> bool {
      // Java: return "http://schemas.openxmlformats.org/markup-compatibility/2006".equals(uri) || null != this.mIgnoredSet.lookup(uri, (String)null, true);
      NS_MCE == uri || self.mIgnoredSet.lookup(Some(uri), None, true).is_some()
    }

    // Java: boolean shouldSkipStartElement(NamespaceStack namespaceStack, String namespaceURI, String localName, Attributes attributes, int level)
    pub fn shouldSkipStartElement(&mut self, namespaceStack: &NamespaceStack, namespace_uri: &str, local_name: &str, attributes: &Attributes, level: i32) -> bool {
      // Java: boolean skip = this.shouldSkipSkipStack(level);
      let skip = self.shouldSkipSkipStack(level);
      // Java: if (skip)
      if skip
      {
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: this.processMCEAttributes(namespaceStack, attributes, level);
         self.processMCEAttributes(namespaceStack, attributes, level);
         // Java: if ("http://schemas.openxmlformats.org/markup-compatibility/2006".equals(namespaceURI))
         if NS_MCE == namespace_uri
         {
            // Java: if ("AlternateContent".equals(localName))
            if "AlternateContent" == local_name
            {
               // Java: this.mMceSkipStack.push(level, level + 1, MCEContext.MceSkipState.MCE_SKIP_STATE_ALTERNATE_CONTENT);
               self.mMceSkipStack.push(level, level + 1, MceSkipState::MCE_SKIP_STATE_ALTERNATE_CONTENT);
            }
            // Java: else if ("Choice".equals(localName))
            else if "Choice" == local_name
            {
               // Java: this.processRequiresAttr(namespaceStack, attributes, level);
               self.processRequiresAttr(namespaceStack, attributes, level);
            }
            // Java: else if ("Fallback".equals(localName))
            else if "Fallback" == local_name
            {
               // Java: MceSkipItem top = this.mMceSkipStack.mStack.size() <= 0 ? null : (MceSkipItem)this.mMceSkipStack.mStack.peek();
               // Java: if (top != null)
               if let Some(top) = self.mMceSkipStack.mStack.last()
               {
                  // Java: if (MCEContext.MceSkipState.MCE_SKIP_STATE_CHOICE_MATCHED != top.state)
                  if MceSkipState::MCE_SKIP_STATE_CHOICE_MATCHED != top.state
                  {
                     // Java: this.mMceSkipStack.push(level, level + 1, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
                     self.mMceSkipStack.push(level, level + 1, MceSkipState::MCE_SKIP_STATE_IGNORE);
                  }
                  // Java: else
                  else
                  {
                     // Java: this.mMceSkipStack.push(level, Integer.MAX_VALUE, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
                     self.mMceSkipStack.push(level, i32::MAX, MceSkipState::MCE_SKIP_STATE_IGNORE);
                  }
               }
            }
         }
         // Java: else if (null != this.mIgnoredSet.lookup(namespaceURI, (String)null, true))
         else if self.mIgnoredSet.lookup(Some(namespace_uri), None, true).is_some()
         {
            // Java: if (null != this.mProcessContentSet.lookup(namespaceURI, localName, false))
            if self.mProcessContentSet.lookup(Some(namespace_uri), Some(local_name), false).is_some()
            {
               // Java: this.mMceSkipStack.push(level, level + 1, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
               self.mMceSkipStack.push(level, level + 1, MceSkipState::MCE_SKIP_STATE_IGNORE);
            }
            // Java: else
            else
            {
               // Java: this.mMceSkipStack.push(level, Integer.MAX_VALUE, MCEContext.MceSkipState.MCE_SKIP_STATE_IGNORE);
               self.mMceSkipStack.push(level, i32::MAX, MceSkipState::MCE_SKIP_STATE_IGNORE);
            }
         }
         // Java: return this.shouldSkipSkipStack(level);
         self.shouldSkipSkipStack(level)
      }
    }

    // Java: boolean shouldSkipEndElement(NamespaceStack namespaceStack, String namespaceURI, String localName, int level)
    pub fn shouldSkipEndElement(&mut self, _namespaceStack: &NamespaceStack, _namespace_uri: &str, _local_name: &str, level: i32) -> bool {
      // Java: boolean skip = this.shouldSkipSkipStack(level);
      let skip = self.shouldSkipSkipStack(level);
      // Java: if (skip)
      if skip
      {
         // Java: MceSkipItem top = this.mMceSkipStack.mStack.size() <= 0 ? null : (MceSkipItem)this.mMceSkipStack.mStack.peek();
         // Java: Assert.assertNotNull("top should not be null", top);
         // Java: if (top.level_start == level)
         if let Some(top) = self.mMceSkipStack.mStack.last() {
            if top.level_start == level
            {
               // Java: this.mMceSkipStack.mStack.pop();
               self.mMceSkipStack.mStack.pop();
            }
            // Java: else if (top.level_end == level)
            else if top.level_end == level
            {
               // Java: --top.level_end;
               let top_mut = self.mMceSkipStack.mStack.last_mut().unwrap();
               top_mut.level_end -= 1;
            }
         }
      }
      // Java: this.mIgnoredSet.cleanup(level);
      self.mIgnoredSet.cleanup(level);
      // Java: this.mProcessContentSet.cleanup(level);
      self.mProcessContentSet.cleanup(level);
      // Java: this.mUnderstandsSet.cleanup(level);
      self.mUnderstandsSet.cleanup(level);
      // Java: return skip;
      skip
    }

    // Java: boolean shouldSkipSkipStack(int level)
    pub fn shouldSkipSkipStack(&self, level: i32) -> bool {
      // Java: if (this.mMceSkipStack.mStack.size() <= 0)
      if self.mMceSkipStack.mStack.is_empty()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: MceSkipItem top = (MceSkipItem)this.mMceSkipStack.mStack.peek();
         let top = self.mMceSkipStack.mStack.last().unwrap();
         // Java: return level >= top.level_start && level < top.level_end;
         level >= top.level_start && level < top.level_end
      }
    }

}

// Java: public class MceQNameLevel
impl MceQNameLevel {
    // Java: MceQNameLevel(String ns, String ln, int level)
    pub fn new(ns: Option<&str>, ln: Option<&str>, level: i32) -> Self {
         // Java: super();
         // Java: this.ns = ns;
         // Java: this.ln = ln;
         // Java: this.level = level;
         MceQNameLevel {
             ns: ns.map(|s| s.to_string()),
             ln: ln.map(|s| s.to_string()),
             level,
         }
    }

}

// Java: public class MceQNameLevelSet
impl MceQNameLevelSet {
    // Java: MceQNameLevelSet()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.mList = new ArrayList(4);
         // Java: this.mPos = new MutableInt(0);
         MceQNameLevelSet {
             mList: Vec::with_capacity(4),
             mMaxLevel: 0,
             mPos: MutableInt::new(),
         }
    }

    // Java: static int compare(String lhs, String rhs)
    pub fn compare(lhs: Option<&str>, rhs: Option<&str>) -> i32 {
         // Java: return null == lhs ? (null == rhs ? 0 : -1) : (null == rhs ? 1 : lhs.compareTo(rhs));
         match (lhs, rhs) {
             (None, None) => 0,
             (None, Some(_)) => -1,
             (Some(_), None) => 1,
             (Some(l), Some(r)) => l.cmp(r) as i32,
         }
    }

    // Java: boolean lookup(String ns, String ln, MutableInt pos, boolean ignore_ln)
    pub fn lookup_with_pos(&self, ns: Option<&str>, ln: Option<&str>, pos: &mut MutableInt, ignore_ln: bool) -> bool {
         // Java: Assert.assertNotNull("mList should not be null.", this.mList);
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: int j = this.mList.size();
         let mut j: i32 = self.mList.len() as i32;
         // Java: while(i < j)
         while i < j
         {
            // Java: int m = i + (j - i) / 2;
            let m = i + (j - i) / 2;
            // Java: Assert.assertTrue("(i <= m) && (m < j) should be true.", i <= m && m < j);
            Assert::assertTrue_msg("(i <= m) && (m < j) should be true.", i <= m && m < j);
            // Java: MceQNameLevel q2 = (MceQNameLevel)this.mList.get(m);
            let q2 = &self.mList[m as usize];
            // Java: int ns_cmp = compare(ns, q2.ns);
            let ns_cmp = Self::compare(ns, q2.ns.as_deref());
            // Java: int cmp = ignore_ln ? ns_cmp : (0 == ns_cmp ? compare(ln, q2.ln) : ns_cmp);
            let cmp = if ignore_ln { ns_cmp } else { if ns_cmp == 0 { Self::compare(ln, q2.ln.as_deref()) } else { ns_cmp } };
            // Java: if (cmp < 0)
            if cmp < 0
            {
               // Java: j = m;
               j = m;
            }
            // Java: else
            else
            {
               // Java: if (cmp <= 0)
               if cmp <= 0
               {
                  // Java: pos.value = m;
                  pos.value = m;
                  // Java: return true;
                  return true;
               }
               // Java: i = m + 1;
               i = m + 1;
            }
         }
         // Java: Assert.assertTrue("(i == j) should be true.", i == j);
         Assert::assertTrue_msg("(i == j) should be true.", i == j);
         // Java: pos.value = i;
         pos.value = i;
         // Java: return false;
         false
    }

    // Java: MceQNameLevel lookup(String ns, String ln, boolean ignore_ln)
    pub fn lookup(&mut self, ns: Option<&str>, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
         // Java: Assert.assertNotNull("mList should not be null.", this.mList);
         // Java: return this.lookup(ns, ln, this.mPos, ignore_ln) ? (MceQNameLevel)this.mList.get(this.mPos.value) : null;
         let mut pos = MutableInt::new_with_value(self.mPos.value);
         let found = self.lookup_with_pos(ns, ln, &mut pos, ignore_ln);
         self.mPos.value = pos.value;
         if found {
             Some(&self.mList[self.mPos.value as usize])
         } else {
             None
         }
    }

    // Java: void add(String ns, String ln, int level)
    pub fn add(&mut self, ns: Option<&str>, ln: Option<&str>, level: i32) {
         // Java: Assert.assertNotNull("mList should not be null.", this.mList);
         // Java: if (!this.lookup(ns, ln, this.mPos, false))
         let mut pos = MutableInt::new_with_value(self.mPos.value);
         let found = self.lookup_with_pos(ns, ln, &mut pos, false);
         self.mPos.value = pos.value;
         if !found
         {
            // Java: this.mList.add(this.mPos.value, new MceQNameLevel(ns, ln, level));
            self.mList.insert(self.mPos.value as usize, MceQNameLevel::new(ns, ln, level));
            // Java: if (this.mMaxLevel < level)
            if self.mMaxLevel < level
            {
               // Java: this.mMaxLevel = level;
               self.mMaxLevel = level;
            }
         }
    }

    // Java: boolean cleanup(int level)
    pub fn cleanup(&mut self, level: i32) -> bool {
         // Java: Assert.assertNotNull("mList should not be null.", this.mList);
         // Java: if (this.mMaxLevel >= level)
         if self.mMaxLevel >= level
         {
            // Java: this.mMaxLevel = 0;
            self.mMaxLevel = 0;
            // Java: for(int i = 0; i < this.mList.size(); ++i)
            let mut i: i32 = 0;
            while (i as usize) < self.mList.len()
            {
               // Java: int levelAtI = ((MceQNameLevel)this.mList.get(i)).level;
               let level_at_i = self.mList[i as usize].level;
               // Java: if (levelAtI >= level)
               if level_at_i >= level
               {
                  // Java: Assert.assertTrue("(levelAtI == level) should be true.", levelAtI == level);
                  Assert::assertTrue_msg("(levelAtI == level) should be true.", level_at_i == level);
                  // Java: this.mList.remove(i);
                  self.mList.remove(i as usize);
                  // Java: --i;
                  i -= 1;
               }
               // Java: else if (levelAtI > this.mMaxLevel)
               else if level_at_i > self.mMaxLevel
               {
                  // Java: this.mMaxLevel = level;
                  self.mMaxLevel = level;
               }
               i += 1;
            }
         }
         // Java: Assert.assertTrue("(level == 0) || (mMaxLevel < level) should be true", level == 0 || this.mMaxLevel < level);
         Assert::assertTrue_msg("(level == 0) || (mMaxLevel < level) should be true", level == 0 || self.mMaxLevel < level);
         // Java: return true;
         true
    }

}

// Java: public enum MceSkipState
impl MceSkipState {
    // Java: MceSkipState()
    pub fn new() -> Self {
        MceSkipState::MCE_SKIP_STATE_IGNORE
    }

}

// Java: public class MceSkipItem
impl MceSkipItem {
    // Java: MceSkipItem(int level_start, int level_end, MceSkipState state)
    pub fn new(level_start: i32, level_end: i32, state: MceSkipState) -> Self {
         // Java: super();
         // Java: this.level_start = level_start;
         // Java: this.level_end = level_end;
         // Java: this.state = state;
         MceSkipItem {
             level_start,
             level_end,
             state,
         }
    }

}

// Java: public class MceSkipStack
impl MceSkipStack {
    // Java: MceSkipStack()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.mStack = new Stack();
         MceSkipStack {
             mStack: Vec::new(),
         }
    }

    // Java: MceSkipItem push(int level_start, int level_end, MceSkipState state)
    pub fn push(&mut self, level_start: i32, level_end: i32, state: MceSkipState) {
         // Java: Assert.assertNotNull("mStack should not be null.", this.mStack);
         // Java: return (MceSkipItem)this.mStack.push(new MceSkipItem(level_start, level_end, state));
         self.mStack.push(MceSkipItem::new(level_start, level_end, state));
    }

    // Java: MceSkipItem pop()
    pub fn pop(&mut self) -> Option<MceSkipItem> {
         // Java: return this.mStack.size() <= 0 ? null : (MceSkipItem)this.mStack.pop();
         if self.mStack.is_empty() { None } else { self.mStack.pop() }
    }

    // Java: MceSkipItem top()
    pub fn top(&self) -> Option<&MceSkipItem> {
         // Java: return this.mStack.size() <= 0 ? null : (MceSkipItem)this.mStack.peek();
         self.mStack.last()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
         // Java: Assert.assertNotNull("mStack should not be null.", this.mStack);
         // Java: return this.mStack.size();
         self.mStack.len() as i32
    }

}
