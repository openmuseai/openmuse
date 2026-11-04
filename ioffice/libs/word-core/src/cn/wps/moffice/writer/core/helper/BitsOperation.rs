// java public final class BitsOperation
#[derive(Default)]
pub struct BitsOperation {
}

// java public final class BitsOperation
impl BitsOperation {
    // java public static int setBits(int flags, int mask)
    pub fn setBits(flags: i32, mask: i32) -> i32 {
      // java return flags | mask;
      return flags | mask;
    }

    // java public static int resetBits(int flags, int mask)
    pub fn resetBits(flags: i32, mask: i32) -> i32 {
      // java return flags & ~mask;
      return flags & !mask;
    }

}
