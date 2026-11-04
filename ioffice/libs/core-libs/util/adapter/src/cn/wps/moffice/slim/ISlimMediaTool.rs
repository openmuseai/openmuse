use std::sync::{OnceLock, Mutex};

// Java field: int TYPE_UNKNOWN
pub(crate) const TYPE_UNKNOWN: i32 = 0;
// Java field: int TYPE_JPG
pub(crate) const TYPE_JPG: i32 = 1;
// Java field: int TYPE_PNG
pub(crate) const TYPE_PNG: i32 = 2;
// Java field: int TYPE_BMP
pub(crate) const TYPE_BMP: i32 = 3;
// Java field: ISlimMediaTool sInstance
static S_INSTANCE: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();

fn get_sInstance() -> &'static Mutex<Option<Box<dyn std::any::Any + Send>>> {
    S_INSTANCE.get_or_init(|| Mutex::new(None))
}

// Java: public class ISlimMediaTool
pub struct ISlimMediaTool {
}

// Java: public class ISlimMediaTool
impl ISlimMediaTool {
    // Java: ISlimMediaTool()
    pub fn new() -> Self {
      // Java: super();
      ISlimMediaTool {}
    }

    // Java: static ISlimMediaTool instance(Object owner)
    pub fn instance(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: if (null == sInstance)
      let mut inst = get_sInstance().lock().unwrap();
      if inst.is_none() {
         // Java: sInstance = ReflectionHelper.getSlimMediaTool(owner);
         todo!();
      }
      // Java: return sInstance;
      todo!()
    }

    // Java: abstract String slimPicture(String var1, int var2)
    // Java: abstract String slimVedio(String var1, int var2)
    // Java: abstract String slimAudio(String var1, int var2)
}

pub trait ISlimMediaToolTrait {
    // Java: abstract String slimPicture(String var1, int var2)
    fn slimPicture(&self, _var1: &str, _var2: i32) -> String;

    // Java: abstract String slimVedio(String var1, int var2)
    fn slimVedio(&self, _var1: &str, _var2: i32) -> String;

    // Java: abstract String slimAudio(String var1, int var2)
    fn slimAudio(&self, _var1: &str, _var2: i32) -> String;
}
