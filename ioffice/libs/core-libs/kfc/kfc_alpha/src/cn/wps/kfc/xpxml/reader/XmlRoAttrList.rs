use super::XmlRoAttrLeaf::XmlRoAttrLeaf;

// Java: public class XmlRoAttrList implements XmlRoAttr
pub struct XmlRoAttrList {
    // Java field: int mName
    pub(crate) mName: i32,
    // Java field: XmlRoAttrLeaf[] mLeafs
    pub(crate) mLeafs: Vec<XmlRoAttrLeaf>,
    // Java field: int mTop
    pub(crate) mTop: usize,
}

// Java: public class XmlRoAttrList implements XmlRoAttr
impl XmlRoAttrList {
    // Java: XmlRoAttrList()
    pub fn new() -> Self {
        // Java: super();
        let mut leafs = Vec::with_capacity(16);
        // Java: for(int i = 0; i < this.mLeafs.length; ++i)
        for _i in 0..16 {
            // Java: this.mLeafs[i] = new XmlRoAttrLeaf();
            leafs.push(XmlRoAttrLeaf::new());
        }
        // Java: this.mTop = 0;
        XmlRoAttrList {
            mName: 0,
            mLeafs: leafs,
            mTop: 0,
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.mTop = 0;
        self.mTop = 0;
    }

    // Java: void setName(int name)
    pub fn set_name(&mut self, name: i32) {
        // Java: this.mName = name;
        self.mName = name;
    }

    // Java: void addAttr(int attr, String value)
    pub fn add_attr(&mut self, attr: i32, value: &str) {
        // Java: if (this.mTop >= this.mLeafs.length)
        if self.mTop >= self.mLeafs.len() {
            // Java: XmlRoAttrLeaf[] leafs = new XmlRoAttrLeaf[this.mLeafs.length * 2];
            let new_len = self.mLeafs.len() * 2;
            // Java: for(int i = 0; i < this.mLeafs.length; ++i)
            // Java: leafs[i] = this.mLeafs[i];
            // Java: for(int i = this.mLeafs.length; i < leafs.length; ++i)
            while self.mLeafs.len() < new_len {
                // Java: leafs[i] = new XmlRoAttrLeaf();
                self.mLeafs.push(XmlRoAttrLeaf::new());
            }
            // Java: this.mLeafs = leafs;
        }
        // Java: this.mLeafs[this.mTop++].reset(attr, value);
        self.mLeafs[self.mTop].reset(attr, value);
        self.mTop += 1;
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: StringBuilder builder = new StringBuilder();
        let mut builder = String::new();
        // Java: builder.append("[class XmlRoAttrList]\n\n");
        builder.push_str("[class XmlRoAttrList]\n\n");
        // Java: for(int i = 0; i < this.mTop; ++i)
        for i in 0..self.mTop {
            // Java: XmlRoAttrLeaf leaf = this.mLeafs[i];
            let leaf = &self.mLeafs[i];
            // Java: builder.append(leaf.getName());
            builder.push_str(&leaf.get_name().to_string());
            // Java: builder.append("=");
            builder.push('=');
            // Java: builder.append(leaf.toString());
            builder.push_str(&leaf.to_string());
            // Java: builder.append("\n");
            builder.push('\n');
        }
        // Java: return builder.toString();
        builder
    }

    // Java: int toInt()
    pub fn to_int(&self) -> i32 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: byte toByte()
    pub fn to_byte(&self) -> i8 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: short toShort()
    pub fn to_short(&self) -> i16 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: boolean toBoolean()
    pub fn to_boolean(&self) -> bool {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: float toFloat()
    pub fn to_float(&self) -> f32 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: double toDouble()
    pub fn to_double(&self) -> f64 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: long toLong()
    pub fn to_long(&self) -> i64 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: double toPercentage()
    pub fn to_percentage(&self) -> f64 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: XmlRoAttr getByName(int name)
    pub fn get_by_name(&self, name: i32) -> Option<&XmlRoAttrLeaf> {
        // Java: for(int i = 0; i < this.mTop; ++i)
        for i in 0..self.mTop {
            // Java: XmlRoAttrLeaf leaf = this.mLeafs[i];
            let leaf = &self.mLeafs[i];
            // Java: if (leaf.getName() == name)
            if leaf.get_name() == name {
                // Java: return leaf;
                return Some(leaf);
            }
        }
        // Java: return null;
        None
    }

    // Java: XmlRoAttr getAttrAt(int index)
    pub fn get_attr_at(&self, index: usize) -> &XmlRoAttrLeaf {
        // Java: return this.mLeafs[index];
        &self.mLeafs[index]
    }

    // Java: int getNameAt(int index)
    pub fn get_name_at(&self, index: usize) -> i32 {
        // Java: return this.mLeafs[index].getName();
        self.mLeafs[index].get_name()
    }

    // Java: int count()
    pub fn count(&self) -> i32 {
        // Java: return this.mTop;
        self.mTop as i32
    }

    // Java: String getCharacters()
    pub fn get_characters(&self) -> String {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: int getName()
    pub fn get_name(&self) -> i32 {
        // Java: return this.mName;
        self.mName
    }

    // Java: XmlRoAttr getByName(int name, boolean bLeaf)
    pub fn get_by_name_with_flag(&self, name: i32, b_leaf: bool) -> Option<&XmlRoAttrLeaf> {
        // Java: if (!bLeaf)
        if !b_leaf {
            // Java: return null;
            return None;
        }
        // Java: else
        // Java: for(int i = 0; i < this.mTop; ++i)
        for i in 0..self.mTop {
            // Java: XmlRoAttrLeaf leaf = this.mLeafs[i];
            let leaf = &self.mLeafs[i];
            // Java: if (leaf.getName() == name)
            if leaf.get_name() == name {
                // Java: return leaf;
                return Some(leaf);
            }
        }
        // Java: return null;
        None
    }

    // Java: boolean isLeaf()
    pub fn is_leaf(&self) -> bool {
        // Java: return false;
        false
    }
}

impl super::XmlRoAttrValue::XmlRoAttrValue for XmlRoAttrList {
    fn to_string(&self) -> String { XmlRoAttrList::to_string(self) }
    fn to_int(&self) -> i32 { XmlRoAttrList::to_int(self) }
    fn to_byte(&self) -> i8 { XmlRoAttrList::to_byte(self) }
    fn to_short(&self) -> i16 { XmlRoAttrList::to_short(self) }
    fn to_boolean(&self) -> bool { XmlRoAttrList::to_boolean(self) }
    fn to_float(&self) -> f32 { XmlRoAttrList::to_float(self) }
    fn to_double(&self) -> f64 { XmlRoAttrList::to_double(self) }
    fn to_long(&self) -> i64 { XmlRoAttrList::to_long(self) }
    fn to_percentage(&self) -> f64 { XmlRoAttrList::to_percentage(self) }
}

impl super::XmlRoAttr::XmlRoAttr for XmlRoAttrList {
    fn get_by_name(&self, name: i32) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        XmlRoAttrList::get_by_name(self, name).map(|leaf| leaf as &dyn super::XmlRoAttr::XmlRoAttr)
    }
    fn get_attr_at(&self, index: i32) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        Some(XmlRoAttrList::get_attr_at(self, index as usize))
    }
    fn get_name_at(&self, index: i32) -> i32 { XmlRoAttrList::get_name_at(self, index as usize) }
    fn count(&self) -> i32 { XmlRoAttrList::count(self) }
    fn get_characters(&self) -> String { XmlRoAttrList::get_characters(self) }
    fn get_name(&self) -> i32 { XmlRoAttrList::get_name(self) }
    fn get_by_name_with_flag(&self, name: i32, flag: bool) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        XmlRoAttrList::get_by_name_with_flag(self, name, flag).map(|leaf| leaf as &dyn super::XmlRoAttr::XmlRoAttr)
    }
    fn is_leaf(&self) -> bool { XmlRoAttrList::is_leaf(self) }
}
