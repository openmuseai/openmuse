// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.tree.NamespaceStack
// Java: import cn.wps.moffice.util.MutableInt
// Java: import java.util.ArrayList
// Java: import java.util.Stack
// Java: import org.xml.sax.Attributes

// Java: public final class MCEContext
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
    pub mMceDisabled: bool,
}

pub const NS_MCE: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
pub const MCE_ATTR_IGNORABLE: &str = "Ignorable";
pub const MCE_ATTR_PROCESS_CONTENT: &str = "ProcessContent";
pub const MCE_ATTR_MUST_UNDERSTAND: &str = "MustUnderstand";
pub const MCE_ELEMENT_ALTERNATE_CONTENT: &str = "AlternateContent";
pub const MCE_ELEMENT_CHOICE: &str = "Choice";
pub const MCE_ATTR_REQUIRES: &str = "Requires";
pub const MCE_ELEMENT_FALLBACK: &str = "Fallback";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MceSkipState {
    MCE_SKIP_STATE_IGNORE,
    MCE_SKIP_STATE_ALTERNATE_CONTENT,
    MCE_SKIP_STATE_CHOICE_MATCHED,
}

struct MceQNameLevel {
    pub ns: String,
    pub ln: Option<String>,
    pub level: i32,
}

struct MceQNameLevelSet {
    mList: Vec<MceQNameLevel>,
    mMaxLevel: i32,
    mPos: i32,
}

struct MceSkipItem {
    pub level_start: i32,
    pub level_end: i32,
    pub state: MceSkipState,
}

struct MceSkipStack {
    pub mStack: Vec<MceSkipItem>,
}

impl MceQNameLevelSet {
    fn new() -> Self {
        MceQNameLevelSet {
            mList: Vec::with_capacity(4),
            mMaxLevel: 0,
            mPos: 0,
        }
    }

    fn compare(lhs: Option<&str>, rhs: Option<&str>) -> i32 {
        match (lhs, rhs) {
            (None, None) => 0,
            (None, Some(_)) => -1,
            (Some(_), None) => 1,
            (Some(l), Some(r)) => l.cmp(r) as i32,
        }
    }

    fn lookup_pos(&self, ns: &str, ln: Option<&str>, ignore_ln: bool) -> (bool, usize) {
        let mut i = 0;
        let mut j = self.mList.len();
        while i < j {
            let m = i + (j - i) / 2;
            let q2 = &self.mList[m];
            let ns_cmp = ns.cmp(&q2.ns) as i32;
            let cmp = if ignore_ln {
                ns_cmp
            } else {
                if ns_cmp == 0 {
                    Self::compare(ln, q2.ln.as_deref())
                } else {
                    ns_cmp
                }
            };
            if cmp < 0 {
                j = m;
            } else if cmp > 0 {
                i = m + 1;
            } else {
                return (true, m);
            }
        }
        (false, i)
    }

    pub fn lookup(&self, ns: &str, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
        let (found, pos) = self.lookup_pos(ns, ln, ignore_ln);
        if found {
            Some(&self.mList[pos])
        } else {
            None
        }
    }

    pub fn add(&mut self, ns: &str, ln: Option<&str>, level: i32) {
        let (found, pos) = self.lookup_pos(ns, ln, false);
        if !found {
            self.mList.insert(pos, MceQNameLevel {
                ns: ns.to_string(),
                ln: ln.map(|s| s.to_string()),
                level,
            });
            if self.mMaxLevel < level {
                self.mMaxLevel = level;
            }
        }
    }

    pub fn cleanup(&mut self, level: i32) -> bool {
        if self.mMaxLevel >= level {
            self.mMaxLevel = 0;
            let mut i = 0;
            while i < self.mList.len() {
                let level_at_i = self.mList[i].level;
                if level_at_i >= level {
                    self.mList.remove(i);
                } else {
                    if level_at_i > self.mMaxLevel {
                        self.mMaxLevel = level_at_i;
                    }
                    i += 1;
                }
            }
        }
        true
    }
}

impl MceSkipStack {
    fn new() -> Self {
        MceSkipStack { mStack: Vec::new() }
    }

    pub fn push(&mut self, level_start: i32, level_end: i32, state: MceSkipState) {
        self.mStack.push(MceSkipItem { level_start, level_end, state });
    }

    pub fn pop(&mut self) -> Option<MceSkipItem> {
        self.mStack.pop()
    }

    pub fn top(&self) -> Option<&MceSkipItem> {
        self.mStack.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut MceSkipItem> {
        self.mStack.last_mut()
    }

    pub fn size(&self) -> usize {
        self.mStack.len()
    }
}

// Java: public final class MCEContext
impl MCEContext {
    // Java: MCEContext()
    pub fn new() -> Self {
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
        self.mMceDisabled = disable;
    }

    // Java: boolean getDisableMCE()
    pub fn getDisableMCE(&self) -> bool {
        self.mMceDisabled
    }

