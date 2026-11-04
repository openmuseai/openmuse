use std::collections::HashMap;

// Java: public class Attributes
pub struct Attributes {
    // Java field: HtmlTextWriterAttribute mAttribute
    pub mAttribute: Option<String>,
    // Java field: StringBuilder mPendingAttributeValue
    pub mPendingAttributeValue: String,
    // Java field: HashMap<HtmlTextWriterAttribute, String> mMap
    pub mMap: HashMap<String, String>,
    // Java field: VmlCssStyle mCssStyle
    pub mCssStyle: String,
}

// Java: public class Attributes
impl Attributes {
    // Java: Attributes()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.init();
        let mut a = Attributes {
            mAttribute: None,
            mPendingAttributeValue: String::new(),
            mMap: HashMap::new(),
            mCssStyle: String::new(),
        };
        a.init();
        a
    }

    // Java: void init()
    fn init(&mut self) {
        // Java: this.mMap = new HashMap();
        self.mMap = HashMap::new();
        // Java: this.mPendingAttributeValue = new StringBuilder();
        self.mPendingAttributeValue = String::new();
        // Java: this.mCssStyle = VmlCssStyle.obtain();
        self.mCssStyle = String::new();
    }

    // Java: static Attributes obtain()
    pub fn obtain() -> Self {
        // Java: return (Attributes)AttributesPool.sInstance.obtain();
        Attributes::new()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: this.mMap.clear();
        self.mMap.clear();
        // Java: this.mAttribute = null;
        self.mAttribute = None;
        // Java: this.mCssStyle.recycle();
        // Java: this.mCssStyle = VmlCssStyle.obtain();
        self.mCssStyle = String::new();
        // Java: this.mPendingAttributeValue.delete(0, this.mPendingAttributeValue.length());
        self.mPendingAttributeValue.clear();
        // Java: AttributesPool.sInstance.give(this);
    }

    // Java: void appendAttributeName(String prefix, String attrName)
    pub fn appendAttributeName(&mut self, prefix: &str, attr_name: &str) {
        // Java: IOAssert.assertNotNull("prefix should not be null！", prefix);
        // Java: IOAssert.assertNotNull("attrName should not be null！", attrName);
        // Java: this.mAttribute = this.findAttributeId(prefix, attrName);
        self.mAttribute = Some(Self::find_attribute_id(prefix, attr_name));
    }

    // Java: HtmlTextWriterAttribute findAttributeId(String prefix, String attrName)
    fn find_attribute_id(prefix: &str, attr_name: &str) -> String {
        // Java: IOAssert.assertNotNull("prefix should not be null!", prefix);
        // Java: IOAssert.assertNotNull("attrName should not be null!", attrName);
        // Java: StringBuilder sb = new StringBuilder(prefix);
        let mut sb = String::from(prefix);
        // Java: if (0 != sb.length())
        if !sb.is_empty() {
            // Java: sb.append(':');
            sb.push(':');
        }
        // Java: sb.append(attrName);
        sb.push_str(attr_name);
        // Java: return HtmlTextWriterAttribute.getHtmlTextWriterAttribute(sb.toString());
        sb
    }

    // Java: void emitAttribute()
    pub fn emitAttribute(&mut self) {
        // Java: if (null != this.mAttribute)
        if let Some(attr) = self.mAttribute.take() {
            // Java: IOAssert.assertNotNull("mMap should not be null！", this.mMap);
            // Java: this.mMap.put(this.mAttribute, this.mPendingAttributeValue.toString());
            self.mMap.insert(attr, self.mPendingAttributeValue.clone());
            // Java: this.mAttribute = null;
            // Java: this.mPendingAttributeValue.delete(0, this.mPendingAttributeValue.length());
            self.mPendingAttributeValue.clear();
        }
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: IOAssert.assertNotNull("mMap should not be null!", this.mMap);
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: for(HtmlTextWriterAttribute key : this.mMap.keySet())
        for (key, value) in &self.mMap {
            // Java: if (HtmlTextWriterAttribute.Style != key)
            if key != "style" {
                // Java: sb.append(key.toString());
                sb.push_str(key);
                // Java: sb.append("='");
                sb.push_str("='");
                // Java: sb.append((String)this.mMap.get(key));
                sb.push_str(value);
                // Java: sb.append("' ");
                sb.push_str("' ");
            }
        }
        // Java: String sb2 = this.mCssStyle.toString();
        let sb2 = &self.mCssStyle;
        // Java: if (0 != sb2.length())
        if !sb2.is_empty() {
            // Java: sb.append("style= ");
            sb.push_str("style= ");
        }
        // Java: sb.append(sb2);
        sb.push_str(sb2);
        // Java: return sb.toString();
        sb
    }
}
