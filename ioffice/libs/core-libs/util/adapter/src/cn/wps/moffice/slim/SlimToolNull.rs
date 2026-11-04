use super::ISlimTool::ISlimTool;

// Java: public class SlimToolNull extends ISlimTool
pub struct SlimToolNull {
    pub(crate) base: ISlimTool,
}

// Java: public class SlimToolNull extends ISlimTool
impl SlimToolNull {
    // Java: SlimToolNull(SlimContext context, ISlimListener listener)
    pub fn new(context: Option<Box<dyn std::any::Any>>, listener: Option<Box<dyn std::any::Any>>) -> Self {
        // Java: super(context, listener);
        SlimToolNull {
            base: ISlimTool::new(context, listener),
        }
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: boolean checkSlim(Object obj)
    pub fn checkSlim_with_obj(&self, _obj: &dyn std::any::Any) -> bool {
        // Java: return false;
        false
    }

    // Java: long slim()
    pub fn slim(&self) -> i64 {
        // Java: return 0L;
        0
    }

    // Java: boolean checkSlim()
    pub fn checkSlim(&self) -> bool {
        // Java: return false;
        false
    }

}
