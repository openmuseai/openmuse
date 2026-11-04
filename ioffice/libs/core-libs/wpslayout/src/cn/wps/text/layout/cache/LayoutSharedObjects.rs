// Java: import cn.wps.text.layout.ITextLayout
// Java: import cn.wps.text.layout.data.DataObjectPool
// Java: import cn.wps.text.layout.data.TextBoxData
// Java: import cn.wps.text.layout.pool.ObjectPool
// Java: import cn.wps.text.layout.pool.PoolableObjectFactory
use crate::cn::wps::text::layout::data::DataObjectPool::DataObjectPool;

// Java: public class LayoutSharedObjects
pub struct LayoutSharedObjects {
}

// Java: public class LayoutSharedObjects
impl LayoutSharedObjects {
    // Java: LayoutSharedObjects()
    pub fn new() -> Self {
      // Java: super();
      LayoutSharedObjects {}
    }

    // Java: static void destroy()
    pub fn destroy() {
      // Java: showTextDataPool.clear();
      // Java: DataObjectPool.destroy();
      DataObjectPool::destroy();
    }

    // Java: static void returnLayout(ITextLayout layout)
    pub fn returnLayout(_layout: &dyn std::any::Any) {
      // Java: showTextDataPool.returnObject(layout.getTextBoxData());
    }

}
