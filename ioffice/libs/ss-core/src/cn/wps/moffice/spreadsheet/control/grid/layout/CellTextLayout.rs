use super::{
    data::CellTextData::CellTextData,
    typo::{
        AngledTextLayout2::AngledTextLayout2, CellRichTextLayout::CellRichTextLayout,
        SimpleTextLayout::SimpleTextLayout,
    },
    ICellTextLayout::ICellTextLayout,
};
// java public final class CellTextLayout
pub struct CellTextLayout {
    mTextData: Option<CellTextData>,
    mTextLayouts: [Option<Box<dyn ICellTextLayout>>; 5],
}
impl Default for CellTextLayout {
    fn default() -> Self {
        Self {
            mTextData: Some(CellTextData::default()),
            mTextLayouts: std::array::from_fn(|_| None),
        }
    }
}
impl CellTextLayout {
    // java private ICellTextLayout getTextLayout(CellTextData data) {
    // java if (data.isVerticalText()) {
    // java return this.getInstance(VERTICAL_TEXT_LAYOUT);
    // java } else if (data.isAngledText()) {
    // java return this.getInstance(ANGLED_TEXT_LAYOUT);
    // java } else {
    // java switch (data.mCellType) {
    // java case 1:
    // java case 5:
    // java case 6:
    // java return this.getInstance(SIMPLE_TEXT_LAYOUT);
    // java case 2:
    // java if (this.getDLanguageTypo(data)) {
    // java return this.getInstance(THAI_RICH_TEXT_LAYOUT);
    // java }
    // java
    // java return this.getInstance(RICH_TEXT_LAYOUT);
    // java case 3:
    // java case 4:
    // java default:
    // java return this.getInstance(SIMPLE_TEXT_LAYOUT);
    // java }
    // java }
    // java }
    pub fn getTextLayout(&mut self, d: &CellTextData) -> &mut dyn ICellTextLayout {
        let id = if d.isVerticalText() {
            3
        } else if d.isAngledText() {
            2
        } else if d.mCellType == 2 {
            if d.source().iter().any(|c| (0x0e00..=0x0e7f).contains(c)) {
                4
            } else {
                1
            }
        } else {
            0
        };
        self.getInstance(id)
    }
    // java private ICellTextLayout getInstance(int id) {
    // java ICellTextLayout textLayout = this.mTextLayouts[id];
    // java if (textLayout == null) {
    // java try {
    // java textLayout = (ICellTextLayout)LAYOUT_CLASSES[id].newInstance();
    // java this.mTextLayouts[id] = textLayout;
    // java } catch (Exception e) {
    // java e.printStackTrace();
    // java }
    // java }
    // java
    // java return textLayout;
    // java }
    fn getInstance(&mut self, id: usize) -> &mut dyn ICellTextLayout {
        if self.mTextLayouts[id].is_none() {
            self.mTextLayouts[id] = Some(match id {
                0 => Box::new(SimpleTextLayout::default()),
                1 => Box::new(CellRichTextLayout::default()),
                2 => Box::new(AngledTextLayout2::default()),
                3 => panic!("Unported VerticalTextLayout"),
                4 => panic!("Unported ThaiRichTextLayout"),
                _ => unreachable!(),
            });
        }
        self.mTextLayouts[id].as_mut().unwrap().as_mut()
    }
    // java public boolean layout(GridSheetLayoutInfo layoutInfo, Canvas canvas, Rect rect, int rowIdx, int colIdx, boolean forceTypo, boolean hasCFRule, CellXFCache cellXfCache, TableStyle.FontStyle fontStyle) {
    // java int width = rect.width();
    // java int height = rect.height();
    // java if (!this.mTextData.updateData(layoutInfo, width, height, rowIdx, colIdx, hasCFRule, cellXfCache)) {
    // java return false;
    // java } else {
    // java this.updateFontStyle(this.mTextData, fontStyle);
    // java ICellTextLayout layout = this.getTextLayout(this.mTextData);
    // java layout.typo(layoutInfo, this.mTextData, width, height, forceTypo);
    // java if (forceTypo || layout.getTextRect().width() > width) {
    // java canvas.translate((float)rect.left, (float)rect.top);
    // java canvas.save();
    // java if (layoutInfo.isSheetRightToLeft()) {
    // java if (!BidiTools.needRtl(this.mTextData.mSourceText)) {
    // java canvas.translate((float)(width - layout.getTextRect().width()), 0.0F);
    // java } else {
    // java canvas.translate((float)(layout.getTextRect().width() - width), 0.0F);
    // java }
    // java }
    // java
    // java layoutInfo.convertRtl(canvas, layout.getTextRect());
    // java layout.renderer(canvas);
    // java canvas.restore();
    // java canvas.translate((float)(-rect.left), (float)(-rect.top));
    // java }
    // java
    // java this.mTextData.clear();
    // java return true;
    // java }
    // java }
    pub fn layout(
        &mut self,
        l: &super::super::core::GridSheetLayoutInfo::GridSheetLayoutInfo,
        c: &mut super::super::render::graphics::Canvas,
        r: &super::super::render::graphics::Rect,
        row: i32,
        col: i32,
        force: bool,
        hasCF: bool,
        cache: Option<&super::cache::CellXFCache::CellXFCache>,
        fontStyle: Option<&dyn super::super::TableStyle::FontStyle>,
    ) -> bool {
        let mut data = self.mTextData.take().expect("CellTextLayout destroyed");
        let width = r.width();
        let height = r.height();
        if !data.updateData(l, width, height, row, col, hasCF, cache) {
            self.mTextData = Some(data);
            return false;
        }
        if let Some(style) = fontStyle {
            if !style.isEmpty() {
                style.replace(&mut data.mFirstSpan);
                for span in &mut data.mOtherSpans {
                    style.replace(span);
                }
            }
        }
        let tl = self.getTextLayout(&data);
        tl.typo(l, &mut data, width, height, force);
        let textRect = tl.getTextRect();
        if force || textRect.width() > width {
            c.translate(r.left as f32, r.top as f32);
            c.save();
            if l.isSheetRightToLeft() {
                if !super::layouter::BidiTools::BidiTools::needRtl(data.source()) {
                    c.translate((width - textRect.width()) as f32, 0.0);
                } else {
                    c.translate((textRect.width() - width) as f32, 0.0);
                }
            }
            l.convertRtl(c, &textRect);
            tl.renderer(c);
            c.restore();
            c.translate(-r.left as f32, -r.top as f32);
        }
        data.clear();
        self.mTextData = Some(data);
        true
    }
}
