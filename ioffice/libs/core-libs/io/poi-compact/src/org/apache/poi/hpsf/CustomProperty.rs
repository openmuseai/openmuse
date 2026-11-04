use super::MutableProperty::MutableProperty;
use super::Property::Property;

// Java: public class CustomProperty extends MutableProperty
pub struct CustomProperty {
    pub(crate) base: MutableProperty,
    // Java field: String name
    name: Option<String>,
    // Java field: String linkTarget
    linkTarget: Option<String>,
}

// Java: public class CustomProperty extends MutableProperty
impl CustomProperty {
    // Java: CustomProperty()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.name = null;
        // Java: this.linkTarget = null;
        CustomProperty {
            base: MutableProperty::new(),
            name: None,
            linkTarget: None,
        }
    }

    // Java: CustomProperty(Property property)
    pub fn new_from_property(property: &Property) -> Self {
        // Java: this(property, (String)null);
        Self::new_from_property_with_name(property, None)
    }

    // Java: CustomProperty(Property property, String name)
    pub fn new_from_property_with_name(property: &Property, name: Option<String>) -> Self {
        // Java: super(property);
        // Java: this.name = name;
        // Java: this.linkTarget = null;
        CustomProperty {
            base: MutableProperty::new_from_property(property),
            name,
            linkTarget: None,
        }
    }

    // Java: String getName()
    pub fn getName(&self) -> Option<&str> {
        // Java: return this.name;
        self.name.as_deref()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // Java: this.name = name;
        self.name = name;
    }

    // Java: String getLinkTarget()
    pub fn getLinkTarget(&self) -> Option<&str> {
        // Java: return this.linkTarget;
        self.linkTarget.as_deref()
    }

    // Java: void setLinkTarget(String linkTarget)
    pub fn setLinkTarget(&mut self, link_target: Option<String>) {
        // Java: this.linkTarget = linkTarget;
        self.linkTarget = link_target;
    }

    // Java: boolean equalsContents(Object o)
    pub fn equalsContents(&self, o: &CustomProperty) -> bool {
        // Java: CustomProperty c = (CustomProperty)o;
        // Java: String name1 = c.getName();
        let name1 = o.getName();
        // Java: String name2 = this.getName();
        let name2 = self.getName();
        // Java: boolean equalNames = true;
        let equal_names;
        // Java: if (name1 == null)
        if name1.is_none()
        {
            // Java: equalNames = name2 == null;
            equal_names = name2.is_none();
        }
        // Java: else
        else
        {
            // Java: equalNames = name1.equals(name2);
            equal_names = name1 == name2;
        }
        // Java: return equalNames && c.getID() == this.getID() && c.getType() == this.getType() && c.getValue().equals(this.getValue());
        equal_names && o.base.getID() == self.base.getID() && o.base.getType() == self.base.getType()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return (int)this.getID();
        self.base.getID() as i32
    }
}
