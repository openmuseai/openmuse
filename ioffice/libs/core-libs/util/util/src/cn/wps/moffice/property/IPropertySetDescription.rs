// Java: public interface IPropertySetDescription
pub trait IPropertySetDescription {
    // Java: abstract String getDescription(PropertySet var1)
    fn getDescription(&self, property_set: &dyn std::any::Any) -> String;
}