    // Java: MceQNameLevel lookupUnderstandsSet(String ns, String ln, boolean ignore_ln)
    fn lookupUnderstandsSet(&self, ns: &str, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
        self.mUnderstandsSet.lookup(ns, ln, ignore_ln)
    }

    // Java: MceQNameLevel lookupIgnoredSet(String ns, String ln, boolean ignore_ln)
    fn lookupIgnoredSet(&self, ns: &str, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
        self.mIgnoredSet.lookup(ns, ln, ignore_ln)
    }

    // Java: MceQNameLevel lookupProcesscontentSet(String ns, String ln, boolean ignore_ln)
    fn lookupProcesscontentSet(&self, ns: &str, ln: Option<&str>, ignore_ln: bool) -> Option<&MceQNameLevel> {
        self.mProcessContentSet.lookup(ns, ln, ignore_ln)
    }

    // Java: void addIntoUnderstandsSet(String ns)
    pub fn addIntoUnderstandsSet(&mut self, ns: &str) {
        self.mUnderstandsSet.add(ns, None, 0);
    }

    // Java: void addIntoIgnoredSet(String ns, int level)
    fn addIntoIgnoredSet(&mut self, ns: &str, level: i32) {
        self.mIgnoredSet.add(ns, None, level);
    }

    // Java: void addIntoProcesscontentSet(String ns, String ln, int level)
    fn addIntoProcesscontentSet(&mut self, ns: &str, ln: &str, level: i32) {
        self.mProcessContentSet.add(ns, Some(ln), level);
    }

    // Java: void processIgnorableAttr(NamespaceStack namespaceStack, String attrValue, int level)
    fn processIgnorableAttr(&mut self, _namespaceStack: &dyn NamespaceStackLike, attrValue: &str, level: i32) {
        todo!()
    }

    // Java: void processProcessContentAttr(NamespaceStack namespaceStack, String attrValue, int level)
    fn processProcessContentAttr(&mut self, _namespaceStack: &dyn NamespaceStackLike, attrValue: &str, level: i32) {
        todo!()
    }

    // Java: void processMustUnderstandAttr(NamespaceStack namespaceStack, String attrValue, int level)
    fn processMustUnderstandAttr(&mut self, _namespaceStack: &dyn NamespaceStackLike, attrValue: &str, level: i32) {
        todo!()
    }

    // Java: void processMCEAttributes(NamespaceStack namespaceStack, Attributes attributes, int level)
    fn processMCEAttributes(&mut self, _namespaceStack: &dyn NamespaceStackLike, _attributes: &dyn AttributesLike, level: i32) {
        todo!()
    }

    // Java: String getRequiresAttrValue(Attributes attributes)
    fn getRequiresAttrValue(&self, _attributes: &dyn AttributesLike) -> Option<String> {
        todo!()
    }

    // Java: void processRequiresAttr(NamespaceStack namespaceStack, Attributes attributes, int level)
    fn processRequiresAttr(&mut self, _namespaceStack: &dyn NamespaceStackLike, _attributes: &dyn AttributesLike, level: i32) {
        todo!()
    }

    // Java: boolean isInMCEOrIgnoredNamespace(String uri)
    pub fn isInMCEOrIgnoredNamespace(&self, uri: &str) -> bool {
        NS_MCE == uri || self.mIgnoredSet.lookup(uri, None, true).is_some()
    }

    // Java: boolean shouldSkipStartElement(NamespaceStack namespaceStack, String namespaceURI, String localName, Attributes attributes, int level)
    pub fn shouldSkipStartElement(&mut self, _namespaceStack: &dyn NamespaceStackLike, namespaceURI: &str, localName: &str, _attributes: &dyn AttributesLike, level: i32) -> bool {
        todo!()
    }

    // Java: boolean shouldSkipEndElement(NamespaceStack namespaceStack, String namespaceURI, String localName, int level)
    pub fn shouldSkipEndElement(&mut self, _namespaceStack: &dyn NamespaceStackLike, namespaceURI: &str, localName: &str, level: i32) -> bool {
        todo!()
    }

    // Java: boolean shouldSkipSkipStack(int level)
    pub fn shouldSkipSkipStack(&self, level: i32) -> bool {
        if self.mMceSkipStack.mStack.is_empty() {
            return false;
        }
        let top = self.mMceSkipStack.mStack.last().unwrap();
        level >= top.level_start && level < top.level_end
    }
}

pub trait NamespaceStackLike {
    fn getNamespaceForPrefix(&self, prefix: &str) -> Option<(&str, &str)>;
}

pub trait AttributesLike {
    fn getValue(&self, uri: &str, local_name: &str) -> Option<String>;
    fn getLength(&self) -> i32;
    fn getLocalName(&self, index: i32) -> String;
    fn getValue_at(&self, index: i32) -> String;
}
