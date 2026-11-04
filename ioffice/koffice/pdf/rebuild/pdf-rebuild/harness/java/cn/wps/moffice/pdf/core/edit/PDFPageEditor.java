package cn.wps.moffice.pdf.core.edit;

import android.graphics.RectF;

/** Minimal exact-FQCN surface for the original page-selection ABI. */
public final class PDFPageEditor {
    public native boolean native_hasSelection(long page);
    public native RectF[] native_selectionRects(long page);
    public native Object[] native_selectionRectsEx(long page);
    public native int native_startSelection(long page);
    public native int native_endSelection(long page);
}
