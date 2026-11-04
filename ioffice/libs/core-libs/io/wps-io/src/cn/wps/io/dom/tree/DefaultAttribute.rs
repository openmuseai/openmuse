// Java: import cn.wps.io.dom.Element

// Java field: String TAG
const TAG: &str = "";
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: Object sPoolSync
static S_POOL_SYNC: std::sync::Mutex<()> = std::sync::Mutex::new(());
// Java field: DefaultAttribute sPool
static S_POOL: std::sync::OnceLock<std::sync::Mutex<Option<Box<DefaultAttribute>>>> = std::sync::OnceLock::new();
// Java field: int sPoolSize
static S_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int DEFAULT_MAX_POOL_SIZE
pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = 140;
// Java field: int sMaxPoolSize
static S_MAX_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int sNewInsCount
static S_NEW_INS_COUNT: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

// Java: public class DefaultAttribute extends FlyweightAttribute
pub struct DefaultAttribute {
    // Java: DefaultAttribute extends FlyweightAttribute ⇒ Rust 用组态表达继承（模板既有 `base` 约定）
    pub(crate) base: crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute,
    // Java field: Element parent
    pub(crate) parent: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: DefaultAttribute next
    next: Option<Box<DefaultAttribute>>,
}

// Java: public class DefaultAttribute extends FlyweightAttribute
impl DefaultAttribute {
    // Java: DefaultAttribute(String namespaceURI, String prefix, String localName, String value)（DOM 通路构造）
    pub(crate) fn create(ns: &str, prefix: &str, local: &str, value: &str) -> Self {
        DefaultAttribute {
            base: crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute::create(ns, prefix, local, value),
            parent: None,
            next: None,
        }
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        self.base.getName()
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
        self.base.mValue.clone().unwrap_or_default()
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
        self.base.mNamespaceURI.clone().unwrap_or_default()
    }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&self) -> String {
        match self.base.mPrefix.clone() {
            Some(p) if !p.is_empty() => format!("{}:{}", p, self.base.getName()),
            _ => self.base.getName(),
        }
    }

    // Java: Node clone()（浅拷贝，DOM 通路需要）
    pub(crate) fn clone_shallow(&self) -> DefaultAttribute {
        DefaultAttribute {
            base: crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute {
                mNamespaceURI: self.base.mNamespaceURI.clone(),
                mPrefix: self.base.mPrefix.clone(),
                mLocalName: self.base.mLocalName.clone(),
                mValue: self.base.mValue.clone(),
            },
            parent: None,
            next: None,
        }
    }

    // Java field: String TAG
    // Java field: boolean DEBUG
    // Java field: Object sPoolSync
    // Java field: DefaultAttribute sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: int sNewInsCount
    // Java field: Element parent
    // Java field: DefaultAttribute next

    // Java: static void setMaxPoolSize(int size)
    pub fn setMaxPoolSize(&mut self) {
      // Java: sMaxPoolSize = size;
      todo!();
      // Java: synchronized(sPoolSync)
      todo!();
      {
         // Java: while(sPoolSize > sMaxPoolSize)
      todo!();
         {
            // Java: DefaultAttribute m = sPool;
      todo!();
            // Java: sPool = m.next;
      todo!();
            // Java: m.next = null;
      todo!();
            // Java: --sPoolSize;
      todo!();
         }
      }
    }

    // Java: static void clearPool()
    pub fn clearPool(&mut self) {
      // Java: synchronized(sPoolSync)
      todo!();
      {
         // Java: while(null != sPool)
      todo!();
         {
            // Java: DefaultAttribute m = sPool;
      todo!();
            // Java: sPool = m.next;
      todo!();
            // Java: m.next = null;
      todo!();
            // Java: --sPoolSize;
      todo!();
         }
      }
    }

    // Java: static DefaultAttribute obtain()
    pub fn obtain(&mut self) -> DefaultAttribute {
      // Java: synchronized(sPoolSync)
      todo!();
      {
         // Java: if (sPool != null)
      todo!();
         {
            // Java: DefaultAttribute m = sPool;
      todo!();
            // Java: sPool = m.next;
      todo!();
            // Java: m.next = null;
      todo!();
            // Java: --sPoolSize;
      todo!();
            // Java: return m;
      todo!();
         }
      }
      // Java: return new DefaultAttribute();
      todo!();
    }

    // Java: static DefaultAttribute obtain(String namespaceURI, String prefix, String localName)
    pub fn obtain_2(&mut self) -> DefaultAttribute {
      // Java: DefaultAttribute attr = obtain();
      todo!();
      // Java: attr.mNamespaceURI = namespaceURI;
      todo!();
      // Java: attr.mPrefix = prefix;
      todo!();
      // Java: attr.mLocalName = localName;
      todo!();
      // Java: return attr;
      todo!();
    }

    // Java: static DefaultAttribute obtain(String namespaceURI, String prefix, String localName, String value)
    pub fn obtain_3(&mut self) -> DefaultAttribute {
      // Java: DefaultAttribute attr = obtain();
      todo!();
      // Java: attr.mNamespaceURI = namespaceURI;
      todo!();
      // Java: attr.mPrefix = prefix;
      todo!();
      // Java: attr.mLocalName = localName;
      todo!();
      // Java: attr.mValue = value;
      todo!();
      // Java: return attr;
      todo!();
    }

    // Java: static DefaultAttribute obtain(Element parent, String namespaceURI, String prefix, String localName, String value)
    pub fn obtain_4(&mut self) -> DefaultAttribute {
      // Java: DefaultAttribute attr = obtain();
      todo!();
      // Java: attr.parent = parent;
      todo!();
      // Java: attr.mNamespaceURI = namespaceURI;
      todo!();
      // Java: attr.mPrefix = prefix;
      todo!();
      // Java: attr.mLocalName = localName;
      todo!();
      // Java: attr.mValue = value;
      todo!();
      // Java: return attr;
      todo!();
    }

    // Java: static DefaultAttribute obtain(String localName, String value)
    pub fn obtain_5(&mut self) -> DefaultAttribute {
      // Java: DefaultAttribute attr = obtain();
      todo!();
      // Java: attr.mLocalName = localName;
      todo!();
      // Java: attr.mValue = value;
      todo!();
      // Java: return attr;
      todo!();
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: this.parent = null;
      todo!();
      // Java: this.mNamespaceURI = null;
      todo!();
      // Java: this.mPrefix = null;
      todo!();
      // Java: this.mLocalName = null;
      todo!();
      // Java: this.mValue = null;
      todo!();
      // Java: synchronized(sPoolSync)
      todo!();
      {
         // Java: if (sPoolSize < sMaxPoolSize)
      todo!();
         {
            // Java: this.next = sPool;
      todo!();
            // Java: sPool = this;
      todo!();
            // Java: ++sPoolSize;
      todo!();
         }
      }
    }

    // Java: DefaultAttribute()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: DefaultAttribute(String namespaceURI, String prefix, String localName, String qualifiedName)
    pub fn new_1() -> Self {
      // Java: super(namespaceURI, prefix, localName, qualifiedName);
      todo!();
    }

    // Java: DefaultAttribute(String namespaceURI, String prefix, String localName, String qualifiedName, String value)
    pub fn new_2() -> Self {
      // Java: super(namespaceURI, prefix, localName, qualifiedName, value);
      todo!();
    }

    // Java: DefaultAttribute(Element parent, String namespaceURI, String prefix, String localName, String qualifiedName, String value)
    pub fn new_3() -> Self {
      // Java: super(namespaceURI, prefix, localName, qualifiedName, value);
      todo!();
      // Java: this.parent = parent;
      todo!();
    }

    // Java: DefaultAttribute(String name, String value)
    pub fn new_4() -> Self {
      // Java: super(name, value);
      todo!();
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self) {
      // Java: this.mValue = value;
      todo!();
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.parent;
      todo!();
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&mut self) {
      // Java: this.parent = parent;
      todo!();
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&mut self) -> bool {
      // Java: return true;
      todo!();
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&mut self) -> bool {
      // Java: return false;
      todo!();
    }

}
