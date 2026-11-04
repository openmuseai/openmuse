package cn.wps.moffice.pdf.core.select;

import android.graphics.RectF;
import java.util.ArrayList;

/** Minimal exact-FQCN JNI contract used by host and device probes. */
public class PDFTextPage {
    public native long native_new(long page);
    public native int native_getUnicode(long textPage, int index);
    public native String native_getText(long textPage, int start, int end);
    public native String native_getPageText(long textPage);
    public native int native_getCPAtPoint(long textPage, float x, float y,
                                          float xTolerance, float yTolerance);
    public native int native_getCharRangeIndex(long textPage, float left, float top,
                                               float right, float bottom,
                                               int[] position, boolean isExport);
    public native int native_countRects(long textPage, int start, int end);
    public native int native_getRect(long textPage, int index, RectF result);
    public native ArrayList<String> native_getText2(long textPage, int start, int end);
    public native void native_getPageTextAndRects(long textPage,
                                                   ArrayList<String> text,
                                                   ArrayList<RectF> rects);
}
