// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.util.Date
// Java: import java.util.List

// Java: public enum PropertyType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    BOOLVAL,
    INTVAL,
    DOUBLEVAL,
    LPWSTRVAL,
    DATEVAL,
}

// Java: public enum PropertyType
impl PropertyType {
    // Java: PropertyType()
}

#[derive(Debug, Clone)]
pub enum PropertyValue {
    Str(String),
    Bool(bool),
    Int(i32),
    Double(f64),
    Date(i64),
}

// Java: public class Property
#[derive(Debug, Clone)]
pub struct Property {
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: Object mVal
    pub(crate) mVal: PropertyValue,
    // Java field: PropertyType mType
    pub(crate) mType: PropertyType,
    // Java field: String mLinkTarge
    pub mLinkTarge: Option<String>,
}

// Java: public class Property
impl Property {
    // Java: Property(String name, String val)
    pub fn new_str(name: String, val: String) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("name should not be null", name);
        // Java: IOAssert.assertNotNull("val should not be null", val);
        // Java: this.mName = name;
        // Java: this.mVal = val;
        // Java: this.mType = CustomPackageProperties.PropertyType.LPWSTRVAL;
        Property { mName: name, mVal: PropertyValue::Str(val), mType: PropertyType::LPWSTRVAL, mLinkTarge: None }
    }

    // Java: Property(String name, boolean val)
    pub fn new_bool(name: String, val: bool) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("name should not be null", name);
        // Java: this.mName = name;
        // Java: this.mVal = val;
        // Java: this.mType = CustomPackageProperties.PropertyType.BOOLVAL;
        Property { mName: name, mVal: PropertyValue::Bool(val), mType: PropertyType::BOOLVAL, mLinkTarge: None }
    }

    // Java: Property(String name, int val)
    pub fn new_int(name: String, val: i32) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("name should not be null", name);
        // Java: this.mName = name;
        // Java: this.mVal = val;
        // Java: this.mType = CustomPackageProperties.PropertyType.INTVAL;
        Property { mName: name, mVal: PropertyValue::Int(val), mType: PropertyType::INTVAL, mLinkTarge: None }
    }

    // Java: Property(String name, Date val)
    pub fn new_date(name: String, val: i64) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("name should not be null", name);
        // Java: IOAssert.assertNotNull("val should not be null", val);
        // Java: this.mName = name;
        // Java: this.mVal = val;
        // Java: this.mType = CustomPackageProperties.PropertyType.DATEVAL;
        Property { mName: name, mVal: PropertyValue::Date(val), mType: PropertyType::DATEVAL, mLinkTarge: None }
    }

    // Java: Property(String name, Double val)
    pub fn new_double(name: String, val: f64) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("name should not be null!", name);
        // Java: IOAssert.assertNotNull("val should not be null!", val);
        // Java: this.mName = name;
        // Java: this.mVal = val;
        // Java: this.mType = CustomPackageProperties.PropertyType.DOUBLEVAL;
        Property { mName: name, mVal: PropertyValue::Double(val), mType: PropertyType::DOUBLEVAL, mLinkTarge: None }
    }

    // rust: no-java —— Java 的 5 个同名构造器在 Rust 无重载；§3.9 门面对齐要求「Java public 构造器 ↔ 同名 Rust 构造器」，
    //       故本同名门面转发到 `new_str`（对应 `(String, String)` 重载），其余 4 个见
    //       `new_bool`/`new_int`/`new_date`/`new_double`。必须放在 5 个重载**之后**，
    //       否则会顶掉 layout 门禁「按同名次序配对 Java 构造器 ↔ Rust fn」的对应关系（登记 §1.7.1）
    pub fn Property(name: String, val: String) -> Self {
        Self::new_str(name, val)
    }

    // Java: PropertyType getType()
    pub fn getType(&self) -> PropertyType {
        // Java: return this.mType;
        self.mType
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.mName;
        &self.mName
    }

    // Java: Object getValue()
    pub fn getValue(&self) -> &PropertyValue {
        // Java: return this.mVal;
        &self.mVal
    }
}

// Java: public interface CustomPackageProperties
pub trait CustomPackageProperties {
    // Java: abstract void setPropertyArray(List<Property> var1)
    fn setPropertyArray(&mut self, var1: Vec<Property>);

    // Java: abstract List<Property> getPropertyArray()
    fn getPropertyArray(&self) -> &Vec<Property>;

    // Java: abstract boolean hasProperty()
    fn hasProperty(&self) -> bool;
}
