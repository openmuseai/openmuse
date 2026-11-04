// Java: import cn.wps.moffice.property.MutablePropertyMap
use util::moffice::property::MutablePropertyMap::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class SummaryBase
#[derive(Default, Clone)]
pub struct SummaryBase {
    // java protected MutablePropertyMap mProperty = new MutablePropertyMap();
    pub(crate) mProperty: Option<MutablePropertyMap>,
}

// java public class SummaryBase
impl SummaryBase {
    // java public boolean hasData()
    pub fn hasData(&self) -> bool {
      // java return !this.mProperty.isEmpty();
      return !(self.mProperty.as_ref().expect("mProperty")).isEmpty();
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.mProperty != null)
      // rust: Java `x.dispose(); x = null;` ⇒ 先 `take()` 取出再 dispose。
      if let Some(mut mProperty) = self.mProperty.take() {
        // java this.mProperty.dispose();
        mProperty.dispose();
      }
    }

    // java public Integer getCodePage()
    pub fn getCodePage(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(1);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(1).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setCodePage(int codePage)
    pub fn setCodePage(&mut self, codePage: i32) {
      // java this.mProperty.setInt(1, codePage);
      (self.mProperty.as_mut().expect("mProperty")).setInt(1, codePage);
    }

    // java public Object getValue(int nProp)
    pub fn getValue(&self, nProp: i32) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
      // java return this.mProperty.getValue(nProp);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(nProp);
    }

}
