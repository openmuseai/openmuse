// PageRender - faithful clone of pdf-lib's renderer (natives bind to the ORIGINAL .so).
package cn.wps.pdf.lib.render;

import android.graphics.Bitmap;
import android.graphics.RectF;

public class PageRender {
    public static final class State {
        public static final int ERROR = -1;
        public static final int FINISHED = 3;
    }
    public static final class RenderOption {
        public static final int NONE = 0;
    }

    private long mHandle;

    public PageRender(long pageHandle) {
        mHandle = nCreate(pageHandle, false);
    }

    public int state() { return mHandle == 0 ? State.ERROR : nState(mHandle); }

    public int start(Bitmap bmp, RectF display, RectF clip) {
        return nStart(mHandle, bmp, display, clip, RenderOption.NONE, 0);
    }

    public int continueRender(long millis, long stoper, Bitmap bitmap) {
        return nContinue(mHandle, millis, stoper, bitmap);
    }

    public int dispose() {
        int ret = nDispose(mHandle);
        mHandle = 0;
        return ret;
    }

    public static native long nCreate(long pageHandle, boolean isExport);
    public static native int nState(long handle);
    public static native int nStart(long handle, Bitmap bmp, RectF display, RectF clip,
                                    int coreRenderOption, int otherOption);
    public static native int nContinue(long handle, long millis, long stoper, Bitmap bitmap);
    public static native int nDispose(long handle);
}
