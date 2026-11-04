// Java field: SlimToolNull sNULL
// Java field: int SLIM_NULL
pub const SLIM_NULL: i32 = 0;
// Java field: int WPS_START
pub const WPS_START: i32 = 1;
// Java field: int SLIM_WPS_STYLE
pub const SLIM_WPS_STYLE: i32 = 2;
// Java field: int SLIM_WPS_PICTURE
pub const SLIM_WPS_PICTURE: i32 = 3;
// Java field: int SLIM_WPS_VEDIO
pub const SLIM_WPS_VEDIO: i32 = 4;
// Java field: int SLIM_WPS_AUDIO
pub const SLIM_WPS_AUDIO: i32 = 5;
// Java field: int WPS_END
pub const WPS_END: i32 = 31;
// Java field: int ET_START
pub const ET_START: i32 = 32;
// Java field: int SLIM_ET_EMPTY_SHEET
pub const SLIM_ET_EMPTY_SHEET: i32 = 33;
// Java field: int SLIM_ET_INVISIBLE_DRAWING
pub const SLIM_ET_INVISIBLE_DRAWING: i32 = 34;
// Java field: int SLIM_ET_EMPTY_CELL
pub const SLIM_ET_EMPTY_CELL: i32 = 35;
// Java field: int SLIM_ET_CHART_STYLE
pub const SLIM_ET_CHART_STYLE: i32 = 36;
// Java field: int SLIM_ET_CELL_STYLE
pub const SLIM_ET_CELL_STYLE: i32 = 37;
// Java field: int SLIM_ET_PICTURE
pub const SLIM_ET_PICTURE: i32 = 38;
// Java field: int SLIM_ET_VEDIO
pub const SLIM_ET_VEDIO: i32 = 39;
// Java field: int SLIM_ET_AUDIO
pub const SLIM_ET_AUDIO: i32 = 40;
// Java field: int ET_END
pub const ET_END: i32 = 62;
// Java field: int WPP_START
pub const WPP_START: i32 = 63;
// Java field: int SLIM_WPP_MASTER
pub const SLIM_WPP_MASTER: i32 = 64;
// Java field: int SLIM_WPP_INVISIBLE_ANIMATION
pub const SLIM_WPP_INVISIBLE_ANIMATION: i32 = 65;
// Java field: int SLIM_WPP_CROP_BEYOND_SLIDE_PICTURE
pub const SLIM_WPP_CROP_BEYOND_SLIDE_PICTURE: i32 = 66;
// Java field: int SLIM_WPP_DUPLICATE_PICTURE
pub const SLIM_WPP_DUPLICATE_PICTURE: i32 = 67;
// Java field: int SLIM_WPP_STACK_PICTURE_MERGE
pub const SLIM_WPP_STACK_PICTURE_MERGE: i32 = 68;
// Java field: int SLIM_WPP_GROUP_DRAWING_TO_PICTURE
pub const SLIM_WPP_GROUP_DRAWING_TO_PICTURE: i32 = 69;
// Java field: int SLIM_WPP_PICTURE
pub const SLIM_WPP_PICTURE: i32 = 70;
// Java field: int SLIM_WPP_VEDIO
pub const SLIM_WPP_VEDIO: i32 = 71;
// Java field: int SLIM_WPP_AUDIO
pub const SLIM_WPP_AUDIO: i32 = 72;
// Java field: int WPP_END
pub const WPP_END: i32 = 93;
// Java field: int PDF_START
pub const PDF_START: i32 = 94;
// Java field: int SLIM_PDF_PICTURE
pub const SLIM_PDF_PICTURE: i32 = 95;
// Java field: int SLIM_PDF_OTHER
pub const SLIM_PDF_OTHER: i32 = 96;
// Java field: int SLIM_PDF_ALL
pub const SLIM_PDF_ALL: i32 = 97;
// Java field: int PDF_END
pub const PDF_END: i32 = 124;

// Java: public class ISlimTool
pub struct ISlimTool {
    // Java field: ISlimListener mListener
    pub(crate) mListener: Option<Box<dyn std::any::Any>>,
    // Java field: SlimContext mSlimContext
    pub(crate) mSlimContext: Option<Box<dyn std::any::Any>>,
    // Java field: Boolean mCheckSlim
    pub(crate) mCheckSlim: Option<bool>,
    // Java field: Long mSlimSize
    pub(crate) mSlimSize: Option<i64>,
}

// Java: public class ISlimTool
impl ISlimTool {
    // Java: ISlimTool(SlimContext context, ISlimListener listener)
    pub fn new(context: Option<Box<dyn std::any::Any>>, listener: Option<Box<dyn std::any::Any>>) -> Self {
        // Java: super();
        // Java: this.mListener = listener;
        // Java: this.mSlimContext = context;
        ISlimTool {
            mListener: listener,
            mSlimContext: context,
            mCheckSlim: None,
            mSlimSize: None,
        }
    }

    // Java: abstract int type()

    // Java: long slim()
    pub fn slim(&self) -> i64 {
        // Java: return 0L;
        0
    }

    // Java: abstract boolean checkSlim()

    // Java: void cancelSlim()
    pub fn cancelSlim(&self) {
    }

    // Java: boolean checkSlim(Object obj)
    pub fn checkSlim_with_obj(&self, _obj: &dyn std::any::Any) -> bool {
        // Java: return this.mSlimContext.isFindSlimItem();
        todo!()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: synchronized(this)
        {
            // Java: this.mCheckSlim = null;
            self.mCheckSlim = None;
            // Java: this.mSlimSize = null;
            self.mSlimSize = None;
        }
    }

}
