// java public static final int MODE_PAGE = 0;
pub const MODE_PAGE: i32 = 0;
// java public static final int MODE_WEB = 1;
pub const MODE_WEB: i32 = 1;
// java public static final int MODE_PHONE = 2;
pub const MODE_PHONE: i32 = 2;
// java public static final int MODE_PHONE_SCROLL = 3;
pub(crate) const MODE_PHONE_SCROLL: i32 = 3;
// java public static final int MODE_SHARE = 4;
pub(crate) const MODE_SHARE: i32 = 4;
// java public static final int _MODE_COUNT = 5;
pub(crate) const _MODE_COUNT: i32 = 5;
// java public static final int MODE_DEF = 0;
pub(crate) const MODE_DEF: i32 = 0;
// java public static final int MODE_PHONE_DEF = 3;
pub(crate) const MODE_PHONE_DEF: i32 = 3;
// java public static final int MODE_SEARCH = 11;
pub(crate) const MODE_SEARCH: i32 = 11;

// java public class ViewMode
#[derive(Default)]
pub struct ViewMode {
}

// java public class ViewMode
impl ViewMode {
    // java public static boolean isPageMode(int mode)
    pub fn isPageMode(mode: i32) -> bool {
      // java return mode == 0;
      return mode == 0;
    }

    // java public static boolean isWebMode(int mode)
    pub fn isWebMode(mode: i32) -> bool {
      // java return mode == 1;
      return mode == 1;
    }

    // java public static boolean isPhoneMode(int mode)
    pub fn isPhoneMode(mode: i32) -> bool {
      // java return mode == 2;
      return mode == 2;
    }

    // java public static boolean isPhoneScrollMode(int mode)
    pub fn isPhoneScrollMode(mode: i32) -> bool {
      // java return mode == 3;
      return mode == 3;
    }

    // java public static boolean isPhoneReadMode(int mode)
    pub fn isPhoneReadMode(mode: i32) -> bool {
      // java return mode == 2 || mode == 3;
      return mode == 2 || mode == 3;
    }

    // java public static boolean isWebLayoutMode(int mode)
    pub fn isWebLayoutMode(mode: i32) -> bool {
      // java return mode == 1 || mode == 3;
      return mode == 1 || mode == 3;
    }

    // java public static boolean isWebStyleMode(int mode)
    pub fn isWebStyleMode(mode: i32) -> bool {
      // java return mode == 1 || mode == 2 || mode == 3;
      return mode == 1 || mode == 2 || mode == 3;
    }

    // java public static boolean shouldShowCommentMarkUp(int layoutMode)
    pub fn shouldShowCommentMarkUp(layoutMode: i32) -> bool {
      // java return true;
      return true;
    }

    // java public static boolean shouldShowRevisionMarkUp(int mode)
    pub fn shouldShowRevisionMarkUp(mode: i32) -> bool {
      // java return mode == 0 || mode == 1;
      return mode == 0 || mode == 1;
    }

    // java public static boolean shouldIgnoreBackgroundAndSHD(int mode)
    pub fn shouldIgnoreBackgroundAndSHD(mode: i32) -> bool {
      // java return mode == 4;
      return mode == 4;
    }

    // java public static boolean shouldIgnoreBackground(int mode)
    pub fn shouldIgnoreBackground(mode: i32) -> bool {
      // java return mode == 2 || mode == 3 || mode == 4 || mode == 1;
      return mode == 2 || mode == 3 || mode == 4 || mode == 1;
    }

    // java public static boolean isDrawingAnchorValid(int mode)
    pub fn isDrawingAnchorValid(mode: i32) -> bool {
      // java return mode == 2 || mode == 3 || mode == 4;
      return mode == 2 || mode == 3 || mode == 4;
    }

    // java public static boolean isPageFlip(int mode)
    pub fn isPageFlip(mode: i32) -> bool {
      // java return mode == 2;
      return mode == 2;
    }

}
