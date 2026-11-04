package cn.wps.moffice.pdf.core.formfill;

public final class PDFFormFill {
    private final long form;
    private final long page;

    public PDFFormFill(long form, long page) {
        this.form = form;
        this.page = page;
    }

    public long editorHandle() { return native_getFromHandle(form); }
    public boolean click(float x, float y) {
        native_onMouseMove(form, page, x, y);
        boolean result = native_onLButtonDown(form, page, x, y);
        return native_onLButtonUp(form, page, x, y) || result;
    }
    public boolean onChars(String value) { return native_onChars(form, page, value); }

    private native boolean native_onLButtonDown(long form, long page, float x, float y);
    private native boolean native_onLButtonUp(long form, long page, float x, float y);
    private native boolean native_onMouseMove(long form, long page, float x, float y);
    private native boolean native_onChars(long form, long page, String value);
    private native long native_getFromHandle(long form);

    public static native void native_undoEdit(long handle);
    public static native void native_redoEdit(long handle);
    public static native void native_deleteTransEdit(long handle);
}
