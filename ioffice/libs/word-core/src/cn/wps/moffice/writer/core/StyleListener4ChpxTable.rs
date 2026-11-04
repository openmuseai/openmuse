// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.core.StyleListenerBase
use crate::cn::wps::moffice::writer::core::StyleListenerBase::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;

// rust: ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— **修复从未编译的产物**：
//   ① 模板给 struct 加了 `#[derive(Default)]`，但 Java 无默认构造器，且基类
//      `StyleListenerBase` 持有 `KStyles`（内含 `TextDocument` 值语义）不可 `Default` ⇒ 删除。
//   ② 模板调用 `StyleListenerBase::new(&styles)`，真实基类构造器名为 `StyleListenerBase`
//      （Java `protected StyleListenerBase(KStyles styles)`）⇒ 对齐。
//   ③ 模板把 `entry.getProperty()` 再包一层 `Some(...)`，但 `ITable.Entry.getProperty()`
//      已声明为 `Option<&PropertySet>` ⇒ 双重包裹，去掉。
//   ④ 模板以 `&self.mStyles`（`&Option<KStyles>`）传参，真实签名需 `&mut KStyles`
//      （Java `HashMap` 可经共享引用变更，Rust 需显式可变）⇒ 加 null-guard 展开。

// java public static final int PROP_ID = 2;
pub(crate) const PROP_ID: i32 = 2;
// java public class StyleListener4ChpxTable extends StyleListenerBase implements IPapxTable.Listener
pub struct StyleListener4ChpxTable {
    // java StyleListenerBase super_
    pub(crate) super_: StyleListenerBase,
}

// java public class StyleListener4ChpxTable extends StyleListenerBase implements IPapxTable.Listener
impl StyleListener4ChpxTable {
    // java public  StyleListener4ChpxTable(KStyles styles)
    pub fn StyleListener4ChpxTable(styles: Option<KStyles>) -> Self {
      // java super(styles);
      Self { super_: StyleListenerBase::StyleListenerBase(styles) }
    }

    // java public void beforeAddEntry(int cp, PropertySet data)
    pub fn beforeAddEntry(&mut self, cp: i32, data: &PropertySet) {
    }

    // java public void afterAddEntry(ITable.Entry entry)
    pub fn afterAddEntry(&mut self, entry: &dyn ITableEntry) {
    }

    // java public void beforeRemoveEntry(ITable.Entry entry)
    pub fn beforeRemoveEntry(&mut self, entry: &dyn ITableEntry) {
    }

    // java public void afterRemoveEntry(ITable.Entry entry)
    pub fn afterRemoveEntry(&mut self, entry: &dyn ITableEntry) {
    }

    // java public void beforeDataChange(ITable.Entry entry, PropertySet data)
    pub fn beforeDataChange(&mut self, entry: &dyn ITableEntry, data: &PropertySet) {
      // rust: Java `this.mStyles` 为共享引用；Rust 侧为 `Option<KStyles>`，展开后传入。
      if let Some(styles) = self.super_.mStyles.as_mut() {
        // java changeStyleRef(this.mStyles, entry.getProperty(), data, 2);
        StyleListenerBase::changeStyleRef(styles, entry.getProperty(), Some(&data), 2);
      }
    }

    // java public void afterDataChange(ITable.Entry entry, PropertySet data)
    pub fn afterDataChange(&mut self, entry: &dyn ITableEntry, data: &PropertySet) {
    }

}

impl std::ops::Deref for StyleListener4ChpxTable {
    type Target = StyleListenerBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for StyleListener4ChpxTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
