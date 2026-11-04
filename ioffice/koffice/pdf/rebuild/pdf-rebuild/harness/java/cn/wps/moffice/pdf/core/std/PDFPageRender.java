// PDFPageRender - stub matching WPS's class so the ORIGINAL libkwopdf.so render
// natives bind (same FQCN + method names + signature).
package cn.wps.moffice.pdf.core.std;

import android.graphics.Bitmap;

public class PDFPageRender {
    public static final int RENDER_NOMAL_TEXT_SMOOTH_NORMAL = 0x00001000;

    public long mNativeRender;

    public long createRendering(long pageHandle, boolean isExport) {
        mNativeRender = native_create(pageHandle, isExport);
        return mNativeRender;
    }

    public int startRendering(Bitmap bm, float l, float t, float r, float b,
                              float clipL, float clipT, float clipR, float clipB) {
        return native_startRendering(mNativeRender, bm, l, t, r, b,
                clipL, clipT, clipR, clipB, RENDER_NOMAL_TEXT_SMOOTH_NORMAL, false, false);
    }

    public int continueRendering(int millis, long stoper, Bitmap bitmap) {
        return native_continueRendering(mNativeRender, millis, stoper, bitmap);
    }

    public int close() {
        int ret = native_closeRendering(mNativeRender);
        mNativeRender = 0;
        return ret;
    }

    public int getState() {
        return native_getState(mNativeRender);
    }

    // native surface (exact WPS declarations)
    public native long native_create(long page, boolean isExport);
    public native int native_getState(long handle);
    public native int native_startRendering(long handle, Bitmap bitmap,
                                            float left, float top, float right, float bottom,
                                            float clipLeft, float clipTop, float clipRight,
                                            float clipBottom, int option,
                                            boolean isNightMode, boolean isRenderComment);
    public native int native_continueRendering(long handle, long millis, long stoper, Bitmap bitmap);
    public native int native_closeRendering(long handle);
}