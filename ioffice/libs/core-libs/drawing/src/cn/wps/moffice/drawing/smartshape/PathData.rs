#![allow(non_snake_case)]

use std::collections::HashMap;

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

// Java: public class PathData
pub struct PathData {
    // Java field: int[] _varList
    pub _varList: Vec<i32>,
    // Java field: RectF _rect
    pub _rect: Option<RectF>,
    // Java field: int[] _textBoxRectList
    pub _textBoxRectList: Vec<i32>,
    // Java field: boolean _isAdjusted
    pub _isAdjusted: bool,
    // Java field: int mShapeType
    mShapeType: i32,
}

// Java: public class PathData
impl PathData {
    // Java: PathData()
    pub fn new() -> Self {
        // Java: super();
        PathData {
            _varList: Vec::new(),
            // Java: RectF _rect = new RectF()
            _rect: Some(RectF::new()),
            _textBoxRectList: Vec::new(),
            // Java: boolean _isAdjusted = false
            _isAdjusted: false,
            mShapeType: 0,
        }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: this._rect = null;
        self._rect = None;
        // Java: this._textBoxRectList = null;
        self._textBoxRectList = Vec::new();
    }

    // Java: void reset(float w, float h)
    pub fn reset(&mut self, w: f32, h: f32) {
        // Java: if (w != this._rect.width() || h != this._rect.height())
        if let Some(ref mut rect) = self._rect {
            if w != rect.width() || h != rect.height() {
                // Java: this._rect.right = this._rect.left + w;
                rect.right = rect.left + w;
                // Java: this._rect.bottom = this._rect.top + h;
                rect.bottom = rect.top + h;
            }
        }
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> Option<&RectF> {
        // Java: return this._rect;
        self._rect.as_ref()
    }

    // Java: RectF getTextBoxRect()
    pub fn getTextBoxRect(&self, get_value: impl Fn(i32, &mut HashMap<i32, f32>, &[i32]) -> f32) -> Option<RectF> {
        // Java: if (this._textBoxRectList.length == 0)
        if self._textBoxRectList.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: assert this._textBoxRectList.length == 4;
            debug_assert!(self._textBoxRectList.len() == 4);
            // Java: Map<Integer, Float> gdValueBuf = new HashMap();
            let mut gd_value_buf: HashMap<i32, f32> = HashMap::new();
            // Java: float l = this.getValue(this._textBoxRectList[0], gdValueBuf, this._varList);
            let l = get_value(self._textBoxRectList[0], &mut gd_value_buf, &self._varList);
            // Java: float t = this.getValue(this._textBoxRectList[1], gdValueBuf, this._varList);
            let t = get_value(self._textBoxRectList[1], &mut gd_value_buf, &self._varList);
            // Java: float r = this.getValue(this._textBoxRectList[2], gdValueBuf, this._varList);
            let r = get_value(self._textBoxRectList[2], &mut gd_value_buf, &self._varList);
            // Java: float b = this.getValue(this._textBoxRectList[3], gdValueBuf, this._varList);
            let b = get_value(self._textBoxRectList[3], &mut gd_value_buf, &self._varList);
            // Java: assert r >= l && b >= t;
            debug_assert!(r >= l && b >= t);
            // Java: return new RectF(l, t, r, b);
            Some(RectF::new_with(l, t, r, b))
        }
    }

    // Java: void setAdjustValue(int i, int value)
    pub fn setAdjustValue(&mut self, i: usize, value: i32) {
        // Java: if (i < this._varList.length)
        if i < self._varList.len() {
            // Java: this._varList[i] = value;
            self._varList[i] = value;
            // Java: this._isAdjusted = true;
            self._isAdjusted = true;
        }
    }

    // Java: int getAdjustValue(int index)
    pub fn getAdjustValue(&self, index: usize) -> i32 {
        // Java: return this._varList[index];
        self._varList[index]
    }

    // Java: boolean getIsAdjusted()
    pub fn getIsAdjusted(&self) -> bool {
        // Java: return this._isAdjusted;
        self._isAdjusted
    }

    // Java: void setIsAdjusted(boolean v)
    pub fn setIsAdjusted(&mut self, v: bool) {
        // Java: this._isAdjusted = v;
        self._isAdjusted = v;
    }

    // Java: void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
        // Java: this.mShapeType = type;
        self.mShapeType = r#type;
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
        // Java: return this.mShapeType;
        self.mShapeType
    }

    // Java: abstract GraphicsPath[] calcShapePath()

    // Java: abstract float getValue(int var1, Map<Integer, Float> var2, int[] var3)

    // Java: abstract int[][] getPathList()
}
