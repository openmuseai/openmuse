// Java: public class System 
#[derive(Default)]
pub struct System {
}

// Java: public class System 
impl System {
    // Java: public static void currentTimeMillis() 
    // rust: ★M-STAGE2 P1-D1 / T7-3 —— **破损桩补全**。
    //   Java `System.arraycopy(Object src, int srcPos, Object dest, int destPos, int length)`
    //   对基本类型数组有原生重载；Rust 侧以泛型 `Clone` 忠实覆盖（`int[]` 等）。
    pub fn arraycopy<T: Clone>(src: &[T], srcPos: usize, dest: &mut [T], destPos: usize, length: usize) {
        if length == 0 { return; }
        dest[destPos..destPos + length].clone_from_slice(&src[srcPos..srcPos + length]);
    }
    pub fn currentTimeMillis() -> i64 {
        return 0;
    }

}
