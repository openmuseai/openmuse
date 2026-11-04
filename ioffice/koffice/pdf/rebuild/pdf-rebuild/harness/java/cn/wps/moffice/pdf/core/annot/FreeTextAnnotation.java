// Rebuilt native surface with the EXACT WPS FQCN + native method names, so the
// rebuilt dylib (libkspfreetext.dylib) binds via default JNI name resolution.
package cn.wps.moffice.pdf.core.annot;

import android.graphics.RectF;

public class FreeTextAnnotation {
    // WPS public API (subset used by the spike), driven natively:
    public native int native_getFreeTextColor(long jpageHandle);
    public native void native_setFreeTextColor(long jpageHandle, int color);
    public native float native_getSelectFontSize(long jpageHandle);
    public native void native_setSelectFontSize(long jpageHandle, float fontSize);
    public native void native_setFreeTextRect(long jpageHandle, RectF rect);
    public native void native_getFreeTextRect(long jpageHandle, long handle, RectF rect);
}