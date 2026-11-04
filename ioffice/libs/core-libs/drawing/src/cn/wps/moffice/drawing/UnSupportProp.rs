#![allow(non_snake_case)]

// Java: import java.util.Vector

pub trait Cloneable {}

// Java: public class UnSupportProp implements Cloneable
pub struct UnSupportProp {
    // Java field: Vector<Prop> props
    pub props: Vec<Prop>,
}

impl Cloneable for UnSupportProp {}

// Java: public class Prop
#[derive(Clone)]
pub struct Prop {
    // Java field: int propID
    pub propID: i32,
    // Java field: int len
    pub len: i32,
    // Java field: char[] buf
    pub buf: Vec<u16>,
}

// Java: public class UnSupportProp implements Cloneable
impl UnSupportProp {
    // Java: UnSupportProp()
    pub fn new() -> Self {
        // Java: super();
        UnSupportProp {
            props: Vec::new(),
        }
    }

    // Java: UnSupportProp clone()
    pub fn clone(&self) -> Result<UnSupportProp, String> {
        // Java: UnSupportProp unsupportedProp = (UnSupportProp)super.clone();
        let mut unsupportedProp = UnSupportProp { props: self.props.clone() };
        // Java: unsupportedProp.props = null != this.props ? (Vector)this.props.clone() : null;
        unsupportedProp.props = self.props.clone();
        // Java: return unsupportedProp;
        Ok(unsupportedProp)
    }

    // Java: void addProp(Prop prop)
    pub fn addProp(&mut self, prop: Prop) {
        // Java: int size = this.size();
        let size = self.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: Prop propExist = this.getProp(i);
            let propExist = self.getProp(i);
            // Java: assert propExist != null;
            assert!(propExist.is_some());
            let propExist_id = propExist.unwrap().propID;
            // Java: if (propExist.propID == prop.propID)
            if propExist_id == prop.propID {
                // Java: propExist.len = prop.len;
                self.props[i as usize].len = prop.len;
                // Java: propExist.buf = prop.buf;
                self.props[i as usize].buf = prop.buf;
                // Java: return;
                return;
            }
        }
        // Java: this.props.add(prop);
        self.props.push(prop);
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.props.size();
        self.props.len() as i32
    }

    // Java: Prop getProp(int n)
    pub fn getProp(&self, n: i32) -> Option<&Prop> {
        // Java: return n >= 0 && n < this.size() ? (Prop)this.props.elementAt(n) : null;
        if n >= 0 && n < self.size() {
            Some(&self.props[n as usize])
        } else {
            None
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.props.clear();
        self.props.clear();
    }
}

// Java: public class Prop
impl Prop {
    // Java: Prop(int id, int len, char[] buf)
    pub fn new(id: i32, len: i32, buf: Vec<u16>) -> Self {
        let mut instance = Self { propID: 0, len: 0, buf: Vec::new() };
        instance._init(id, len, buf);
        instance
    }

    fn _init(&mut self, id: i32, len: i32, buf: Vec<u16>) {
        // Java: super();
        *self = Self { propID: 0, len: 0, buf: Vec::new() };
        // Java: this.propID = id;
        self.propID = id;
        // Java: this.len = len;
        self.len = len;
        // Java: this.buf = buf;
        self.buf = buf;
    }
}
