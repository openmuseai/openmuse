// Java: import android.graphics.PointF
// Java: import cn.wps.text.layout.typo.ArtTransform
// Java: import java.util.ArrayList
// Java: import java.util.List
use super::TextRange::TextRange;
use super::WpsPath::WpsPath;

pub struct ArtTransform;
#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }

// Java field: byte CARETPOSITIONFLAG_GLYPHHEAD
pub const CARETPOSITIONFLAG_GLYPHHEAD: u8 = 1;
// Java field: byte CARETPOSITIONFLAG_GLYPHTAIL
pub const CARETPOSITIONFLAG_GLYPHTAIL: u8 = 2;
// Java field: byte CARETPOSITIONFLAG_LINEHEAD
pub const CARETPOSITIONFLAG_LINEHEAD: u8 = 5;
// Java field: byte CARETPOSITIONFLAG_LINETAIL
pub const CARETPOSITIONFLAG_LINETAIL: u8 = 10;
// Java field: int TEXT_HIT_DEFAULT
pub const TEXT_HIT_DEFAULT: i32 = 0;
// Java field: int TEXT_HIT_EXACT
pub const TEXT_HIT_EXACT: i32 = 1;
// Java field: float LAST_SEL_POS_TAG
pub const LAST_SEL_POS_TAG: f32 = -1.0;

// Java: public class TextSelectionParam
pub struct TextSelectionParam {
    // Java field: TextRange range
    pub(crate) range: TextRange,
    // Java field: List<WpsPath> paths
    pub(crate) paths: Vec<WpsPath>,
    // Java field: List<PointF> moffsets
    pub(crate) moffsets: Vec<PointF>,
    // Java field: byte bytFlag
    pub(crate) bytFlag: u8,
    // Java field: int rotateDegrees
    pub(crate) rotateDegrees: i32,
    // Java field: PointF selectDragStart
    pub(crate) selectDragStart: Option<PointF>,
    // Java field: PointF selectDragEnd
    pub(crate) selectDragEnd: Option<PointF>,
    // Java field: boolean isDragStartLtr
    pub(crate) isDragStartLtr: bool,
    // Java field: boolean isDragEndLtr
    pub(crate) isDragEndLtr: bool,
    // Java field: float startLineHeight
    pub(crate) startLineHeight: f32,
    // Java field: float endLineHeight
    pub(crate) endLineHeight: f32,
    // Java field: boolean isSameLineForStartEndDrag
    pub(crate) isSameLineForStartEndDrag: bool,
    // Java field: ArtTransform mArtTransform
    pub(crate) mArtTransform: Option<ArtTransform>,
    // Java field: boolean isValid
    pub(crate) isValid: bool,
    // Java field: boolean isHidden
    pub(crate) isHidden: bool,
    // Java field: int mTextHitType
    pub(crate) mTextHitType: i32,
    // Java field: int vertical
    pub(crate) vertical: i32,
}

