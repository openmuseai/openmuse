// Java: import java.lang.Object
use crate::java::lang::Object::*;

// java public class Thread
#[derive(Default)]
pub struct Thread {
}

// java public class Thread
impl Thread {
    // java public static void currentThread()
    pub fn currentThread() {
    }

    // java public String getName()
    pub fn getName(&self) -> String { todo!() }

    // java public Object getId()
    pub fn getId(&self) -> Box<dyn Object>  { todo!() }

}
