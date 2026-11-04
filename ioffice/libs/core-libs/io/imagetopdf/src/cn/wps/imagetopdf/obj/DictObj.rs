// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
use std::sync::{Mutex, OnceLock};

// Java field: int objCounter
fn get_obj_counter() -> &'static Mutex<i32> {
    static OBJ_COUNTER: OnceLock<Mutex<i32>> = OnceLock::new();
    OBJ_COUNTER.get_or_init(|| Mutex::new(0))
}

// Java: public class DictObj
pub struct DictObj {
    // Java field: int objNum
    pub(crate) objNum: i32,
    // Java field: String dict
    pub(crate) dict: String,
    // Java field: String endobj
    pub(crate) endobj: &'static str,
}

// Java: public class DictObj
impl DictObj {
    // Java: DictObj(String dict)
    pub fn new(dict: &str) -> Self {
        // Java: super();
        // Java: this.objNum = ++objCounter;
        let obj_num = {
            let mut counter = get_obj_counter().lock().unwrap();
            *counter += 1;
            *counter
        };
        // Java: this.setDict(dict);
        DictObj { objNum: obj_num, dict: String::from(dict), endobj: "endobj" }
    }

    // Java: void setDict(String dict)
    pub fn setDict(&mut self, dict: &str) {
        // Java: this.dict = dict;
        self.dict = String::from(dict);
    }

    // Java: byte[] getBytes()
    pub fn getBytes(&self) -> Vec<u8> {
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app(this.getStartobj());
        sb.app_str(&self.getStartobj());
        // Java: sb.app(this.dict);
        sb.app_str(&self.dict);
        // Java: sb.app("endobj");
        sb.app_str(self.endobj);
        // Java: return sb.getBytes();
        sb.getBytes()
    }

    // Java: int getObjNum()
    pub fn getObjNum(&self) -> i32 {
        // Java: return this.objNum;
        self.objNum
    }

    // Java: String getStartobj()
    pub fn getStartobj(&self) -> String {
        // Java: return this.objNum + " 0 obj";
        format!("{} 0 obj", self.objNum)
    }

    // Java: static int getObjCounter()
    pub fn getObjCounter() -> i32 {
        // Java: return objCounter;
        *get_obj_counter().lock().unwrap()
    }

    // Java: static void clearObjCounter()
    pub fn clearObjCounter() {
        // Java: objCounter = 0;
        *get_obj_counter().lock().unwrap() = 0;
    }
}