// Java: public class TextSelectionParam
impl TextSelectionParam {
    // Java: TextSelectionParam()
    pub fn new() -> Self {
        // Java: super();
        TextSelectionParam {
            range: TextRange::new(),
            paths: Vec::new(),
            moffsets: Vec::new(),
            bytFlag: 0,
            rotateDegrees: 0,
            selectDragStart: None,
            selectDragEnd: None,
            isDragStartLtr: true,
            isDragEndLtr: true,
            startLineHeight: 0.0,
            endLineHeight: 0.0,
            isSameLineForStartEndDrag: true,
            mArtTransform: None,
            isValid: false,
            isHidden: false,
            mTextHitType: 0,
            vertical: 0,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.paths.clear();
        self.paths.clear();
        // Java: this.moffsets.clear();
        self.moffsets.clear();
        // Java: this.range.putStart(0);
        self.range.putStart(0);
        // Java: this.range.putEnd(0);
        self.range.putEnd(0);
        // Java: this.bytFlag = 0;
        self.bytFlag = 0;
        // Java: this.selectDragStart = null;
        self.selectDragStart = None;
        // Java: this.selectDragEnd = null;
        self.selectDragEnd = None;
        // Java: this.isDragStartLtr = true;
        self.isDragStartLtr = true;
        // Java: this.isDragEndLtr = true;
        self.isDragEndLtr = true;
        // Java: this.startLineHeight = 0.0F;
        self.startLineHeight = 0.0;
        // Java: this.endLineHeight = 0.0F;
        self.endLineHeight = 0.0;
        // Java: this.isSameLineForStartEndDrag = true;
        self.isSameLineForStartEndDrag = true;
        // Java: this.isValid = false;
        self.isValid = false;
        // Java: this.mTextHitType = 0;
        self.mTextHitType = 0;
    }

    // Java: boolean isHidden()
    pub fn isHidden(&self) -> bool {
        // Java: return this.isHidden;
        self.isHidden
    }

    // Java: void setHidden(boolean isHidden)
    pub fn setHidden(&mut self, isHidden: bool) {
        // Java: this.isHidden = isHidden;
        self.isHidden = isHidden;
    }

    // Java: void setValid(boolean valid)
    pub fn setValid(&mut self, valid: bool) {
        // Java: this.isValid = valid;
        self.isValid = valid;
    }

    // Java: boolean isInvalid()
    pub fn isInvalid(&self) -> bool {
        // Java: return !this.isValid;
        !self.isValid
    }

    // Java: void setRange(int start, int end)
    pub fn setRange(&mut self, start: i32, end: i32) {
        // Java: this.range.putStart(start);
        self.range.putStart(start);
        // Java: this.range.putEnd(end);
        self.range.putEnd(end);
    }

    // Java: TextRange getRange()
    pub fn getRange(&self) -> &TextRange {
        // Java: return this.range;
        &self.range
    }

    // Java: void setPaths(List<WpsPath> paths)
    pub fn setPaths(&mut self, paths: Vec<WpsPath>) {
        // Java: this.paths = paths;
        self.paths = paths;
    }

    // Java: List<WpsPath> getPaths()
    pub fn getPaths(&self) -> &Vec<WpsPath> {
        // Java: return this.paths;
        &self.paths
    }

    // Java: void setMoffsets(List<PointF> moffsets)
    pub fn setMoffsets(&mut self, moffsets: Vec<PointF>) {
        // Java: this.moffsets = moffsets;
        self.moffsets = moffsets;
    }

    // Java: List<PointF> getMoffsets()
    pub fn getMoffsets(&self) -> &Vec<PointF> {
        // Java: return this.moffsets;
        &self.moffsets
    }

    // Java: void setSelectDragParam(PointF start, PointF end, int degrees, boolean isDragStartLeftToRight, boolean isDragEndLeftToRight, float startLineHeight, float endLineHeight, boolean isSameLineForStartEndDrag, int vertical)
    pub fn setSelectDragParam(&mut self, start: PointF, end: PointF, degrees: i32, isDragStartLeftToRight: bool, isDragEndLeftToRight: bool, startLineHeight: f32, endLineHeight: f32, isSameLineForStartEndDrag: bool, vertical: i32) {
        // Java: this.rotateDegrees = degrees;
        self.rotateDegrees = degrees;
        // Java: this.selectDragStart = start;
        self.selectDragStart = Some(start);
        // Java: this.selectDragEnd = end;
        self.selectDragEnd = Some(end);
        // Java: this.isDragStartLtr = isDragStartLeftToRight;
        self.isDragStartLtr = isDragStartLeftToRight;
        // Java: this.isDragEndLtr = isDragEndLeftToRight;
        self.isDragEndLtr = isDragEndLeftToRight;
        // Java: this.startLineHeight = startLineHeight;
        self.startLineHeight = startLineHeight;
        // Java: this.endLineHeight = endLineHeight;
        self.endLineHeight = endLineHeight;
        // Java: this.isSameLineForStartEndDrag = isSameLineForStartEndDrag;
        self.isSameLineForStartEndDrag = isSameLineForStartEndDrag;
        // Java: this.vertical = vertical;
        self.vertical = vertical;
    }

    // Java: int getRotateDegrees()
    pub fn getRotateDegrees(&self) -> i32 {
        // Java: return this.rotateDegrees;
        self.rotateDegrees
    }

    // Java: void setRotateDegrees(int degrees)
    pub fn setRotateDegrees(&mut self, degrees: i32) {
        // Java: this.rotateDegrees = degrees;
        self.rotateDegrees = degrees;
    }

    // Java: PointF getSelectDragStart()
    pub fn getSelectDragStart(&self) -> Option<&PointF> {
        // Java: return this.selectDragStart;
        self.selectDragStart.as_ref()
    }

    // Java: PointF getSelectDragEnd()
    pub fn getSelectDragEnd(&self) -> Option<&PointF> {
        // Java: return this.selectDragEnd;
        self.selectDragEnd.as_ref()
    }

    // Java: boolean isDragStartLeftToRight()
    pub fn isDragStartLeftToRight(&self) -> bool {
        // Java: return this.isDragStartLtr;
        self.isDragStartLtr
    }

    // Java: boolean isDragEndLeftToRight()
    pub fn isDragEndLeftToRight(&self) -> bool {
        // Java: return this.isDragEndLtr;
        self.isDragEndLtr
    }

    // Java: float getStartLineHeight()
    pub fn getStartLineHeight(&self) -> f32 {
        // Java: return this.startLineHeight;
        self.startLineHeight
    }

    // Java: void setStartLineHeight(float height)
    pub fn setStartLineHeight(&mut self, height: f32) {
        // Java: this.startLineHeight = height;
        self.startLineHeight = height;
    }

    // Java: float getEndLineHeight()
    pub fn getEndLineHeight(&self) -> f32 {
        // Java: return this.endLineHeight;
        self.endLineHeight
    }

    // Java: void setEndLineHeight(float height)
    pub fn setEndLineHeight(&mut self, height: f32) {
        // Java: this.endLineHeight = height;
        self.endLineHeight = height;
    }

    // Java: boolean getIsSameLineForStartEndDrag()
    pub fn getIsSameLineForStartEndDrag(&self) -> bool {
        // Java: return this.isSameLineForStartEndDrag;
        self.isSameLineForStartEndDrag
    }

    // Java: byte flag()
    pub fn flag(&self) -> u8 {
        // Java: return this.bytFlag;
        self.bytFlag
    }

    // Java: void setFlag(byte flag)
    pub fn setFlag(&mut self, flag: u8) {
        // Java: this.bytFlag = flag;
        self.bytFlag = flag;
    }

    // Java: boolean isAtLineEnd()
    pub fn isAtLineEnd(&self) -> bool {
        // Java: return this.bytFlag == 10;
        self.bytFlag == 10
    }

    // Java: boolean isSingleLine()
    pub fn isSingleLine(&self) -> bool {
        // Java: if (this.selectDragStart != null && this.selectDragEnd != null)
        if let (Some(start), Some(end)) = (&self.selectDragStart, &self.selectDragEnd) {
            // Java: return this.selectDragStart.y == this.selectDragEnd.y;
            return start.y == end.y;
        }
        // Java: else
        // Java: return false;
        false
    }

    // Java: void setTextHitType(int type)
    pub fn setTextHitType(&mut self, hit_type: i32) {
        // Java: this.mTextHitType = type;
        self.mTextHitType = hit_type;
    }

    // Java: int getTextHitType()
    pub fn getTextHitType(&self) -> i32 {
        // Java: return this.mTextHitType;
        self.mTextHitType
    }

    // Java: boolean isExactHit()
    pub fn isExactHit(&self) -> bool {
        // Java: return this.mTextHitType == 1;
        self.mTextHitType == 1
    }

    // Java: void setArtTransform(ArtTransform artTransform)
    pub fn setArtTransform(&mut self, artTransform: ArtTransform) {
        // Java: this.mArtTransform = artTransform;
        self.mArtTransform = Some(artTransform);
    }

    // Java: ArtTransform getArtTransform()
    pub fn getArtTransform(&self) -> Option<&ArtTransform> {
        // Java: return this.mArtTransform;
        self.mArtTransform.as_ref()
    }

    // Java: void setVertical(int vertical)
    pub fn setVertical(&mut self, vertical: i32) {
        // Java: this.vertical = vertical;
        self.vertical = vertical;
    }

    // Java: int getVertical()
    pub fn getVertical(&self) -> i32 {
        // Java: return this.vertical;
        self.vertical
    }
}
