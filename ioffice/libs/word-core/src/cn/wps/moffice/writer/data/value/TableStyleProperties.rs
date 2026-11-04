// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;

// java public class TableStyleProperties
#[derive(Default)]
pub struct TableStyleProperties {
    // java private PropertySet mProperty;
    mProperty: Option<PropertySet>,
}

// java public class TableStyleProperties
impl TableStyleProperties {
    // java public  TableStyleProperties()
    pub fn TableStyleProperties() -> Self {
      // java this.mProperty = PropertySet.empty;
      Self { mProperty: Some((*empty).clone()) }
    }

    // java public PropertySet getProperty()
    pub fn getProperty(&self) -> Option<PropertySet> {
      // java return this.mProperty;
      return self.mProperty.clone();
    }

    // java public void setProperty(PropertySet p)
    pub fn setProperty(&mut self, p: Option<&PropertySet>) {
      // java this.mProperty = p;
      self.mProperty = p.cloned();
    }

}
