use super::super::graphics::{Canvas, Paint, Path, Rect};
use crate::cn::wps::moffice::spreadsheet::utils::RenderDrawablePool::{
    NativeBitmap, RenderDrawablePool,
};
use crate::cn::wps::moss::app::condfmt::rule::KmoIconSet::IconSetTypes;
use crate::cn::wps::moss::{app::IGridSheet::IGridSheet, util::KmoColor::KmoColor};
use serde_json::json;
use std::sync::atomic::{AtomicI32, Ordering};
pub static MAX_FILTER_ICON_WIDTH: AtomicI32 = AtomicI32::new(0);
pub static MIN_FILTER_ICON_WIDTH: AtomicI32 = AtomicI32::new(0);
#[derive(Default)]
pub struct IconSetData {
    mIconBitmap: Option<NativeBitmap>,
}
impl IconSetData {
    // java static { MAX_FILTER_ICON_WIDTH=(int)(30.0F*DrawUtils.density); MIN_FILTER_ICON_WIDTH=(int)(10.0F*DrawUtils.density); DEFAULT_TYPE=KmoIconSet.IconSetTypes.$3TrafficLights1; }
    pub(crate) fn init() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let density = super::super::DrawUtils::density.get();
            MAX_FILTER_ICON_WIDTH.store((30.0 * density) as i32, Ordering::Relaxed);
            MIN_FILTER_ICON_WIDTH.store((10.0 * density) as i32, Ordering::Relaxed);
        });
    }
    // java public boolean reset(IGridSheet gridSheet, int rowIndex, int colIndex) {
    // java CfIconSetValue iconSetValue = this.getIconSet(gridSheet, rowIndex, colIndex);
    // java if (iconSetValue == null) {
    // java return false;
    // java } else {
    // java KmoIconSet.IconSetTypes type = iconSetValue.getType();
    // java if (type == null) {
    // java type = DEFAULT_TYPE;
    // java }
    // java
    // java int iconIndex = iconSetValue.getIndex();
    // java String iconName = this.getIconName(type.name(), iconIndex);
    // java this.mIconBitmap = RenderDrawablePool.getIconBitmap(iconName);
    // java return true;
    // java }
    // java }
    pub fn reset(&mut self, g: &dyn IGridSheet, row: i32, col: i32) -> bool {
        Self::init();
        let core = g.coreSheet();
        let s = core.borrow();
        let Some(icon) = s.getCFormatter().getIconSet(row, col) else {
            return false;
        };
        let kind = icon.getType().unwrap_or(IconSetTypes::I3TrafficLights1);
        let name = Self::getIconName(kind.name(), icon.getIndex());
        self.mIconBitmap = RenderDrawablePool::getIconBitmap(&name);
        true
    }
    // java private String getIconName(String typeName, int index) {
    // java StringBuffer buffer = new StringBuffer();
    // java buffer.append("et_iconset_");
    // java buffer.append(typeName.substring(1));
    // java buffer.append("_");
    // java buffer.append(index);
    // java buffer.append("_bmw");
    // java return buffer.toString().toLowerCase();
    // java }
    fn getIconName(name: &str, index: i32) -> String {
        format!("et_iconset_{}_{}_bmw", &name[1..], index).to_lowercase()
    }
    // java public void drawIcon(Canvas canvas, Paint paint, Rect rect) {
    // java int width = MAX_FILTER_ICON_WIDTH;
    // java int cellHeight = rect.height();
    // java if (cellHeight < width) {
    // java if (cellHeight < MIN_FILTER_ICON_WIDTH) {
    // java return;
    // java }
    // java
    // java width = cellHeight;
    // java }
    // java
    // java int space = (rect.bottom - rect.top - width) / 2;
    // java Rect drawRect = new Rect(rect.left, rect.top + space, rect.left + width, rect.top + space + width);
    // java if (this.mIconBitmap != null) {
    // java canvas.drawBitmap(this.mIconBitmap, (Rect)null, drawRect, paint);
    // java }
    // java
    // java }
    pub fn drawIcon(&self, c: &mut Canvas, p: &Paint, r: &Rect) {
        let mut width = MAX_FILTER_ICON_WIDTH.load(Ordering::Relaxed);
        let h = r.height();
        if h < width {
            if h < MIN_FILTER_ICON_WIDTH.load(Ordering::Relaxed) {
                return;
            }
            width = h;
        }
        let space = (r.bottom - r.top - width) / 2;
        let dst = Rect::new(r.left, r.top + space, r.left + width, r.top + space + width);
        if let Some(bitmap) = &self.mIconBitmap {
            c.drawBitmap(&bitmap.name, &dst, p);
        }
    }
}
