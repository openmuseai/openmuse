use super::super::graphics::{Canvas, Paint, Path, Rect};
use crate::cn::wps::moss::{app::IGridSheet::IGridSheet, util::KmoColor::KmoColor};
use serde_json::json;
#[derive(Default)]
pub struct DataBarsData {
    mBarColor: i32,
    mLength: i32,
    mFillStyle: i16,
}
impl DataBarsData {
    // java public boolean reset(IGridSheet gridSheet, int rowIndex, int colIndex) {
    // java this.clear();
    // java CfDataBarValue dataBarValue = this.getDataBar(gridSheet, rowIndex, colIndex);
    // java if (dataBarValue == null) {
    // java return false;
    // java } else {
    // java int color = dataBarValue.getColor();
    // java int red = color >> 16 & 255;
    // java int green = color >> 8 & 255;
    // java int blue = color & 255;
    // java this.mBarColor = Color.rgb(red, green, blue);
    // java this.mLength = dataBarValue.getLength();
    // java return true;
    // java }
    // java }
    pub fn reset(&mut self, g: &dyn IGridSheet, row: i32, col: i32) -> bool {
        self.clear();
        let core = g.coreSheet();
        let sheet = core.borrow();
        let Some(bar) = sheet.getCFormatter().getDataBar(row, col) else {
            return false;
        };
        let color = bar.getColor();
        self.mBarColor =
            -16777216 | ((color >> 16) & 255) << 16 | ((color >> 8) & 255) << 8 | (color & 255);
        self.mLength = bar.getLength();
        true
    }
    // java public void draw(Canvas canvas, Paint paint, Rect rect) {
    // java if (!rect.isEmpty() && !KmoColor.isAutomaticColor(this.mBarColor)) {
    // java Rect dataBarRect = this.getDataBarRect(rect);
    // java if (!dataBarRect.isEmpty()) {
    // java paint.reset();
    // java if (this.mFillStyle != 1) {
    // java Shader mShader = new LinearGradient((float)dataBarRect.left, (float)dataBarRect.top, (float)dataBarRect.right, (float)dataBarRect.bottom, new int[]{this.mBarColor, -1}, (float[])null, TileMode.REPEAT);
    // java paint.setShader(mShader);
    // java } else {
    // java paint.setColor(this.mBarColor);
    // java }
    // java
    // java canvas.drawRect(dataBarRect, paint);
    // java this.setBorder(canvas, dataBarRect);
    // java }
    // java
    // java }
    // java }
    pub fn draw(&self, c: &mut Canvas, p: &mut Paint, r: &Rect) {
        if !r.isEmpty() && !KmoColor::isAutomaticColor(self.mBarColor) {
            let bar = self.getDataBarRect(r);
            if !bar.isEmpty() {
                *p = Paint::default();
                if self.mFillStyle != 1 {
                    p.shader = Some(
                        json!({"kind":"linear","points":[bar.left,bar.top,bar.right,bar.bottom],"colors":[self.mBarColor,-1],"positions":null,"tileMode":"repeat"}),
                    );
                } else {
                    p.color = self.mBarColor;
                }
                c.drawRect(&bar, p);
                self.setBorder(c, &bar);
            }
        }
    }
    // java private Rect getDataBarRect(Rect rect) {
    // java Rect dataBar = new Rect();
    // java int padding = 2;
    // java dataBar.right = rect.width() * this.mLength / 100 + rect.left - padding;
    // java dataBar.left = rect.left;
    // java dataBar.top = rect.top + padding;
    // java dataBar.bottom = rect.bottom - padding;
    // java return dataBar;
    // java }
    fn getDataBarRect(&self, r: &Rect) -> Rect {
        Rect::new(
            r.left,
            r.top + 2,
            r.width().wrapping_mul(self.mLength) / 100 + r.left - 2,
            r.bottom - 2,
        )
    }
    // java private void setBorder(Canvas canvas, Rect rect) {
    // java Paint paint = new Paint();
    // java paint.setStyle(Style.STROKE);
    // java paint.setColor(this.mBarColor);
    // java Path path = new Path();
    // java path.moveTo((float)rect.left, (float)rect.top);
    // java path.lineTo((float)rect.right, (float)rect.top);
    // java path.lineTo((float)rect.right, (float)rect.bottom);
    // java path.lineTo((float)rect.left, (float)rect.bottom);
    // java path.close();
    // java canvas.drawPath(path, paint);
    // java }
    fn setBorder(&self, c: &mut Canvas, r: &Rect) {
        let p = Paint {
            style: 1,
            color: self.mBarColor,
            ..Paint::default()
        };
        let mut path = Path::default();
        path.moveTo(r.left as f32, r.top as f32);
        path.lineTo(r.right as f32, r.top as f32);
        path.lineTo(r.right as f32, r.bottom as f32);
        path.lineTo(r.left as f32, r.bottom as f32);
        path.close();
        c.drawPath(&path, &p);
    }
    // java public final void clear() {
    // java this.mBarColor = 16777215;
    // java }
    pub fn clear(&mut self) {
        self.mBarColor = 16777215;
    }
    // java public final void copyFrom(DataBarsData source) {
    // java this.mBarColor = source.mBarColor;
    // java }
    pub fn copyFrom(&mut self, s: &Self) {
        self.mBarColor = s.mBarColor;
    }
}
