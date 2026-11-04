// Java: import cn.wps.moffice.kfs.File
use std::path::MAIN_SEPARATOR;

// Java: public class POIFSDocumentPath
pub struct POIFSDocumentPath {
    // Java field: String[] components
    components: Option<Vec<String>>,
    // Java field: int hashcode
    hashcode: i32,
}

// Java: public class POIFSDocumentPath
impl POIFSDocumentPath {
    // Java: POIFSDocumentPath(String[] components)
    pub fn new_with_components(components: Option<&[String]>) -> Self {
        // Java: super();
        // Java: if (components == null)
        if components.is_none() {
            // Java: this.components = new String[0];
            return POIFSDocumentPath {
                components: Some(Vec::new()),
                hashcode: 0,
            };
        }
        // Java: else
        else {
            let comps = components.unwrap();
            // Java: this.components = new String[components.length];
            let mut result = Vec::with_capacity(comps.len());
            // Java: for(int j = 0; j < components.length; ++j)
            for j in 0..comps.len() {
                // Java: if (components[j] == null || components[j].length() == 0)
                if comps[j].is_empty() {
                    // Java: throw new IllegalArgumentException("components cannot contain null or empty strings");
                    panic!("components cannot contain null or empty strings");
                }
                // Java: this.components[j] = components[j];
                result.push(comps[j].clone());
            }
            POIFSDocumentPath {
                components: Some(result),
                hashcode: 0,
            }
        }
    }

    // Java: POIFSDocumentPath()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.components = new String[0];
        POIFSDocumentPath {
            components: Some(Vec::new()),
            hashcode: 0,
        }
    }

    // Java: POIFSDocumentPath(POIFSDocumentPath path, String[] components)
    pub fn new_with_path(path: &POIFSDocumentPath, components: Option<&[String]>) -> Self {
        // Java: super();
        let path_comps = path.components.as_ref().unwrap();
        let mut result;
        // Java: if (components == null)
        if components.is_none() {
            // Java: this.components = new String[path.components.length];
            result = path_comps.clone();
        }
        // Java: else
        else {
            let comps = components.unwrap();
            // Java: this.components = new String[path.components.length + components.length];
            result = Vec::with_capacity(path_comps.len() + comps.len());
            // Java: for(int j = 0; j < path.components.length; ++j)
            for j in 0..path_comps.len() {
                // Java: this.components[j] = path.components[j];
                result.push(path_comps[j].clone());
            }
            // Java: if (components != null)
            // Java: for(int j = 0; j < components.length; ++j)
            for j in 0..comps.len() {
                // Java: if (components[j] == null || components[j].length() == 0)
                if comps[j].is_empty() {
                    // Java: throw new IllegalArgumentException("components cannot contain null or empty strings");
                    panic!("components cannot contain null or empty strings");
                }
                // Java: this.components[j + path.components.length] = components[j];
                result.push(comps[j].clone());
            }
        }
        POIFSDocumentPath {
            components: Some(result),
            hashcode: 0,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.components = null;
        self.components = None;
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&POIFSDocumentPath>) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: if (o != null && o.getClass() == this.getClass())
        if let Some(path) = o {
            // Java: if (this == o)
            if std::ptr::eq(self, path) {
                // Java: rval = true;
                rval = true;
            }
            // Java: else
            else {
                // Java: POIFSDocumentPath path = (POIFSDocumentPath)o;
                let self_comps = self.components.as_ref().unwrap();
                let path_comps = path.components.as_ref().unwrap();
                // Java: if (path.components.length == this.components.length)
                if path_comps.len() == self_comps.len() {
                    // Java: rval = true;
                    rval = true;
                    // Java: for(int j = 0; j < this.components.length; ++j)
                    for j in 0..self_comps.len() {
                        // Java: if (!path.components[j].equals(this.components[j]))
                        if path_comps[j] != self_comps[j] {
                            // Java: rval = false;
                            rval = false;
                            // Java: break;
                            break;
                        }
                    }
                }
            }
        }
        // Java: return rval;
        rval
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
        // Java: if (this.hashcode == 0)
        if self.hashcode == 0 {
            let comps = self.components.as_ref().unwrap();
            // Java: for(int j = 0; j < this.components.length; ++j)
            for j in 0..comps.len() {
                // Java: this.hashcode += this.components[j].hashCode();
                let mut h: i32 = 0;
                for ch in comps[j].chars() {
                    h = h.wrapping_mul(31).wrapping_add(ch as i32);
                }
                self.hashcode = self.hashcode.wrapping_add(h);
            }
        }
        // Java: return this.hashcode;
        self.hashcode
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return this.components.length;
        self.components.as_ref().unwrap().len() as i32
    }

    // Java: String getComponent(int n)
    pub fn getComponent(&self, n: i32) -> &str {
        // Java: return this.components[n];
        &self.components.as_ref().unwrap()[n as usize]
    }

    // Java: POIFSDocumentPath getParent()
    pub fn getParent(&self) -> Option<POIFSDocumentPath> {
        // Java: int length = this.components.length - 1;
        let comps = self.components.as_ref().unwrap();
        let length = comps.len() as i32 - 1;
        // Java: if (length < 0)
        if length < 0 {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: POIFSDocumentPath parent = new POIFSDocumentPath((String[])null);
            let mut parent = POIFSDocumentPath::new_with_components(None);
            // Java: parent.components = new String[length];
            // Java: System.arraycopy(this.components, 0, parent.components, 0, length);
            parent.components = Some(comps[..length as usize].to_vec());
            // Java: return parent;
            return Some(parent);
        }
    }

    // Java: String toString()
    pub fn to_string_repr(&self) -> String {
        // Java: StringBuffer b = new StringBuffer();
        let mut b = String::new();
        // Java: int l = this.length();
        let l = self.length();
        // Java: b.append(File.separatorChar);
        b.push(MAIN_SEPARATOR);
        // Java: for(int i = 0; i < l; ++i)
        for i in 0..l {
            // Java: b.append(this.getComponent(i));
            b.push_str(self.getComponent(i));
            // Java: if (i < l - 1)
            if i < l - 1 {
                // Java: b.append(File.separatorChar);
                b.push(MAIN_SEPARATOR);
            }
        }
        // Java: return b.toString();
        b
    }
}
