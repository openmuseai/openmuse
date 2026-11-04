// Java: import cn.wps.io.dom.Element

// Java field: String TAG
const TAG: &str = "";
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: Object sPoolSync
static S_POOL_SYNC: std::sync::Mutex<()> = std::sync::Mutex::new(());
// Java field: DefaultText sPool
static S_POOL: std::sync::OnceLock<std::sync::Mutex<Option<Box<DefaultText>>>> = std::sync::OnceLock::new();
// Java field: int sPoolSize
static S_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int DEFAULT_MAX_POOL_SIZE
pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = 20;
// Java field: int sMaxPoolSize
static S_MAX_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int sNewInsCount
static S_NEW_INS_COUNT: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

// Java: public class DefaultText extends FlyweightText
pub struct DefaultText {
    // Java: DefaultText extends FlyweightText ⇒ Rust 用组态表达继承（模板既有 `base` 约定）
    pub(crate) base: crate::io::dom::tree::FlyweightText::FlyweightText,
    // Java field: Element parent
    pub(crate) parent: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: DefaultText next
    next: Option<Box<DefaultText>>,
}

// Java: public class DefaultText extends FlyweightText
impl DefaultText {
    // Java: String getText()（FlyweightText；DOM 通路取文本值）
    pub fn getText(&self) -> String {
        self.base.getText()
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
        self.base.getText()
    }

    // Java: Node clone()（浅拷贝，DOM 通路需要）
    pub(crate) fn clone_shallow(&self) -> DefaultText {
        DefaultText { base: crate::io::dom::tree::FlyweightText::FlyweightText { text: self.base.text.clone() }, parent: None, next: None }
    }

    // Java field: String TAG
    // Java field: boolean DEBUG
    // Java field: Object sPoolSync
    // Java field: DefaultText sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: int sNewInsCount

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
            // Java: DefaultText m = sPool;
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
            // Java: DefaultText m = sPool;
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

    // Java: static DefaultText obtain()
    pub fn obtain(&mut self) -> DefaultText {
      // Java: synchronized(sPoolSync)
      todo!();
      {
         // Java: if (sPool != null)
      todo!();
         {
            // Java: DefaultText m = sPool;
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
      // Java: return new DefaultText();
      todo!();
    }

    // Java: static DefaultText obtain(String text)
    pub fn obtain_2(&mut self) -> DefaultText {
      // Java: DefaultText o = obtain();
      todo!();
      // Java: o.text = text;
      todo!();
      // Java: return o;
      todo!();
    }

    // Java: static DefaultText obtain(Element parent, String text)
    pub fn obtain_3(&mut self) -> DefaultText {
      // Java: DefaultText o = obtain();
      todo!();
      // Java: o.parent = parent;
      todo!();
      // Java: o.text = text;
      todo!();
      // Java: return o;
      todo!();
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: this.parent = null;
      todo!();
      // Java: this.text = null;
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

    // Java: DefaultText()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: DefaultText(String text)
    pub fn new_1(text: &str) -> Self {
      // Java: super(text);
      DefaultText { base: crate::io::dom::tree::FlyweightText::FlyweightText::new_1(text), parent: None, next: None }
    }

    // Java: DefaultText(Element parent, String text)
    pub fn new_2() -> Self {
      // Java: super(text);
      todo!();
      // Java: this.parent = parent;
      todo!();
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: &str) {
      // Java: this.text = text;
      self.base.setText(text);
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
