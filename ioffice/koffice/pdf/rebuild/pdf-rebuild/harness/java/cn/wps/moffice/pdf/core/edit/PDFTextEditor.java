package cn.wps.moffice.pdf.core.edit;

import android.graphics.RectF;

public final class PDFTextEditor {
    public native void native_loadFont(String filePath);
    public native boolean native_setReplaceDisplayFont(String originalName,
                                                        String replacementName);
    public native void native_addSupportReplaceFont(String originalName);
    public native int native_getSelDirection(long handle);
    public native boolean native_isEditing(long handle);
    public native int native_length(long handle);
    public native boolean native_setCursor(long handle, int position);
    public native int native_cursor(long handle);
    public native String native_text(long handle, int start, int end);
    public native boolean native_getCaretInfo(long handle, RectF rect, boolean handleRotation);
    public native String native_fontName(long handle, long pageHandle);
    public native boolean native_setFontName(long handle, long pageHandle, String name);
}
