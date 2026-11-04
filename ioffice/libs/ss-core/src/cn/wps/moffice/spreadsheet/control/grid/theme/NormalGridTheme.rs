use super::super::layout::{data::SpanInfo::SpanInfo, typo::PaintFontHelper::PaintFontHelper};
use super::DefaultGridTheme::DefaultGridTheme;
use crate::cn::wps::moss::app::{IGridSheet::IGridSheet, UnitsConverter::UnitsConverter};
// java public class NormalGridTheme extends DefaultGridTheme
pub struct NormalGridTheme {
    pub base: DefaultGridTheme,
}
impl std::ops::Deref for NormalGridTheme {
    type Target = DefaultGridTheme;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl std::ops::DerefMut for NormalGridTheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl NormalGridTheme {
    // java public NormalGridTheme(IGridSheet gridSheet, UnitsConverter converter) {
    // java super(converter);
    // java }
    pub fn new(_g: &dyn IGridSheet, c: &UnitsConverter) -> Self {
        Self {
            base: DefaultGridTheme::new(c),
        }
    }
    // java protected void setNormalText(IGridSheet gridSheet, UnitsConverter converter) {
    // java KmoXFManager xfManager = gridSheet.getXFMgr();
    // java if (xfManager != null) {
    // java KmoFont font = xfManager.getNormalStyle().getFont();
    // java PaintFontHelper.setNormalCharWidthTwip(xfManager, this.mNormalTextPaint);
    // java SpanInfo fontInfo = new SpanInfo();
    // java fontInfo.reset(converter, gridSheet.getPalette(), font, 0, 0);
    // java PaintFontHelper.setPaintForText(fontInfo, this.mNormalTextPaint);
    // java xfManager.setNormalCharWidth(this.getNormalCharWidth());
    // java this.mNormalFontSize = fontInfo.mFontSize;
    // java } else {
    // java super.setNormalText(gridSheet, converter);
    // java }
    // java
    // java }
    pub fn setNormalText(&mut self, g: &dyn IGridSheet, c: &UnitsConverter) {
        if let Some(mgr) = g.getXFMgr() {
            let mgr = mgr.borrow();
            let font = mgr
                .getNormalStyle()
                .expect("Normal style")
                .getFont()
                .expect("Normal font");
            PaintFontHelper::setNormalCharWidthTwip(&mgr, &mut self.mNormalTextPaint);
            let mut info = SpanInfo::default();
            info.reset(c, &g.getPalette(), &font, 0, 0);
            PaintFontHelper::setPaintForText(&info, &mut self.mNormalTextPaint, true);
            mgr.setNormalCharWidth(self.getNormalCharWidth());
            self.mNormalFontSize = info.mFontSize;
        } else {
            self.base.setNormalText(g, c);
        }
    }
    // java public final void resetTheme(IGridSheet gridSheet, UnitsConverter converter) {
    // java this.setNormalText(gridSheet, converter);
    // java this.setHeaderText(converter.getUnscaleZoom());
    // java }
    pub fn resetTheme(&mut self, g: &dyn IGridSheet, c: &UnitsConverter) {
        self.setNormalText(g, c);
        self.setHeaderText(c.getUnscaleZoom());
    }
}
