#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
use super::InkMLSerializer::{InkMLWriter, InkMLSerializer};
use super::InkElement::InkElement;
use super::ContextElement::ContextElement;

// Java field: String TAG
const TAG: &str = "Timestamp";

// Java: public class Timestamp implements ContextElement, Cloneable
pub struct Timestamp {
    // Java field: String id
    pub(crate) id: String,
}

// Java: public class Timestamp implements ContextElement, Cloneable
impl Timestamp {
    // Java: Timestamp()
    pub fn Timestamp() -> Self {
        // Java: super();
        Self { id: String::new() }
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: return this.id;
        self.id.clone()
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.id = id;
        self.id = id.to_string();
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Timestamp";
        "Timestamp".to_string()
    }

    // Java: static Timestamp getDefaultTimestamp()
    pub fn getDefaultTimestamp() -> Timestamp {
        // Java: Timestamp timestamp = new Timestamp();
        let mut timestamp = Timestamp::Timestamp();
        // Java: timestamp.setId("DefaultTimestamp");
        timestamp.setId("DefaultTimestamp");
        // Java: return timestamp;
        timestamp
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: KSLog.w(TAG, "The timestamp.toInkML method not implemented.");
        KSLog::w(TAG, "The timestamp.toInkML method not implemented.");
        // Java: return "";
        String::new()
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: KSLog.w(TAG, "The timestamp.writeXML method not implemented.");
        KSLog::w(TAG, "The timestamp.writeXML method not implemented.");
    }

    // Java: void override(Timestamp source)
    pub fn override_ts(&mut self, source: &Timestamp) {
        // Java: KSLog.w(TAG, "The timestamp.override method not implemented.");
        KSLog::w(TAG, "The timestamp.override method not implemented.");
    }

    // Java: Timestamp clone()
    pub fn clone_ts(&self) -> Timestamp {
        // Java: Timestamp ret = new Timestamp();
        let mut ret = Timestamp::Timestamp();
        // Java: if (null != this.id)
        if !self.id.is_empty() {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: return ret;
        ret
    }
}
