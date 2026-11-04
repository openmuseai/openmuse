package cn.wps.rebuild;

import android.graphics.RectF;

/** Test-only raw-PDFium surface. Production WPS JNI uses opaque registry handles. */
public final class FreeTextStub {
    private FreeTextStub() {}

    public static native boolean native_addFreeText(long doc, long page, float left, float top);
    public static native void native_setFreeTextColor(long page, int color);
    public static native int native_getFreeTextColor(long page);
    public static native void native_setSelectFontSize(long page, float size);
    public static native float native_getSelectFontSize(long page);
    public static native void native_setFreeTextRect(long page, RectF rect);
    public static native void native_getFreeTextRect(long page, long annotation, RectF rect);
}
