// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.RectF
use util::cn::wps::graphics::RectF;
// Java: import cn.wps.text.layout.data.TextBoxData
use wpslayout::cn::wps::text::layout::data::TextBoxData;
// Java: import cn.wps.text.layout.typo.ArtTransform
use wpslayout::cn::wps::text::layout::typo::ArtTransform;

// Java: public interface ITextLayout extends ITextEdit
pub trait ITextLayout {
    // Java: abstract int id()
    fn id(&self) -> i32;

    // Java: abstract TextBoxData getTextBoxData()
    fn getTextBoxData(&self) -> TextBoxData;

    // Java: abstract void typo(TextBoxData var1, TypoParam var2)
    fn typo(&self, TextBoxData var1, TypoParam var2);

    // Java: abstract void typo(TextBoxData var1, float var2, float var3, boolean var4)
    fn typo(&self, TextBoxData var1, float var2, float var3, boolean var4);

    // Java: abstract RectF getTextRect()
    fn getTextRect(&self) -> RectF;

    // Java: abstract void renderText(Canvas var1)
    fn renderText(&self, Canvas var1);

    // Java: abstract void renderText(Canvas var1, int var2, int var3)
    fn renderText(&self, Canvas var1, int var2, int var3);

    // Java: abstract void clear()
    fn clear(&self);

    // Java: abstract void destroy()
    fn destroy(&self);

    // Java: abstract int state()
    fn state(&self) -> i32;

    // Java: abstract int[] getTextLayoutPos(int var1)
    fn getTextLayoutPos(&self, int var1) -> int[];

    // Java: abstract int getTextGlobalPos(int var1, int var2, int var3)
    fn getTextGlobalPos(&self, int var1, int var2, int var3) -> i32;

    // Java: abstract RectF getTextDrawArea()
    fn getTextDrawArea(&self) -> RectF;

    // Java: abstract ArtTransform getArtTransform()
    fn getArtTransform(&self) -> ArtTransform;

}
