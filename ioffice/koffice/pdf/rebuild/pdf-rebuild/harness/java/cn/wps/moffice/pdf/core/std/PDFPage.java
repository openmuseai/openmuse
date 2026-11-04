// PDFPage stub - page loading/parse natives for the ORIGINAL render path.
package cn.wps.moffice.pdf.core.std;

public class PDFPage {
    public static final int LOAD_STATE_FINISHED = 2;  // PDFium FPDF_RENDER_DONE family

    private long mNativePdfPage;

    public PDFPage(long nativePdfPage) {
        mNativePdfPage = nativePdfPage;
    }

    /** replicate WPS PDFPage.parsePage(true): start + loop continueLoading till done */
    public void parsePageBlocked() {
        if (native_getLoadState(mNativePdfPage) == LOAD_STATE_FINISHED) return;
        native_startLoading(mNativePdfPage);
        int guard = 0;
        while (native_getLoadState(mNativePdfPage) != LOAD_STATE_FINISHED && guard++ < 500) {
            native_continueLoading(mNativePdfPage, Integer.MAX_VALUE);
        }
    }

    public long getHandle() { return mNativePdfPage; }

    public android.graphics.RectF getPageSize() {
        android.graphics.RectF r = new android.graphics.RectF();
        native_getPageSize(mNativePdfPage, r);
        return r;
    }

    public static native void native_getPageSize(long handle, android.graphics.RectF pageSize);
    public native int native_getDisplayMatirx(long handle, float[] matrix,
                                               android.graphics.RectF displayRect, int rotation);
    public native boolean native_isValid(long handle);
    public native int native_getPageIndex(long handle);
    public native int native_unloadPage2(long handle);
    public native void native_reloadTextPage(long handle);
    public native void native_regenerateContent(long handle);
    public native boolean native_IsModified(long handle);
    public native boolean native_CheckFuncModified(long handle, int editStatus);
    public native int native_getTextEditorCount(long handle);
    public native boolean native_getTextEditorRect(long handle, int index,
                                                    android.graphics.RectF rect,
                                                    float[] matrix, boolean reverse);
    public native void native_onAfterLoadPage(long handle);
    public native boolean native_SectionRecognize(long handle);
    public native boolean native_OnLButtonDown(long handle, float x, float y, int flags);
    public native boolean native_OnLButtonDoubleClick(long handle, float x, float y);
    public native boolean native_OnLButtonTripleClick(long handle, float x, float y);
    public native boolean native_OnLButtonUp(long handle, float x, float y);
    public native boolean native_OnMouseMove(long handle, float x, float y, int flags);
    public native int native_GetHoverType(long handle, float x, float y);
    public native int native_GetFocusType(long handle);
    public native int native_GetFocusStatus(long handle);
    public native int native_GetFocusObjectCount(long handle);
    public native boolean native_GetFocusObjectRect(long handle, android.graphics.RectF rect);
    public native boolean native_SetFontSize(long handle, float size);
    public native float native_GetFontSize(long handle);
    public native boolean native_SetFontName(long handle, String fontName);
    public native String native_GetFontName(long handle);
    public native boolean native_setTextBold(long handle, boolean bold);
    public native boolean native_isTextBold(long handle);
    public native boolean native_SetTextColor(long handle, int color);
    public native int native_GetTextColor(long handle);
    public native boolean native_setAlignment(long handle, int alignment);
    public native int native_getAlignment(long handle);
    public native boolean native_setLeading(long handle, float leading);
    public native float native_getLeading(long handle);
    public native boolean native_setLineIndent(long handle, boolean increase);
    public native int native_GetTextLength(long handle);
    public native String native_GetText(long handle);
    public native int native_GetEditAllTextLength(long handle);
    public native String native_GetEditAllText(long handle);
    public native String native_GetEditSelText(long handle);
    public native int[] native_GetTextSelectRange(long handle);
    public native boolean native_SetTextSelectRange(long handle, int start, int end);
    public native int native_GetRangeIndexAtPos(long handle, float x, float y);
    public native boolean native_SetCaret(long handle, int position);
    public native int native_GetCaret(long handle);
    public native void native_GetCaretInfo(long handle, android.graphics.RectF rect);
    public native android.graphics.PointF[] native_GetCaretPoints(long handle);
    public native void native_GetEditSelectionRect(long handle, int index,
                                                    android.graphics.RectF rect);
    public native float[] native_GetEditSelectionQuadRect(long handle, int index);
    public native boolean native_OnChars(long handle, String chars, boolean preEdit, int cursor);
    public native boolean native_OnChar(long handle, int character, int flags);
    public native boolean native_OnKeyDown(long handle, int keyCode, int flags);
    public native long[] native_getImages(long handle);
    public native long native_getImageObjectAtPoint(long handle, float x, float y);
    public native void native_getImageRect(long handle, long image, android.graphics.RectF rect);
    public native boolean native_removeImageFromPage(long handle, long image);
    public native int native_resizeImageRect(long handle, long image, android.graphics.RectF rect);
    public native int native_rotateImageClockwise(long handle, long image);
    public native int native_reverseImageHorizontal(long handle, long image);
    public native int native_getImageDegree(long handle, long image);
    public native boolean native_setImageDegree(long handle, long image, int degree);
    public native boolean native_setImageOpacity(long handle, long image, float opacity);
    public native float native_getImageOpacity(long handle, long image);
    public native long native_getPathObjectAtPoint(long handle, float x, float y);
    public native void native_getPathRect(long handle, long path, android.graphics.RectF rect);
    public native long native_FindImageObjectWithPoint(long handle, float x, float y);
    public native long native_FindImageObjectWithImageIndex(long handle, int imageIndex);
    public native boolean native_GetImageRectFromPage(long handle, long image,
                                                       android.graphics.RectF rect);
    public native int native_GetImageIndex(long handle, long image);

    public static native int native_startLoading(long handle);
    public static native int native_continueLoading(long handle, long milliseconds);
    public static native int native_getLoadState(long handle);
    public static native int native_stopLoading(long handle);
}
