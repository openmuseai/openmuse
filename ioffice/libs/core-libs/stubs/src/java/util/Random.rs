// Java: public class Random 
#[derive(Default)]
pub struct Random {
}

// Java: public class Random 
impl Random {
    pub fn new() -> Self { Self::default() }
    // Java: public Object nextInt() { return null; 
    pub fn nextInt(&mut self) -> i32 {
        return 0;
    }

}
